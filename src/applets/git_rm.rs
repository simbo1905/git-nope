use anyhow::{Context, Result, bail};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub fn run(args: &[String]) -> Result<()> {
    let repo = git2::Repository::discover(".").context("Failed to discover repository")?;
    let workdir = repo.workdir().context("Repository has no working directory")?;
    let mut index = repo.index().context("Failed to open repository index")?;

    // Skip the applet name (argv[0])
    let patterns: Vec<&String> = args.iter().skip(1).collect();

    if patterns.is_empty() {
        println!("GitRm requires explicit paths or glob patterns.");
        return Ok(());
    }

    let mut paths_to_remove = HashSet::new();

    for pattern_str in patterns {
        // Treat as glob pattern
        match glob::glob(pattern_str) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(path) => {
                            paths_to_remove.insert(path);
                        }
                        Err(e) => eprintln!("Error matching pattern '{}': {}", pattern_str, e),
                    }
                }
            }
            Err(e) => {
                // If it's not a valid glob pattern, try it as a literal path
                let path = Path::new(pattern_str);
                if path.exists() {
                    paths_to_remove.insert(path.to_path_buf());
                } else {
                    eprintln!("Invalid pattern or non-existent path '{}': {}", pattern_str, e);
                }
            }
        }
    }

    if paths_to_remove.is_empty() {
        bail!("No files matched the provided patterns.");
    }

    // Sort paths by length descending to handle nested structures correctly (files before parent dirs)
    let mut sorted_paths: Vec<PathBuf> = paths_to_remove.into_iter().collect();
    sorted_paths.sort_by(|a, b| b.as_os_str().len().cmp(&a.as_os_str().len()));

    for path in sorted_paths {
        // Resolve absolute path
        let abs_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir()?.join(&path)
        };

        // Skip if it's the .git directory or inside it
        if abs_path.components().any(|c| c.as_os_str() == ".git") {
            continue;
        }

        // Skip symlinks entirely for now
        if let Ok(metadata) = std::fs::symlink_metadata(&abs_path) {
            if metadata.file_type().is_symlink() {
                println!("Skipping symlink: {}", path.display());
                continue;
            }
        }

        // Canonicalize to match workdir prefix safely
        // Note: canonicalize might fail if the file was already deleted by a parent directory removal
        let canonical_abs = match abs_path.canonicalize() {
            Ok(p) => p,
            Err(_) => {
                // If it doesn't exist, we still try to remove it from index if it's relative to workdir
                abs_path.to_path_buf()
            }
        };

        if let Ok(rel_path) = canonical_abs.strip_prefix(workdir) {
            // Remove from disk
            if canonical_abs.exists() {
                if canonical_abs.is_dir() {
                    if let Err(e) = std::fs::remove_dir_all(&canonical_abs) {
                        eprintln!("Warning: Failed to delete directory {}: {}", canonical_abs.display(), e);
                    } else {
                        println!("Deleted directory: {}", rel_path.display());
                    }
                    // Remove from index
                    if let Err(e) = index.remove_dir(rel_path, 0) {
                        if e.code() != git2::ErrorCode::NotFound {
                            eprintln!("Warning: Failed to remove directory {} from index: {}", rel_path.display(), e);
                        }
                    }
                } else {
                    if let Err(e) = std::fs::remove_file(&canonical_abs) {
                        eprintln!("Warning: Failed to delete file {}: {}", canonical_abs.display(), e);
                    } else {
                        println!("Deleted: {}", rel_path.display());
                    }
                    // Remove from index
                    if let Err(e) = index.remove(rel_path, 0) {
                        if e.code() != git2::ErrorCode::NotFound {
                            eprintln!("Warning: Failed to remove file {} from index: {}", rel_path.display(), e);
                        }
                    }
                }
            } else {
                // Already gone from disk, just ensure it's gone from index
                let _ = index.remove(rel_path, 0);
                let _ = index.remove_dir(rel_path, 0);
                println!("Removed from index: {}", rel_path.display());
            }
        } else {
            eprintln!("Path is outside of repository: {}", path.display());
        }
    }

    index.write().context("Failed to write index to disk")?;
    Ok(())
}
