# GitRm(1) — Git-Nope Manual

## NAME
GitRm — Aggressively remove files from the worktree and index.

## SYNOPSIS
`GitRm <path-or-glob>...`

## DESCRIPTION
**GitRm** is a specialized tool within the **git-nope** suite designed to simplify the removal of files and directories. Unlike the standard `git rm` command, which often requires additional flags or steps to handle staged changes or untracked files, **GitRm** follows a "make it gone" philosophy.

If you target a file or pattern with **GitRm**, it will:
1.  **Delete** the matching files and directories from the disk.
2.  **Remove** the corresponding entries from the Git index (staging area).

This tool is implemented as an applet within the `git-nope` binary. It is activated when the binary is invoked via a name matching `GitRm` (e.g., through a symlink or by renaming the binary).

## PATTERNS
**GitRm** supports shell-like glob patterns for all arguments. Every argument is treated as a pattern that can match multiple files or directories.

> **Note:** `GitRm` does not traverse symbolic links. Any symlink arguments are skipped.

- `*` : Matches any sequence of non-separator characters.
- `?` : Matches any single non-separator character.
- `**` : Matches any sequence of characters, including path separators (recursive match).
- `[abc]` : Matches any of the characters `a`, `b`, or `c`.
- `[a-z]` : Matches any character in the range `a` to `z`.

Arguments that do not contain metacharacters are treated as literal paths, but they are still evaluated through the globbing engine.

## BEHAVIOR
- **Tracked Files**: If a matched file is currently tracked by Git (even if it has staged changes or has never been committed), it is deleted from the disk and its entry is removed from the index.
- **Untracked Files**: If a matched file is not tracked by Git, it is still deleted from the disk.
- **Directories**: If a pattern matches a directory (e.g., `src/**`), **GitRm** recursively deletes all matching files and subdirectories and removes them from the index.
- **Safety**:
  - The `.git` directory is never traversed or modified.
  - Submodules (gitlinks) are currently skipped to prevent accidental recursive deletion of separate repositories.
  - Files are only removed from the index if the disk deletion succeeds (or if the file was already missing from the disk).
  - Symbolic links are skipped; `GitRm` does not follow them.

## EXIT STATUS
- **0** : Success. At least one file was matched and removed, and no errors occurred.
- **1** : Failure. This could be due to invalid patterns, permission errors, or if no files matched the provided patterns.

## EXAMPLES
Remove a specific file:
```bash
GitRm foo.txt
```

Remove all `.tmp` files recursively:
```bash
GitRm "**/*.tmp"
```

Remove an entire directory and all its contents:
```bash
GitRm "logs/**"
```

## SEE ALSO
`GitAdd`, `GitCommit`, `git-nope`
