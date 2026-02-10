mod common;

use git_nope::applets::{git_add, git_commit, git_rm};

use common::{init_git_repo, run_git, temp_root_dir, with_repo, write_file};

fn to_utf8(bytes: &[u8]) -> String {
    String::from_utf8(bytes.to_vec()).expect("utf8")
}

#[test]
fn git_commit_multiple_messages_join_with_blank_line() {
    let tmp = temp_root_dir();
    let repo_dir = init_git_repo(tmp.path());

    write_file(&repo_dir.join("file.txt"), "one");
    run_git(&repo_dir, &["add", "file.txt"]);

    let args = vec![
        "GitCommit".to_string(),
        "-m".to_string(),
        "first".to_string(),
        "-m".to_string(),
        "second".to_string(),
    ];
    with_repo(&repo_dir, || git_commit::run(&args).expect("commit"));

    let log = run_git(&repo_dir, &["log", "-1", "--pretty=%B"]);
    let body = to_utf8(&log.stdout);
    assert_eq!(body.trim(), "first\n\nsecond");
}

#[test]
fn git_commit_requires_message_flag() {
    let tmp = temp_root_dir();
    let repo_dir = init_git_repo(tmp.path());

    let args = vec!["GitCommit".to_string()];
    let err = with_repo(&repo_dir, || git_commit::run(&args)).expect_err("missing -m");
    assert!(err.to_string().contains("requires -m/--message"));
}

#[test]
fn git_commit_requires_message_value_after_flag() {
    let tmp = temp_root_dir();
    let repo_dir = init_git_repo(tmp.path());

    let args = vec!["GitCommit".to_string(), "-m".to_string()];
    let err = with_repo(&repo_dir, || git_commit::run(&args)).expect_err("missing message");
    assert!(err.to_string().contains("Expected message after -m"));
}

#[test]
fn git_commit_rejects_unsupported_flag() {
    let tmp = temp_root_dir();
    let repo_dir = init_git_repo(tmp.path());

    let args = vec![
        "GitCommit".to_string(),
        "--amend".to_string(),
        "-m".to_string(),
        "msg".to_string(),
    ];
    let err = with_repo(&repo_dir, || git_commit::run(&args)).expect_err("unsupported flag");
    assert!(err.to_string().contains("Unsupported GitCommit flag"));
}

#[test]
fn git_rm_rejects_glob_metachars_in_args() {
    let tmp = temp_root_dir();
    let repo_dir = init_git_repo(tmp.path());

    write_file(&repo_dir.join("a.txt"), "a");
    run_git(&repo_dir, &["add", "a.txt"]);

    let args = vec!["GitRm".to_string(), "*.txt".to_string()];
    let result = with_repo(&repo_dir, || git_rm::run(&args));
    result.expect("glob rejection should succeed");

    let status = run_git(&repo_dir, &["status", "--short"]);
    let status_str = to_utf8(&status.stdout);
    assert!(status_str.contains("A  a.txt"), "status: {status_str}");
    assert!(repo_dir.join("a.txt").exists());
}

#[cfg(unix)]
#[test]
fn git_rm_skips_symlink_and_does_not_delete_target() {
    use std::os::unix::fs::symlink;

    let tmp = temp_root_dir();
    let repo_dir = init_git_repo(tmp.path());

    write_file(&repo_dir.join("target.txt"), "target");
    run_git(&repo_dir, &["add", "target.txt"]);
    run_git(&repo_dir, &["commit", "-m", "add target"]);

    symlink("target.txt", repo_dir.join("link.txt")).expect("create symlink");

    let args = vec!["GitRm".to_string(), "link.txt".to_string()];
    let _ = with_repo(&repo_dir, || git_rm::run(&args)).expect("skip succeeds");

    assert!(repo_dir.join("link.txt").exists());
    assert!(repo_dir.join("target.txt").exists());
}

#[test]
fn git_add_rejects_path_outside_repo() {
    let tmp = temp_root_dir();
    let repo_dir = init_git_repo(tmp.path());

    let outside = tmp.path().join("outside.txt");
    write_file(&outside, "outside");

    let args = vec!["GitAdd".to_string(), outside.to_string_lossy().to_string()];
    let err = with_repo(&repo_dir, || git_add::run(&args)).expect_err("should reject outside path");
    assert!(err.to_string().contains("outside of repository"));
}
