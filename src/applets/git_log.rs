use std::borrow::Cow;
use std::io::Write;
use std::process::{Command, Stdio};

use anyhow::{Context, Result};
use gix::{object::TreeRefIter, refs::transaction::PreviousValue, Repository};

use crate::util::color::{green_style, red_style, yellow_style, ColorConfig};

pub fn run(args: &[String]) -> Result<()> {
    let options = parse_args(args)?;
    let color_config = ColorConfig::from_env_and_flag(options.no_colors);
    let repo = Repository::discover(".")?;

    let mut output = String::new();
    if options.topology {
        render_topology(&repo, &color_config, &mut output)?;
    } else {
        render_full(&repo, &color_config, &mut output)?;
    }

    emit_with_pager(&output, options.no_pager)
}

struct Options {
    no_colors: bool,
    topology: bool,
    no_pager: bool,
}

fn parse_args(args: &[String]) -> Result<Options> {
    let mut opts = Options {
        no_colors: false,
        topology: false,
        no_pager: false,
    };
    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "--no-colors" | "--no-color" => opts.no_colors = true,
            "-t" | "--topology" => opts.topology = true,
            "--no-pager" => opts.no_pager = true,
            other if other.starts_with('-') => anyhow::bail!("Unsupported flag: {other}"),
            _ => {}
        }
    }
    Ok(opts)
}

fn render_full(repo: &Repository, colors: &ColorConfig, output: &mut String) -> Result<()> {
    let head = repo.head()?;
    let commit_id = head.peel_to_commit()?.id;

    let mut revwalk = repo.revwalk();
    revwalk.push(commit_id)?;

    for id in revwalk {
        let id = id?;
        let commit = repo.find_commit(id)?;
        let tree = commit.tree()?;

        let header = format_commit_header(colors, &commit, &head, None);
        output.push_str(&header);
        output.push('\n');

        let author = commit.author();
        output.push_str(&format!(
            "| Author: {} <{}>\n",
            author.name().unwrap_or("unknown"),
            author.email().unwrap_or("unknown")
        ));
        output.push_str(&format!("| Date:   {}\n", commit.time()));
        output.push_str("|\n");

        let message = commit.message().unwrap_or("(no message)");
        for line in message.lines() {
            output.push_str("|     ");
            output.push_str(line);
            output.push('\n');
        }
        output.push_str("|\n");
        let _ = tree; // placeholder for future tree usage
    }
    Ok(())
}

fn render_topology(repo: &Repository, colors: &ColorConfig, output: &mut String) -> Result<()> {
    let mut revwalk = repo.revwalk();
    revwalk.push_head()?;
    revwalk.set_sorting(gix::revwalk::Sort::TOPOLOGICAL)?;

    for id in revwalk {
        let id = id?;
        let commit = repo.find_commit(id)?;
        let header = format_topology_line(colors, &commit, repo)?;
        output.push_str(&header);
        output.push('\n');
    }
    Ok(())
}

fn format_commit_header(
    colors: &ColorConfig,
    commit: &gix::Commit,
    head_ref: &gix::Reference,
    decorations: Option<&str>,
) -> String {
    let oid = commit.id;
    let label = format!("commit {}", oid);
    let decorated = match decorations {
        Some(deco) => format!("{} ({})", label, deco),
        None => label,
    };
    format!("* {decorated}")
}

fn format_topology_line(colors: &ColorConfig, commit: &gix::Commit, repo: &Repository) -> Result<String> {
    let short_id = commit.id.to_hex_with_len(7);
    let message = commit.summary().unwrap_or("(no message)");
    let mut decorations = Vec::new();
    collect_decorations(repo, commit.id, &mut decorations)?;
    let decorated = if decorations.is_empty() {
        String::new()
    } else {
        format!(" ({})", decorations.join(", "))
    };
    let line = format!("* {}{} {}", short_id, decorated, message);
    Ok(line)
}

fn collect_decorations(repo: &Repository, oid: gix::ObjectId, out: &mut Vec<String>) -> Result<()> {
    for reference in repo.references()? {
        let mut reference = reference?;
        if let Some(target) = reference.target() {
            if target == oid {
                if let Some(name) = reference.name().map(|n| n.to_string()) {
                    out.push(name);
                }
            }
        }
    }
    Ok(())
}

fn emit_with_pager(output: &str, no_pager: bool) -> Result<()> {
    if no_pager || !atty::is(atty::Stream::Stdout) {
        print!("{}", output);
        return Ok(());
    }

    let pager = std::env::var("GIT_PAGER")
        .filter(|s| !s.is_empty())
        .or_else(|| std::env::var("PAGER").ok());

    let pager_cmd = pager.unwrap_or_else(|| "less -R".to_string());
    let mut parts = pager_cmd.split_whitespace();
    let program = parts.next().unwrap();
    let args: Vec<_> = parts.collect();

    let mut child = Command::new(program)
        .args(args)
        .stdin(Stdio::piped())
        .spawn();

    let mut child = match child {
        Ok(child) => child,
        Err(_) => {
            print!("{}", output);
            return Ok(());
        }
    };

    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(output.as_bytes())?;
    }
    let _ = child.wait();
    Ok(())
}