# Suite 2: GitCommit Applet

## Objective
Verify GitCommit creates commits correctly when files are pre-staged.

## Working Directory
`.tmp/suite2_YYYYMMDD_HHMMSS`

## Setup
1. Build binary: `cargo build --release`
2. Create timestamped test dir:
   ```bash
   TIMESTAMP=$(date +%Y%m%d_%H%M%S)
   TEST_DIR=".tmp/suite2_${TIMESTAMP}"
   mkdir -p "$TEST_DIR" && cd "$TEST_DIR"
   ```
3. Init repo and stage a file using **system git**:
   ```bash
   git init repo && cd repo
   echo "hello" > test.txt
   git add test.txt
   ```
4. Symlink GitCommit:
   ```bash
   GIT_NOPE_BIN="$(pwd)/../../../target/release/git-nope"
   ln -s "$GIT_NOPE_BIN" ../GitCommit
   export PATH="$(pwd)/..:$PATH"
   ```

## Scenarios

### Scenario 1: Successful Commit
1. Run `GitCommit -m "Initial commit"`
2. Verify: `git log --oneline` shows "Initial commit"
3. Exit code: 0

### Scenario 2: Missing -m Flag
1. Run `GitCommit`
2. Verify: Error message about requiring `-m`
3. Exit code: non-zero

### Scenario 3: Unsupported Flag
1. Run `GitCommit --amend -m "Bad"`
2. Verify: Error about unsupported flag
3. Exit code: non-zero

## Success Criteria
- Commit created with correct message
- HEAD updated
- Guardrails reject missing/unsupported flags
