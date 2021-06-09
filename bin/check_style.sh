#!/bin/bash
# ----- Setup ------
set -e # fail on error
# Make sure we are in the repo base directory
cd "$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )/.."
# ------------------

echo "[Check rustfmt]"
cargo fmt -- --check

echo "[Check prettier in typescript project]"
cd typescript
npx prettier --check "src/**/*.{js,ts,ts,html}"