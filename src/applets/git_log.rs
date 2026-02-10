use anyhow::{Context, Result};
use git2::{Repository, Sort, Time};
use std::collections::HashMap;
use std::io::{IsTerminal, Write};
use std::process::{Command, Stdio};
use crate::util::color::ColorConfig;

pub fn run(args: &[String]) -> Result<()> {
    let mut no_colors = false;
    let mut topology_mode = false;

    // Skip argv[0]
    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "--no-colors" => no_colors = true,
            "-t" => topology_mode = true,
            _ => {} // Ignore others for now
        }
    }

    let colors = ColorConfig::from_env_and_flag(no_colors);
    let repo = Repository::discover(".").context("Failed to discover repository")?;

    let mut revwalk = repo.revwalk().context("Failed to create revwalk")?;
    
    if topology_mode {
        revwalk.push_glob("refs/*").context("Failed to push refs")?;
        revwalk.set_sorting(Sort::TOPOLOGICAL | Sort::TIME)?;
    } else {
        revwalk.push_head().context("Failed to push HEAD")?;
        revwalk.set_sorting(Sort::TIME)?;
    }

    // Pager setup
    let mut pager_process = if std::io::stdout().is_terminal() {
        setup_pager()
    } else {
        None
    };

    let mut out: Box<dyn Write> = if let Some(ref mut child) = pager_process {
        Box::new(child.stdin.take().unwrap())
    } else {
        Box::new(std::io::stdout())
    };

    // Cache refs for decoration
    let decorations = load_decorations(&repo)?;

    for oid_res in revwalk {
        let oid = oid_res?;
        let commit = repo.find_commit(oid)?;
        
        // Linear mode for now (no graph lanes)
        // Stretch goal: Add graph rendering here
        
        let refs = decorations.get(&oid).map(|v| v.as_slice()).unwrap_or(&[]);
        
        if topology_mode {
            print_oneline(&mut out, &repo, &commit, refs, &colors)?;
        } else {
            print_full(&mut out, &commit, refs, &colors)?;
        }
    }

    // Ensure we flush and wait for pager if used
    out.flush()?;
    drop(out); // Close stdin of pager
    
    if let Some(mut child) = pager_process {
        let _ = child.wait();
    }

    Ok(())
}

fn setup_pager() -> Option<std::process::Child> {
    let pager_cmd = std::env::var("GIT_PAGER")
        .or_else(|_| std::env::var("PAGER"))
        .unwrap_or_else(|_| "less -R".to_string());

    if pager_cmd.is_empty() || pager_cmd == "cat" {
        return None;
    }

    // Split command and args roughly
    let mut parts = pager_cmd.split_whitespace();
    let cmd = parts.next()?;
    let args: Vec<&str> = parts.collect();

    Command::new(cmd)
        .args(args)
        .stdin(Stdio::piped())
        .spawn()
        .ok()
}

fn load_decorations(repo: &Repository) -> Result<HashMap<git2::Oid, Vec<String>>> {
    let mut map: HashMap<git2::Oid, Vec<String>> = HashMap::new();

    // Helper to add ref
    let mut add_ref = |name: &str, target: git2::Oid| {
        map.entry(target).or_default().push(name.to_string());
    };

    // HEAD
    if let Ok(head) = repo.head() {
        if let Some(target) = head.target() {
            if let Ok(resolved) = head.resolve() {
                 if let Some(name) = resolved.shorthand() {
                     add_ref(&format!("HEAD -> {}", name), target);
                 } else {
                     add_ref("HEAD", target);
                 }
            } else {
                add_ref("HEAD", target);
            }
        }
    }

    // Local branches
    if let Ok(branches) = repo.branches(Some(git2::BranchType::Local)) {
        for b in branches {
            if let Ok((branch, _)) = b {
                if let (Some(name), Some(target)) = (branch.name().ok().flatten(), branch.get().target()) {
                    // Avoid duplicating HEAD -> branch if we already handled it?
                    // Git decorations show "HEAD -> main, origin/main".
                    // Our logic above adds "HEAD -> main".
                    // If we see "main" here, we should check if it's already covered?
                    // Actually git shows (HEAD -> main, origin/main).
                    // So we want "main" in the list too?
                    // Wait, if HEAD points to main, we added "HEAD -> main".
                    // If we add "main" again, we get "HEAD -> main, main". That's redundant.
                    // Let's keep it simple: just dump all refs, but special case HEAD.
                    
                     // Simple check: is this ref already in the list via HEAD -> ...?
                     // A bit hacky. Let's just add them and maybe dedup later?
                     // Or better: Don't bake "HEAD ->" into the string value, handle formatting later.
                     // But for MVP, let's just add strings.
                     
                     // We will filter out the branch name if "HEAD -> {name}" is present in the list?
                     // Let's just add normal branches.
                     add_ref(name, target);
                }
            }
        }
    }

    // Remote branches
    if let Ok(branches) = repo.branches(Some(git2::BranchType::Remote)) {
        for b in branches {
            if let Ok((branch, _)) = b {
                 if let (Some(name), Some(target)) = (branch.name().ok().flatten(), branch.get().target()) {
                    add_ref(name, target);
                }
            }
        }
    }

    // Tags
    if let Ok(tags) = repo.references_glob("refs/tags/*") {
        for tag in tags {
            if let Ok(r) = tag {
                if let Some(target) = r.target() {
                     let name = r.shorthand().unwrap_or("?");
                     add_ref(&format!("tag: {}", name), target);
                }
            }
        }
    }
    
    // Dedup and format logic is complex to match git exactly.
    // For now we might have "HEAD -> main", "main".
    // We can clean this up in the print function or just live with it for MVP.
    // Let's leave as is.

    Ok(map)
}

