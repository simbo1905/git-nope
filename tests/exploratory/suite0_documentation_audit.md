# Suite 0: Documentation & Build Audit

## Objective

Verify that the `git-nope` binary builds correctly in release mode, reports the correct version, responds to standard help flags, and that the internal help text aligns with the generated man pages.

## Working Directory

A timestamped directory in `.tmp/`, e.g., `.tmp/suite0_20240101_120000`.

## Setup

1.  **Build Release Binary**:
    Run the build command and verify success.
    ```bash
    cargo build --release
    # Verify binary exists at ./target/release/git-nope
    ```

2.  **Build Man Page**:
    Use the provided `docs/git-nope.1.md` (or similar source) to generate the man page, or use the existing one if committed.
    *Assumes standard tool `pandoc` or `asciidoctor` is available if generation is required, otherwise verifies the existing `docs/git-nope.1` file.*

3.  **Prepare Test Dir**:
    ```bash
    TIMESTAMP=$(date +%Y%m%d_%H%M%S)
    TEST_DIR=".tmp/suite0_${TIMESTAMP}"
    mkdir -p "$TEST_DIR"
    ```

## Scenarios

### Scenario 1: Version Flag

1.  Run `./target/release/git-nope --version`.
2.  Verify output format matches `git-nope X.Y.Z`.
3.  Check `Cargo.toml` to confirm the version matches the source.

### Scenario 2: Help Flags

1.  Run `./target/release/git-nope -h`.
2.  Run `./target/release/git-nope --help`.
3.  Verify both exit with 0.
4.  Verify they print usage information listing the Applet names (`GitAdd`, `GitRm`, etc.).

### Scenario 3: Man Page Consistency Audit

1.  **Read the Binary Help**: Capture the output of `./target/release/git-nope --help`.
2.  **Read the Man Page**: Read `docs/git-nope.1` (or render it with `man ./docs/git-nope.1`).
3.  **Cross-Check**:
    - Verify every applet listed in `--help` is documented in the man page.
    - Verify the "Refusal Mode" (exit code 42) described in the man page is mentioned or implied in the help text (or at least consistent with design).
    - Ensure argument syntax (e.g., `GitCommit -m <msg>`) matches between both.

### Scenario 4: Doc Source vs Man Page

1.  If `docs/git-nope.md` (Markdown source) exists, verify it matches the content of `docs/git-nope.1` (troff source).
2.  (Optional) Regenerate the man page if build scripts exist and compare with committed file to ensure it's up-to-date.

## Success Criteria

- Binary builds successfully.
- Version matches `Cargo.toml`.
- Help text is accurate and references the correct applet names.
- Man page documents all commands exposed by the binary.

## Failure Modes

- Version mismatch (forgot to bump Cargo.toml or internal const).
- `--help` is missing new commands.
- Man page refers to deprecated or non-existent commands.
