#!/usr/bin/env bash
# External FreeCAD/OCC import acceptance for exported STEP files
# (M4 PR 7) — the admesh pattern, STEP-shaped: an independent CAD
# system imports every committed fixture and we assert validity plus
# exact topology counts and volume (expectations live in the
# per-fixture .expect sidecars; the fixtures themselves are byte-golden
# against the writer via the cargo test suite).
#
# Usage: scripts/check_step.sh [dir-with-step-files]
#   (default: crates/step-export/tests/fixtures)
#
# FreeCAD discovery: $FREECADCMD if set, else the documented local
# install. When absent the check SKIPS LOUDLY with exit 0 so the test
# suite and gate stay hermetic — set REQUIRE_FREECAD=1 to make absence
# a failure (e.g. on machines known to have it).
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
dir="${1:-$repo_root/crates/step-export/tests/fixtures}"
freecadcmd="${FREECADCMD:-$HOME/.local/share/cad-work/freecad/squashfs-root/usr/bin/freecadcmd}"

if ! [ -x "$freecadcmd" ]; then
    echo "check_step: SKIP — freecadcmd not found at '$freecadcmd'" >&2
    echo "check_step: set FREECADCMD to a FreeCAD headless binary to run this check" >&2
    if [ "${REQUIRE_FREECAD:-0}" = "1" ]; then
        echo "check_step: REQUIRE_FREECAD=1, treating absence as failure" >&2
        exit 1
    fi
    exit 0
fi

shopt -s nullglob
files=("$dir"/*.step)
if [ ${#files[@]} -eq 0 ]; then
    echo "check_step: no .step files in $dir" >&2
    exit 1
fi

fail=0
for f in "${files[@]}"; do
    echo "== freecad import: $f"
    expect="${f%.step}.expect"
    if [ ! -f "$expect" ]; then
        echo "FAIL($f): missing expectations sidecar $expect" >&2
        fail=1
        continue
    fi
    # Sidecar format: KEY=VALUE lines (EXPECT_SOLIDS=..., etc.).
    if ! env $(grep -E '^EXPECT_[A-Z0-9_]+=' "$expect" | xargs) \
        "$freecadcmd" "$repo_root/scripts/step_import_check.py" "$f"; then
        fail=1
    fi
done
exit $fail
