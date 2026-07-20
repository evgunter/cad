#!/usr/bin/env bash
# External watertight/manifold verification of exported STLs (M2 PR 7).
#
# Runs admesh in CHECK-ONLY spirit: admesh always *reports* what it
# would fix; we accept NO repair as success — any disconnected
# (non-manifold) facet, degenerate facet, removed/added/reversed
# facet, fixed edge/normal, or a part count != 1 fails the gate.
#
# Usage: scripts/check_admesh.sh <dir-with-stl-files>
set -euo pipefail

dir="${1:?usage: check_admesh.sh <dir>}"
shopt -s nullglob
files=("$dir"/*.stl)
if [ ${#files[@]} -eq 0 ]; then
    echo "check_admesh: no .stl files in $dir" >&2
    exit 1
fi

fail=0
for f in "${files[@]}"; do
    echo "== admesh: $f"
    out="$(admesh "$f")"
    echo "$out"
    # Extract "<label> : <original> <final>" style counters; assert the
    # ORIGINAL side is already clean (no repair accepted).
    check_zero() {
        local label="$1"
        local val
        val="$(printf '%s\n' "$out" | sed -n "s/^ *${label} *: *\([0-9][0-9]*\).*$/\1/p" | head -1)"
        if [ -z "$val" ]; then
            echo "FAIL($f): counter '$label' not found in admesh report" >&2
            fail=1
        elif [ "$val" != "0" ]; then
            echo "FAIL($f): $label = $val (expected 0)" >&2
            fail=1
        fi
    }
    check_zero "Facets with 1 disconnected edge"
    check_zero "Facets with 2 disconnected edges"
    check_zero "Facets with 3 disconnected edges"
    check_zero "Total disconnected facets"
    check_zero "Degenerate facets"
    check_zero "Edges fixed"
    check_zero "Facets removed"
    check_zero "Facets added"
    check_zero "Facets reversed"
    check_zero "Backwards edges"
    check_zero "Normals fixed"
    parts="$(printf '%s\n' "$out" | sed -n 's/^ *Number of parts *: *\([0-9][0-9]*\).*$/\1/p' | head -1)"
    if [ "$parts" != "1" ]; then
        echo "FAIL($f): Number of parts = ${parts:-missing} (expected 1)" >&2
        fail=1
    fi
done
exit $fail
