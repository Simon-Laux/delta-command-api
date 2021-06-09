#!/bin/bash
# ----- Setup ------
set -e # fail on error
# Make sure we are in the repo base directory
cd "$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )/.."
# ------------------

# fix rust formatting
cargo fmt

# fix typescript/javascript formatting
cd typescript
npx prettier --write "src/**/*.{js,ts,ts,html}" --loglevel warn