fn format_decorations(refs: &[String], colors: &ColorConfig) -> String {
    if refs.is_empty() {
        return String::new();
    }
    
    // Naive dedup: if we have "HEAD -> main" and "main", remove "main".
    let mut final_refs = refs.to_vec();
    
    // Find if we have a "HEAD -> branch" entry
    let head_target = final_refs.iter()
        .find(|r| r.starts_with("HEAD -> "))
        .map(|r| r.strip_prefix("HEAD -> ").unwrap().to_string());
        
    if let Some(target_branch) = head_target {
        final_refs.retain(|r| r != &target_branch);
    }
    
    let joined = final_refs.join(", ");
    let styled = colors.paint(colors.yellow_style(), joined);
    format!(" ({})", styled)
}

fn print_full(out: &mut Box<dyn Write>, commit: &git2::Commit, refs: &[String], colors: &ColorConfig) -> Result<()> {
    let oid_str = commit.id().to_string();
    let styled_oid = colors.paint(colors.yellow_style(), &oid_str);
    let decorations = format_decorations(refs, colors);
    
    writeln!(out, "commit {}{}", styled_oid, decorations)?;
    
    let author = commit.author();
    let name = author.name().unwrap_or("Unknown");
    let email = author.email().unwrap_or("unknown");
    
    writeln!(out, "Author: {} <{}>", name, email)?;
    
    let time = commit.time();
    let formatted_time = format_time(time);
    writeln!(out, "Date:   {}", formatted_time)?;
    
    writeln!(out)?;
    
    if let Some(msg) = commit.message() {
        for line in msg.lines() {
            writeln!(out, "    {}", line)?;
        }
    }
    
    writeln!(out)?;
    Ok(())
}

fn print_oneline(out: &mut Box<dyn Write>, _repo: &Repository, commit: &git2::Commit, refs: &[String], colors: &ColorConfig) -> Result<()> {
    let short_oid = &commit.id().to_string()[..7];
    let styled_oid = colors.paint(colors.yellow_style(), short_oid);
    let decorations = format_decorations(refs, colors);
    
    let summary = commit.summary().unwrap_or("");
    
    writeln!(out, "{}{} {}", styled_oid, decorations, summary)?;
    Ok(())
}

fn format_time(time: Time) -> String {
    // We don't have a chrono dep in the plan, so we'll do basic formatting or add chrono?
    // The plan didn't strictly specify chrono, but git2 returns seconds since epoch.
    // For MVP, just printing the raw timestamp or simple formatting is safer than adding deps mid-flight?
    // Wait, Cargo.toml already has `chrono = "0.4"`.
    // So we can use chrono.
    
    let dt = chrono::DateTime::from_timestamp(time.seconds(), 0);
    if let Some(d) = dt {
         d.format("%a %b %e %H:%M:%S %Y %z").to_string()
    } else {
        format!("{}", time.seconds())
    }
}
