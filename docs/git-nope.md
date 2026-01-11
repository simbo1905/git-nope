# git-nope

A Git wrapper with strong guardrails for agent workflows. Provides only a minimal set of Git operations when invoked under specific "applet" names, and refuses to act as full git.

## NAME

git-nope — safe Git facade for agents; only permits GitAdd, GitRm, GitAddAll, GitAddDot, GitCommit. Also provides a deliberate `git nope` subcommand that prints "Nope" and exits 0.

## SYNOPSIS

### Git dashed-command mode (recommended human UX)
```
git nope
```

### Direct invocation
```
git-nope
git-nope <args...>
```

### Agent / applet invocation (argv[0]-driven)
```
GitAdd [path...]
GitRm <path> [<path>...]
GitAddAll
GitAddDot
GitCommit -m <message>
```

`git-nope` selects its behavior based on `argv[0]` (how it was invoked), BusyBox-style.

## DESCRIPTION

git-nope is designed to stop LLM agents (and fast "test" agents in particular) from running destructive Git operations like `git reset`, `git checkout`, `git rebase`, branch manipulation, etc.

It does this by:
- Not attempting to emulate git in general
- Allowing only a small set of safe operations under explicit applet names:
  - GitAdd (stage paths)
  - GitAddAll (stage entire repo, including deletions)
  - GitAddDot (stage current directory subtree)
  - GitRm (remove explicit files)
  - GitCommit (commit with explicit message)
- Refusing to traverse symbolic links, submodules, or worktrees. Agent applets operate only on regular files and directories inside the primary worktree.

It also implements a deliberate novelty / sentinel command:
- `git nope` → prints "Nope" and exits 0 (success)

## INVOCATION & DISPATCH RULES

### 1) When invoked as git-nope (standalone)
Behavior is the same as when invoked as `git` (see below): it refuses, unless explicitly treated as the nope command by argument rules (optional - see next section).

### 2) When invoked via Git's dashed-command mechanism
If git-nope is on PATH and named git-nope, then:
```
git nope
```
will execute git-nope.

In this mode:
If invoked as git-nope via `git nope`, it should:
- print "Nope"
- exit 0

This makes `git nope` an intentional "successfully did nothing" operation.

### 3) When invoked as an agent applet name (argv[0])
When argv[0] matches:
- GitAdd
- GitRm
- GitAddAll
- GitAddDot
- GitCommit

git-nope performs that operation.

## REFUSAL MODE (MASKING FULL GIT)

When invoked as `git`

If the binary is named `git` (or symlinked as `git`) and invoked as:
```
git <anything>
```

then:
- If exactly one parameter and it is "nope":
  - stdout: "Nope"
  - exit: 0
  - stderr: empty (or minimal; up to you)

Otherwise (anything other than the single-arg nope case):
- stdout: "Nope, use GitAdd, GitCommit, ..." listing the full set of applets. 
- stderr: diagnostic text:
  - git-nope version X.Y.Z
  - explains it is there to block direct use of general git commands.
  - lists supported applet names (GitAdd, GitAddAll, GitAddDot, GitRm, GitCommit)
  - points to the main git repo docs
- exit: 42

This is specifically so you can symlink the tool to `git` and add it to the agent's PATH. It will then tell the agent "Nope, use GitAdd, GitRm,...". 

## COMMANDS (AGENT APPLETS)

### GitAdd
Stages paths.

Recommended semantics: equivalent to `git add -- <path...>`.

If no paths are provided, you may choose either:
- behave like `git add .` (stage subtree), or
- refuse and require explicit GitAddDot / GitAddAll

(Recommend: require explicit and print guidance.)

### GitAddAll
Stages all changes in the repository, including deletions.

Equivalent to:
```
git add -A
```

### GitAddDot
Stages changes in the current directory and below.

Equivalent to:
```
git add .
```

### GitRm
Removes tracked files and stages deletions.

Equivalent to:
```
git rm -- <path>...
```

Strongly recommended guardrail:
- require explicit paths (no globs)
- reject arguments containing `* ? [` to reduce accidental mass deletion

### GitCommit
Creates a commit.

Recommended minimal interface:
```
GitCommit -m "message"
```

Guardrail recommendation:
- require `-m`
- no editor invocation
- no advanced options

## EXIT STATUS

| Code | Meaning |
|------|---------|
| 0 | Success (including deliberate git nope) |
| 42 | Refused: attempted to use git for anything except git nope |

Other non-zero codes may be used for operational failures (not in repo, lock file, underlying git failed), but 42 is reserved for "policy refusal".

## OUTPUT CONVENTIONS

### Refusal (invoked as git, disallowed usage)
- stdout: "Nope" (always)
- stderr: explanation + allowed commands
- exit: 42

### Allowed git nope
- stdout: "Nope"
- exit: 0

### Applets
stdout/stderr follow underlying git conventions.

## EXAMPLES

### Put real git first, and git-nope later (for git nope only)
```bash
# PATH: ...:/usr/bin (git) ...:/opt/git-nope/bin (git-nope)
git nope  # -> runs git-nope via git's dashed-command dispatch
# stdout: Nope
# exit: 0
```

### Agent applets (argv[0] dispatch)
```bash
ln -s /opt/git-nope/bin/git-nope /opt/git-nope/bin/GitAdd
ln -s /opt/git-nope/bin/git-nope /opt/git-nope/bin/GitAddAll
ln -s /opt/git-nope/bin/git-nope /opt/git-nope/bin/GitAddDot
ln -s /opt/git-nope/bin/git-nope /opt/git-nope/bin/GitRm
ln -s /opt/git-nope/bin/git-nope /opt/git-nope/bin/GitCommit

GitAddDot
GitCommit -m "Update docs"
```

### Mask git (strict mode)
```bash
ln -s /opt/git-nope/bin/git-nope /usr/local/bin/git

git status  # stdout: Nope
            # stderr: git-nope version ...
            # exit: 42
```

## NOTES / DESIGN RATIONALE

- This is policy, not a hard security boundary.
- It can be bypassed if the real git is invoked via absolute path, another tool, or different environment. Pair with sandboxing / allowlists for stronger enforcement.
- `git nope` is intentionally non-error: it provides a clean "do nothing" response that is semantically correct and script-friendly.
