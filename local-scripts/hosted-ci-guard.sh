# shellcheck shell=bash
# HOSTED CI IS THE MERGE GATE. Sourced by every local full-matrix entry
# point (ci-local.sh, gate.sh) as its first act.
#
# Deliberately modelled on demos/hosted-render-guard.sh, which solved the
# same problem for renders. Read that file for the reasoning; the three
# design choices are reproduced here because they are the point:
#
# WHY A GUARD AND NOT A README LINE. The full matrix is the heaviest thing
# this repo asks of a developer box — it self-acquires ALL machine build
# slots (with-build-slot.sh), runs every row sequentially against one
# shared target/, and on a cold cache rebuilds the workspace several times
# over. ci-local.sh's own header dates its mandate to a specific outage
# ("the merge gate while hosted Actions is unavailable — GitHub free-plan
# minutes exhausted, 2026-07-22"). That outage is over. Hosted CI has been
# the gate since 2026-07-25 (see local-scripts/gate.sh's STATUS line), and
# `memories/local-battery-scope.md` states the standing rule: local testing
# is an ITERATION-SPEED tool, hosted CI is the only gate. A pointer in a
# header is advice, and advice loses to muscle memory. This exits nonzero
# instead.
#
# THE OVERRIDE IS A SENTENCE, DELIBERATELY. `CAD_LOCAL_CI_OVERRIDE` must
# equal the exact string below — not "1", not "yes", not "true". Those are
# values an agent or a developer reaches for reflexively when a script
# complains about an environment variable; a sentence naming what you are
# certifying is one nobody types by accident, and one that reads as a claim
# in the shell history that produced the run.
#
# THE RULE IS STRUCTURAL, NOT SNIFFED. There is no GITHUB_ACTIONS check
# here on purpose — hosted CI does not call these scripts at all (verified:
# ci.yml mirrors the matrix, it does not invoke the mirror), and a sniffed
# exemption is invisible at the call site and grows silently with every new
# runner and local emulator.
#
# WHAT THIS DOES NOT GATE. Targeted local runs are the encouraged path and
# are untouched: `cargo nextest run -p <crate>`, local-scripts/test-fast.sh,
# scripts/doc-gate.sh, a single ci-local row run by hand. The guard is on
# the WHOLE-MATRIX entry points only. Per memories/local-battery-scope.md,
# scoping the local battery to the change shape is the rule; this guard
# enforces the ceiling, not the floor.
CAD_LOCAL_CI_OVERRIDE_SENTENCE='i-certify-hosted-ci-is-unavailable'

# $1: the entry point's name, for the message. Returns (0) only when the
# override is set to the exact sentence; otherwise prints the pointer and
# EXITS nonzero from the calling script.
require_hosted_ci() {
    local entry="$1"
    local got="${CAD_LOCAL_CI_OVERRIDE:-}"

    if [ "$got" = "$CAD_LOCAL_CI_OVERRIDE_SENTENCE" ]; then
        echo "[$entry] LOCAL FULL-MATRIX OVERRIDE in effect." >&2
        echo "[$entry]   You have certified that hosted CI is unavailable." >&2
        echo "[$entry]   This run takes every build slot on the machine and is" >&2
        echo "[$entry]   NOT the merge gate — hosted CI still is, the moment it" >&2
        echo "[$entry]   is back. Re-push and let it run before merging." >&2
        return 0
    fi

    {
        echo
        echo "REFUSING: hosted CI is the merge gate. $entry is the fallback."
        echo
        echo "THE DEFAULT WAY TO RUN THE FULL MATRIX IS TO PUSH."
        echo "ci.yml runs the whole matrix on the PR's merge ref, in parallel,"
        echo "on hardware that is not yours:"
        echo
        echo "  git push          # PR checks green = mergeable"
        echo
        echo "This script runs the same rows SEQUENTIALLY against one shared"
        echo "target/, and self-acquires every build slot on this machine — so"
        echo "it also blocks every other lane on the box for its duration."
        echo
        echo "FOR ITERATION, RUN SOMETHING SMALLER. That is the encouraged path"
        echo "and it is not gated (memories/local-battery-scope.md — scope the"
        echo "local battery to the change shape, by time-to-signal):"
        echo
        echo "  cargo nextest run -p <crate>          # the crate you touched"
        echo "  local-scripts/test-fast.sh            # warm-cache fast suite"
        echo "  scripts/doc-gate.sh                   # rustdoc only"
        echo "  cargo clippy --workspace --all-targets -- -D warnings"
        echo
        echo "IF HOSTED CI IS ACTUALLY DOWN (billing outage, Actions incident,"
        echo "no network) this script is the fallback gate. Say so explicitly:"
        echo
        echo "  CAD_LOCAL_CI_OVERRIDE=$CAD_LOCAL_CI_OVERRIDE_SENTENCE $entry"
        echo
        echo "Spelled out in full on purpose — see local-scripts/hosted-ci-guard.sh."
        echo "It is a certification, not a switch: it says you checked, not that"
        echo "you would rather not push."
        echo
        if [ -n "$got" ]; then
            echo "(CAD_LOCAL_CI_OVERRIDE is set to '$got', which is not the"
            echo "sentence above.)"
            echo
        fi
    } >&2
    exit 1
}
