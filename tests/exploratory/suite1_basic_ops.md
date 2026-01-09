# Suite 1: Basic Operations - Applets & Guardrails

## Objective

Verify that `git-nope` correctly implements the applet behaviors (`GitAdd`, `GitRm`, etc.), the `git nope` command, and the refusal mechanism when invoked as `git`.

## Working Directory

A timestamped directory in `.tmp/`, e.g., `.tmp/suite1_20240101_120000`.

## Setup

1.  **Build the binary**:
    Ensure you have the `git-nope` binary built.
    ```bash
    cargo build --release
    # Binary is at ./target/release/git-nope
    ```

2.  **Prepare Test Environment**:
    Create a fresh timestamped directory and clone the demo repo.
    ```bash
    # Set timestamp variable
    TIMESTAMP=$(date +%Y%m%d_%H%M%S)
    TEST_DIR=".tmp/suite1_${TIMESTAMP}"
    mkdir -p "$TEST_DIR"
    
    # Clone the demo repo into the test dir
    git clone https://github.com/simbo1905/agt-demo.git "$TEST_DIR/repo"
    cd "$TEST_DIR/repo"
    ```

3.  **Setup Applet Symlinks**:
    Create a `bin` directory and symlink `git-nope` to the applet names.
    ```bash
    mkdir -p ../bin
    GIT_NOPE_BIN="$(pwd)/../../../target/release/git-nope" # Adjust path to repo root if needed
    
    ln -s "$GIT_NOPE_BIN" ../bin/git-nope
    ln -s "$GIT_NOPE_BIN" ../bin/GitAdd
    ln -s "$GIT_NOPE_BIN" ../bin/GitRm
    ln -s "$GIT_NOPE_BIN" ../bin/GitAddAll
    ln -s "$GIT_NOPE_BIN" ../bin/GitAddDot
    ln -s "$GIT_NOPE_BIN" ../bin/GitCommit
    ln -s "$GIT_NOPE_BIN" ../bin/git # For refusal mode test
    
    # Add to PATH
    export PATH="$(pwd)/../bin:$PATH"
    ```

## Reference

Read `docs/git-nope.md` for expected behavior of each command.

## Scenarios

### Scenario 1: `git nope` (Dashed Command)

Verify that `git nope` works when `git-nope` is on the PATH.
*Note: This requires the real `git` to be available and `git-nope` to be in PATH.*

Steps:
1. Ensure real `git` is first in PATH or call it directly (e.g. `/usr/bin/git`).
2. Run `git nope`.
3. Verify output is "Nope" and exit code is 0.

### Scenario 2: Applet `GitAdd`

Steps:
1. Modify an existing file (e.g., `README.md`).
   ```bash
   echo "update" >> README.md
   ```
2. Run `GitAdd README.md`.
3. Verify file is staged: `/usr/bin/git status`.

### Scenario 3: Applet `GitRm`

Steps:
1. Create a dummy file and commit it (using real git for setup if needed, or use existing file).
2. Run `GitRm <file>`.
3. Verify file is removed and deletion is staged.

### Scenario 4: Applet `GitAddAll`

Steps:
1. Modify a file.
2. Delete a file.
3. Create a new file.
4. Run `GitAddAll`.
5. Verify all changes (modifications, deletions, new files) are staged.

### Scenario 5: Applet `GitAddDot`

Steps:
1. Create a subdirectory `subdir/`.
2. Create/modify files in `subdir/` and in root.
3. `cd subdir`
4. Run `GitAddDot`.
5. Verify only changes in `subdir/` are staged.

### Scenario 6: Applet `GitCommit`

Steps:
1. Stage some changes (using `GitAdd`).
2. Run `GitCommit -m "Test commit"`.
3. Verify commit is created with message "Test commit".
4. **Guardrail Test**: Run `GitCommit` (no args). Verify it fails/refuses (requires `-m`).

### Scenario 7: Refusal Mode (Invoked as `git`)

Verify that the tool refuses to act as a general `git` replacement.

Steps:
1. Use the `git` symlink created in Setup (ensure it maps to `git-nope`).
   ```bash
   # Verify we are running the fake git
   which git 
   # Should be .../bin/git
   ```
2. Run `git status`.
3. Verify:
   - Output contains "Nope".
   - Stderr explains it's `git-nope` and lists allowed applets.
   - Exit code is 42.

### Scenario 8: `git nope` (Refusal Mode)

Even when masquerading as `git`, `git nope` should still work.

Steps:
1. Run `git nope` (using the `git` symlink).
2. Verify output is "Nope" and exit code is 0.

## Success Criteria

- All Applets perform their specific Git task successfully.
- `git nope` always prints "Nope" and exits 0.
- `git <any_other_command>` prints "Nope", help text, and exits 42 when `argv[0]` is `git`.

## Failure Modes

- `git-nope` executes arbitrary git commands (e.g. `git status` works).
- Applets fail to perform the git operation.
- Applets allow unsafe flags (e.g., `GitCommit --amend` if not allowed).
