#!/bin/bash
# Modified to run commands directly without spawning a terminal
# This allows pkexec to show authentication popup without terminal window

# If arguments are provided, execute them directly as a command
if [ $# -gt 0 ]; then
    # Execute the command directly without spawning interactive shell
    exec bash -c "$*"
else
    # If no arguments, just run bash non-interactively (should not happen)
    exec bash --norc --noprofile
fi
