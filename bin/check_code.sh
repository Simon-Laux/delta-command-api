#!/bin/bash
# ----- Setup ------
set -e # fail on error
# Make sure we are in the repo base directory
cd "$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )/.."
# ------------------

echo "[Cargo Check]"
cargo check --all-targets

echo "[Check typescript]"
cd typescript
npx tsc

# TODO/IDEA MAYBE add eslint linting
# (later becasue now there might be too much unused in the development process)