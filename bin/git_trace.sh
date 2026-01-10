#!/bin/sh

# Function to display help message
usage() {
    cat <<EOF
Usage: $(basename "$0") [OPTIONS] [GIT_COMMAND] [ARGS...]

Wrapper script to run git commands with GIT_TRACE2_EVENT=1 enabled.
This generates structured trace events helpful for debugging and analysis.

Options:
  -h, --help    Show this help message and exit

Examples:
  $(basename "$0") add file.txt
  $(basename "$0") status
  $(basename "$0") commit -m "fix: bug"

Output:
  Trace output is written to stderr. You can redirect it as needed:
  $(basename "$0") status 2> trace.log

Environment:
  Sets GIT_TRACE2_EVENT=1 before executing the provided git command.
EOF
}

# Check for help flag or no arguments
if [ "$#" -eq 0 ]; then
    usage
    exit 1
fi

case "$1" in
    -h|--help)
        usage
        exit 0
        ;;
esac

# Execute git with tracing enabled
export GIT_TRACE2_EVENT=1
exec git "$@"
