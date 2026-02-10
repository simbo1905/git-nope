use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::Mutex;

use tempfile::{tempdir_in, TempDir};

static ENV_MUTEX: Mutex<()> = Mutex::new(());

pub fn temp_root_dir() -> TempDir {
    std::fs::create_dir_all(".tmp").expect("create .tmp");
    tempdir_in(".tmp").expect("tempdir_in .tmp")
}

pub fn init_git_repo(root: &Path) -> PathBuf {
    let repo_dir = root.join("repo");
    std::fs::create_dir_all(&repo_dir).expect("create repo dir");

    run_git(&repo_dir, &["init"]);
    run_git(&repo_dir, &["config", "user.name", "Test User"]);
    run_git(&repo_dir, &["config", "user.email", "test@example.com"]);
    run_git(
        &repo_dir,
        &["-c", "user.name=Test User", "-c", "user.email=test@example.com", "commit", "--allow-empty", "-m", "initial"],
    );
    repo_dir
}

pub fn run_git(repo_dir: &Path, args: &[&str]) -> Output {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo_dir)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .env_remove("GIT_CEILING_DIRECTORIES")
        .output()
        .expect("run git");
    if !out.status.success() {
        panic!(
            "git {:?} failed.\nstatus: {}\nstdout:\n{}\nstderr:\n{}",
            args,
            out.status,
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
    }
    out
}

pub fn write_file(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create parent dirs");
    }
    std::fs::write(path, content).expect("write file");
}

#[allow(dead_code)]
pub fn with_dir<T, F>(dir: &Path, f: F) -> T
where
    F: FnOnce() -> T,
{
    let _lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let prev = std::env::current_dir().expect("current dir");
    std::env::set_current_dir(dir).expect("set dir");
    let result = catch_unwind(AssertUnwindSafe(f));
    std::env::set_current_dir(prev).expect("restore dir");
    match result {
        Ok(value) => value,
        Err(err) => std::panic::resume_unwind(err),
    }
}

pub fn with_repo<T, F>(repo_dir: &Path, f: F) -> T
where
    F: FnOnce() -> T,
{
    let _lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let prev_dir = std::env::current_dir().expect("current dir");
    let prev_git_dir = std::env::var_os("GIT_DIR");
    let prev_git_work_tree = std::env::var_os("GIT_WORK_TREE");
    let prev_ceiling = std::env::var_os("GIT_CEILING_DIRECTORIES");

    std::env::set_var("GIT_DIR", repo_dir.join(".git"));
    std::env::set_var("GIT_WORK_TREE", repo_dir);
    if let Some(parent) = repo_dir.parent() {
        std::env::set_var("GIT_CEILING_DIRECTORIES", parent);
    }
    std::env::set_current_dir(repo_dir).expect("set dir");

    let result = catch_unwind(AssertUnwindSafe(f));

    std::env::set_current_dir(prev_dir).expect("restore dir");
    match prev_git_dir {
        Some(val) => std::env::set_var("GIT_DIR", val),
        None => std::env::remove_var("GIT_DIR"),
    }
    match prev_git_work_tree {
        Some(val) => std::env::set_var("GIT_WORK_TREE", val),
        None => std::env::remove_var("GIT_WORK_TREE"),
    }
    match prev_ceiling {
        Some(val) => std::env::set_var("GIT_CEILING_DIRECTORIES", val),
        None => std::env::remove_var("GIT_CEILING_DIRECTORIES"),
    }

    match result {
        Ok(value) => value,
        Err(err) => std::panic::resume_unwind(err),
    }
}
