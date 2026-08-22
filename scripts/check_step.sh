#!/usr/bin/env bash
# External FreeCAD/OCC import acceptance for exported STEP files
# (M4 PR 7) — the admesh pattern, STEP-shaped: an independent CAD
# system imports every committed fixture and we assert validity plus
# exact topology counts and volume (expectations live in the
# per-fixture .expect sidecars; the fixtures themselves are byte-golden
# against the writer via the cargo test suite). A fixture may also carry
# a sibling <fixture>.probe.py, run under the same interpreter with
# $STEP_FILE set, for geometric facts the generic count/volume checks
# cannot state.
#
# Usage: scripts/check_step.sh [dir-with-step-files]
#   (default: crates/step-export/tests/fixtures)
#
# FreeCAD discovery: $FREECADCMD if set, else the documented local
# install. Off the gate of record — a developer's box — an absent binary
# SKIPS LOUDLY with exit 0 so the test suite and gate stay hermetic, and
# REQUIRE_FREECAD=1 promotes that skip to a failure on a machine known to
# have it.
#
# ON THE GATE OF RECORD A MISSING FreeCAD IS FATAL, AND THAT DOES NOT REST ON
# A FLAG. `GITHUB_ACTIONS` is set by the runner itself: no edit to this repo
# can unset it, which is exactly what an edit to `REQUIRE_FREECAD=1` in ci.yml
# could do. Deleting that assignment used to turn this row into "SKIP, exit 0,
# no STEP fixture verified" on every PR thereafter, with nothing reading the
# flag to notice — a check that does not run LESS often but stops running at
# all. The declaration is kept anyway and is CHECKED below, for the reason it
# was written: it is the workflow saying out loud, where a reader of the job
# meets it, that this row is not allowed to skip.
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
dir="${1:-$repo_root/crates/step-export/tests/fixtures}"
freecadcmd="${FREECADCMD:-$HOME/.local/share/cad-work/freecad/squashfs-root/usr/bin/freecadcmd}"

# The declaration is a marker, so it is checked. Unchecked it is a claim that
# a checker is mandatory with nothing enforcing it — this row's own defect,
# re-minted inside the fix for it. Runs BEFORE the discovery below, so it fires
# on a runner that does have FreeCAD too.
if [ -n "${GITHUB_ACTIONS:-}" ] && [ "${REQUIRE_FREECAD:-}" != "1" ]; then
    echo "check_step: running under GITHUB_ACTIONS with REQUIRE_FREECAD='${REQUIRE_FREECAD:-}';" >&2
    echo "check_step: on the gate of record it must be \"1\". That line in .github/workflows/ci.yml's" >&2
    echo "check_step: step-import step is the workflow saying out loud that a missing FreeCAD fails" >&2
    echo "check_step: this job, and this check is what keeps it from becoming a sentence nothing" >&2
    echo "check_step: enforces. FreeCAD is required here either way — the requirement is read from" >&2
    echo "check_step: GITHUB_ACTIONS, not from this variable — so restore the declaration rather" >&2
    echo "check_step: than working around it." >&2
    exit 1
fi

if ! [ -x "$freecadcmd" ]; then
    echo "check_step: SKIP — freecadcmd not found at '$freecadcmd'" >&2
    echo "check_step: set FREECADCMD to a FreeCAD headless binary to run this check" >&2
    if [ -n "${GITHUB_ACTIONS:-}" ]; then
        echo "check_step: this is hosted CI, the gate of record — a missing FreeCAD is a failure" >&2
        echo "check_step: here, and no repo edit can make it a skip." >&2
        exit 1
    fi
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
    # Sidecar format: KEY=VALUE lines (EXPECT_SOLIDS=..., etc.). The
    # file path travels via $STEP_FILE — a positional path would make
    # freecadcmd import the file a SECOND time as a document (see
    # step_import_check.py's header).
    # KERNEL_* lines (the native kernel census, consumed by the cargo
    # suites) do not match the EXPECT_ grep and are ignored here.
    if ! env $(grep -E '^EXPECT_[A-Z0-9_]+=' "$expect" | xargs) STEP_FILE="$f" \
        "$freecadcmd" "$repo_root/scripts/step_import_check.py"; then
        fail=1
    fi
    # Optional per-fixture geometric probe: a sibling <fixture>.probe.py
    # runs under the same interpreter with $STEP_FILE set. Counts and
    # volume are generic; a probe is for facts only that fixture can
    # state (e.g. nurbs_wireframe's: OCC must reconstruct the rational
    # record's exact conic, weights honoured).
    probe="${f%.step}.probe.py"
    if [ -f "$probe" ]; then
        if ! STEP_FILE="$f" "$freecadcmd" "$probe"; then
            echo "FAIL($f): geometric probe $probe" >&2
            fail=1
        fi
    fi
done
exit $fail
