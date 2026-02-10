# Suite 3: GitRm Aggressive & Explicit Path Operations

## Objective

Verify that `GitRm` correctly implements the "make it gone" semantics using explicit paths:
- Removes tracked, staged, and untracked files from disk.
- Removes matching entries from the Git index.
- Rejects glob patterns.
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

### Scenario 4: Directory Removal

Steps:
1. Run `../GitRm src`.
2. Verify:
   - `src/` directory and all its contents are missing from disk.
   - All `src/` entries are removed from index.

### Scenario 5: Non-existent Path

Steps:
1. Run `../GitRm "no_such_file"`.
2. Verify:
   - Output indicates no files matched ("No matching paths found").
   - Exit code is non-zero.

### Scenario 6: Glob Rejection & .git Guardrail

Steps:
1. Attempt to run with glob: `../GitRm ".git/**"`.
2. Verify:
   - Output states "GitRm does not accept glob patterns".
   - `.git` directory still exists and is functional.

### Scenario 7: Symlink Is Skipped (No Traversal)

Steps:
1. Create a real file and a symlink to it:
   ```bash
   echo "target" > symlink_target.txt
   ln -s symlink_target.txt link.txt
   ```
2. Run `../GitRm link.txt`.
3. Verify:
   - `link.txt` still exists (was skipped).
   - `symlink_target.txt` still exists.
   - Output includes "Skipping symlink".

### Scenario 8: Submodule Path Is Skipped

Steps:
1. Create a child repository and add it as a submodule:
   ```bash
   mkdir -p "$TEST_DIR/child"
   cd "$TEST_DIR/child"
   git init
   echo "child" > child.txt
   git add child.txt
   git commit -m "child init"
   cd "$TEST_DIR/repo"
   git submodule add "../child" submod
   git commit -m "add submodule"
   ```
2. Run `../GitRm submod`.
3. Verify:
   - `submod/` still exists.
   - `git status` does not show the submodule removed.

### Scenario 9: Path Outside Repo Is Rejected

Steps:
1. Create a file outside the repo:
   ```bash
   echo "outside" > "$TEST_DIR/outside.txt"
   ```
2. Run `../GitRm "$TEST_DIR/outside.txt"`.
3. Verify:
   - Output indicates path is outside of repository.
   - Exit code is non-zero (no changes were made).

## Success Criteria

- Files are deleted from disk in all cases (tracked, staged, untracked).
- Index is cleaned up for tracked and staged files.
- Glob patterns are rejected.
- `.git` directory is protected.
