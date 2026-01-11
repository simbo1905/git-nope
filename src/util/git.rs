use anyhow::{Context, Result};
use git2::{BranchType, ErrorCode, Repository};

pub fn head_branch_name(repo: &Repository) -> Result<String> {
    let head = repo.head()?;
    if !head.is_branch() {
        anyhow::bail!("HEAD is not a branch");
    }
    let shorthand = head
        .shorthand()
        .ok_or_else(|| anyhow::anyhow!("Branch name not valid UTF-8"))?;
    Ok(shorthand.to_string())
}

pub fn short_commit_id(repo: &Repository) -> Result<String> {
    let head = repo.head()?;
    let oid = head
        .target()
        .ok_or_else(|| anyhow::anyhow!("Detached HEAD is not supported"))?;
    let short = repo.find_object(oid, None)?.short_id()?;
    if let Some(text) = short.as_str() {
        Ok(text.to_string())
    } else {
        Ok(short.to_string())
    }
}

pub fn head_branch(repo: &Repository) -> Result<git2::Branch> {
    let head = repo.head()?;
    if !head.is_branch() {
        anyhow::bail!("HEAD is not a branch");
    }
    let name = head
        .shorthand()
        .ok_or_else(|| anyhow::anyhow!("Branch name not valid UTF-8"))?;
    repo.find_branch(name, BranchType::Local)
        .map_err(Into::into)
}

pub fn upstream_remote_url(repo: &Repository, branch: &git2::Branch) -> Result<Option<String>> {
    let upstream = match branch.upstream() {
        Ok(b) => b,
        Err(e) if e.code() == ErrorCode::NotFound => return Ok(None),
        Err(e) => return Err(e.into()),
    };

    let upstream_name = upstream.name_bytes()?;
    let upstream_str = std::str::from_utf8(upstream_name)
        .context("Upstream name not valid UTF-8")?;
    let buf = repo.branch_remote_name(upstream_str)?;
    let remote_name = buf
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Remote name not valid UTF-8"))?;
    let remote = repo.find_remote(remote_name)?;
    Ok(remote.url().map(|s| s.to_string()))
}

pub fn parse_remote_slug(url: &str) -> Option<String> {
    if let Some(stripped) = url.strip_prefix("https://") {
        return extract_after_domain(stripped);
    }
    if let Some(stripped) = url.strip_prefix("http://") {
        return extract_after_domain(stripped);
    }
    if let Some(stripped) = url.strip_prefix("git@") {
        return stripped
            .split_once(':')
            .and_then(|(_, path)| normalize_path(path));
    }
    normalize_path(url)
}

fn extract_after_domain(input: &str) -> Option<String> {
    let (_, path) = input.split_once('/')?;
    normalize_path(path)
}

fn normalize_path(path: &str) -> Option<String> {
    let trimmed = path.trim_end_matches(".git");
    let mut segments = trimmed.split('/');
    let org = segments.next()?;
    let repo = segments.next()?;
    if segments.next().is_some() {
        return None;
    }
    Some(format!("{}/{}", org, repo))
}