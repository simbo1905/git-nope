# Makefile Usage

The `Makefile` is the primary interface for building, cleaning, and testing the `git-nope` project.

## Usage

Run `make help` to see a list of available targets and their descriptions.

## Build Artifacts

Build artifacts are placed in the `dist/` directory.

- **Clean Builds**: Named `dist/git-nope_<commit_sha>`
- **Dirty Builds**: Named `dist/git-nope_<commit_sha>_<file_hash>`

## Release Process

To create a release binary:
1. Ensure the working tree is clean (`git status` shows no changes).
2. Run `make clean build`.
3. The resulting binary in `dist/` will be named with the commit SHA and is ready for distribution.
