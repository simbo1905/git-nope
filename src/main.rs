use anyhow::Result;
use git_nope::{APPLETS, EXIT_POLICY_REFUSAL, REFUSAL_STDOUT, SENTINEL, VERSION};
use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let program_name = get_program_name(args.get(0));

    match program_name.as_deref() {
        Some("GitAdd") | Some("git-nope-git-add") => git_nope::applets::git_add::run(&args),
        Some("GitCommit") | Some("git-nope-git-commit") => git_nope::applets::git_commit::run(&args),
        Some("GitRm") | Some("git-nope-git-rm") => git_nope::applets::git_rm::run(&args),
        Some("GitStatus") | Some("git-nope-git-status") => {
            git_nope::applets::git_status::run(&args)
        }
        Some("GitLog") | Some("git-nope-git-log") => git_nope::applets::git_log::run(&args),
        Some("GitAddAll") | Some("GitAddDot") => {
            println!("TODO: implement {}", program_name.unwrap());
            Ok(())
        }
        Some("git") | Some("git-nope") | None => handle_git_invocation(&args),
        Some(other) => handle_unknown(other, &args),
    }
}

fn get_program_name(arg: Option<&String>) -> Option<String> {
    arg.map(|raw| {
        Path::new(raw)
            .file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or(raw)
            .to_string()
    })
}

fn handle_git_invocation(args: &[String]) -> Result<()> {
    match args.get(1).map(|s| s.as_str()) {
        Some("nope") if args.len() == 2 => {
            println!("{SENTINEL}");
            Ok(())
        }
        _ => {
            policy_refusal(args.get(0), "Direct git usage is blocked.");
            Ok(())
        }
    }
}

fn handle_unknown(program: &str, args: &[String]) -> Result<()> {
    let detail = format!(
        "Unknown invocation '{program}'. Expected git nope or one of {}.",
        APPLETS.join(", ")
    );
    policy_refusal(args.get(0), &detail);
    Ok(())
}

fn policy_refusal(invoked_as: Option<&String>, detail: &str) {
    println!("{REFUSAL_STDOUT}");
    let _ = write_refusal_diagnostics(invoked_as, detail);
    process::exit(EXIT_POLICY_REFUSAL);
}

fn write_refusal_diagnostics(invoked_as: Option<&String>, detail: &str) -> io::Result<()> {
    let mut stderr = io::stderr();
    writeln!(stderr, "git-nope version {VERSION}")?;
    writeln!(stderr, "Direct git usage is blocked to protect the repository.")?;
    writeln!(stderr, "Allowed applets:")?;
    for applet in APPLETS {
        writeln!(stderr, "  {applet}")?;
    }
    writeln!(stderr, "Detail: {detail}")?;
    if let Some(name) = invoked_as {
        writeln!(stderr, "Invoked as: {name}")?;
    }
    writeln!(stderr, "Docs: docs/git-nope.md")?;
    Ok(())
}
