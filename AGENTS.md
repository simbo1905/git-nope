# Agent Instructions

## Context
Read `README.md` for project overview and links to documentation.

## Navigation
- `bin/`: Contains executable tools. Use `ls bin/` to discover them.
- `docs/`: Contains architectural and design documents.
- `src/`: Source code.

## Build System
Use `make` to build the project.
- `make help`: List build targets.
- `make build`: Create release binaries in `dist/`.

## Rules
- Do not perform recursive searches unless necessary.
- Do not run destructive git commands (use provided applets).
- Follow instructions in `docs/DOCUMENT_STANDARDS.md`.
