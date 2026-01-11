pub mod applets;
pub mod util;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const EXIT_POLICY_REFUSAL: i32 = 42;
pub const SENTINEL: &str = "Nope";
pub const REFUSAL_STDOUT: &str = "Nope, use GitAdd, GitAddAll, GitAddDot, GitRm, GitCommit.";
pub const APPLETS: &[&str] = &[
    "GitAdd",
    "GitAddAll",
    "GitAddDot",
    "GitRm",
    "GitCommit",
    "GitStatus",
    "GitLog",
];
