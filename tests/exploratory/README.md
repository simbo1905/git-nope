# Exploratory Test Suites for git-nope

## Overview

These test suites are designed to be executed by AI coding agents (or humans) to manually verify `git-nope` functionality.

## Philosophy

1.  **Documentation is the spec** - The tool must behave exactly as described in `docs/git-nope.md`.
2.  **Isolation** - Tests run in temporary directories in `.tmp/`.
3.  **Consistent State** - We clone a known "demo" repo to ensure predictable starting conditions.

## Suites

| Suite | File | Focus |
|-------|------|-------|
| 0 | `tests/exploratory/suite0_documentation_audit.md` | Build verification, help flags, version checks, and man page consistency. |
| 1 | `tests/exploratory/suite1_basic_ops.md` | Basic operations: Applets, `git nope`, and Refusal Mode. |

## Running a Suite

1.  Read the suite Markdown file (e.g., `tests/exploratory/suite1_basic_ops.md`).
2.  Follow the **Setup** instructions carefully (creating `.tmp` dirs, cloning repos).
3.  Execute the **Scenarios**.
4.  Verify against **Success Criteria**.

## Cleanup

```bash
rm -rf .tmp/suite*
```
