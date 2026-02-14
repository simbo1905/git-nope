# Agent Instructions

## Context
Read `README.md` for project overview and links to documentation.

## Navigation
- `bin/`: Contains executable tools. Use `ls bin/` to discover them.
- `docs/`: Contains architectural and design documents.
- `src/`: Source code.

## Build System
Use `make` to build the project.
- Run `make help` to discover available build targets.

## Rules
- Do not perform recursive searches unless necessary.
- Do not run destructive git commands (use provided applets).
- Follow instructions in `docs/DOCUMENT_STANDARDS.md`.

## Prohibited Actions
Agents must NEVER perform destructive or irreversible actions, including but not limited to:
- Any git command that discards work not yet committed or reachable from a ref.
- Any git command that rewrites, deletes, or force-pushes refs (branches, tags, stashes).
- `git rebase` without explicit user instruction in the current session.
- Any action framed as "cleanup" — stale branches, old tags, and unused files are
  not the agent's concern. Disk is vast; lost work is irreplaceable.

Normal development is not destruction: removing a committed file, deleting build
artifacts, or clearing intermediate outputs is routine and permitted.

When in doubt, do nothing. Prefer the safe applets in `bin/` over raw git commands.
