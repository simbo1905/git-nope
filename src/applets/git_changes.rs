use anyhow::{Context, Result};
use git2::{Repository, StatusOptions, Status};

pub fn run(_args: &[String]) -> Result<()> {
    let repo = Repository::discover(".").context("Failed to discover repository")?;

    let mut opts = StatusOptions::new();
    opts.include_untracked(true);
    opts.include_ignored(false);
    opts.recurse_untracked_dirs(true);

    let statuses = repo.statuses(Some(&mut opts))
        .context("Failed to get repository status")?;

    for entry in statuses.iter() {
        let status = entry.status();
        let path = entry.path().unwrap_or("?");

        let (index_char, worktree_char) = status_to_porcelain(status);
        
        println!("{}{} {}", index_char, worktree_char, path);
    }

    Ok(())
}

fn status_to_porcelain(status: Status) -> (char, char) {
    let mut index_char = ' ';
    let mut worktree_char = ' ';

    if status.is_index_new() {
        index_char = 'A';
    } else if status.is_index_modified() {
        index_char = 'M';
    } else if status.is_index_deleted() {
        index_char = 'D';
    } else if status.is_index_renamed() {
        index_char = 'R';
    } else if status.is_index_typechange() {
        index_char = 'T';
    }

    if status.is_wt_new() {
        worktree_char = '?';
        index_char = '?';
    } else if status.is_wt_modified() {
        worktree_char = 'M';
    } else if status.is_wt_deleted() {
        worktree_char = 'D';
    } else if status.is_wt_renamed() {
        worktree_char = 'R';
    } else if status.is_wt_typechange() {
        worktree_char = 'T';
    }

    if status.is_conflicted() {
        index_char = 'U';
        worktree_char = 'U';
    }

    (index_char, worktree_char)
}
