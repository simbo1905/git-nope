use anyhow::{Context, Result};
use std::path::Path;

pub fn run(args: &[String]) -> Result<()> {
    let repo = git2::Repository::discover(".").context("Failed to discover repository")?;

    // Skip the applet name (argv[0])
    let paths: Vec<&String> = args.iter().skip(1).collect();

    if paths.is_empty() {
        println!("GitAdd requires explicit paths. Use GitAddDot to stage '.' or GitAddAll for all changes.");
        return Ok(());
    }

    let mut index = repo.index().context("Failed to open repository index")?;
    let workdir = repo
        .workdir()
        .context("Repository has no working directory")?;

    for path_str in paths {
        let path = Path::new(path_str);

        // Resolve absolute path to handle relative paths from CWD correctly
        let abs_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir()?.join(path)
        };

        // Canonicalize to resolve .. and symlinks, but note that git2 expects
        // paths relative to the workdir root.
        let canonical_abs = abs_path
            .canonicalize()
            .with_context(|| format!("Failed to resolve path: {}", path_str))?;

        // Strip the workdir prefix to get the relative path within the repo
        let rel_path = canonical_abs.strip_prefix(workdir).with_context(|| {
            format!(
                "Path is outside of repository working directory: {}",
                path_str
            )
        })?;

        index
            .add_path(rel_path)
            .with_context(|| format!("Failed to add path to index: {}", path_str))?;
        println!("Staged: {}", rel_path.display());
    }

    index.write().context("Failed to write index to disk")?;
    Ok(())
}
