# GitAudit(1) — Git-Nope Manual

## NAME
GitAudit — classify repository cleanliness and upstream state with color-aware summaries.

## SYNOPSIS
`GitAudit [--no-colors] [-r]`

## DESCRIPTION
`GitAudit` reports the cleanliness of the current Git working tree using three distinct
states that map onto the repository workflows adopted by `git-nope`:

- **Clean** — no tracked modifications and no untracked files that escape `.gitignore`.
- **Dirty** — at least one tracked change is staged in the index or present in the worktree.
- **Tainted** — no tracked changes, but untracked files that are not ignored exist.

The tool prints a single-line summary containing the cleanliness state, decorated branch
information, and a short commit identifier. When the `-r` flag is provided, an additional
line summarises the relationship between the local branch and its upstream, using the
same color vocabulary.

## CLEANLINESS STATES
`GitAudit` evaluates repository paths via libgit2. The precedence of states is:

1. **Dirty** — any staged change or tracked worktree modification immediately classifies
   the repository as Dirty, even if untracked files are present.
2. **Tainted** — only entered when the index and tracked worktree are clean but at least
   one untracked, non-ignored path exists.
3. **Clean** — reached only when there are no staged changes, no tracked worktree
   modifications, and no untracked, non-ignored files.

The output includes repository identity (derived from the primary remote), current
branch, and tag decorations for the HEAD commit.

## REMOTE STATUS (`-r`)
When invoked with `-r`, `GitAudit` examines the configured upstream for the current
branch and emits a second line describing sync state:

- **Green — UpToDate**: local and upstream point to the same commit.
- **Yellow — Ahead / Behind**: branch has commits to push *or* fetch (but not both).
- **Red — Diverged**: branch is both ahead and behind its upstream.
- **NoUpstream**: displayed when no upstream branch is configured; the line is printed
  without emphasis and shows the remote URL as `<none>`.

## COLORS
Color output is enabled by default when the terminal supports ANSI escapes. It can be
disabled with `--no-colors` or by setting `GIT_NOPE_COLORS=false`. The CLI flag takes
precedence over the environment variable. When colors are disabled, plain text is emitted.

## ENVIRONMENT
- `GIT_NOPE_COLORS`: when set to a falsey value (`false`, `0`, `no`), disables color
  output unless `--no-colors` is explicitly overridden.

## EXIT STATUS
- `0` — success.
- Non-zero — misuse (e.g., outside a repository) or internal errors.

## SEE ALSO
`GitChanges`, `GitLog`, `GitRm`, `git-nope`
