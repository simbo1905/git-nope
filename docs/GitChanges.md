# GitChanges(1) — Git-Nope Manual

## NAME
GitChanges — list modified and untracked files in porcelain format.

## SYNOPSIS
`GitChanges`

## DESCRIPTION
`GitChanges` provides a machine-readable listing of all changes in the working tree,
including staged modifications, unstaged changes, and untracked files. The output format
matches `git status --porcelain`, making it suitable for scripting and automation.

Each line represents one file, prefixed by a two-character status code:
- First character: index (staging area) status
- Second character: worktree status

## OUTPUT FORMAT
The output follows the porcelain v1 format:

```
XY PATH
```

Where `X` and `Y` are status codes:
- `M` — modified
- `A` — added
- `D` — deleted
- `R` — renamed
- `C` — copied
- `U` — updated but unmerged
- `?` — untracked
- `!` — ignored (not shown by default)
- ` ` (space) — unmodified

## EXAMPLES
```
M  Cargo.lock        # Modified in index only
 M src/main.rs       # Modified in worktree only
MM src/lib.rs        # Modified in both index and worktree
A  new_file.rs       # Added to index
D  old_file.rs       # Deleted from index
?? untracked.rs      # Untracked file
```

## USE CASES
`GitChanges` is designed to support agent-based workflows that need to:
- List only tracked and staged files (excluding untracked clutter)
- Parse repository state programmatically
- Filter changes by status for selective operations

Unlike `GitAudit`, which provides a human-readable summary, `GitChanges` offers the
raw data needed for automation.

## EXIT STATUS
- `0` — success
- Non-zero — misuse (e.g., outside a repository) or internal errors

## SEE ALSO
`GitAudit`, `GitAdd`, `GitRm`, `git-nope`
