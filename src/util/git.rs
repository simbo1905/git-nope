use anyhow::{Context, Result};
use git2::{Branch, Repository};

pub fn discover_repo() -> Result<Repository> {
    Repository::discover(".").context("Failed to discover repository")
}

pub fn head_branch(repo: &Repository) -> Result<Branch<'_>> {
    let head = repo.head().context("Failed to get HEAD")?;
    if !head.is_branch() {
        anyhow::bail!("HEAD is detached (not supported for this operation)");
    }
    Ok(Branch::wrap(head))
}

pub fn head_branch_name(repo: &Repository) -> Result<String> {
    let branch = head_branch(repo)?;
    let name = branch
        .name()?
        .unwrap_or("unknown")
        .to_string();
    Ok(name)
}

pub fn short_head_commit(repo: &Repository, len: usize) -> Result<String> {
    let head = repo.head().context("Failed to get HEAD")?;
    let commit = head.peel_to_commit()?;
    let id = commit.id();
    let s = id.to_string();
    if s.len() > len {
        Ok(s[..len].to_string())
    } else {
        Ok(s)
    }
}

pub fn upstream_remote_url(repo: &Repository, branch: &Branch) -> Result<Option<String>> {
    let upstream = match branch.upstream() {
        Ok(b) => b,
        Err(_) => return Ok(None),
    };

    let refname = upstream.get().name().unwrap_or("");
    let remote_name_buf = repo
        .branch_remote_name(refname)
        .context("Failed to get remote name")?;

    let remote_name_str = remote_name_buf.as_str().unwrap_or("origin");
    let remote = repo.find_remote(remote_name_str)?;
    let url = remote.url().map(|s| s.to_string());
    Ok(url)
}

pub fn parse_remote_slug(url: &str) -> Option<String> {
    // Handle https://github.com/org/repo.git
    // Handle git@github.com:org/repo.git
    
    let trimmed = url.trim_end_matches(".git");
    
    let path_part = if let Some(idx) = trimmed.rfind(':') {
        // git@host:org/repo
        &trimmed[idx + 1..]
    } else if let Some(idx) = trimmed.find("://") {
        // https://host/org/repo
        let after_scheme = &trimmed[idx + 3..];
        match after_scheme.find('/') {
            Some(slash_idx) => &after_scheme[slash_idx + 1..],
            None => return None,
        }
    } else {
        return None;
    };
    
    Some(path_part.to_string())
}
