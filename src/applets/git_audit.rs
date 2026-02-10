use anyhow::{Context, Result};
use git2::{Repository, StatusOptions};
use crate::util::color::ColorConfig;

pub fn run(args: &[String]) -> Result<()> {
    let mut no_colors = false;
    let mut check_remote = false;

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "--no-colors" => no_colors = true,
            "-r" => check_remote = true,
            _ => {}
        }
    }

    let colors = ColorConfig::from_env_and_flag(no_colors);
    let repo = Repository::discover(".").context("Failed to discover repository")?;

    let cleanliness = classify_cleanliness(&repo)?;
    
    print_status_line(&repo, &cleanliness, &colors)?;

    if check_remote {
        print_remote_status(&repo, &colors)?;
    }

    Ok(())
}

#[derive(Debug)]
enum Cleanliness {
    Clean,
    Dirty,
    Tainted,
}

fn classify_cleanliness(repo: &Repository) -> Result<Cleanliness> {
    let mut opts = StatusOptions::new();
    opts.include_untracked(true);
    opts.include_ignored(false);
    
    let statuses = repo.statuses(Some(&mut opts))
        .context("Failed to get repository status")?;

    let mut has_tracked_changes = false;
    let mut has_untracked = false;

    for entry in statuses.iter() {
        let status = entry.status();
        
        if status.is_index_new()
            || status.is_index_modified()
            || status.is_index_deleted()
            || status.is_index_renamed()
            || status.is_index_typechange()
            || status.is_wt_modified()
            || status.is_wt_deleted()
            || status.is_wt_renamed()
            || status.is_wt_typechange()
        {
            has_tracked_changes = true;
            break;
        }

        if status.is_wt_new() {
            has_untracked = true;
        }
    }

    Ok(if has_tracked_changes {
        Cleanliness::Dirty
    } else if has_untracked {
        Cleanliness::Tainted
    } else {
        Cleanliness::Clean
    })
}

fn print_status_line(repo: &Repository, cleanliness: &Cleanliness, colors: &ColorConfig) -> Result<()> {
    let state_text = match cleanliness {
        Cleanliness::Clean => colors.paint(colors.green_style(), "Clean"),
        Cleanliness::Dirty => colors.paint(colors.red_style(), "Dirty"),
        Cleanliness::Tainted => colors.paint(colors.yellow_style(), "Tainted"),
    };

    let head = repo.head().context("Failed to get HEAD")?;
    let branch_name = if head.is_branch() {
        head.shorthand().unwrap_or("unknown")
    } else {
        "detached"
    };

    let commit = head.peel_to_commit().context("Failed to get HEAD commit")?;
    let short_oid = &commit.id().to_string()[..7];

    let remote_info = get_remote_origin(&repo)?;

    println!(
        "{} | {} | {} | {}",
        state_text,
        remote_info.unwrap_or_else(|| "no-remote".to_string()),
        branch_name,
        short_oid
    );

    Ok(())
}

fn print_remote_status(repo: &Repository, colors: &ColorConfig) -> Result<()> {
    let head = repo.head().context("Failed to get HEAD")?;
    
    if !head.is_branch() {
        println!("NoUpstream (detached HEAD)");
        return Ok(());
    }

    let branch = git2::Branch::wrap(head);
    let upstream = match branch.upstream() {
        Ok(u) => u,
        Err(_) => {
            println!("NoUpstream | <none>");
            return Ok(());
        }
    };

    let local_oid = branch.get().target().context("Failed to get local branch target")?;
    let upstream_oid = upstream.get().target().context("Failed to get upstream target")?;

    if local_oid == upstream_oid {
        let status = colors.paint(colors.green_style(), "UpToDate");
        println!("{}", status);
        return Ok(());
    }

    let (ahead, behind) = repo.graph_ahead_behind(local_oid, upstream_oid)
        .context("Failed to calculate ahead/behind")?;

    let status = if ahead > 0 && behind > 0 {
        colors.paint(colors.red_style(), format!("Diverged (ahead {}, behind {})", ahead, behind))
    } else if ahead > 0 {
        colors.paint(colors.yellow_style(), format!("Ahead ({})", ahead))
    } else if behind > 0 {
        colors.paint(colors.yellow_style(), format!("Behind ({})", behind))
    } else {
        colors.paint(colors.green_style(), "UpToDate")
    };

    println!("{}", status);
    Ok(())
}

fn get_remote_origin(repo: &Repository) -> Result<Option<String>> {
    let remote = match repo.find_remote("origin") {
        Ok(r) => r,
        Err(_) => return Ok(None),
    };

    let url = remote.url().unwrap_or("unknown");
    
    let short_name = if let Some(path) = url.strip_suffix(".git") {
        path.rsplit('/').next().unwrap_or("origin")
    } else {
        url.rsplit('/').next().unwrap_or("origin")
    };

    Ok(Some(short_name.to_string()))
}
