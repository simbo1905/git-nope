# Suite 3: GitRm Aggressive & Glob Operations

## Objective

Verify that `GitRm` correctly implements the "make it gone" semantics using the `glob` crate:
- Removes tracked, staged, and untracked files from disk.
- Removes matching entries from the Git index.
- Supports glob patterns including `**`.
- Skips `.git` and submodules.

## Setup

1.  **Build the binary**:
    Ensure the `git-nope` binary is built with the new `GitRm` implementation.
    ```bash
    cargo build --release
    ```

2.  **Prepare Test Environment**:
    Create a fresh timestamped directory.
    ```bash
    TIMESTAMP=$(date +%Y%m%d_%H%M%S)
    TEST_DIR=".tmp/suite3_${TIMESTAMP}"
    mkdir -p "$TEST_DIR"
    ```

3.  **Setup GitRm Applet**:
    Copy the binary to the test directory and rename it to `GitRm`.
    ```bash
    GIT_NOPE_BIN="target/release/git-nope"
    cp "$GIT_NOPE_BIN" "$TEST_DIR/GitRm"
    chmod +x "$TEST_DIR/GitRm"
    ```

4.  **Initialize Test Repository**:
    ```bash
    mkdir -p "$TEST_DIR/repo"
    cd "$TEST_DIR/repo"
    git init
    
    # Create some files
    mkdir -p src/nested
    echo "content" > root.txt
    echo "content" > src/a.rs
    echo "content" > src/b.txt
    echo "content" > src/nested/c.rs
    echo "content" > untracked.tmp
    
    # Stage and commit some
    git add root.txt src/a.rs src/b.txt
    git commit -m "initial commit"
    
    # Stage but don't commit one
    echo "new" > staged_only.txt
    git add staged_only.txt
    
    # Leave untracked.tmp as untracked
    ```

## Scenarios

### Scenario 1: Basic File Removal (Tracked)

Steps:
1. From `$TEST_DIR/repo`, run `../GitRm root.txt`.
2. Verify:
   - `root.txt` is missing from disk.
   - `git status` shows `root.txt` as deleted and staged.

### Scenario 2: Staged-Only File Removal

Steps:
1. Run `../GitRm staged_only.txt`.
2. Verify:
   - `staged_only.txt` is missing from disk.
   - `git status` does not show `staged_only.txt` as a staged change (it's completely gone from index).

### Scenario 3: Untracked File Removal

Steps:
1. Run `../GitRm untracked.tmp`.
2. Verify:
   - `untracked.tmp` is missing from disk.
   - Index is unchanged (it wasn't tracked anyway).

### Scenario 4: Glob Matching (Recursive `**`)

Steps:
1. Run `../GitRm "src/**/*.rs"`.
2. Verify:
   - `src/a.rs` is missing from disk and index.
   - `src/nested/c.rs` is missing from disk and index.
   - `src/b.txt` remains on disk and index.

### Scenario 5: Directory Pattern

Steps:
1. Run `../GitRm "src/**"`.
2. Verify:
   - `src/` directory and all its contents are missing from disk.
   - All `src/` entries are removed from index.

### Scenario 6: Non-existent Pattern

Steps:
1. Run `../GitRm "no_such_file"`.
2. Verify:
   - Output indicates no files matched.
   - Exit code is non-zero (or handles as per implementation - current impl prints and returns Ok but we can adjust if needed).

### Scenario 7: .git Guardrail

Steps:
1. Attempt to remove .git: `../GitRm ".git/**"`.
2. Verify:
   - `.git` directory still exists and is functional.

## Success Criteria

- Files are deleted from disk in all cases (tracked, staged, untracked).
- Index is cleaned up for tracked and staged files.
- Glob patterns work as expected (recursive `**`, wildcards).
- `.git` directory is protected.
