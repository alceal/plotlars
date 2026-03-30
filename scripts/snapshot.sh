#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_DIR"

echo "Building snapshot binary..."
cargo build -p plotlars --bin snapshot 2>&1

echo "Running snapshot binary..."
cargo run -p plotlars --bin snapshot 2>&1

echo "Snapshots written to snapshots/"

# If a baseline directory exists, diff against it
if [ -d "snapshots-baseline" ]; then
    echo ""
    echo "Diffing against baseline..."
    if diff -r snapshots/ snapshots-baseline/ > /dev/null 2>&1; then
        echo "OK: No regressions detected."
    else
        echo "REGRESSION DETECTED: Differences found."
        diff -r snapshots/ snapshots-baseline/ || true
        exit 1
    fi
else
    echo ""
    echo "No baseline found. To create one:"
    echo "  cp -r snapshots/ snapshots-baseline/"
fi
