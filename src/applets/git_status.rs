use anyhow::{Context, Result};
use git2::{BranchType, Repository, Status, StatusOptions};

use crate::util::{
    color::{green_style, red_style, yellow_style, ColorConfig},
    git::{head_branch, head_branch_name, parse_remote_slug, short_commit_id, upstream_remote_url},
};

pub fn run(args: &[String]) -> Result<()> {
    let (disable_colors, show_remote) = parse_args(args)?;
    let color_config = ColorConfig::from_env_and_flag(disable_colors);

    let repo = Repository::discover(".").context("Failed to discover repository")?;
    let head_branch = head_branch(&repo)?;
    let branch_name = head_branch_name(&repo)?;
    let short_id = short_commit_id(&repo)?;

    let (state, decorations) = classify_status(&repo)?;
    let remote_slug = upstream_remote_url(&repo, &head_branch)?
        .flatten()
        .and_then(|url| parse_remote_slug(&url));

    let decorations = decorations?;
    let line = format_status_line(
        &color_config,
        state,
        remote_slug.as_deref(),
        &branch_name,
        &short_id,
        &decorations,
    );
    println!("{line}");

    if show_remote {
        let remote_line = remote_status_line(&repo, head_branch, &color_config)?;
        println!("{remote_line}");
    }

    Ok(())
}

fn parse_args(args: &[String]) -> Result<(bool, bool)> {
    let mut disable_colors = false;
    let mut show_remote = false;
    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "--no-colors" | "--no-color" => disable_colors = true,
            "-r" | "--remote" => show_remote = true,
            other if other.starts_with('-') => anyhow::bail!("Unsupported flag: {other}"),
            _ => {}
        }
    }
    Ok((disable_colors, show_remote))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CleanState {
    Clean,
    Dirty,
    Tainted,
}

fn classify_status(repo: &Repository) -> Result<(CleanState, Decorations)> {
    let mut opts = StatusOptions::new();
    opts.include_untracked(true)
        .recurse_untracked_dirs(true)
        .include_ignored(false);

    let statuses = repo.statuses(Some(&mut opts))?;

    let mut has_index = false;
    let mut has_tracked_worktree = false;
    let mut has_untracked = false;

    for entry in statuses.iter() {
        let status = entry.status();
        if status.intersects(Status::INDEX_NEW | Status::INDEX_MODIFIED | Status::INDEX_DELETED | Status::INDEX_RENAMED | Status::INDEX_TYPECHANGE) {
            has_index = true;
        }
        if status.intersects(Status::WT_MODIFIED | Status::WT_DELETED | Status::WT_TYPECHANGE | Status::WT_RENAMED) {
            has_tracked_worktree = true;
        }
        if status.contains(Status::WT_NEW) {
            if let Some(path) = entry.path() {
                if !repo.is_path_ignored(path)? {
                    has_untracked = true;
                }
            }
        }
    }

    let state = if has_index || has_tracked_worktree {
        CleanState::Dirty
    } else if has_untracked {
        CleanState::Tainted
    } else {
        CleanState::Clean
    };

    let decorations = collect_decorations(repo)?;

    Ok((state, decorations))
}

struct Decorations {
    head_branch: String,
    tags: Vec<String>,
    remote_branches: Vec<String>,
}

fn collect_decorations(repo: &Repository) -> Result<Decorations> {
    let head_branch = head_branch_name(repo)?.into_owned();
    let head = repo.head()?;
    let head_id = head.target().ok_or_else(|| anyhow::anyhow!("Detached HEAD"))?;

    let mut tags = Vec::new();
    let mut remotes = Vec::new();

    let tag_names = repo.tag_names(None)?;
    for tag in tag_names.iter().flatten() {
        if let Ok(tag_ref) = repo.revparse_single(&format!("refs/tags/{tag}")) {
            if tag_ref.id() == head_id {
                tags.push(tag.to_string());
            }
        }
    }

    for branch in repo.branches(Some(BranchType::Remote))? {
        let (branch, _) = branch?;
        if let Some(target) = branch.get().target() {
            if target == head_id {
                if let Some(name) = branch.name()? {
                    remotes.push(name.to_string());
                }
            }
        }
    }

    Ok(Decorations {
        head_branch,
        tags,
        remote_branches: remotes,
    })
}

fn format_status_line(color_config: &ColorConfig, state: CleanState, slug: Option<&str>, branch: &str, short_id: &str, decorations: &Decorations) -> String {
    let (state_label, style) = match state {
        CleanState::Clean => ("Clean", green_style()),
        CleanState::Dirty => ("Dirty", red_style()),
        CleanState::Tainted => ("Tainted", yellow_style()),
    };

    let state_text = color_config.paint(style, state_label);
    let slug_text = slug.unwrap_or("<no-remote>");
    let mut parts = vec![format!("{} {} {}", state_text, slug_text, branch)];
    if !decorations.tags.is_empty() || !decorations.remote_branches.is_empty() {
        let mut deco = Vec::new();
        deco.push(format!("HEAD -> {}", decorations.head_branch));
        deco.extend(decorations.remote_branches.iter().cloned());
        deco.extend(decorations.tags.iter().cloned());
        parts.push(format!("({})", deco.join(", ")));
    }
    parts.push(short_id.to_string());

    parts.join(" ")
}

fn remote_status_line(repo: &Repository, branch: git2::Branch, color_config: &ColorConfig) -> Result<String> {
    let upstream = match branch.upstream() {
        Ok(up) => up,
        Err(e) if e.code() == git2::ErrorCode::NotFound => {
            return Ok("NoUpstream <none>".to_string());
        }
        Err(e) => return Err(e.into()),
    };

    let local_commit = branch.get().target().ok_or_else(|| anyhow::anyhow!("Branch has no target"))?;
    let upstream_commit = upstream.get().target().ok_or_else(|| anyhow::anyhow!("Upstream has no target"))?;

    let (ahead, behind) = repo.graph_ahead_behind(local_commit, upstream_commit)?;

    let (label, style) = match (ahead, behind) {
        (0, 0) => ("UpToDate".to_string(), green_style()),
        (a, 0) => (format!("Ahead({a})"), yellow_style()),
        (0, b) => (format!("Behind({b})"), yellow_style()),
        (a, b) => (format!("Diverged(+{a}/-{b})"), red_style()),
    };

    let remote_url = upstream_remote_url(repo, branch)?.flatten().unwrap_or_else(|| "<none>".to_string());
    let slug = parse_remote_slug(&remote_url).unwrap_or_else(|| remote_url.clone());
    let text = color_config.paint(style, label);
    Ok(format!("{text} {slug} remote {remote_url}"))
}
