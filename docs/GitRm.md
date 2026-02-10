# GitRm(1) — Git-Nope Manual

## NAME
GitRm — Aggressively remove files from the worktree and index.

## SYNOPSIS
`GitRm <path>`

## DESCRIPTION
**GitRm** is a specialized tool within the **git-nope** suite designed to simplify the removal of files and directories. Unlike the standard `git rm` command, which often requires additional flags or steps to handle staged changes or untracked files, **GitRm** follows a "make it gone" philosophy.

If you target a file or directory with **GitRm**, it will:
1.  **Delete** the specified file or directory from the disk.
2.  **Remove** the corresponding entry from the Git index (staging area).

This tool is implemented as an applet within the `git-nope` binary. It is activated when the binary is invoked via a name matching `GitRm` (e.g., through a symlink or by renaming the binary).

## PATHS
`GitRm` requires a single explicit path. Multiple paths are rejected to minimize the risk of accidental mass deletions. Glob patterns (containing `*`, `?`, `[`, or `]`) are also rejected.

> **Note:** `GitRm` does not traverse symbolic links. Any symlink arguments are skipped.

## BEHAVIOR
- **Tracked Files**: If a matched file is currently tracked by Git (even if it has staged changes or has never been committed), it is deleted from the disk and its entry is removed from the index.
- **Untracked Files**: If a matched file is not tracked by Git, it is still deleted from the disk.
- **Directories**: When given an explicit directory path, **GitRm** recursively deletes the directory and its contents, removing them from the index.
- **Safety**:
  - The `.git` directory is never traversed or modified.
  - Submodules (gitlinks) are currently skipped to prevent accidental recursive deletion of separate repositories.
  - Files are only removed from the index if the disk deletion succeeds (or if the file was already missing from the disk).
  - Symbolic links are skipped; `GitRm` does not follow them.

## EXIT STATUS
- **0** : Success. The file or directory was matched and removed, and no errors occurred.
- **1** : Failure. This could be due to an invalid path, multiple paths provided, permission errors, or if no file matched the provided path.

## EXAMPLES
Remove a specific file:
```bash
GitRm foo.txt
```

Remove an entire directory and all its contents by providing its explicit path:
```bash
GitRm logs
```

## SEE ALSO
`GitAdd`, `GitCommit`, `git-nope`
