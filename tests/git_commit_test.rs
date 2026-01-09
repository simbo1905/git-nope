use assert_cmd::Command;
use predicates::str::contains;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

fn prep_git_commit_bin(tmp: &TempDir) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let binary_name = if cfg!(windows) { "GitCommit.exe" } else { "GitCommit" };
    let dest = tmp.path().join(binary_name);
    if dest.exists() {
        fs::remove_file(&dest)?;
    }
    fs::copy(env!("CARGO_BIN_EXE_git-nope"), &dest)?;
    Ok(dest)
}

#[test]
fn test_git_commit_applet_success() -> Result<(), Box<dyn std::error::Error>> {
    let tmp = TempDir::new()?;
    let repo_dir = tmp.path().join("repo");
    fs::create_dir(&repo_dir)?;

    // Init repo with real git
    std::process::Command::new("git")
        .arg("init")
        .current_dir(&repo_dir)
        .status()?;

    // Set user config for commit
    std::process::Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(&repo_dir)
        .status()?;
    std::process::Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&repo_dir)
        .status()?;

    fs::write(repo_dir.join("test.txt"), "hello")?;
    
    // Stage with real git
    std::process::Command::new("git")
        .args(["add", "test.txt"])
        .current_dir(&repo_dir)
        .output()?;

    // Prepare a copy of the binary with argv[0] = GitCommit
    let git_commit_bin = prep_git_commit_bin(&tmp)?;

    Command::new(&git_commit_bin)
        .args(["-m", "feat: initial commit"])
        .current_dir(&repo_dir)
        .assert()
        .success()
        .stdout(contains("Created commit"));

    // Verify with real git
    let output = std::process::Command::new("git")
        .args(["log", "-1", "--pretty=%s"])
        .current_dir(&repo_dir)
        .output()?;
    
    assert_eq!(String::from_utf8(output.stdout)?.trim(), "feat: initial commit");

    Ok(())
}

#[test]
fn test_git_commit_no_message() -> Result<(), Box<dyn std::error::Error>> {
    let tmp = TempDir::new()?;
    let repo_dir = tmp.path().join("repo");
    fs::create_dir(&repo_dir)?;
    std::process::Command::new("git").arg("init").current_dir(&repo_dir).status()?;

    let git_commit_bin = prep_git_commit_bin(&tmp)?;

    Command::new(&git_commit_bin)
        .current_dir(&repo_dir)
        .assert()
        .failure()
        .stderr(contains("GitCommit requires -m/--message"));

    Ok(())
}

#[test]
fn test_git_commit_fails_with_conflicts() -> Result<(), Box<dyn std::error::Error>> {
    let tmp = TempDir::new()?;
    let repo_dir = tmp.path().join("repo");
    fs::create_dir(&repo_dir)?;

    std::process::Command::new("git")
        .arg("init")
        .current_dir(&repo_dir)
        .status()?;
    std::process::Command::new("git")
        .args(["checkout", "-b", "main"])
        .current_dir(&repo_dir)
        .status()?;
    std::process::Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(&repo_dir)
        .status()?;
    std::process::Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&repo_dir)
        .status()?;

    fs::write(repo_dir.join("conflict.txt"), "base\n")?;
    std::process::Command::new("git")
        .args(["add", "conflict.txt"])
        .current_dir(&repo_dir)
        .status()?;
    std::process::Command::new("git")
        .args(["commit", "-m", "base"])
        .current_dir(&repo_dir)
        .status()?;

    std::process::Command::new("git")
        .args(["checkout", "-b", "feature"])
        .current_dir(&repo_dir)
        .status()?;
    fs::write(repo_dir.join("conflict.txt"), "feature change\n")?;
    std::process::Command::new("git")
        .args(["commit", "-am", "feature change"])
        .current_dir(&repo_dir)
        .status()?;

    std::process::Command::new("git")
        .args(["checkout", "main"])
        .current_dir(&repo_dir)
        .status()?;
    fs::write(repo_dir.join("conflict.txt"), "main change\n")?;
    std::process::Command::new("git")
        .args(["commit", "-am", "main change"])
        .current_dir(&repo_dir)
        .status()?;

    let merge_status = std::process::Command::new("git")
        .args(["merge", "feature"])
        .current_dir(&repo_dir)
        .status()?;
    assert!(!merge_status.success(), "Expected merge to produce conflicts");

    let git_commit_bin = prep_git_commit_bin(&tmp)?;

    Command::new(&git_commit_bin)
        .args(["-m", "should fail"])
        .current_dir(&repo_dir)
        .assert()
        .failure()
        .stderr(contains("Cannot commit with unmerged paths"));

    Ok(())
}
