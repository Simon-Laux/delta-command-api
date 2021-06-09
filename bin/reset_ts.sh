#!/bin/bash
# ----- Setup ------
set -e # fail on error
# Make sure we are in the repo base directory
cd "$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )/.."
# ------------------
# This file sets up / resets the ts enviroment

cd typescript
# remove dependencies, they get sometimes broken
rm -r node_modules || true
# remove build artifacts
rm -r dist || true

npm install