use anyhow::{anyhow, Context, Result};
use git2::ErrorCode;
use std::path::Path;

pub fn run(args: &[String]) -> Result<()> {
    let repo = git2::Repository::discover(".").context("Failed to discover repository")?;
    let workdir = repo
        .workdir()
        .context("Repository has no working directory")?;
    let mut index = repo.index().context("Failed to open repository index")?;
    let mut any_change = false;

    // Skip the applet name (argv[0])
    let paths: Vec<&String> = args.iter().skip(1).collect();

    if paths.is_empty() {
        println!("GitRm requires exactly one explicit path.");
        return Ok(());
    }

    if paths.len() > 1 {
        println!("GitRm only accepts a single explicit path for safety.");
        return Ok(());
    }

    let path_str = paths[0];

    if path_str.contains('*') || path_str.contains('?') || path_str.contains('[') || path_str.contains(']') {
        println!(
            "GitRm does not accept glob patterns. Found disallowed characters in '{}'.",
            path_str
        );
        return Ok(());
    }

    let path = Path::new(path_str).to_path_buf();

    // Resolve absolute path
    let abs_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()?.join(&path)
    };

    // Skip if it's the .git directory or inside it
    if abs_path.components().any(|c| c.as_os_str() == ".git") {
        println!("Access denied: Path is inside .git directory.");
        return Ok(());
    }

    // Skip symlinks entirely for now
    if let Ok(metadata) = std::fs::symlink_metadata(&abs_path) {
        if metadata.file_type().is_symlink() {
            println!("Skipping symlink: {}", path.display());
            return Ok(());
        }
    }

    // Canonicalize to match workdir prefix safely
    let canonical_abs = match abs_path.canonicalize() {
        Ok(p) => p,
        Err(_) => {
            // If it doesn't exist, we still try to remove it from index if it's relative to workdir
            abs_path.to_path_buf()
        }
    };

    if let Ok(rel_path) = canonical_abs.strip_prefix(workdir) {
        let rel_path = rel_path.to_path_buf();
        // Remove from disk
        if canonical_abs.exists() {
            if canonical_abs.is_dir() {
                if let Err(e) = std::fs::remove_dir_all(&canonical_abs) {
                    eprintln!(
                        "Warning: Failed to delete directory {}: {}",
                        canonical_abs.display(),
                        e
                    );
                } else {
                    println!("Deleted directory: {}", rel_path.display());
                    any_change = true;
                }
                match index.remove_dir(rel_path.as_path(), 0) {
                    Ok(_) => any_change = true,
                    Err(e) if e.code() != ErrorCode::NotFound => {
                        eprintln!(
                            "Warning: Failed to remove directory {} from index: {}",
                            rel_path.display(),
                            e
                        );
                    }
                    _ => {}
                }
            } else {
                if let Err(e) = std::fs::remove_file(&canonical_abs) {
                    eprintln!(
                        "Warning: Failed to delete file {}: {}",
                        canonical_abs.display(),
                        e
                    );
                } else {
                    println!("Deleted: {}", rel_path.display());
                    any_change = true;
                }
                match index.remove(rel_path.as_path(), 0) {
                    Ok(_) => any_change = true,
                    Err(e) if e.code() != ErrorCode::NotFound => {
                        eprintln!(
                            "Warning: Failed to remove file {} from index: {}",
                            rel_path.display(),
                            e
                        );
                    }
                    _ => {}
                }
            }
        } else {
            // Check if path exists in index as a file
            let is_file_in_index = index.get_path(rel_path.as_path(), 0).is_some();

            // Check if path exists in index as a directory (prefix)
            let is_dir_in_index = index.find_prefix(rel_path.as_path()).is_ok();

            if is_file_in_index {
                if let Err(e) = index.remove(rel_path.as_path(), 0) {
                    eprintln!(
                        "Warning: Failed to remove file {} from index: {}",
                        rel_path.display(),
                        e
                    );
                } else {
                    println!("Removed from index: {}", rel_path.display());
                    any_change = true;
                }
            }

            if is_dir_in_index {
                if let Err(e) = index.remove_dir(rel_path.as_path(), 0) {
                    eprintln!(
                        "Warning: Failed to remove directory {} from index: {}",
                        rel_path.display(),
                        e
                    );
                } else {
                    if !is_file_in_index {
                        println!("Removed directory from index: {}", rel_path.display());
                    }
                    any_change = true;
                }
            }

            if !is_file_in_index && !is_dir_in_index {
                println!("No matching path found: {}", rel_path.display());
            }
        }
    } else {
        eprintln!("Path is outside of repository: {}", path.display());
    }

    if any_change {
        index.write().context("Failed to write index to disk")?;
        Ok(())
    } else {
        Err(anyhow!("No matching paths found."))
    }
}
