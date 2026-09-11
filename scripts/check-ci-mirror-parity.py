#!/usr/bin/env python3
"""The two halves of CI run the same checks WITH THE SAME REPORTING SEMANTICS, the
gates that must fire on docs-tier inputs are SITED where they can, and the prune
exception stays one job.

WHY THIS IS PYTHON AND NOT A `scripts/gates/*.sh` MEMBER. Its subject is the
STRUCTURE of `.github/workflows/*.yml` — which job a step belongs to, whether
that job carries an `if:`, whether a prune step precedes a step that reads the
pruned tree. The first version of this check was bash, and read that structure
with `awk '/^  [a-z0-9_-]+:$/'`. A reviewer planted a job named `buildXtra:`
with a checkout and a read of `local-scripts/`: the matcher did not see an
uppercase letter as a job start, folded the steps into the PREVIOUS job, and the
gate exited 0. The claim was stated as total and was approximately total, which
is the failure mode this whole track is about. `scripts/gates/`'s own roster
argument (see `gate-roster.sh`) is that the directory means `lib.sh`'s two-mode
bash contract, and that a python check either reimplements it or meets none of
it — so this lives in `scripts/` beside its sibling
`scripts/check-interval-cfg-additive.py`, is named by hand in BOTH halves like
every other check out here, and is covered by its own claim 1.

Stdlib only, no YAML library: the runner image is not asked for one, matching
the posture of every other cheap tripwire here. What replaces a parser is a
RECOGNISER THAT ENUMERATES, and the claim it supports is narrow enough to be
worth stating exactly, because it has twice been written wider than it held:

  Inside a workflow's `jobs:` block, every line must match one of the forms
  listed at `JOB_KEYS` / `STEP_KEYS` and the shapes around them; anything else
  raises `Bail`, which fails the check.

Four things that claim does NOT cover, none of them hidden:
  * `read_workflow` looks only INSIDE `jobs:`, and a file with no top-level
    `jobs:` key Bails rather than being half-read. The one thing read outside
    it is the workflow-level `env:` block, by `workflow_env`, because it is
    the bottom rung of the ladder claim 10's env arm compares.
  * The body of a block scalar and of `with:` is opaque text: scanned for
    invocations, never parsed, and nothing in it can Bail. An `env:` block is
    the exception — claim 10 compares the variables in it, so `_env_block`
    reads it as `NAME: value` lines and REFUSES any other spelling.
  * A recognised key's VALUE is only interpreted where a claim needs it —
    `if:`, `needs:`, `continue-on-error:`, `uses:`, and `env:` as above.
    Every other value is text.
  * This is not YAML. Anchors, merge keys, flow mappings and multi-document
    files are all refused, not supported. Valid YAML this repo does not use
    reds CI with *I do not understand this file*, and the fix is to teach the
    recogniser in a diff someone reviews.

WHAT IT CANNOT SEE, stated because a disclosed blind spot is a work order.
Claims 1-4 are about PATHS, so a hosted row that runs work inline — `cargo` in a
`run:` block, with no `scripts/` or `demos/` path to match — is outside them.
Claim 9 is what covers that population: it is about JOBS rather than paths, and
it is deliberately coarse — one citation per hosted job, not per row inside it,
because a hosted job's steps and a local row's function body are not the same
partition of the work and pretending they are would be a checker of a
coincidence. What it proves is that no hosted JOB is unaccounted for locally.
It does NOT prove the two run the same commands, and it does not prove the
local row a citation sits above still exists — a marker is a comment, and
deleting the row under it leaves the citation resolving perfectly well against
the hosted step it names.

CLAIM 10 IS WHERE THE ROSTER STOPS AND THE COMMANDS START. It takes the pairs
claim 9 leaves and asks the one question a roster cannot: do the two halves run
this check the SAME WAY. IT HAS TWO ARMS and each is narrow on purpose. The
FLAG arm: `cargo` invocations only, the flags in `SEMANTIC_FLAGS` only,
between the two sides of one `HOSTED MIRROR` pair only, and only for a cargo
subcommand BOTH sides run. The ENV arm, described below, is per PAIR rather
than per command and reads the names in `SEMANTIC_ENV` only. Neither turns
claim 9's coarse job correspondence into a claim about equal argv,
which would be false.

TWO HOLES FOLLOW ON THE FLAG ARM, both by construction. A pair whose halves
name different cargo subcommands is not compared at all, and a flag outside
the allowlist is not read.

IT HAS A SECOND ARM, OVER THE ENVIRONMENT, because the flag arm reads argv
from the `cargo` token rightwards and everything to the LEFT of it decides as
much: `RUSTFLAGS='--cfg getrandom_backend="wasm_js"'` rides both halves of the
wasm pair, and dropping it from one half left the pair green. THE TWO
SPELLINGS ARE ONE FACT and are read as one — hosted's job-level `env:`, its
step-level `env:` and an inline prefix on its `run:` line, against the local
half's prefixes — because an arm that read one spelling and not the other
would pass exactly the divergence it exists to catch. Its extent is the PAIR
rather than a shared cargo command, so it reads render rows that run no cargo
at all; its allowlist is `SEMANTIC_ENV`, so the throughput knobs the local
half deliberately does not mirror (`CARGO_PROFILE_*`, the mold link flag) are
out by name at that table rather than by exemption. `CAD_RENDER_LOCAL_OVERRIDE`
— the divergence that is live, deliberate and correct — is declared in
`PAIR_EXEMPT` with side `both` and reds if the two halves ever agree.

WHAT THE ENV ARM REFUSES rather than guesses at, because an unread prefix and
an unset variable are the same empty map: a variable exported through
`$GITHUB_ENV` by any step of the cited job; one set for the whole shell in
either half (`export`, `declare -x`, or a bare assignment with no command
after it); an assignment it cannot attribute to a command; and a mirrored pair
cited at a `uses:` step, whose behaviour is its `with:` inputs. None of the
four exists today, each Bails, and each names the site.

WHAT IT GENUINELY CANNOT SEE, four things, none of them hidden:
  * A variable set OUTSIDE these files — `.cargo/config.toml`, the runner
    image, a `uses:` action's side effects.
  * A name outside `SEMANTIC_ENV`, which is an allowlist for the reason
    `SEMANTIC_FLAGS` is one; the argument is at that table.
  * A name written INSIDE a `${{ … }}` expression. ci.yml's two archived-test
    rows build their whole prefix that way, so that half is reported
    INCOMPLETE and no one-sided verdict is passed against it — rename the
    variable inside the expression and nothing here says a word.
  * WHICH command in a row carries the variable. A row's prefixes are unioned,
    last value winning, so a variable dropped from the command that matters
    is still reported present if a sibling line in the same row carries it.
    MEASURED, on the pair this arm was filed over: `uv_sheet_drift` writes
    `CAD_RENDER_LOCAL_OVERRIDE` twice, and changing ONE of the two to hosted's
    sentence passes here. The flag arm answers this shape with `_presence`'s
    every-invocation rule; the env arm has no equivalent, because a variable
    is a name and a value rather than a per-invocation set.

CLAIM 4 HAS TWO ARMS OVER ONE POPULATION: does a script under `scripts/` or
`demos/` run at all, and does its `--selftest` mode run. They are one question
with one parameter different and they are written as one loop, because the
second one written separately grew a second invocation matcher, a second local
population and a third read of the workflow directory inside a day.
`gate-roster.sh` enforces the second arm's rule over `scripts/gates/*`, which
is why that directory is outside both. What counts as a caller for the second
arm is a WORKFLOW and nothing else: every hosted job deletes `local-scripts/`
at checkout, so a selftest whose only caller lives there runs in no CI at all.

CLAIM 11 IS THE OTHER DIRECTION ENTIRELY: not what the two halves RUN, but a
value one of them RETYPES. ci.yml's workflow-level `env:` block is this repo's
single source of truth for tool versions, and the local half restates some of
them as literals a human reads — the prereq note, the cargo-nextest failure
text, gate.sh's sccache line. Nothing read those, so nothing checked them, and
a bumped pin left the local half telling a developer to install a version
hosted no longer runs. This claim reconciles them, deriving BOTH sides — the
pins from the block through `scripts/ci-pin.py`, the literals from the tracked
files under `local-scripts/`. The argument, the two arms and the five things it cannot see
are at `PIN_FREE`.

ONE EXEMPTION TABLE COVERS BOTH ARMS. `PAIR_EXEMPT` is keyed on `(pair,
token)`, where a token is a flag or a variable name, because the substance of
an exemption table is its expiry arms and they are the same sentences for
both: an entry whose asymmetry has closed, inverted, lost its pair or lost
the token it excused is an error rather than a fossil. Its third side value,
`both`, is what the env class needed and the flag class had invited without
having — two halves that are MEANT to set one variable differently. Claim 6 reads
`.github/workflows/*.yml` and nothing else that can trigger a checkout, so a
composite action under `.github/actions/` is outside it. And, as everywhere,
wiring is not execution: a step disabled by an `if:` on the STEP still satisfies
claims 7 and 8 — only job-level `if:` is read.

  check-ci-mirror-parity.py [--selftest] [--root DIR]
"""

from __future__ import annotations

import importlib.util
import os
import re
import shlex
import shutil
import subprocess
import sys
import tempfile

HOSTED_HALF = ".github/workflows/ci.yml"
LOCAL_HALF = "local-scripts/ci-local.sh"
WORKFLOW_DIR = ".github/workflows"

# The one job allowed to keep `local-scripts/` after checkout, because the
# agreement between the halves is what it checks.
SITING_JOB = "mirror"

# THE RULE THIS FILE ENFORCES (Ev, 2026-08-20, on S61): *a gate must be sited
# where it can fire on its own inputs.* Each entry is an invocation whose inputs
# are prose, documentation or `local-scripts/` — file classes that make a change
# set TIER=docs, on which every `if: run_build` job is skipped. Each must be
# invoked from a ci.yml job carrying NO `if:`, and from the local half BEFORE
# its docs-tier early exit.
#
# Without this claim the rule is prose in three script headers: hollowing the
# `mirror` job and moving its three steps back into `discipline` restores the
# exact state S61 recorded, and nothing fires.
TIER_BLIND = (
    "scripts/gates/gate-roster.sh",
    "scripts/gates/probe-suite-census.sh --citations",
    # The viewer crate's vocabulary/driver gate. Two of its checks read
    # crates/viewer/README.md's tables and one reads that crate's Cargo.toml
    # `app` feature; a change set of only the README is TIER=docs, so sited
    # under `if: run_build` the arms that exist for a table edit could not
    # fire on a table edit.
    "scripts/gates/viewer-module-kinds.sh",
    # The viewer crate's closed-vocabulary gate. Its allowlist IS
    # crates/viewer/README.md's "The lists that stay hand-written" table and
    # the kinds a row may claim are that section's own bullets, so a docs-only
    # change set can falsify it outright — and TIER=docs skips every
    # `if: run_build` job.
    "scripts/gates/viewer-vocab-declared-once.sh",
    "scripts/check-ci-mirror-parity.py",
    # The status-capture check. Its inputs are every workflow file and every
    # tracked shell script — `local-scripts/` among them, a tree that
    # classifies TIER=docs and that every hosted job but `mirror` deletes at
    # checkout. Sited under `if: run_build` the change class that can break it
    # is the class that would skip it.
    "scripts/check-status-capture.py",
    # The render-lane parity check. Its inputs are `.github/workflows/render.yml`
    # and `local-scripts/render-hosted.sh` — the second in a tree that
    # classifies TIER=docs and that every hosted job but `mirror` deletes at
    # checkout, so sited under `if: run_build` it could not fire on the half of
    # its own subject that changes most often.
    "scripts/check-render-lane-parity.py",
    # Not because its inputs are prose — because a change WIDENING the
    # filter's docs branch classifies itself as docs, so the tier that would
    # skip this self-test is the tier it is about.
    "scripts/ci-filter.py --selftest",
    # The python linter. Its inputs are every tracked .py and .pyi, some of
    # them under `local-scripts/` — a tree whose changes classify TIER=docs and
    # which every hosted job but `mirror` deletes at checkout. Sited anywhere
    # else it would skip those while claiming the repo. No count is written
    # here: the row derives and prints its own from `git ls-files`.
    "scripts/check-python-lint.py",
    # The work tracker's lint. Its inputs are work/ and docs/ — markdown,
    # TIER=docs; work/README.md is the contract it enforces.
    "scripts/work.py lint",
)

# Declared asymmetries in claim 1. `path: (half, reason)`. An entry is a
# confession, not a disposition: it says a check runs in one half only.
MIRROR_EXEMPT = {
    "scripts/interval-only-selection.py": (
        "local",
        "the interval lane's set difference against the default build's test "
        "list. Its hosted mirror was retired 2026-08-22 with configuration "
        "sampling: a sampled run draws ONE lane, so on an interval draw the "
        "default legs the selection subtracts are not running and their 93% "
        "of the suite would be gated by nothing. The local half still runs "
        "both lanes on one tree, where the overlap is the pure re-execution "
        "it always was, so the script keeps exactly one caller; ci-local.sh "
        "says so at the row",
    ),
    # (`demos/render-uv.sh` was declared local-only here until 2026-08-22.
    # The entry described the ci.yml gate row that was retired 2026-08-17 —
    # true of ci.yml, and false of the tree: render.yml's `uv` lane composes
    # with that script on every run. Widening the hosted population above to
    # every workflow file made both halves name it, which expired the
    # confession, which is exactly what an expiring confession is for.)
    "demos/render.sh": (
        "hosted",
        "the two FreeCAD-drawn montage lanes. It needs the pinned FreeCAD "
        "AppImage, software GL and Xvfb, none of which a developer box is "
        "asked to install; local-scripts/render-hosted.sh DISPATCHES the lane "
        "rather than reproducing it, and render.yml's `montage` job says the "
        "same thing at its own key",
    ),
    "demos/render-wild.sh": (
        "hosted",
        "the wild-corpus montage. Its renderer is a pinned numpy/matplotlib "
        "venv built at run time, and the lane RE-BASELINES — commits the drawn "
        "cells from a bot token — which is the half no developer box performs: "
        "locally, being told about drift is the whole point. What IS mirrored "
        "is that lane's committed output, through ci-local.sh's `render "
        "provenance (demos)` row",
    ),
    "demos/render-gui.sh": (
        "hosted",
        "the viewer's GUI montage. It starts the real `viewer` binary and "
        "photographs its window, so it needs a software Vulkan ICD (lavapipe), "
        "an X server with no window manager, xdotool to drive the window and a "
        "RELEASE build of the eframe/wgpu toolkit — a stack no developer box is "
        "asked to install, and one whose committed pixels are a function of the "
        "runner image's mesa exactly as the two FreeCAD lanes' are. The lane "
        "also RE-BASELINES, committing the captured cells from a bot token, "
        "which is the half no developer box performs. render.yml's `gui` job "
        "says the same thing at its own key",
    ),
    "scripts/ci-pin.py": (
        "hosted",
        "the reader for ci.yml's workflow-level tool pins. The asymmetry is "
        "not a lane's: nothing in the local half INSTALLS from ci.yml's `env:` "
        "block, so there is no step there whose input a programmatic read of "
        "that block would be. ci-local.sh runs whatever nextest the "
        "developer's box already has and refuses if there is none; the one "
        "place the local half acts on a pinned version, "
        "scripts/check-python-lint.py, reads the ruff pin THROUGH THIS READER "
        "and reports the disagreement — so what is one-sided is the INVOCATION "
        "as an installer's input, not the reader: it runs in both halves, "
        "reached from a script rather than named at a row. NOTE WHAT THIS DOES "
        "NOT SAY: the local half does restate pins, as literals a human reads "
        "— ci-local.sh's prereq note and its cargo-nextest error text, and "
        "gate.sh's sccache line. Those stay literals on purpose (a command "
        "pasted on a box with no cargo-nextest is not the place for a command "
        "substitution) and claim 11 below reconciles them against the same "
        "block, so they are copies something checks",
    ),
    "scripts/apt-install.sh": (
        "hosted",
        "the apt preamble. What it narrows is the RUNNER IMAGE's source "
        "lists — third-party lists this repo never asked for, whose broken "
        "index makes `apt-get update` exit non-zero after fetching every "
        "index the job's packages actually come from. A developer box has no "
        "such image and its /etc/apt is the developer's; ci-local.sh installs "
        "nothing and instead REFUSES when a prereq is absent (its admesh row "
        "says so at its own key), which is the right local act and leaves "
        "this script with no local caller. The half that is not one-sided is "
        "its `--selftest`, which ci.yml's tier-blind `mirror` job runs: it "
        "needs no root and no archive, so a developer runs it directly",
    ),
    "scripts/criterion-emit.py": (
        "hosted",
        "the criterion benchmark lane's history writer. The lane is hosted-only "
        "for the reason nightly.yml states at its key: the entries are "
        "comparable only because one box class produced all of them, and a "
        "developer box's milliseconds committed as a trend point are exactly "
        "the failure memories/perf-measurement-lane.md was written about. The "
        "HARNESS is not one-sided — `cd benches && cargo bench` is the right "
        "local act and every number in this document came from it; what does "
        "not mirror is stapling an environment block to a local reading and "
        "committing it",
    ),
}

# Declared asymmetries in claim 2 (gate MODES). Empty, and that is the point:
# every flagged gate invocation is spelled in both halves today.
GATE_MODE_EXEMPT: dict[str, tuple[str, str]] = {}

# ------------------------------------------------------------------ claim 11
#
# CLAIM 11: EVERY VERSION LITERAL IN THE LOCAL TREE IS A PIN ci.yml STILL SETS,
# OR IS DECLARED NOT TO BE ONE. `ci.yml`'s workflow-level `env:` block is this
# repo's single source of truth for tool versions, and `scripts/ci-pin.py` is
# the one reader of it — but reading is only half the population. The other
# half is the copies that RESTATE a pin's VALUE: ci-local.sh's prereq note and
# its `nextest_check()` failure text name `0.9.140`, gate.sh names sccache
# `0.16.0`. Nothing read those, so nothing checked them, and the day a pin is
# bumped the local half goes on telling a developer to install a version hosted
# no longer runs — quietly, at the moment the local gate is trusted most.
#
# WHY THEY STAY LITERALS. `nextest_check()`'s text is a command a human copies
# onto a box that has just been told its tooling is broken. Substituting
# `$(scripts/ci-pin.py NEXTEST_VERSION)` there hands them one more thing to get
# wrong, in the one place they cannot check it. So the text stays text and this
# claim reconciles it instead.
#
# THE POPULATION IS DERIVED, IN BOTH DIRECTIONS, and that is the whole design.
# The pins come from `read_pins` — the block itself, never a roster of names
# retyped here. The literals come from a scan of the local tree — every
# `x.y.z`, never a roster of sites. What IS written by hand is `PIN_FREE`
# below, and note which way round it runs: it declares the literals that are
# NOT pins, so a version literal added anywhere in that tree is an ERROR until
# someone says what it is. A roster of the pin copies would have to be extended
# for every new copy and would silently under-report; this one fails closed.
#
# THE TWO ARMS, and neither subsumes the other:
#   A. VALUE. Every literal in the tree either equals a value ci.yml pins
#      today, or is declared in PIN_FREE. This is the arm that fires on a bump:
#      the moment NEXTEST_VERSION moves, all six `0.9.140` sites stop naming a
#      pin at once — including the ones arm B cannot see, the URL
#      `https://get.nexte.st/0.9.140/linux` (whose hostname is `nexte.st`, not
#      the token `nextest`) and the bare "the pinned 0.9.140".
#   B. NAME. A line that names a pinned tool AND carries a version must carry
#      THAT tool's current pin among its literals. This is the arm that fires
#      when a copy drifts onto some OTHER pin's value, which arm A would wave
#      through. Every underscore-separated part of the pin's key is a spelling
#      it looks for (`tool_names`), matched case-insensitively, with `_` a
#      boundary — so `NEXTEST_VERSION=0.16.0` and `# Nextest 0.16.0` are both
#      in its population, which they were not when this claim was written.
#
# WHAT THIS CANNOT SEE. The list is what has been looked for and found no way
# to cover; IT IS NOT A COUNT AND NOT A PROOF OF COMPLETENESS. An earlier
# version of this comment stated a number, which reads as completeness, and a
# review then planted four shapes it did not contain and one it described
# wrongly. Add to it when you find another; do not tally it.
#   * TWO TOOLS ON ONE LINE WITH THEIR VALUES SWAPPED. Arm B asks whether a
#     tool's pin is among the line's literals, not which literal belongs to
#     which tool, so `nextest 0.16.0 sccache 0.9.140` satisfies both tools and
#     arm A sees two pinned values. Deciding which version a line means for
#     which tool is a parse of English, and a wrong answer there would red
#     correct lines.
#   * A LITERAL THAT DRIFTS ONTO ANOTHER PIN'S VALUE ON A LINE THAT NAMES NO
#     TOOL. `ci-local.sh`'s "against the pinned 0.9.140" and the get.nexte.st
#     URL are covered for a BUMP by arm A and NOT for a cross-pin drift, which
#     needs a tool name arm B can see beside the version.
#   * VERSION SHAPE, in both directions. A version that is not three
#     dot-separated numbers is not a literal here — `1.2`, `1.2.3.4`, a value
#     built at run time — and widening to two components would swallow `3.12`,
#     `0.16` and every ratio in a comment. In the other direction the shape
#     OVER-matches: any such run is a literal, so `3.12.4` inside
#     `python3.12.4`, a date like `2026.08.22`, and the `1.2.3` inside
#     `v1.2.3-rc1` all count and must be a pinned value or declared. (This
#     comment used to claim `v1.2.3-rc1` was invisible here. It is not.)
#   * A SHORT DERIVED TOKEN MATCHING AN UNRELATED WORD. `TY_VERSION` derives
#     `ty`, and `_` is a boundary, so `my_ty_thing 1.2.3` is in arm B's
#     population. That direction over-checks rather than under-checks — it can
#     red a correct line, and the message tells the author to separate the two.
#   * A PIN WHOSE KEY IS NOT HOW ANYONE SPELLS THE TOOL. Arm B is silently
#     inert for it: nothing names it, so nothing is checked and nothing is
#     said. Arm A still covers its value.
#   * THE TREE IS `local-scripts/`, tracked files only.
#     `.claude/hooks/session-start.sh` restates three pins the same way and is
#     NOT reachable from here: every hosted job deletes `.claude/` at checkout,
#     so a claim about it would pass hosted and red locally, which is worse
#     than not making it. `work/ciw/session-start-hook-restates-ci-pins`
#     carries that one.
#   * A PIN ci.yml SETS THAT NOTHING IN THE LOCAL TREE NAMES is not an error.
#     A tool the local half does not mention is not drift.
#   * ARM B READS ONE LINE. A tool named in a sentence whose version sits on
#     the next line is arm A's business only.
PIN_TREE = "local-scripts"

# Three dot-separated numbers with no digit or dot on either side, so
# `0.0.0.0`, `127.0.0.1` and `1.1.1.1` are addresses rather than three
# overlapping versions each.
VERSION_LITERAL_RE = re.compile(r"(?<![.\d])\d+\.\d+\.\d+(?![.\d])")

# Declared NON-pins in claim 11. `(path, literal): reason`. Every other version
# literal under PIN_TREE has to be a value ci.yml pins today.
#
# AN ENTRY IS A CONFESSION AND IT EXPIRES: a declaration whose literal is no
# longer in that file is an error, exactly as MIRROR_EXEMPT's is. A stale
# not-a-pin note is how a real pin copy would come to sit under cover.
PIN_FREE = {
    ("local-scripts/ci-local.sh", "0.98.4"): (
        "admesh's version FLOOR and not a pin: the prereq note says `0.98.4+`, "
        "a lower bound on a tool the developer installs from apt or source. It "
        "sits in the SAME SENTENCE as the cargo-nextest pin, which is the trap "
        "this claim was written around — a reconciler that read every literal "
        "in that sentence as a pin copy would demand this one track "
        "NEXTEST_VERSION. ci.yml pins no admesh at all"
    ),
    ("local-scripts/hooks/pre-push", "1.9.0"): (
        "the rustfmt release whose `--check` on stdin exits 0 even on a diff, "
        "named as the reason that hook does not use stdin. It is an "
        "observation about a tool's behaviour, not a version anything "
        "installs; the compiler and its components are pinned by "
        "rust-toolchain.toml, which is a different source of truth from "
        "ci.yml's `env:` block and has its own"
    ),
    ("local-scripts/seal-oracle.sh", "0.0.0"): (
        "the `version =` field of the throwaway Cargo manifests that script "
        "generates for its probe crates. A crate version, and deliberately the "
        "null one: nothing publishes them and nothing installs them"
    ),
}

# CLAIM 10'S ALLOWLIST — the `cargo` flags that change what a run MEANS rather
# than what it executes. `flag -> takes a value`.
#
# AN ALLOWLIST AND NOT AN ARGV DIFF, deliberately. The two halves differ for
# reasons that are correct and permanent — hosted runs from an archive with
# `--archive-file` and `--workspace-remap`, local builds in a shared `target/`
# — so a full argv comparison would be a list of exceptions with a check
# hidden in it. What is enumerated here is the small set whose presence a
# reader will not diff by eye and whose absence leaves no future red: a flag
# that drops off one half merges once, silently.
#
# DERIVED AGAINST THE TREE, not copied from the issue that asked for it. Three
# corrections came out of that reading, and they are the reason this list is
# not the issue's list:
#   * `--profile` appears in neither half. The release profile is spelled
#     `--release` here — four rows in ci-local.sh, paired with the demo tour,
#     `release-corruption`, `oracle-certify` and the k-lint eps pin — so
#     `--release` is what is listed; `--profile` is kept beside it because the
#     two are the same knob and cargo accepts either spelling.
#   * `--test-threads` appears in ci.yml's PROSE only, on two comment lines
#     about a measurement that runs `--test-threads 1`; `-j` appears nowhere in
#     either half. Neither is passed by a live row. They are listed anyway:
#     this is an absence detector, and the population it is about is flags that
#     are not there yet.
#
# `--` IS NOT HONOURED, deliberately. The reader keeps scanning past it, which
# matters precisely for `--test-threads`: on `cargo test` that flag reaches the
# harness and so can only ever appear AFTER `--`, and a reader that stopped
# there would list a flag it could never see. The cost is that an argument to
# the test binary which happens to spell an allowlisted flag is read as one —
# a false red, and the direction to be wrong in.
#   * `--all-features` and `--no-default-features` are not in the issue's list
#     and are the same knob as `--features` — a selection that narrows or
#     widens under an unchanged row name.
SEMANTIC_FLAGS = {
    "--no-fail-fast": False,
    "--all-targets": False,
    "--release": False,
    "--all-features": False,
    "--no-default-features": False,
    "--features": True,
    "--partition": True,
    "--profile": True,
    "--test-threads": True,
    "-j": True,
    "-E": True,
}

# THE SAME ALLOWLIST RULE, ONE TOKEN-CLASS OVER: the environment a mirrored
# pair's commands run under. A cargo flag and an environment variable are the
# same kind of fact about a row — what the run MEANS, written outside the
# subcommand — and `RUSTFLAGS="--cfg nightly_suite"` decides which tests EXIST
# more thoroughly than any flag in the table above.
#
# WHY AN ALLOWLIST AND NOT A DENY-LIST. The hosted half's `env:` blocks carry
# ~65 distinct names, and the great majority are workflow plumbing — `TIER`,
# `SHA`, `GH_TOKEN`, `${{ steps.… }}` outputs handed to a script as input.
# They have no local analogue and never could, so a deny-list would be a
# roster of everything GitHub does, maintained here, firing on plumbing. What
# is left after the plumbing splits in two, and only one half belongs to a
# parity claim:
#
#   * THROUGHPUT KNOBS ARE OUT, BY NAME AND ON PURPOSE. `CARGO_PROFILE_*`,
#     `CARGO_TARGET_DIR`, `RUSTC_WRAPPER` and the mold link flag change what a
#     run COSTS, not what it proves. TWO OF THEM ARE RATIFIED NON-MIRRORS and
#     say so at ci-local.sh's header — the mold flag and `CARGO_PROFILE_*`,
#     the latter measured across the opt-level sweeps of 2026-08-12 and
#     2026-08-25. The other two are this table's own judgement, of the same
#     kind: a target directory is where artefacts land, a compiler wrapper is
#     a cache. A claim that fired on any of them would be demanding a change
#     the repo has decided against, or arguing about throughput in the one
#     place whose subject is meaning.
#   * SEMANTICS-BEARING NAMES ARE IN. `RUSTFLAGS` and `RUSTDOCFLAGS` select
#     cfgs, backends and target features; `CAD_*` is this kernel's own
#     namespace and every member of it is a knob on what the code DOES —
#     tolerance, fuzz depth, which render acceptor is speaking.
#
# `RUSTDOCFLAGS` IS NOT SET ANYWHERE TODAY and is listed anyway, on
# `SEMANTIC_FLAGS`'s own rule: this is an absence detector, and the population
# it is about is the variables that are not there yet.
SEMANTIC_ENV = ("RUSTFLAGS", "RUSTDOCFLAGS")
SEMANTIC_ENV_PREFIX = "CAD_"


def semantic_env(name: str) -> bool:
    return name in SEMANTIC_ENV or name.startswith(SEMANTIC_ENV_PREFIX)


# Declared asymmetries in claim 10, over both of its token classes.
# `(marker, token): (side, reason)`, where a token is either an allowlisted
# FLAG (it starts with `-`) or an allowlisted environment VARIABLE — the same
# shape and the same expiry rule as MIRROR_EXEMPT: an entry says a token is
# asymmetric across one mirrored pair, and an entry whose asymmetry has closed
# is an error rather than a fossil.
#
# ONE TABLE FOR BOTH CLASSES, not two. The expiry arms are the whole substance
# of an exemption table and they are identical for a flag and for a variable;
# a second table means a second set of them, and this file's four exemption
# tables have already cost it four hand-written expiry arms that a reviewer
# has to check against each other.
#
# THREE SIDES, AND THE THIRD IS WHY THE ENV CLASS NEEDED ONE. `hosted` and
# `local` say a token is carried by that half ALONE. `both` says the two
# halves each set the variable and are MEANT to disagree about its value —
# which the flag class had no spelling for, though its own error text invited
# one ("declare the pair in FLAG_EXEMPT with the reason it differs" landed the
# reader on an entry that then reported itself expired).
#
# THE FLAG ENTRIES: three of them, two facts. The two `--partition` entries
# are the same fact on two archives — hosted shards each test row across a
# pair of jobs and the local half runs one row on one tree. The `--features`
# entry is the other: hosted's interval row executes an ARCHIVE that was
# already compiled with the feature, so the selection is written on a
# different command. Both are the shape a per-pair confession is for — not
# that a half forgot a token, but that the token has nothing to mean there.
PAIR_EXEMPT = {
    ("test / run archived tests", "--partition"): (
        "hosted",
        "hosted splits the default archive across two sharded jobs and the "
        "local half runs it as one row on one tree. There is nothing to "
        "partition locally: the shards exist to buy wall-clock on a runner "
        "billed by the minute, and a developer box running half the suite "
        "would be a worse gate, not a faster one",
    ),
    ("test-interval / run archived tests", "--partition"): (
        "hosted",
        "the same sharding, on the interval archive. Same reason as the "
        "default row above",
    ),
    ("test-interval / run archived tests", "--features"): (
        "local",
        "the feature selection is baked into hosted's ARCHIVE, not written on "
        "the row that runs it: the `build-interval` job compiles "
        "nextest-interval.tar.zst with `cargo nextest archive --features "
        "interval`, and this row only unpacks and runs it. The local half "
        "compiles from the tree in front of you, so the selection has to be "
        "on the row itself. Both halves select the same feature; what differs "
        "is which command carries the flag",
    ),
    # THE ENV ENTRIES. Four, over two pairs and two facts.
    ("scene-inputs / demo tour (STL + STEP + UV SVGs + scenes.json)",
     "CAD_RENDER_LOCAL_OVERRIDE"): (
        "both",
        "the two sentences MEAN different things and each half needs its own "
        "(ratified, PR 1739). The render entry points refuse to run without an "
        "acceptor and deliberately do not sniff for CI, so hosted says "
        "i-am-the-hosted-renderer — the frames it draws are the committed ones "
        "— while the local half says i-accept-local-render-drift, whose "
        "message (preview only, do not commit what this pass draws) is false "
        "of every hosted line. Mirroring either value onto the other half "
        "would be the defect, not the fix",
    ),
    # THE SAME FACT ON THE OTHER TWO CITATIONS. One local function
    # (`uv_sheet_drift`) is cited by three markers, so it is three pairs, and
    # a per-pair table says it three times. Collapsing them onto the job would
    # be a different key and a weaker claim: a variable moved from the job
    # block onto one step would then stop being read.
    ("scene-inputs / compose (demos/render-uv.sh)", "CAD_RENDER_LOCAL_OVERRIDE"): (
        "both",
        "the same deliberate divergence, on the second of the three steps this "
        "one local row mirrors. Same reason as the `demo tour` entry above",
    ),
    ("scene-inputs / publish (demos/render-mc.sh)", "CAD_RENDER_LOCAL_OVERRIDE"): (
        "both",
        "the same deliberate divergence, on the third of the three steps this "
        "one local row mirrors. Same reason as the `demo tour` entry above",
    ),
    ("rebuild-latency / per-document full-rebuild + incremental-recompute table",
     "CAD_LATENCY_EMIT"): (
        "hosted",
        "the measurement SINK is the workflow's call and never the test's: "
        "hosted names a path under RUNNER_TEMP, outside the working tree, and "
        "uploads it as an artifact. The local row runs the same test for its "
        "assertions and emits nothing, because there is no history for a "
        "developer box's numbers to join",
    ),
    ("rebuild-latency / per-document full-rebuild + incremental-recompute table",
     "CAD_LATENCY_COMMIT"): (
        "hosted",
        "provenance stamped onto the emitted measurement. Same reason as "
        "CAD_LATENCY_EMIT above: no emission locally, nothing to stamp",
    ),
    ("rebuild-latency / per-document full-rebuild + incremental-recompute table",
     "CAD_LATENCY_RUNNER"): (
        "hosted",
        "the other half of that provenance — which machine produced the "
        "numbers. Same reason as CAD_LATENCY_EMIT above",
    ),
}

# Claim 8's floor. A hand-maintained number inside a gate whose thesis is that
# hand-maintained rosters drift — kept deliberately, for the reason
# `probe-suite-census.sh`'s CENSUS_FLOOR is kept: what a floor pins is that the
# population cannot silently SHRINK, and there is nothing to derive it from. A
# marker is a sentence someone chose to write; no file lists which sentences
# ought to exist. Lowering it is a decision, and reads as one in a diff.
# LOWERED 38 -> 36, 2026-08-22, deliberately and in the same diff as the
# deletion the sentence above asks for: the hosted `persistence` and `band 4
# corpus` jobs were deleted (ci.yml carries the argument at the tombstone
# where they stood) and the two local rows that cited them went with them. The
# rows did not lose a hosted mirror — the pair went away on both sides at
# once, which is the one shape a floor cannot distinguish from a hollowing and
# so has to be told about.
MIRROR_MARKER_FLOOR = 36

# The clean fixture's dimension — HOW MANY mirrored rows the miniature repo
# has. SEPARATE FROM THE FLOOR ABOVE, and it has to be: one is a claim about
# this repo's local half and the other is how big a test tree needs to be to
# exercise a citation. One knob for both would put the fixture's size under a
# production decision, so moving the floor would silently resize every case and
# the fixture's cost would scale with a number that has nothing to do with it.
# Three rows is what the cases need: enough that "the markers deleted" and "a
# marker naming the wrong job" are distinguishable from an empty half.
FIXTURE_MIRRORED_ROWS = 3

# The shape a `NO LOCAL MIRROR:` reason has to have. A FLOOR ON FORM, and only
# on form — no checker can read whether a reason is TRUE, and this one does not
# pretend to. What it buys is that a placeholder costs the same keystrokes as
# saying the thing: `# NO LOCAL MIRROR: x` satisfies "not empty" while saying
# exactly what an empty one says, and the error text for the empty case ("an
# exception with no reason is where the next one goes") is as true of `x` as it
# is of nothing at all. A subject, a verb and an object is the cheapest
# description of a sentence that a machine can hold.
NO_MIRROR_MIN_WORDS = 4
NO_MIRROR_MIN_CHARS = 24

# The clean fixture's two claim-10 pairs. They carry NO allowlisted flag, so
# every case below can plant exactly one and read back exactly one message;
# a fixture that already agreed on `--no-fail-fast` would hide the case that
# matters most.
FIXTURE_CARGO_STEP = "cargo row"
FIXTURE_CARGO_ROW = "cargo nextest run --workspace"
FIXTURE_CARGO_FN_STEP = "cargo fn row"
FIXTURE_CARGO_FN_ROW = "cargo clippy -- -D warnings"

SCRIPT_RE = re.compile(r"(?:^|[^A-Za-z0-9_/.-])((?:scripts|demos)/[A-Za-z0-9_/.-]+\.(?:sh|py))")
COMMENT_RE = re.compile(r"^\s*#")
MARKER_RE = re.compile(r"#\s*HOSTED MIRROR:\s*(.*?)\s*$")
# Claim 9's confession, written in ci.yml AT THE JOB it excuses. Not a list in
# this file: a central exception table is a place to put things, and nobody
# reading the job would see that it is on it.
NO_MIRROR_RE = re.compile(r"#\s*NO LOCAL MIRROR:\s*(.*?)\s*$")


class Bail(Exception):
    """Structure this reader does not RECOGNISE. Never a pass."""

# WHERE TO GO WHEN THIS REFUSES YOUR FILE. A `Bail` that only says *no* is what
# makes the next person reach for an exemption entry instead of a fix, so every
# one of them names the file, the line, the text that was not recognised, and
# the symbol to extend. Growing the recogniser is the intended response and is
# a small diff someone reviews; that is the whole trade this strictness rests
# on.
SELF = "scripts/check-ci-mirror-parity.py"

# Some refusals have no shape to learn — a tab, a non-UTF-8 byte, a file that is
# not there. They still owe the reader an action, and they say this so the
# self-test can tell "no guidance needed" from "guidance forgotten".
NO_TEACH = "There is nothing to extend here"
# The same sentence as a suffix, for a message that ends in a full stop.
NO_TEACH_TAIL = " " + NO_TEACH + "."


def teach(where: str) -> str:
    return (f" To accept this shape, extend {where} in {SELF} — that is the intended fix here, and it "
            "is a small reviewable diff. Do NOT route around it with an exemption entry: exemptions in "
            "this file are for asymmetries that exist, not for input the reader cannot read.")



# THE RECOGNISER ENUMERATES; THE READER NEVER SKIPS.
#
# Two versions of this check have now failed the same way. The first was
# `awk '/^  [a-z0-9_-]+:$/'` and let an uppercase job name through, because an
# unmatched line meant "not a job". The second was a hand-rolled python reader
# and let a FLUSH-STYLE step sequence through — `    - uses: …` at the same
# indent as `steps:` is ordinary YAML, its first token parsed as a key named
# `- uses`, and an unrecognised key meant "nothing here". Same default, more
# structure: *I did not recognise this* and *there is nothing here* were one
# value, which is the exact defect this track's F6 row spent two instruments on
# one directory over.
#
# So the default is inverted. Every line inside `jobs:` must match one of the
# forms enumerated BELOW, and anything else raises `Bail`, which is a hard
# failure. The cost is real and is the point: a workflow written in a shape
# this repo has not used before reds CI with *I do not understand this file*,
# and the fix is to teach the recogniser, deliberately, in a diff someone
# reviews. The alternative is a checker that keeps saying OK about YAML it has
# never seen.
#
# It is NOT a YAML parser and does not try to be. It recognises the subset
# these two workflows are written in, refuses everything else, and reads the
# structural facts three claims need: which job a step belongs to, whether that
# job can be skipped, and whether a prune step precedes a step that reads the
# pruned tree.
JOB_KEYS = frozenset({
    "name", "runs-on", "needs", "if", "uses", "with", "secrets", "env",
    "permissions", "strategy", "outputs", "steps", "timeout-minutes",
    "continue-on-error", "defaults", "concurrency", "services", "container",
})
STEP_KEYS = frozenset({
    "name", "id", "uses", "with", "run", "env", "if", "shell",
    "working-directory", "continue-on-error", "timeout-minutes",
})
BLOCK_SCALAR_RE = re.compile(r"^[|>][+-]?\d*$")
JOB_NAME_RE = re.compile(r"([A-Za-z0-9_-]+):$")
KEY_RE = re.compile(r"([A-Za-z][A-Za-z0-9_-]*):(?:\s+(.*))?$")


class Step:
    def __init__(self) -> None:
        self.name: str | None = None
        self.lines: list[str] = []
        # The `run:` scalar and its block body, and nothing else. Claim 10
        # reads ARGV, and `step.lines` is not argv: it carries `name:`, `with:`
        # and `env:` too, so a step named "rustfmt (benches — its own cargo
        # root)" parses as an invocation of `cargo root`.
        self.run: list[str] = []
        # This step's own `env:` block, `NAME -> value`. Claim 10's env arm
        # reads it; nothing else does. Kept SEPARATE from `lines` for the same
        # reason `run` is: a variable is a name and a value, and the text it
        # sits in is neither.
        self.env: dict[str, str] = {}


class Job:
    def __init__(self, name: str, line: int) -> None:
        self.name = name
        self.line = line
        self.has_if = False
        self.continue_on_error = False
        self.needs: list[str] = []
        self.uses: str | None = None
        self.steps: list[Step] = []
        # The job-level `env:` block. Every step of the job runs under it, so
        # claim 10's env arm merges it UNDER each step's own — GitHub's
        # precedence, and the reason a pair can be cited at a step while the
        # variable that matters is declared forty lines above it.
        self.env: dict[str, str] = {}


def _significant(path: str) -> list[tuple[int, int, str]]:
    """`(lineno, indent, text)` for every line that carries structure.

    Tabs are refused outright rather than measured: YAML forbids them for
    indentation, and a tab used to make this reader silently lose the rest of
    the file.
    """
    try:
        with open(path, encoding="utf-8") as fh:
            raw = fh.read()
    except UnicodeDecodeError as exc:
        raise Bail(f"{path} is not UTF-8 ({exc}) — this reader will not guess at the bytes. "
                   f"Re-encode the workflow as UTF-8. {NO_TEACH}: the bytes are the problem, not the "
                   "shape") from exc
    if "\t" in raw:
        n = raw[: raw.index("\t")].count("\n") + 1
        raise Bail(f"{path}:{n}: a TAB character. YAML forbids tab indentation and this reader will "
                   "not guess what it was meant to be — re-indent that line with spaces. "
                   f"{NO_TEACH}; GitHub rejects a tab-indented workflow too")
    out = []
    for n, line in enumerate(raw.splitlines(), 1):
        if not line.strip() or COMMENT_RE.match(line):
            continue
        out.append((n, len(line) - len(line.lstrip(" ")), line))
    return out


def read_workflow(path: str) -> list[Job]:
    items = _significant(path)
    start = next((i for i, (_, ind, t) in enumerate(items) if ind == 0 and t.rstrip() == "jobs:"), None)
    if start is None:
        raise Bail(f"{path}: no top-level `jobs:` key at column 0. This reader recognises GitHub "
                   "workflow files and refuses to guess about anything else under "
                   f"{WORKFLOW_DIR}/. If this file belongs here and spells `jobs:` another way,"
                   + teach("`read_workflow`'s search for the `jobs:` key"))
    end = next((i for i in range(start + 1, len(items)) if items[i][1] == 0), len(items))
    block = items[start + 1:end]
    if not block:
        raise Bail(f"{path}:{items[start][0]}: `jobs:` has no body — every line after it is at column "
                   "0. A workflow with no jobs is not something this check can say anything about, and "
                   "reporting OK about it would be the silence this file exists to remove. Give the "
                   f"file jobs, or take it out of {WORKFLOW_DIR}/. {NO_TEACH}: the shape was understood, "
                   "there was simply nothing in it")

    job_indent = block[0][1]
    jobs: list[Job] = []
    i = 0
    while i < len(block):
        n, ind, text = block[i]
        if ind != job_indent:
            raise Bail(f"{path}:{n}: expected a job name at indent {job_indent} (the indent the first "
                       f"job in this file uses), found indent {ind}: {text.strip()!r}. Re-indent it to "
                       f"match, or if mixed job indents are meant to be legal here,"
                       + teach("`read_workflow`'s `job_indent` rule"))
        m = JOB_NAME_RE.fullmatch(text.strip())
        if not m:
            raise Bail(f"{path}:{n}: not a job name at job indent: {text.strip()!r}. A job key must be "
                       f"a bare `name:` on its own line (`JOB_NAME_RE`)."
                       + teach("`JOB_NAME_RE`"))
        job = Job(m.group(1), n)
        jobs.append(job)
        j = i + 1
        while j < len(block) and block[j][1] > job_indent:
            j += 1
        _read_job(path, job, block[i + 1:j])
        i = j
    if not jobs:
        raise Bail(f"{path}: `jobs:` parsed to no jobs at all — the reader scanned nothing, which is "
                   f"not a pass." + teach("`read_workflow`"))
    return jobs


def workflow_env(path: str) -> dict[str, str]:
    """A workflow file's TOP-LEVEL `env:` block, `NAME -> value`.

    THE BOTTOM RUNG OF THE LADDER. GitHub's precedence is workflow < job <
    step < inline prefix, and claim 10's env arm merges all four in that
    order. Reading the top three and calling it "precedence" was this arm's
    own first defect: one line added to ci.yml's top-level block changes the
    environment of every hosted pair in the file, and nothing said a word.

    Read here rather than through `read_workflow`, which by construction
    looks only inside `jobs:` — see this module's docstring. The shape
    accepted is the one both workflow files write, `env:` at column 0 with
    `NAME: value` lines under it; `_env_block`'s refusals apply.
    """
    rows = _significant(path)
    out: dict[str, str] = {}
    for n, (ln, ind, text) in enumerate(rows):
        if ind != 0 or text.strip() != "env:":
            continue
        nested = []
        for later in rows[n + 1:]:
            if later[1] == 0:
                break
            nested.append(later)
        out.update(_env_block(path, "the workflow", ln, "", nested))
    return out


def _read_job(path: str, job: Job, body: list[tuple[int, int, str]]) -> None:
    if not body:
        raise Bail(f"{path}:{job.line}: job `{job.name}` has an empty body. A job with no keys cannot "
                   f"be checked for a prune step or an `if:`." + teach("`_read_job`"))
    key_indent = body[0][1]
    i = 0
    while i < len(body):
        n, ind, text = body[i]
        if ind != key_indent:
            raise Bail(f"{path}:{n}: expected a key of job `{job.name}` at indent {key_indent} (the "
                       f"indent this job's first key uses), found indent {ind}: {text.strip()!r}. "
                       f"Re-indent it to match, or" + teach("`_read_job`'s `key_indent` rule"))
        m = KEY_RE.fullmatch(text.strip())
        if not m:
            raise Bail(f"{path}:{n}: not a `key:` line in job `{job.name}`: {text.strip()!r}. This is "
                       "where a YAML merge key (`<<: *anchor`) lands: anchors are refused, not "
                       f"supported." + teach("`KEY_RE` and `_read_job`"))
        key, value = m.group(1), (m.group(2) or "").strip()
        if key not in JOB_KEYS:
            raise Bail(f"{path}:{n}: job `{job.name}` carries the key `{key}`, which this recogniser "
                       f"does not know. Add `{key}` to `JOB_KEYS` — and, if a claim here depends on what "
                       "it means (the way `if:`, `needs:` and `continue-on-error:` decide whether a job "
                       f"can be skipped), read its value in `_read_job` at the same time."
                       + teach("`JOB_KEYS`"))
        # The key's own nested block: everything more indented, plus — for a
        # sequence — the flush-style `- ` items at the key's own indent.
        j = i + 1
        while j < len(body) and (body[j][1] > key_indent
                                 or (body[j][1] == key_indent and body[j][2].lstrip().startswith("- "))):
            j += 1
        nested = body[i + 1:j]
        if key == "if":
            job.has_if = True
        elif key == "continue-on-error":
            job.continue_on_error = value.lower() in ("true", "'true'", '"true"')
        elif key == "uses":
            job.uses = value
        elif key == "needs":
            job.needs = _read_needs(path, n, value, nested)
        elif key == "env":
            job.env = _env_block(path, f"job `{job.name}`", n, value, nested)
        elif key == "steps":
            _read_steps(path, job, key_indent, nested)
        i = j


def _env_block(path: str, where: str, n: int, value: str,
               nested: list[tuple[int, int, str]]) -> dict[str, str]:
    """An `env:` block as `NAME -> value`, for claim 10's env arm.

    ONE SPELLING RECOGNISED, and every other one refused. `env:` followed by
    `NAME: value` lines is what both workflow files write; a flow mapping
    (`env: {A: b}`), a block scalar value, and a nested mapping under a name
    are all Bails rather than skips. This feeds an ABSENCE detector, and a
    block it half-reads is a set of variables it reports as unset — which
    compares equal to a half that never set them, in silence.
    """
    if value:
        raise Bail(f"{path}:{n}: `env:` on {where} carries a value on its own line ({value!r}). Claim "
                   "10's env arm reads the block spelling — `env:` and then `NAME: value` lines — and "
                   "a flow mapping it half-read would report the variables in it as unset, which is "
                   "indistinguishable from a half that never set them."
                   + teach("`_env_block`"))
    out: dict[str, str] = {}
    indent = nested[0][1] if nested else 0
    for ln, ind, text in nested:
        if ind != indent:
            raise Bail(f"{path}:{ln}: `env:` on {where} carries a line at indent {ind} where its first "
                       f"variable sits at {indent}: {text.strip()!r}. A value nested under a variable "
                       "name is a shape this reader does not take apart."
                       + teach("`_env_block`"))
        m = KEY_RE.fullmatch(text.strip())
        if not m:
            raise Bail(f"{path}:{ln}: not a `NAME: value` line under `env:` on {where}: "
                       f"{text.strip()!r}." + teach("`_env_block` and `KEY_RE`"))
        out[m.group(1)] = _env_value(m.group(2) or "")
    return out


def _env_value(raw: str) -> str:
    """One `env:` value, normalised to what the shell would see.

    `RUSTFLAGS: --cfg nightly_suite` and `RUSTFLAGS="--cfg nightly_suite"` are
    the same fact written two ways, so YAML's optional quotes come off before
    anything is compared. A value carrying an expansion — `${{ … }}` or a
    shell variable — is OPAQUE on the same rule the flag arm uses: only a
    runner knows what it says, and refusing there reds a correct tree.
    """
    v = raw.strip()
    if len(v) > 1 and v[0] == v[-1] and v[0] in "'\"":
        v = v[1:-1]
    return OPAQUE if "$" in v else v


def _read_needs(path: str, n: int, value: str, nested: list[tuple[int, int, str]]) -> list[str]:
    """`needs: a`, `needs: [a, b]`, and the block-sequence spelling. A job whose
    dependency is skipped is skipped, so claim 7 has to read this."""
    if value.startswith("["):
        if not value.endswith("]"):
            raise Bail(f"{path}:{n}: `needs:` flow sequence is not closed on one line: {value!r}. "
                       f"Claim 7 has to know what this job waits on and will not guess."
                       + teach("`_read_needs`"))
        return [p.strip().strip("'\"") for p in value[1:-1].split(",") if p.strip()]
    if value:
        return [value.strip("'\"")]
    out = []
    for ln, _, text in nested:
        t = text.strip()
        if not t.startswith("- "):
            raise Bail(f"{path}:{ln}: expected a `- job` item under `needs:`, found {t!r}."
                       + teach("`_read_needs`"))
        out.append(t[2:].strip().strip("'\""))
    return out


def _read_steps(path: str, job: Job, key_indent: int, nested: list[tuple[int, int, str]]) -> None:
    """Both indent styles for the sequence: items flush with `steps:` and items
    indented under it. Flush style is ordinary YAML, and reading it as a key
    named `- uses` is how a checked-out job that read `local-scripts/` came back
    OK from the version this replaced."""
    if not nested:
        raise Bail(f"{path}:{job.line}: job `{job.name}` has `steps:` with no steps under it."
                   + teach("`_read_steps`"))
    item_indent = nested[0][1]
    if item_indent not in (key_indent, key_indent + 2):
        raise Bail(f"{path}:{nested[0][0]}: `steps:` items of job `{job.name}` sit at indent "
                   f"{item_indent}, which is neither flush with `steps:` ({key_indent}) nor the usual "
                   f"one in ({key_indent + 2}). Both of those are recognised;"
                   + teach("`_read_steps`'s `item_indent` rule"))
    i = 0
    while i < len(nested):
        ln, ind, text = nested[i]
        if ind != item_indent or not text.lstrip().startswith("- "):
            raise Bail(f"{path}:{ln}: expected a `- ` step item at indent {item_indent} in job "
                       f"`{job.name}`, found {text.strip()!r}." + teach("`_read_steps`"))
        j = i + 1
        while j < len(nested) and nested[j][1] > item_indent:
            j += 1
        step = Step()
        job.steps.append(step)
        _read_step(path, job, step, item_indent, nested[i:j])
        i = j


def _read_step(path: str, job: Job, step: Step, item_indent: int,
               item: list[tuple[int, int, str]]) -> None:
    inner = item_indent + 2
    # The `- ` line carries the item's first mapping key at column `inner`.
    first_n, _, first_text = item[0]
    rows = [(first_n, inner, " " * inner + first_text.lstrip()[2:]), *item[1:]]
    k = 0
    while k < len(rows):
        n, ind, text = rows[k]
        step.lines.append(text)
        if ind != inner:
            raise Bail(f"{path}:{n}: expected a key of a step in job `{job.name}` at indent {inner} "
                       f"(the column just after `- `), found indent {ind}: {text.strip()!r}."
                       + teach("`_read_step`"))
        m = KEY_RE.fullmatch(text.strip())
        if not m:
            raise Bail(f"{path}:{n}: not a `key:` line inside a step of job `{job.name}`: "
                       f"{text.strip()!r}." + teach("`KEY_RE` and `_read_step`"))
        key, value = m.group(1), (m.group(2) or "").strip()
        if key not in STEP_KEYS:
            raise Bail(f"{path}:{n}: a step of job `{job.name}` carries the key `{key}`, which this "
                       f"recogniser does not know. Add `{key}` to `STEP_KEYS`, and if its body is a "
                       "nested block rather than a scalar, list it beside `with:` and `env:` in "
                       f"`_read_step`." + teach("`STEP_KEYS`"))
        if key == "name":
            step.name = value.strip("'\"")
        elif key == "run":
            step.run.append(value)
        # A block scalar's body is opaque text, not keys — `run: |` is where
        # every invocation this file reads actually lives.
        j = k + 1
        env_rows: list[tuple[int, int, str]] = []
        while j < len(rows) and rows[j][1] > inner:
            if BLOCK_SCALAR_RE.fullmatch(value) or key in ("with", "env"):
                step.lines.append(rows[j][2])
                if key == "run":
                    step.run.append(rows[j][2])
                if key == "env":
                    env_rows.append(rows[j])
                j += 1
                continue
            raise Bail(f"{path}:{rows[j][0]}: content nested under `{key}:` in a step of job "
                       f"`{job.name}`. Nested content is only expected under a block scalar (`|`, `>`), "
                       f"`with:` or `env:`; the nested text is scanned for invocations and never parsed."
                       + teach("`_read_step`'s list of keys that may carry a nested block"))
        if key == "env":
            step.env = _env_block(path, f"a step of job `{job.name}`", n, value, env_rows)
        k = j


def non_comment(path: str) -> list[str]:
    with open(path, encoding="utf-8") as fh:
        return [line for line in fh.read().splitlines() if not COMMENT_RE.match(line)]


def invocations(lines: list[str]) -> set[str]:
    """Every scripts/** or demos/** path named. The leading boundary in
    SCRIPT_RE keeps `local-scripts/ci-local.sh` from being read as an
    invocation of `scripts/ci-local.sh`."""
    return {m for line in lines for m in SCRIPT_RE.findall(line)}


def _shell_text(lines: list[str]) -> str:
    r"""One `run:` block as ONE unit of matching, with trailing comments gone.

    TWO DECISIONS, BOTH OF THEM ABOUT WHICH WAY TO BE WRONG.

    A BLOCK AND NOT A LINE. `scripts/foo.py --selftest` is one physical line
    today; a `\`-continuation, a `for s in a b; do python3 "$s" --selftest;
    done`, or a variable holding the path are all the same row written by
    someone tidying up, and read line by line each of them says the selftest
    is not invoked. That is a FALSE RED on a correct tree, and claim 4's own
    header settles which direction to take when a matcher cannot be exact:
    under-report rather than red, because a check that reds on a correct
    change gets routed around and then detects nothing at all. So the unit is
    the block, and the cost is stated where the arm is: a block that names one
    path and passes `--selftest` to a DIFFERENT one reads as a caller.

    TRAILING COMMENTS ARE NOT CODE, and this is the one direction worth
    spending exactness on: `true  # was: scripts/foo.py --selftest` is a row
    someone has DELETED, and counting it is the precise failure this arm
    exists to catch. `COMMENT_RE` is full-line only, so it is no help inside a
    block. Quotes are tracked so a `#` inside an argument survives.
    """
    out = []
    for line in _join_continuations(lines):
        q: str | None = None
        cut = len(line)
        for i, ch in enumerate(line):
            if q is not None:
                if ch == q:
                    q = None
            elif ch in "'\"":
                q = ch
            elif ch == "#" and (i == 0 or line[i - 1].isspace()):
                cut = i
                break
        out.append(line[:cut])
    return "\n".join(out)


def declares_selftest(path: str) -> bool:
    """Does this script IMPLEMENT a `--selftest` mode, rather than mention one?

    A full-line comment is not an implementation — `demos/render-wild.sh` names
    the flag once, in prose, about another script's mode. Everything else
    counts: an `add_argument("--selftest")`, a `case` arm, a `[ "$1" =
    --selftest ]`, and the usage string beside them. A file that is not UTF-8
    is not a finding, for the reason `reachable` reads the same population
    tolerantly.
    """
    with open(path, encoding="utf-8", errors="replace") as fh:
        return any("--selftest" in ln for ln in fh.read().splitlines()
                   if not COMMENT_RE.match(ln))


def selftest_callers() -> set[str]:
    """Paths a WORKFLOW passes `--selftest`, read one `run:` block at a time.

    A WORKFLOW, AND NOTHING ELSE. `local-scripts/` is not a caller here, and
    that is the whole point rather than an oversight: EVERY hosted job deletes
    that tree at checkout (the `prune local-only tooling` step, on the
    structural rule this file's claim 6 enforces), so a selftest whose only
    caller lives there runs in NO CI AT ALL. Counting it would let this arm
    pass the exact shape it was written to catch. The local half still owes
    the row — that is claims 1 and 7 — but the row that PROVES a guard fires
    is the hosted one.

    Reading only `step.run` and not `step.lines` is the same distinction claim
    10 draws at `Step`: a step NAMED "opt-level calibrator selftest" is not a
    step that runs one.

    WHAT IT CANNOT SEE, stated because a disclosed blind spot is a work order.
    All three under-report — they read a non-caller as a caller and so miss an
    orphan — which is the direction `reachable` argues for and for the same
    reason. (1) A `run:` block is not executed, so a path named with the flag
    inside a heredoc body, an `echo`, or an `if false` branch counts. (2) A
    path that reaches the command ONLY through a variable set in a different
    step is invisible; the fix is a literal, not an exemption. (3) A `.py`
    under `scripts/gates/` is outside this population (non-recursive walk) and
    outside `gate-roster.sh`'s (`*.sh`); there are none.
    """
    out: set[str] = set()
    for wf in sorted(os.listdir(WORKFLOW_DIR)):
        if not wf.endswith((".yml", ".yaml")):
            continue
        for job in read_workflow(f"{WORKFLOW_DIR}/{wf}"):
            for step in job.steps:
                text = _shell_text(step.run)
                if "--selftest" in text:
                    out |= invocations(text.splitlines())
    return out


def gate_modes(lines: list[str]) -> set[str]:
    """`scripts/gates/X.sh --flag` pairs, ignoring `--selftest` (every half runs
    that for every gate) and `--root` (a fixture argument, never a mode)."""
    out = set()
    for line in lines:
        for m in re.finditer(r"(scripts/gates/[A-Za-z0-9_-]+\.sh)\s+(--[a-z-]+)", line):
            if m.group(2) not in ("--selftest", "--root"):
                out.add(f"{m.group(1)} {m.group(2)}")
    return out


def reachable(root: str, seeds: set[str]) -> set[str]:
    """Transitive closure of scripts/** and demos/** paths named from a seed
    set, following file contents. This is what turns claim 4 from a one-hop
    guess into a total statement: `scripts/step_import_check.py` is named by no
    half and is not an orphan — `scripts/check_step.sh` runs it."""
    seen = set(seeds)
    stack = list(seeds)
    while stack:
        cur = stack.pop()
        full = os.path.join(root, cur)
        if not os.path.isfile(full):
            continue
        try:
            with open(full, encoding="utf-8", errors="replace") as fh:
                body = [line for line in fh.read().splitlines() if not COMMENT_RE.match(line)]
        except OSError:
            continue
        # A script names its siblings by BASENAME as often as by path
        # (`demos/render.sh` runs `strip_png_stamps.py` from its own directory),
        # so both spellings count — resolved against the NAMING script's own
        # directory, which is what the shell does.
        #
        # THE FAILURE DIRECTION, because a heuristic without one is a guess.
        # This resolves a bare `foo.sh` to `<dir of the naming script>/foo.sh`
        # whenever that file exists, and a bare name can be a coincidence: a
        # comment, a string, or a sibling with the same basename as some other
        # directory's script. So the closure is BIASED too large — it can
        # decide a script is owned when nothing really runs it.
        #
        # ONE SPELLING GOES THE OTHER WAY, and the bias above is not a
        # guarantee. `SCRIPT_RE` requires the character before `scripts/` not
        # to be part of a path, which is what stops `local-scripts/ci-local.sh`
        # reading as an invocation of `scripts/ci-local.sh` — and a leading `/`
        # falls inside that same exclusion, so a call written
        # `"$root/scripts/x.sh"` is invisible here while `"$root"/scripts/x.sh`
        # and a bare `scripts/x.sh` are seen. The basename pass below does not
        # rescue it either, for the same reason: the character before the
        # basename is a `/`. A real call written that way therefore reads as an
        # ORPHAN, which is the expensive direction this comment says cannot
        # happen. It is left narrow rather than widened because widening it
        # means admitting a `/` before `scripts/`, which is exactly the
        # `local-scripts/` confusion the boundary exists to refuse; the call
        # sites in this tree are written in the visible form instead.
        #
        # An over-large closure UNDER-reports claim 4 (an orphan reads as
        # owned) and affects nothing else — no other claim consumes it. That is
        # the safe direction here, and deliberately so: claim 4's job is to
        # catch a check nobody runs, and a false positive there would demand a
        # `MIRROR_EXEMPT` entry for a script that is in fact perfectly wired,
        # which teaches the next reader that the exemption list is where
        # inconvenient results go. Under-reporting costs a missed orphan;
        # over-reporting costs the credibility of every other claim in the
        # file. The claim's own wording is written to match: it says a check
        # that neither half names *and no named script reaches*.
        here = os.path.dirname(cur)
        named = invocations(body)
        for line in body:
            # `name.sh`, and `./name.sh` — the spelling a script uses to source
            # a sibling (`. ./hosted-render-guard.sh`), which a path-shaped
            # matcher does not see.
            for m in (re.findall(r"(?:^|[^A-Za-z0-9_/.-])([A-Za-z0-9_.-]+\.(?:sh|py))", line)
                      + re.findall(r"\./([A-Za-z0-9_.-]+\.(?:sh|py))", line)):
                cand = os.path.join(here, m)
                if os.path.isfile(os.path.join(root, cand)):
                    named.add(cand)
        for p in named:
            if p not in seen:
                seen.add(p)
                stack.append(p)
    return seen


def local_docs_exit_line(lines: list[str]) -> int:
    """The line index of the local half's docs-tier early exit. Claim 7 needs
    it: a row placed after it does not run on the tier it is about."""
    for i, line in enumerate(lines):
        if re.search(r'"\$TIER"\s*=\s*docs', line):
            for j in range(i, min(i + 12, len(lines))):
                if re.match(r'\s*exit (0|"?\$[A-Za-z_]\w*"?)\s*$', lines[j]):
                    return j
            raise Bail(f"{LOCAL_HALF}:{i + 1}: the docs-tier branch opens here and no `exit` follows "
                       f"it within 12 lines: {line.strip()!r}. Claim 7's whole question is which rows run "
                       "BEFORE that exit, so this reader will not guess where the branch ends. If the "
                       f"branch now ends another way," + teach("`local_docs_exit_line`"))
    raise Bail(f"{LOCAL_HALF}: no docs-tier branch found — nothing matches `\"$TIER\" = docs`. Claim 7 "
               "measures every tier-blind row against that branch, so without it the claim has no "
               f"reference point and would pass vacuously." + teach("`local_docs_exit_line`"))


# THE SHELL HALF'S RECOGNISER, on the same inverted default as the YAML one.
# `^name() {` was the only spelling read, so `tier_blind_rows () {` and
# `function tier_blind_rows {` — both ordinary bash — parsed as "not a function",
# every body line resolved as a top-level call site, and a definition left above
# `ci-local.sh`'s docs exit with its single call moved below it came back OK.
# That is the MAJOR this resolution was added to close, re-opened by one space.
# So the forms are enumerated, and a line that LOOKS like a definition and is
# not one of them Bails.
FUNC_HINT_RE = re.compile(r"^(?:function\s+[A-Za-z_]|[A-Za-z_][A-Za-z0-9_]*\s*\(\s*\))")
FUNC_OPEN_RE = re.compile(r"^(?:function\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*(?:\(\s*\))?\s*\{\s*$")
FUNC_ONELINE_RE = re.compile(r"^(?:function\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*(?:\(\s*\))?\s*\{.*\}\s*$")


def shell_functions(lines: list[str]) -> dict[str, tuple[int, int]]:
    """`name -> (first, last)` body line indices, inclusive; a one-liner is its
    own body. Bails on a definition-shaped line in a spelling not listed."""
    funcs: dict[str, tuple[int, int]] = {}
    open_at: tuple[str, int] | None = None
    for i, line in enumerate(lines):
        if open_at is not None:
            if re.fullmatch(r"\}\s*", line):
                funcs[open_at[0]] = (open_at[1], i)
                open_at = None
            continue
        if not FUNC_HINT_RE.match(line):
            continue
        m = FUNC_ONELINE_RE.fullmatch(line)
        if m:
            funcs[m.group(1)] = (i, i)
            continue
        m = FUNC_OPEN_RE.fullmatch(line)
        if not m:
            raise Bail(f"{LOCAL_HALF}:{i + 1}: looks like a shell function definition and is not a "
                       f"spelling this recogniser knows: {line.strip()!r}. Recognised today: `name() {{`, "
                       "`name () {`, `function name {`, and the one-line `name() { …; }`. Claim 7 "
                       "has to know where a row is RUN, not where it is written, and it will not guess."
                       + teach("`FUNC_OPEN_RE` / `FUNC_ONELINE_RE`, beside `FUNC_HINT_RE`"))
        open_at = (m.group(1), i)
    if open_at is not None:
        raise Bail(f"{LOCAL_HALF}:{open_at[1] + 1}: function `{open_at[0]}` is opened here and never "
                   "closed by a `}` at column 0, so this reader cannot tell which lines are its body "
                   f"and which run at top level." + teach("`shell_functions`'s close rule"))
    return funcs


def local_call_sites(lines: list[str], want: str) -> list[int]:
    """Line indices at which the local half actually RUNS `want`.

    Not "mentions": a row wrapped in a shell function is run where the FUNCTION
    is called, and a definition placed above the docs-tier exit says nothing
    about when it executes. So an occurrence inside a function body resolves to
    that function's call sites outside it. Without this the claim is satisfied
    by moving a definition, which is not the property.
    """
    funcs = shell_functions(lines)

    def enclosing(i: int) -> str | None:
        for name, (a, b) in funcs.items():
            if (a == b and i == a) or a < i < b:
                return name
        return None

    sites: list[int] = []
    for i, line in enumerate(lines):
        if want not in line:
            continue
        host = enclosing(i)
        if host is None:
            sites.append(i)
            continue
        a, b = funcs[host]
        sites += [j for j, m in enumerate(lines)
                  if re.search(rf"(^|[^A-Za-z0-9_-]){re.escape(host)}([^A-Za-z0-9_-]|$)", m)
                  and not (a <= j <= b) and enclosing(j) is None]
    return sorted(set(sites))


# ------------------------------------------------------- claim 10's reader
#
# THE ARGV READER, on the same inverted default as the two above: a command
# line carrying `cargo` that this cannot take apart raises `Bail`. An absence
# detector that answers "I could not read that, so: OK" detects nothing, and a
# flag dropped from one half leaves no future red for anything else to catch.
#
# IT PARSES ARGV; IT DOES NOT EVALUATE THE SHELL, and the difference decides
# every case below. `--partition count:${{ matrix.shard }}/2` is a well-formed
# argv whose VALUE only a runner knows, so the value is recorded as OPAQUE and
# compares equal to anything — refusing there would red a correct tree, which
# is how an absence detector gets routed around. `cargo test --features` with
# nothing after it, or a quote that never closes, is not a value this reader
# declines to evaluate: it is an argv it cannot read, and that is a refusal.
GH_EXPR_RE = re.compile(r"\$\{\{.*?\}\}")
# Where one simple command ends and the next begins. Only outside quotes, and
# only after the maskings below.
#
# THIS SET IS THE FILE'S OWN FAILURE MODE, MET TWICE. Splitting a command in
# the wrong place drops every flag AFTER the split into a chunk with no `cargo`
# in it — and a flag that vanishes from BOTH halves at once reads as agreement,
# which is the one direction an absence detector must not fail in.
#
#   * BRACES were in this set until the QA-2 reconstruction caught them: both
#     halves splice the gated-suite filter as `${GATED[@]+"${GATED[@]}"}`, and
#     the `--no-fail-fast` after the splice was being lost on both sides.
#   * `&` IS NOT A BREAK IN A REDIRECTION. `2>&1` sits on every one of the five
#     rows claim 10 reads, and bites only because the redirection happens to
#     come last today. `_simple_commands` breaks on `&&` and on a background
#     `&`, and treats `>&`, `<&` and `&>` as text.
#   * `$( … )`, `$(( … ))` AND BACKTICKS ARE MASKED to `$SUB` in the argv they
#     sit in, so a substitution in the middle of a command does not saw it in
#     half — and their contents are then split ON THEIR OWN, so a `cargo`
#     invocation inside one is still read. Dropping the contents instead was
#     tried and is wrong: `listing=$(cargo test … --test all -- --list)` is a
#     live local row, and losing it made a correct pair look like a pair whose
#     halves run a different number of commands.
#
# What the tokenizer still does not survive: a here-document, an `eval`ed
# string, and a cargo invocation built by string concatenation. All three are
# absent from both halves today, and all three would need a `Bail` rather than
# a wider guess.
CMD_BREAK = frozenset("|;()`")
ATTACHED_J_RE = re.compile(r"-j\d+")
OPAQUE = "*"
# What a masked substitution leaves behind: a token carrying `$`, so that if it
# lands where a flag's value goes it is OPAQUE rather than compared as text.
SUB_MASK = "$SUB"


def _join_continuations(lines: list[str]) -> list[str]:
    """A `\\`-continued command is ONE argv. Read line by line it is two, and
    the second one carries the flags — which is where most of this repo's
    `--features` and `--no-fail-fast` spellings sit."""
    out: list[str] = []
    buf = ""
    for line in lines:
        text = line.rstrip()
        if text.endswith("\\"):
            buf += text[:-1] + " "
            continue
        out.append(buf + text)
        buf = ""
    if buf:
        out.append(buf)
    return out


def _skip_substitution(text: str, at: int) -> int:
    """Index just past the `)` closing the `(` at `at`, counting nesting and
    ignoring parens inside quotes. Unclosed runs to end of line, which leaves
    the rest of the argv masked rather than sawn in half."""
    depth = 0
    i = at
    in_single = in_double = False
    while i < len(text):
        c = text[i]
        if in_single:
            in_single = c != "'"
        elif in_double:
            if c == "\\":
                i += 2
                continue
            in_double = c != '"'
        elif c == "'":
            in_single = True
        elif c == '"':
            in_double = True
        elif c == "(":
            depth += 1
        elif c == ")":
            depth -= 1
            if depth == 0:
                return i + 1
        i += 1
    return len(text)


def _simple_commands(where: str, text: str) -> list[str]:
    """Split one logical line into simple commands, quote-aware.

    `cargo fmt --check && cargo clippy --all-targets` is two commands, and
    reading it as one gives `cargo fmt` a flag it does not have. The split has
    to know about quoting because both halves write
    `bash -c 'cd benches && cargo fmt --all --check'`, where the `&&` is text.
    A `#` that starts a word ends the line for the same reason: a comment is
    not argv, and this repo's prose is full of apostrophes.

    EVERY OTHER BRANCH HERE EXISTS TO NOT SPLIT: a redirection's `&`, a command
    substitution and a backtick are consumed as text so that the flags after
    them stay on the command they belong to. See `CMD_BREAK`'s comment for why
    that direction is the one that matters.
    """
    out: list[str] = []
    cur: list[str] = []
    nested: list[str] = []
    i = 0
    in_single = in_double = False
    while i < len(text):
        c = text[i]
        if in_single:
            in_single = c != "'"
            cur.append(c)
            i += 1
            continue
        if in_double:
            if c == "\\" and i + 1 < len(text):
                cur.append(c)
                cur.append(text[i + 1])
                i += 2
                continue
            in_double = c != '"'
            cur.append(c)
            i += 1
            continue
        if c == "'":
            in_single = True
        elif c == '"':
            in_double = True
        elif c == "\\" and i + 1 < len(text):
            cur.append(c)
            cur.append(text[i + 1])
            i += 2
            continue
        elif c == "#" and (i == 0 or text[i - 1].isspace()):
            break
        elif c == "$" and text[i + 1:i + 2] == "(":
            end = _skip_substitution(text, i + 1)
            cur.append(SUB_MASK)
            nested.append(text[i + 2:max(i + 2, end - 1)])
            i = end
            continue
        elif c == "`":
            end = text.find("`", i + 1)
            cur.append(SUB_MASK)
            nested.append(text[i + 1:] if end < 0 else text[i + 1:end])
            i = len(text) if end < 0 else end + 1
            continue
        elif c == "&":
            if text[i + 1:i + 2] == "&":          # `&&`, an and-list
                out.append("".join(cur))
                cur = []
                i += 2
                continue
            if "".join(cur).rstrip()[-1:] in (">", "<") or text[i + 1:i + 2] == ">":
                cur.append(c)                     # `2>&1`, `<&0`, `&>log` — text
                i += 1
                continue
            out.append("".join(cur))              # a background `&`
            cur = []
            i += 1
            continue
        elif c in CMD_BREAK:
            out.append("".join(cur))
            cur = []
            i += 1
            continue
        cur.append(c)
        i += 1
    if in_single or in_double:
        raise Bail(f"{where}: a quote opens on this command line and never closes: {text.strip()[:120]!r}. "
                   "This line names `cargo`, so claim 10 has to read its flags and cannot: the rest of "
                   "the line could be argv or could be text inside the quote, and guessing is how a "
                   "dropped flag reads as agreement. Close the quote, or if the line is not a command,"
                   + teach("`_simple_commands`"))
    out.append("".join(cur))
    for inner in nested:
        out.extend(_simple_commands(where, inner))
    return out


def cargo_flags(where: str, lines: list[str]) -> dict[str, list[dict[str, str | None]]]:
    """`cargo <sub>` (and `cargo nextest <sub>`) -> ONE ENTRY PER INVOCATION,
    each mapping an allowlisted flag to its value.

    A boolean flag's value is `None`; a value this reader cannot evaluate is
    `OPAQUE`. Only `cargo` is read: every flag in `SEMANTIC_FLAGS` is a cargo
    or nextest flag, and widening to `scripts/` invocations would put this
    claim on top of claim 2's ground with none of its exemption vocabulary.

    PER INVOCATION, NOT UNIONED HERE, because the union is where this claim
    fails silently — see `_presence`. Collapsing two `cargo test` rows into one
    flag set is what lets a flag present on one of them stand in for the other.
    """
    out: dict[str, list[dict[str, str | None]]] = {}
    for line in _join_continuations(lines):
        line = GH_EXPR_RE.sub("$GHEXPR", line)
        if "cargo" not in line:
            continue
        for chunk in _simple_commands(where, line):
            if "cargo" not in chunk:
                continue
            try:
                toks = shlex.split(chunk)
            except ValueError as exc:
                raise Bail(f"{where}: cannot read this as a command line ({exc}): {chunk.strip()[:120]!r}. "
                           "It names `cargo`, so claim 10 has to know which flags it carries."
                           + teach("`cargo_flags`")) from exc
            if "cargo" not in toks:
                continue
            rest = toks[toks.index("cargo") + 1:]
            sub = next((t for t in rest if not t.startswith("-")), None)
            if sub is None:
                continue
            key = f"cargo {sub}"
            if sub == "nextest":
                after = rest[rest.index("nextest") + 1:]
                nsub = next((t for t in after if not t.startswith("-")), None)
                if nsub is not None:
                    key = f"cargo nextest {nsub}"
            flags: dict[str, str | None] = {}
            out.setdefault(key, []).append(flags)
            i = 0
            while i < len(rest):
                tok = rest[i]
                value: str | None = None
                if tok.startswith("--") and "=" in tok:
                    name, value = tok.split("=", 1)
                elif ATTACHED_J_RE.fullmatch(tok):
                    name, value = "-j", tok[2:]
                else:
                    name = tok
                if name in SEMANTIC_FLAGS:
                    if SEMANTIC_FLAGS[name] and value is None:
                        if i + 1 >= len(rest):
                            raise Bail(f"{where}: `{name}` is the last token of {chunk.strip()[:100]!r} and "
                                       "takes a value. Claim 10 compares the two halves' selections, so an "
                                       "argument it cannot find is a refusal rather than a flag with no "
                                       f"value." + teach("`SEMANTIC_FLAGS` and `cargo_flags`"))
                        value = rest[i + 1]
                        i += 1
                    flags[name] = OPAQUE if (value is not None and "$" in value) else value
                i += 1
    return out


ASSIGN_RE = re.compile(r"^([A-Za-z_][A-Za-z0-9_]*)=(.*)$", re.S)
# Words that may stand BEFORE a command without being one. A one-line shell
# function is why this list exists rather than being assumed empty:
# `rebuild_latency() { CAD_X=1 cargo test …; }` reaches this reader as a chunk
# beginning `{`, and a scan anchored at token 0 read the whole row as setting
# nothing — on three live pairs, one of them carrying three PAIR_EXEMPT
# entries. The flag arm never noticed because it finds `cargo` wherever it
# sits; this arm is positional, and that is the difference.
CMD_PRELUDE = frozenset({"{", "!", "then", "else", "elif", "do", "time"})
# Commands that RUN another command and may carry assignments of their own.
# `env RUSTFLAGS=… cargo check` is the same fact as the bare prefix, and the
# item's own class list names it.
CMD_WRAPPERS = frozenset({"env", "exec", "nohup", "command", "stdbuf", "timeout"})
# Words that make an assignment STANDING rather than a prefix.
ASSIGN_KEYWORDS = frozenset({"export", "declare", "typeset", "readonly", "local"})


def env_prefixes(where: str, lines: list[str]) -> tuple[dict[str, str], bool]:
    """`(allowlisted variables set as an INLINE PREFIX, is the map incomplete)`.

    `RUSTFLAGS="--cfg nightly_suite" cargo nextest run …` and a workflow's

        env:
          RUSTFLAGS: --cfg nightly_suite

    ARE THE SAME FACT WRITTEN TWO WAYS, and this reader exists so that claim
    10's env arm can compare them as one. It handles the shell spelling; the
    recogniser handles the YAML one (`_env_block`), and both normalise to
    `NAME -> value` before anything is compared. A reader that saw only one
    spelling would pass exactly the divergence it was built to catch: hosted
    declares the variable in a block, local drops the prefix, and two readers
    that never meet both report "not set here".

    NOT ONLY CARGO COMMANDS, unlike the flag arm. A flag is a cargo flag by
    construction; an environment variable is a property of the ROW, and the
    pair this whole arm was filed over sets `CAD_RENDER_LOCAL_OVERRIDE` on
    `demos/render-uv.sh`.

    AN UNRECOGNISED PREFIX IS A REFUSAL, NEVER AN EMPTY MAP, and that is the
    whole discipline of this function. An empty map is indistinguishable from
    "this half sets nothing", which is the silent pass claim 10's env arm
    exists to close — so every shape this cannot take apart raises rather than
    returning quietly:

      * a `NAME=value` token for an allowlisted variable that this reader
        could not attribute to a command — after the command word, or inside
        a construct it does not know — Bails;
      * `export NAME=…`, `declare -x NAME=…` and a bare `NAME=…` with no
        command after it are STANDING assignments, set for everything that
        follows rather than for one command, and Bail with what to write
        instead;
      * a quote that never closes on a line carrying an assignment Bails.

    THE ONE THING IT DOES NOT REFUSE IS A RUNNER EXPRESSION, and the reason is
    that refusing there would red a correct tree: ci.yml's two archived-test
    rows write their whole prefix as a `${{ … }}` expression that expands to
    `CAD_TOLERANCE_EPS=<eps>`, to nothing, or to `env -u CAD_TOLERANCE_EPS`,
    and only a runner knows which. That is the OPAQUE rule this file already
    applies to a flag VALUE, lifted to the map: the half is reported
    INCOMPLETE (the second return value), and claim 10 then declines to call a
    variable one-sided against it — it cannot know whether the expression sets
    that name. Values it CAN see are still compared. The cost is stated at the
    claim and in the module docstring: a variable named only inside such an
    expression is invisible here.
    """
    out: dict[str, str] = {}
    incomplete = False
    for line in _join_continuations(lines):
        line = GH_EXPR_RE.sub("$GHEXPR", line)
        if "=" not in line and "$GHEXPR" not in line:
            continue
        for chunk in _simple_commands(where, line):
            try:
                toks = shlex.split(chunk)
            except ValueError as exc:
                raise Bail(f"{where}: cannot read this as a command line ({exc}): "
                           f"{chunk.strip()[:120]!r}. It carries an assignment, so claim 10 has to "
                           "know whether it sets one of the variables that decide what the run means."
                           + teach("`env_prefixes`")) from exc
            if not toks:
                continue
            found: list[tuple[str, str]] = []
            standing = False
            i = 0
            while i < len(toks):
                tok = toks[i]
                if tok in CMD_PRELUDE:
                    i += 1
                    continue
                if tok in ASSIGN_KEYWORDS:
                    standing = True
                    i += 1
                    while i < len(toks) and toks[i].startswith("-"):
                        i += 1
                    continue
                if tok in CMD_WRAPPERS and not standing:
                    i += 1
                    # `env -u NAME` UNSETS, and an unset compares as absent —
                    # right in both directions: against a half that never set
                    # the variable they agree, and against one that sets it
                    # the setting half is one-sided, which is the truth.
                    while i < len(toks) and toks[i].startswith("-"):
                        if toks[i] in ("-u", "--unset") and i + 1 < len(toks):
                            i += 1
                        i += 1
                    continue
                m = ASSIGN_RE.match(tok)
                if m is not None:
                    found.append((m.group(1), m.group(2)))
                    i += 1
                    continue
                if "$GHEXPR" in tok or SUB_MASK in tok:
                    # An expression standing where a prefix would: unreadable,
                    # and the rest of the chunk may still carry a real one.
                    incomplete = True
                    i += 1
                    continue
                break                                   # the command word
            # A prefix is followed by the command it applies to; assignments
            # with nothing after them are the shell's own scope, which is the
            # same fact `export` states out loud.
            standing = standing or i >= len(toks)
            for name, value in found:
                if not semantic_env(name):
                    continue
                if standing:
                    raise Bail(f"{where}: `{name}` is set for the rest of the shell here, not as "
                               f"a prefix on one command: {chunk.strip()[:100]!r}. Claim 10's env arm "
                               "attributes a variable to the command it prefixes, so a standing "
                               "assignment would be read as set on nothing and compare equal to a half "
                               "that never set it. Write it as a prefix on the command that needs it,"
                               + teach("`env_prefixes`"))
                out[name] = OPAQUE if "$" in value else value
            # WHAT THE WALK COULD NOT ATTRIBUTE. Anything left that looks like
            # an allowlisted assignment sits somewhere this reader does not
            # understand — after the command word, or inside a construct not
            # in the lists above. Reading it as absent is the one answer that
            # cannot be right.
            for tok in toks[i:]:
                m = ASSIGN_RE.match(tok)
                if m is not None and semantic_env(m.group(1)):
                    raise Bail(f"{where}: `{m.group(1)}` is assigned somewhere claim 10's env arm "
                               f"cannot attribute it to a command: {chunk.strip()[:100]!r}. It sits "
                               "after the command word, or inside a construct this reader does not "
                               "take apart — and an assignment read as absent compares equal to a "
                               "half that never made it."
                               + teach("`env_prefixes`'s `CMD_PRELUDE`/`CMD_WRAPPERS` walk"))
    return out, incomplete


def _presence(invocations: list[dict[str, str | None]], flag: str) -> tuple[bool, bool, set[str | None]]:
    """`(on any invocation, on every invocation, the values seen)`.

    TWO PRESENCE RULES, AND BOTH ARE COMPARED, because the obvious one is
    biased the wrong way. Flags aggregate as a union, and `marker_row`'s
    closure is biased LARGE — so a row whose own `cargo` line lost a flag still
    reports the flag as present the moment any sibling folded into that row
    carries it. More extent makes AGREEMENT easier, which for an absence
    detector is precisely the direction that must not fail: the flag is gone
    and the checker says the halves match. The same union hides a second
    invocation inside one row.

    The `every` rule closes both. In the masking case the losing side goes
    any=True / every=False while the other stays every=True, and the pair reds.
    Its cost is a pair where one half legitimately splits a command into two
    invocations and flags only one — that reds, and reding is the direction to
    be wrong in here; the fix is a `PAIR_EXEMPT` sentence saying so.
    """
    hits = [inv for inv in invocations if flag in inv]
    return bool(hits), len(hits) == len(invocations), {inv[flag] for inv in hits}


# ---------------------------------------------- PAIR_EXEMPT's expiry arms
#
# ONE TABLE, ONE SET OF ARMS. The header's argument for merging FLAG_EXEMPT
# and an env table was that the substance of an exemption table is its expiry
# directions and they are the same sentences for a flag and for a variable —
# and an argument like that is only worth the diff if the sentences are
# actually written once. These two functions are where they are written; both
# arms of claim 10 call them, and the four selftest cases per arm exercise the
# same code twice over.
#
# The two token classes differ by two words and one clause: what the halves DO
# with the token (`passes` a flag, `sets` a variable) and, for a flag, which
# command it was read on.


def _exempt_unwatched(marker: str, token: str, want: str, reason: str, verb: str) -> str:
    """The direction MIRROR_EXEMPT had all along and this table went without:
    an entry that stopped matching anything is not a watched asymmetry, it is
    an UNWATCHED ABSENCE with a confession sitting on top of it."""
    return (f"`{token}` is declared {want} for the pair `{marker}` in PAIR_EXEMPT and NEITHER half "
            f"{verb} it. The confession has nothing left to excuse: either the token was dropped — "
            "which is the absence this claim exists to catch, and it is now hidden behind the entry "
            f'— or ("{reason}") describes a row that changed shape. Delete the entry, or say in it '
            "where the token went")


# How each class's verb inflects, because these sentences are read by whoever
# the gate stopped: `("pass", "passes", "passed")` for a flag, `("set",
# "sets", "set")` for a variable.
FLAG_VERB = ("pass", "passes", "passed")
ENV_VERB = ("set", "sets", "set")


def _exempt_present(marker: str, token: str, want: str, reason: str, h_here: bool, l_here: bool,
                    agree: bool, verb: tuple[str, str, str], where: str = "") -> str | None:
    """The other three directions, for a token at least one half carries."""
    plural, singular, past = verb
    if h_here and l_here:
        if want != "both":
            return (f"`{token}`{where} is declared {want}-only for the pair `{marker}` in "
                    f'PAIR_EXEMPT and BOTH halves now {plural} it. The reason ("{reason}") has '
                    "expired — delete the entry, so the list stays a record of asymmetries that "
                    "exist rather than of ones that once did")
        if agree:
            return (f"`{token}`{where} is declared to DIFFER across the pair `{marker}` in "
                    f'PAIR_EXEMPT, and the two halves now agree. The reason ("{reason}") has '
                    "expired — delete the entry, so the list stays a record of asymmetries that "
                    "exist rather than of ones that once did")
        return None
    side = "hosted" if h_here else "local"
    if want == "both":
        return (f"`{token}`{where} is declared to DIFFER across the pair `{marker}` in PAIR_EXEMPT, "
                f"and only the {side} half {singular} it now. A divergence that became one-sided is a "
                f'different fact — re-read the reason ("{reason}") and either restore the other half '
                "or re-declare the entry")
    if want != side:
        return (f"`{token}`{where} is declared {want}-only for the pair `{marker}` in PAIR_EXEMPT "
                f"and is {past} by the {side} half. The exemption now describes the "
                f'opposite of the tree — re-read the reason ("{reason}") and fix whichever side moved')
    return None


def marker_row(raw: list[str], at: int, funcs: dict[str, tuple[int, int]]) -> list[str]:
    """The local half's side of the pair a `HOSTED MIRROR` marker on line `at`
    declares: the lines whose argv answers to the hosted step it cites.

    ONE EXTENT RULE, AND IT IS NARROW. Below the marker (past further markers,
    prose and blank lines) is either a shell function definition — then the row
    is that function's body — or a command, and then the row is THAT COMMAND
    AND NOTHING AFTER IT. The looser reading, "every line down to the next
    comment", swallows the three rows that follow `run_row "clippy (interval)"`
    and hands the interval clippy pair the interval TEST rows' flags.

    Then the shell functions the row calls are folded in, transitively:
    `run_row "wasm32 check (#807)" wasm_check` is a row whose whole argv lives
    in a function twenty lines up, and a rule that stopped at the dispatch line
    would read it as a pair with no cargo command in it — a pass, for the
    reason this claim exists to refuse.

    THE FAILURE DIRECTION, CORRECTED. The closure matches function names
    against the row's code, so it is biased LARGE — and a first version of this
    comment said that therefore fails toward a false RED. It does not, and the
    error is worth keeping written down because it is the same species as the
    defect this claim exists for: a true mechanism with a false sentence over
    it. Flags aggregate as a union, so a bigger extent can only ADD flags to a
    side, which makes the two sides AGREE more easily. The bias is toward a
    false pass — a flag deleted from the row's own command, still reported
    present because a folded-in sibling carries one. Measured: strip
    `--no-fail-fast` from the demoted row and add one live line naming a
    sibling that has it, and an any-presence rule returns OK.
    `_presence`'s second rule is what answers that, and it is why there is one.
    """
    i = at + 1
    while i < len(raw) and (not raw[i].strip() or COMMENT_RE.match(raw[i])):
        i += 1
    if i >= len(raw):
        # WHAT THIS GUARD CATCHES, exactly: a marker with no code line ANYWHERE
        # below it — the end of the file. It is not a check that the row under
        # a marker still exists, and it must not be read as one: delete a row
        # in the MIDDLE of the file and this walks past the blank line to the
        # next code line and adopts it, silently, as that marker's row.
        #
        # That is claim 8's disclosed gap ("a marker is a comment, and deleting
        # the row under it leaves the citation resolving perfectly well"), not
        # this claim's, and closing it here is not free: making the absence a
        # refusal fails on a marker written at the foot of the file, which the
        # self-test's own `confession_expired` case plants. Returning an empty
        # row is the honest reading — there is no argv to compare — and the gap
        # is stated rather than papered over.
        return []
    opens = next((nm for nm, (a, _b) in funcs.items() if a == i), None)
    if opens is not None:
        idx = set(range(funcs[opens][0], funcs[opens][1] + 1))
    else:
        end = i
        while end < len(raw) and raw[end].rstrip().endswith("\\"):
            end += 1
        idx = set(range(i, end + 1))
    grown = True
    while grown:
        grown = False
        body = [raw[x] for x in sorted(idx) if not COMMENT_RE.match(raw[x])]
        for name, (a, b) in funcs.items():
            if set(range(a, b + 1)) <= idx:
                continue
            if any(re.search(rf"(^|[^A-Za-z0-9_-]){re.escape(name)}([^A-Za-z0-9_-]|$)", ln) for ln in body):
                idx |= set(range(a, b + 1))
                grown = True
    return [raw[x] for x in sorted(idx)]


# ------------------------------------------------------- claim 11's readers


def ci_pin_module():
    """`scripts/ci-pin.py`, imported by the one idiom its header documents.

    THE ANCHORING IS NOT REIMPLEMENTED HERE. Which lines are the workflow's own
    `env:` block — as against a block indented under a job — is exactly the
    question that script exists to answer, and a second answer to it living in
    this file would be the defect this claim is about, one level up.

    The two lines are `sys.path` plus `import_module("ci-pin")`: the file's
    name has a hyphen in it, because that is the name every caller spells on a
    command line. Resolved against THIS FILE's directory, never against
    `--root`: the self-test's miniature repo has a ci.yml and no scripts/, and
    the reader it must be checked with is this tree's. EVERY FAILURE IS CAUGHT,
    not just the import machinery's — a `SyntaxError` in the reader is the same
    event to this row as a missing file, and a traceback would say so without
    naming what went unchecked.
    """
    sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
    try:
        return importlib.import_module("ci-pin")
    except Exception as exc:
        raise Bail(f"cannot import scripts/ci-pin.py ({exc.__class__.__name__}: {exc}), which is "
                   "this repo's one reader of ci.yml's tool pins and the source of claim 11's "
                   "population. " + NO_TEACH) from exc


def workflow_pins() -> dict[str, str]:
    """`ci.yml`'s workflow-level pins, name -> value, version-shaped ones only.

    THE FILTER IS ON THE VALUE, not on the name. A name-based filter would read
    `*_VERSION` and miss a pin called something else — which is precisely the
    blind spot the sweep behind this claim recorded about ITS OWN pattern, so
    reproducing it here would be a joke at this file's expense.
    """
    ci_pin = ci_pin_module()
    try:
        with open(HOSTED_HALF, encoding="utf-8") as fh:
            pins = ci_pin.read_pins(fh.read(), HOSTED_HALF)
    except ci_pin.Refuse as why:
        raise Bail(f"{HOSTED_HALF}: {why}. Claim 11's population is that block, so a block this "
                   "repo's own pin reader will not read leaves the claim with nothing to check "
                   "rather than with nothing to report. " + NO_TEACH) from why
    return {name: val for name, val in pins.items() if VERSION_LITERAL_RE.fullmatch(val)}


def pin_literals(root: str) -> list[tuple[str, int, str, str]]:
    """Every version literal under PIN_TREE: `(path, line number, literal, line)`.

    DERIVED, NEVER A ROSTER. The point of this claim is that a hand-listed set
    of sites falls behind the tree; a hand-listed set of sites to SCAN would
    fall behind it in the same way, one level further out.

    THE POPULATION IS GIT'S INDEX, NOT THE FILESYSTEM, which is the same
    answer `scripts/check-python-lint.py` gives to the same question one file
    over — and for the sharper reason here. A directory walk reads whatever is
    sitting in the tree: this repo's `.gitignore` carries `*.local.*`, which is
    an INVITATION to keep personal files in place, and a developer's
    `local-scripts/notes.local.md` mentioning a version would red their local
    gate over a file the repo told them was theirs. It cannot red hosted (a
    runner checks out a clean tree), which is exactly what would make it a
    confusing, one-sided red.
    """
    listed = subprocess.run(["git", "ls-files", "-z", "--", PIN_TREE], cwd=root,
                            capture_output=True, text=True, check=False)
    if listed.returncode != 0:
        raise Bail(f"`git ls-files -- {PIN_TREE}` failed under {root} "
                   f"({listed.stderr.strip() or 'no message'}). That listing IS claim 11's "
                   "population — the tracked files of the local half — and this check will not "
                   "fall back to a directory walk, because a walk reads a developer's own "
                   "untracked files and reds their gate over them. " + NO_TEACH)
    out: list[tuple[str, int, str, str]] = []
    for rel in sorted(x for x in listed.stdout.split("\0") if x):
        full = os.path.join(root, rel)
        try:
            with open(full, encoding="utf-8", errors="replace") as fh:
                body = fh.read().splitlines()
        except OSError:
            # A tracked path that is not a readable file today — a symlink to
            # nowhere, a submodule. Nothing to read is nothing to reconcile.
            continue
        for i, line in enumerate(body, 1):
            for lit in VERSION_LITERAL_RE.findall(line):
                out.append((rel, i, lit, line.strip()))
    return out


def tool_names(pin_name: str) -> list[str]:
    """Every spelling of a pin a line might name it by, derived from its key:
    `NEXTEST_VERSION` -> `nextest`; `FREECAD_APPIMAGE_VERSION` -> `freecad`
    and `appimage`.

    EVERY UNDERSCORE-SEPARATED PART, not the key with `_VERSION` stripped,
    because a token that is not the tool's spelling makes arm B silently inert
    for that pin: `freecad_appimage` is a string no line in this repo writes,
    so a claim resting on it would check nothing and say nothing. `VERSION`
    itself is dropped — it is the key's suffix, not a tool.

    THE PIN'S OWN KEY needs no entry of its own: arm B's boundary treats `_` as
    a boundary character, so `nextest` matches inside `NEXTEST_VERSION=0.16.0`
    — which is the most literal restatement a pin can have, and was escaping
    this arm entirely while being exactly its subject.
    """
    return [w for w in pin_name.lower().split("_") if w and w != "version"]


def tool_token(pin_name: str) -> str:
    """The tool a pin names, for a message a human reads."""
    return "_".join(w for w in pin_name.lower().split("_") if w and w != "version")


def unconditional(jobs: dict[str, "Job"], name: str, seen: frozenset[str] = frozenset()) -> str | None:
    """None if job `name` runs on every tier; otherwise the reason it may not.

    `if:` is not the only door. **GitHub skips a job whose `needs:` dependency
    skipped**, so a job with no condition of its own is still conditional if
    anything it waits on is — and adding a `needs:` for an output or for
    ordering is a far likelier accident than deleting a step. Claim 7 read only
    `has_if` until a verification pass restored S61's exact state by giving the
    siting job `needs: discipline`.
    """
    if name in seen:
        return f"`{name}` is part of a `needs:` cycle, which this reader will not reason about"
    job = jobs.get(name)
    if job is None:
        return f"`{name}` is named in a `needs:` and is not a job in this workflow"
    if job.has_if:
        return f"`{name}` carries an `if:`"
    if job.continue_on_error:
        return f"`{name}` carries `continue-on-error: true`, so it runs but cannot redden the PR"
    for dep in job.needs:
        why = unconditional(jobs, dep, seen | {name})
        if why is not None:
            return f"`{name}` waits on {why} — GitHub skips a job whose dependency skipped"
    return None


def check(root: str, floor: int = MIRROR_MARKER_FLOOR) -> list[str]:
    errs: list[str] = []

    def err(msg: str) -> None:
        errs.append(msg)

    os.chdir(root)
    for required in (HOSTED_HALF, LOCAL_HALF):
        if not os.path.isfile(required):
            raise Bail(
                f"{required} does not exist under {root} — half of this check's subject is missing. "
                "This runs in the one job that does NOT prune local-scripts/, so if a runner reports "
                "this, the prune has spread to that job: remove it from that job rather than making "
                f"this check tolerate the absence. {NO_TEACH}"
            )

    # THE HOSTED HALF IS EVERY WORKFLOW FILE, not just ci.yml (widened
    # 2026-08-22). Claims 6, 8 and 9 already span the directory, for the reason
    # each of them states: a rule that reads ONE file offers every other file
    # as the place to put the thing it forbids. Claims 1, 2 and 5 were the last
    # ones still reading ci.yml alone, and the hole was not hypothetical — the
    # `watertight` job MOVED to nightly.yml that same day, and with it
    # `scripts/check_admesh.sh` fell out of the hosted population while
    # continuing to run hosted every night. Read as a one-file rule, the fix
    # would have been an exemption declaring the row local-only: a sentence
    # that is false, inside the gate whose subject is sentences being true.
    #
    # HOSTED_HALF stays what it is — claim 7's siting rule is specifically
    # about ci.yml, where the tier-blind jobs live, and claim 9's placement
    # rule reads each file's own text.
    hosted_lines = [ln for wf in sorted(os.listdir(WORKFLOW_DIR))
                    if wf.endswith((".yml", ".yaml"))
                    for ln in non_comment(f"{WORKFLOW_DIR}/{wf}")]
    local_lines = non_comment(LOCAL_HALF)
    hosted = invocations(hosted_lines)
    local = invocations(local_lines)
    if not hosted or not local:
        raise Bail(f"{WORKFLOW_DIR + '/*' if not hosted else LOCAL_HALF} names no scripts/ or demos/ path at "
                   "all. Every claim below is a comparison between two populations, and a comparison "
                   f"against an empty one passes for the wrong reason." + teach("`SCRIPT_RE`"))

    # CLAIM 1 — invocation parity outside scripts/gates/, both directions.
    for path in sorted((hosted | local) - {p for p in hosted | local if p.startswith("scripts/gates/")}):
        in_h, in_l = path in hosted, path in local
        if in_h and in_l:
            # AN EXEMPTION IS A CONFESSION WITH AN EXPIRY. Nothing used to
            # check that one was still needed, so an entry whose row came back
            # would have sat here forever, reading as a live asymmetry.
            if path in MIRROR_EXEMPT:
                want, reason = MIRROR_EXEMPT[path]
                err(f"{path} is declared {want}-only in MIRROR_EXEMPT and BOTH halves now name it. The "
                    f'reason ("{reason}") has expired — delete the entry, so the list stays a record of '
                    "asymmetries that exist rather than of ones that once did")
            continue
        side = "hosted" if in_h else "local"
        if path in MIRROR_EXEMPT:
            want, reason = MIRROR_EXEMPT[path]
            if want != side:
                err(f"{path} is declared {want}-only in MIRROR_EXEMPT but is invoked by the {side} half. "
                    f'The exemption now describes the opposite of the tree — re-read the reason ("{reason}") '
                    "and fix whichever side moved")
            continue
        if in_h:
            names, blind = f"a workflow in {WORKFLOW_DIR}/", f"{LOCAL_HALF} does not"
            why = "a row added to one side is invisible on the other until someone reads for it"
        else:
            names, blind = LOCAL_HALF, f"no workflow in {WORKFLOW_DIR}/ does"
            why = "A check that runs only on a developer's machine gates nothing on merge"
        err(f"{names} invokes {path} and {blind}. Every check "
            f"outside scripts/gates/ is named by hand in both halves, so {why} — mirror it, or declare it "
            "in MIRROR_EXEMPT with the reason it is one-sided")

    for path, (want, reason) in sorted(MIRROR_EXEMPT.items()):
        if path not in hosted and path not in local:
            err(f"{path} is declared {want}-only in MIRROR_EXEMPT and NEITHER half names it. Either the "
                f'row is gone and the entry should go with it, or ("{reason}") is describing a check '
                "that stopped running anywhere")

    # CLAIM 2 — gate MODE parity. Claim 1 excludes scripts/gates/ because both
    # halves take that roster from the directory, but the directory says
    # nothing about the FLAGS a gate is run with, and a gate's flagged mode is
    # a different check. `--citations` was hosted-only when this claim was
    # written, and nothing could see it.
    h_modes, l_modes = gate_modes(hosted_lines), gate_modes(local_lines)
    for mode, (want, reason) in sorted(GATE_MODE_EXEMPT.items()):
        if mode not in (h_modes ^ l_modes):
            err(f"`{mode}` is declared {want}-only in GATE_MODE_EXEMPT and is no longer one-sided "
                f'("{reason}"). Delete the entry')
    for mode in sorted(h_modes ^ l_modes):
        side = "hosted" if mode in h_modes else "local"
        if mode in GATE_MODE_EXEMPT:
            want, reason = GATE_MODE_EXEMPT[mode]
            if want != side:
                err(f"`{mode}` is declared {want}-only in GATE_MODE_EXEMPT but is invoked by the {side} "
                    f'half ("{reason}")')
            continue
        err(f"`{mode}` is invoked by the {side} half only. A gate's flagged mode is a separate check: the "
            "directory loop in the local half runs every gate in DEFAULT mode and sees no flag, so a mode "
            "wired on one side only runs on one side only — mirror it, or declare it in GATE_MODE_EXEMPT")

    # CLAIM 3 — a mirrored path exists on disk. `gate-roster.sh` makes exactly
    # this check for its own directory; without it a typo present in both
    # halves is perfect parity over a file that is not there.
    for path in sorted(hosted | local):
        if not os.path.isfile(path):
            err(f"both halves name {path} and no such file exists — a renamed or deleted check leaves "
                "rows invoking a stale path in perfect agreement with each other")

    # CLAIM 4 — no orphan executables under scripts/ or demos/. The seed is
    # BOTH halves plus every workflow file: `render.yml` is reached through a
    # `uses:` job and runs the render entry points, so a script owned only by
    # it is owned, not orphaned.
    #
    # TWO ARMS OVER ONE POPULATION, and they are one question with one
    # parameter different: does this script run, and does its `--selftest`
    # mode run. Written as a separate claim the second one grew its own
    # invocation matcher, its own local population and its own read of the
    # workflow directory within a day — three copies of a decision this file
    # exists to keep single — so it is written here, in the loop it belongs
    # to, sharing `SCRIPT_RE`, the closure, and the directory walk. The second
    # arm's own caller rule is narrower than `owned` and says why at
    # `selftest_callers`.
    #
    # THE POPULATION IS NON-RECURSIVE, which is what puts `scripts/gates/`
    # outside BOTH arms: that directory is `gate-roster.sh`'s roster ground,
    # and its rule — *a guard that has never been shown to fire is not a
    # guard* — is the second arm's rule, enforced there for `scripts/gates/*.sh`
    # minus `lib.sh`. A `.py` under `scripts/gates/` would be watched by
    # neither this loop nor that roster. There are none, and if one appears
    # the roster is the place to widen, not this walk.
    wf_seeds: set[str] = set()
    file_env: dict[str, dict[str, str]] = {}
    for wf in sorted(os.listdir(WORKFLOW_DIR)):
        if wf.endswith((".yml", ".yaml")):
            wf_seeds |= invocations(non_comment(f"{WORKFLOW_DIR}/{wf}"))
    owned = reachable(root, hosted | local | wf_seeds)
    selftested = selftest_callers()
    for d in ("scripts", "demos"):
        for name in sorted(os.listdir(d)):
            p = f"{d}/{name}"
            if not os.path.isfile(p) or not name.endswith((".sh", ".py")):
                continue
            if p.startswith("scripts/gates/"):
                continue
            if p not in owned:
                err(f"{p} is an executable check under {d}/ that NEITHER half names and no named "
                    "script reaches. Outside scripts/gates/ there is no roster property at all, so a "
                    "check can be written, committed and never run by anything — wire it into both "
                    "halves, or move it")
                continue
            # ARM TWO. `scripts/opt-level-calibrate.py --selftest` was
            # substantial, was cited by a sibling lane's comment as the
            # precedent for siting such a row, and was invoked by nothing —
            # while the script itself ran three times a night, so arm one was
            # satisfied and said so.
            if declares_selftest(p) and p not in selftested:
                err(f"{p} implements a `--selftest` mode and NO WORKFLOW under {WORKFLOW_DIR}/ passes "
                    "it that flag. A selftest is the only evidence that a guard still fires, so one "
                    "nothing runs makes the guard's greenness a statement about nothing. A row in "
                    f"{os.path.dirname(LOCAL_HALF)}/ is not a substitute and is not read here: every "
                    "hosted job DELETES that tree at checkout, so a selftest whose only caller lives "
                    "there runs in no CI at all. Add the hosted row — and mirror it, which claims 1 "
                    "and 7 want anyway. For a script under scripts/, that row belongs on a PER-PR "
                    "job: a check sited only in a scheduled workflow surfaces at the next fire, to "
                    "nobody, and a row that fails to run at all reports the same green as a row that "
                    "ran and passed")

    # CLAIM 5 — the `tools/` crates. They are checked by `cd tools/X && cargo …`
    # rows, which carry no scripts/ or demos/ path for claim 1 to match; this is
    # the smallest total statement about that directory.
    for name in sorted(os.listdir("tools")):
        if not os.path.isdir(f"tools/{name}"):
            continue
        in_h = any(f"tools/{name}" in line for line in hosted_lines)
        in_l = any(f"tools/{name}" in line for line in local_lines)
        if not (in_h and in_l):
            side = "the hosted half only" if in_h else ("the local half only" if in_l else "neither half")
            err(f"tools/{name} is a workspace-excluded crate named by {side}. The only check "
                "that reaches such a tree without being named in a row is scripts/doc-gate.sh, "
                "which derives its cargo roots and so reads this one's PROSE — nothing else does, "
                "so a tool fmt-ed, linted or tested on one side only is checked on one side only")

    # CLAIM 6 — the prune exception is EXACTLY ONE JOB, across every workflow
    # file. `render.yml` runs four checked-out lanes of its own through a
    # `uses:` job, and a hole there is the same hole. Both trees are checked,
    # and the ORDER is checked: a job that reads local-scripts/ and then
    # deletes it has read it.
    siting_job_seen = False
    for wf in sorted(os.listdir(WORKFLOW_DIR)):
        if not wf.endswith((".yml", ".yaml")):
            continue
        path = f"{WORKFLOW_DIR}/{wf}"
        for job in read_workflow(path):
            steps = job.steps
            if job.uses is not None:
                # A reusable-workflow call has no steps of its own. It is not
                # exempt from this claim — the workflow it calls is read in
                # this same loop, which is how `render.yml`'s four checked-out
                # lanes are covered — but it must actually point in here.
                called = job.uses.removeprefix("./")
                if not called.startswith(f"{WORKFLOW_DIR}/") or not os.path.isfile(called):
                    err(f"{path} job `{job.name}` calls `{job.uses}`, which is not a workflow file in "
                        f"{WORKFLOW_DIR}/. This check reads that directory and nothing else, so a job "
                        "calling out of it runs steps nobody here has looked at")
                continue
            checkout = next((i for i, st in enumerate(steps)
                             if any("actions/checkout" in line for line in st.lines)), None)
            if checkout is None:
                continue
            reads = next((i for i, st in enumerate(steps)
                          if any(("local-scripts" in line or ".claude" in line) and "rm -rf" not in line
                                 for line in st.lines)), None)
            pruned = {t for i, st in enumerate(steps) for line in st.lines
                      if "rm -rf" in line and i > checkout
                      for t in ("local-scripts", ".claude") if t in line}
            first_prune = next((i for i, st in enumerate(steps)
                                if any("rm -rf" in line and "local-scripts" in line for line in st.lines)), None)
            if path == HOSTED_HALF and job.name == SITING_JOB:
                siting_job_seen = True
                if pruned:
                    err(f"job `{SITING_JOB}` prunes {' and '.join(sorted(pruned))}, but it is the one job "
                        "whose subject is the agreement between the halves — with the tree deleted it can "
                        "only check the hosted side, and the gates sited there pass for the wrong reason")
                continue
            missing = {"local-scripts", ".claude"} - pruned
            if missing:
                err(f"{path} job `{job.name}` checks the repo out and does not delete "
                    f"{' and '.join(sorted(missing))}. That deletion is what makes `scripts/ci-filter.py` "
                    "right to classify a change under either tree as non-triggering for the build rows; a "
                    "job that keeps them can couple the hosted gate to a developer's machine or an agent's "
                    "container without anything saying so")
            elif reads is not None and first_prune is not None and reads < first_prune:
                err(f"{path} job `{job.name}` reads local-only tooling at step {reads + 1} and prunes it at "
                    f"step {first_prune + 1}. The prune is only structural while it comes FIRST")
    if not siting_job_seen:
        err(f"{HOSTED_HALF} has no job `{SITING_JOB}` that checks the repo out. That job is where the "
            "checks whose inputs are docs, prose and local-scripts/ are sited; without it they are back in "
            "a job that skips on exactly the change class they are about")

    # CLAIM 7 — THE SITING RULE ITSELF. See TIER_BLIND's comment.
    ci_jobs = {j.name: j for j in read_workflow(HOSTED_HALF)}
    local_exit = local_docs_exit_line(non_comment(LOCAL_HALF))
    for want in TIER_BLIND:
        hosts = [j for j in ci_jobs.values() if any(want in line for st in j.steps for line in st.lines)]
        if not hosts:
            err(f"no ci.yml job runs `{want}`, which is one of the checks whose INPUTS are the docs tier. "
                "A check nobody runs cannot fire anywhere")
            continue
        reasons = {j.name: unconditional(ci_jobs, j.name) for j in hosts}
        if all(r is not None for r in reasons.values()):
            detail = "; ".join(f"{n}: {r}" for n, r in sorted(reasons.items()))
            err(f"`{want}` is run only by job(s) {', '.join(sorted(reasons))}, and not one of them runs "
                f"on every tier — {detail}. Its inputs are prose, documentation or local-scripts/, file "
                "classes that make a change set TIER=docs, on which every `if: run_build` job is skipped, "
                "so it cannot fire on the only change class that breaks it. That is the exact state S61 "
                "recorded. Site it in a job that no condition and no `needs:` chain can skip")
        # The local half is half the pair, and the same rule binds it.
        at = local_call_sites(non_comment(LOCAL_HALF), want)
        if not at:
            err(f"{LOCAL_HALF} never runs `{want}`. The rule is not hosted-side-only: a check the local "
                "half cannot run is a check a developer cannot run before pushing")
        elif min(at) > local_exit:
            err(f"{LOCAL_HALF} RUNS `{want}` only after its docs-tier `exit 0` — a definition above the "
                "exit is not a run — so on a docs-only or "
                "local-scripts-only change the local half runs it not at all — S61's defect, in the other "
                "half of the pair. Move the row above the early exit")

    # CLAIM 8 — mirror citations resolve, job AND step.
    #
    # EVERY WORKFLOW FILE, not just ci.yml. A citation names `<job> / <step>`,
    # and jobs live wherever a workflow file puts them: `render.yml` carries
    # four of this repo's build lanes. Reading one file would make the job half
    # of a citation resolvable only by coincidence of which file it landed in.
    file_jobs: dict[str, dict[str, Job]] = {}
    for wf in sorted(os.listdir(WORKFLOW_DIR)):
        if wf.endswith((".yml", ".yaml")):
            wf_path = f"{WORKFLOW_DIR}/{wf}"
            file_jobs[wf_path] = {j.name: j for j in read_workflow(wf_path)}
            file_env[wf_path] = workflow_env(wf_path)
    all_jobs: dict[str, Job] = {}
    job_file: dict[str, str] = {}
    for wf_path, here in file_jobs.items():
        for j in here.values():
            if j.name in all_jobs:
                # A `<job> / <step>` citation has no file half, so two jobs of
                # one name make every citation of that name ambiguous — and the
                # ambiguity is silent, because either one resolves.
                err(f"job `{j.name}` is defined in both {job_file[j.name]} and {wf_path}. A HOSTED "
                    "MIRROR citation names a job and a step and has no room for a file, so a "
                    "duplicated job name makes every citation of it resolve against whichever "
                    "workflow this check read first. Rename one")
                continue
            all_jobs[j.name] = j
            job_file[j.name] = wf_path
    steps = {(j.name, st.name) for j in all_jobs.values() for st in j.steps if st.name}
    # Markers ARE comments, so they are read from the raw file. Claim 10 needs
    # WHERE each one sits as well as what it says — the row it declares is the
    # code under it — so the sites are kept and the set derived from them.
    with open(LOCAL_HALF, encoding="utf-8") as fh:
        local_raw = fh.read().splitlines()
    marker_sites = [(i, m.group(1)) for i, line in enumerate(local_raw)
                    for m in [MARKER_RE.search(line)] if m]
    markers = sorted({text for _at, text in marker_sites})
    # Against the RAW lines, so a marker's line number and a function's body
    # are measured on one coordinate system. Claim 7 reads the same functions
    # off the comment-stripped copy, which is the right frame for ITS question
    # (where a row runs) and the wrong one for this (which lines a row is).
    local_funcs = shell_functions(local_raw)
    for marker in markers:
        if " / " not in marker:
            err(f"{LOCAL_HALF} has a HOSTED MIRROR marker `{marker}` that is not `<job> / <step name>`. The "
                "job half is the part that drifted last time — the prose named the wrong job for a step "
                "that existed")
            continue
        job_name, step_name = marker.split(" / ", 1)
        if (job_name, step_name) not in steps:
            err(f"{LOCAL_HALF} cites a hosted mirror `{marker}`, and no workflow in {WORKFLOW_DIR}/ "
                f"has a step named `{step_name}` in a job `{job_name}`. Renaming a step or moving it "
                "between jobs leaves every sentence citing it quietly false")
    if len(markers) < floor:
        err(f"{LOCAL_HALF} carries {len(markers)} HOSTED MIRROR marker(s), below the "
            f"{floor} it had when this floor was set. Deleting a marker is how a citation "
            "check becomes vacuous; if a row genuinely lost its hosted mirror, lower the floor deliberately")

    # CLAIM 9 — JOB PARITY. Every job in ci.yml is either MIRRORED (some
    # HOSTED MIRROR marker in the local half names it, and claim 8 above has
    # already proved that citation resolves to a real step) or CONFESSED, by a
    # `# NO LOCAL MIRROR: <reason>` comment written at the job itself.
    #
    # WHY THE HOSTED SIDE IS ENUMERATED AND THE LOCAL SIDE DECLARES. The two
    # halves do not share a partition of the work — one hosted job can be
    # three local rows and one local row can span two hosted jobs — so any
    # rule that DERIVED the local list by a pattern and matched it against job
    # names would be checking a coincidence, which is the mistake both this
    # file's header and `gate-roster.sh`'s record having made. So the side
    # that can be enumerated exactly is enumerated (the recogniser above), and
    # the side that cannot is required to say, in writing, which hosted job
    # each of its rows answers to.
    #
    # WHY THE REASON LIVES IN ci.yml AND NOT IN A TABLE HERE. A list of
    # exceptions in this file is a place to put things, invisible from the job
    # it excuses. At the job, it is in the diff of anyone adding a job and in
    # front of anyone reading one. A confession with no reason is not a
    # confession, so an empty one is an error; and a job that is both cited
    # and excused has an expired confession, exactly as MIRROR_EXEMPT does.
    # AND CLAIM 9 SPANS THEM TOO, for the reason that is the whole point of a
    # parity gate: a rule that reads one file offers every other file as the
    # place to put a job. `render.yml` is not hypothetical — it runs `cargo run
    # -p tour` and `demos/render-uv.sh` on checked-out trees — so a check that
    # stopped at ci.yml would have been declaring parity over a subset of the
    # gate the whole time.
    # PER FILE, against that file's OWN jobs: the placement rule ("the comment
    # block that runs down to a job's key") is a property of one file's text,
    # and reading it against a deduplicated job set would report a correctly
    # placed reason as detached whenever some other file claimed the name.
    excused: dict[str, str] = {}
    for wf_path, here in file_jobs.items():
        excused.update(_no_mirror_reasons(wf_path, here))
    cited_jobs = {m.split(" / ", 1)[0] for m in markers if " / " in m}
    for name in sorted(all_jobs):
        reason = excused.get(name)
        where = job_file[name]
        if name in cited_jobs and reason is not None:
            err(f"{where} job `{name}` carries `NO LOCAL MIRROR` and {LOCAL_HALF} cites it anyway "
                f'("{reason}"). The confession has expired — delete it, so the comment stays a record '
                "of jobs that have no local half rather than of ones that once did")
            continue
        if name in cited_jobs:
            continue
        if reason is None:
            err(f"{where} job `{name}` has no local mirror: no `# HOSTED MIRROR: {name} / <step>` marker "
                f"in {LOCAL_HALF} names it, and the job carries no `# NO LOCAL MIRROR:` line. A hosted "
                "job with no local half is invisible to `local-scripts/gate.sh`, which documents itself "
                "as the merge gate when hosted Actions is unavailable — that is how `oracle-certify` "
                "went unmirrored. Mirror it and cite it, or write the reason it cannot be at the job")
        elif not reason:
            err(f"{where} job `{name}` carries a `# NO LOCAL MIRROR:` line with no reason after it. An "
                "exception with no reason is where the next one goes: say why this job has no local "
                "half, in the same line")
        elif len(reason.split()) < NO_MIRROR_MIN_WORDS or len(reason) < NO_MIRROR_MIN_CHARS:
            err(f'{where} job `{name}` carries the reason "{reason}", which is under '
                f"{NO_MIRROR_MIN_WORDS} words or {NO_MIRROR_MIN_CHARS} characters. That is the empty "
                "case spelled legally: it satisfies the check without saying what the check asked "
                "for. Say what this job does that no local row can do")

    # CLAIM 10 — SEMANTICS-BEARING FLAG AND ENVIRONMENT PARITY, per mirrored
    # pair.
    #
    # Claims 1, 2 and 9 are about the ROSTER: which checks each half names,
    # which gate modes it runs, that every hosted job is cited or says why it
    # has no local half. None of them reads the flags on the commands, so two
    # rows can be paired, identically named, both green, and be running the
    # same test binary under different reporting semantics. That is not
    # hypothetical: `--no-fail-fast` was added to hosted's two sharded nextest
    # rows and not to the local half, hosted reported a shard's whole failure
    # surface while the local half reported one failure per row, and this
    # checker said OK across that state.
    #
    # WHAT A PAIR IS: a `HOSTED MIRROR` citation. Claim 8 has already proved
    # the citation resolves, so the two sides are a hosted step's `run:` and
    # the local row under the marker (`marker_row`).
    #
    # ONLY WHERE BOTH SIDES RUN THE SAME COMMAND. A hosted job's steps and a
    # local row's function body are not the same partition of the work — claim
    # 9's header says so at length — so a `cargo doc` on one side and a `cargo
    # clippy` on the other are not a disagreement about flags, they are the
    # granularity the two halves genuinely have. Comparing only the command
    # keys BOTH sides carry is what keeps this claim about flags. THE COST IS
    # STATED: a pair whose halves name different cargo subcommands is not
    # compared at all, and neither is a flag drop that also changes the
    # subcommand.
    #
    # THE ENV ARM IS THE SAME QUESTION ONE TOKEN-CLASS WIDER, and it is here
    # rather than in a claim of its own because "do these two halves run the
    # same command" does not stop at argv. `RUSTFLAGS='--cfg
    # getrandom_backend="wasm_js"'` rides both halves of the wasm pair and the
    # flag arm reads NONE of it: `cargo_flags` starts at the `cargo` token and
    # discards everything before it, so dropping the prefix from one half, or
    # changing the backend name on one half, left the pair green.
    #
    # ITS EXTENT IS THE PAIR, NOT THE CARGO COMMAND, and the two differ in
    # both directions. Wider: it reads a pair whose halves run no cargo at all
    # (the `scene-inputs` render steps), and a pair whose halves name
    # different subcommands, both of which the flag arm skips. Narrower per
    # name: only `SEMANTIC_ENV`, and the throughput knobs the local half
    # deliberately does not mirror are out by name at that table rather than
    # by exemption. Hosted's environment is its job block, then its step
    # block, then any inline prefix — GitHub's own precedence — and the local
    # half's is the prefixes on the row under the marker.
    for at, marker in marker_sites:
        if " / " not in marker:
            continue
        job_name, step_name = marker.split(" / ", 1)
        job = all_jobs.get(job_name)
        step = next((st for st in job.steps if st.name == step_name), None) if job else None
        if step is None:
            continue  # claim 8 has already reported this citation
        h_cmds = cargo_flags(f"{job_file[job_name]} job `{job_name}` step `{step_name}`", step.run)
        l_cmds = cargo_flags(f"{LOCAL_HALF}:{at + 1} (the row citing `{marker}`)",
                             marker_row(local_raw, at, local_funcs))
        shared = sorted(set(h_cmds) & set(l_cmds))
        # THE OTHER WAY A HOSTED STEP CAN SET A VARIABLE, refused rather than
        # missed. A step that appends to `$GITHUB_ENV` sets it for every LATER
        # step of the job, and nothing in this reader would see it. EVERY step
        # of the job is read, not only those above the cited row: one above it
        # sets the variable for this pair, and one below it may be cited by
        # another marker. The test is a substring — the name of the file
        # GitHub reads — which is coarse in the safe direction. No job does it
        # today; a Bail keeps that from changing in silence.
        for other in job.steps:
            if any("GITHUB_ENV" in ln for ln in other.run):
                raise Bail(f"{job_file[job_name]} job `{job_name}` writes to $GITHUB_ENV in step "
                           f"`{other.name}`, and `{marker}` is a mirrored pair in that job. Claim 10's "
                           "env arm reads `env:` blocks and inline prefixes; a variable exported this "
                           "way reaches the pair's step without appearing in either, and would compare "
                           "equal to a local half that never sets it."
                           + teach("`check`'s env arm and `env_prefixes`"))
        # THE FOUR LAYERS, IN GITHUB'S OWN ORDER: workflow, job, step, inline
        # prefix, each overriding the one before it. The filter is applied to
        # the block layers here and inside `env_prefixes` for the shell one —
        # the blocks carry every variable a workflow declares, and comparing
        # `TIER` or `GH_TOKEN` against a shell script would be a checker of
        # GitHub rather than of this repo.
        # A `uses:` STEP RUNS AN ACTION, and what it does is decided by the
        # `with:` block rather than by any argv or environment this reader
        # compares. No mirrored pair cites one today; the refusal costs a line
        # and keeps that from changing in silence, which is the same bargain
        # the `$GITHUB_ENV` and standing-assignment refusals make.
        if not step.run and any(ln.strip().startswith("uses:") for ln in step.lines):
            raise Bail(f"{job_file[job_name]} job `{job_name}` step `{step_name}` is a `uses:` step "
                       f"and `{marker}` cites it as a mirrored pair. Claim 10 compares argv and "
                       "environment; an action's behaviour is its `with:` inputs, which this reads "
                       "as neither — so the pair would be checked by nothing while every roster "
                       "claim above says the two halves run the same check."
                       + teach("`check`'s env arm"))
        h_env = {k: v for k, v in file_env[job_file[job_name]].items() if semantic_env(k)}
        h_env.update({k: v for k, v in job.env.items() if semantic_env(k)})
        h_env.update({k: v for k, v in step.env.items() if semantic_env(k)})
        h_prefix, h_partial = env_prefixes(
            f"{job_file[job_name]} job `{job_name}` step `{step_name}`", step.run)
        h_env.update(h_prefix)
        l_env, l_partial = env_prefixes(f"{LOCAL_HALF}:{at + 1} (the row citing `{marker}`)",
                                        marker_row(local_raw, at, local_funcs))
        # THE UNWATCHED-ABSENCE DIRECTION, first for both arms and the one
        # MIRROR_EXEMPT has had all along at its "NEITHER half names it"
        # branch. Without it an exemption is not a watched asymmetry but an
        # UNWATCHED ABSENCE: delete `--features interval` from the local
        # interval row and the entry excusing it stops matching anything, so
        # the flag is gone from both halves and nothing says a word — this
        # claim's own headline defect, reintroduced by its exemption table.
        for flag in sorted(f for (m, f) in PAIR_EXEMPT if m == marker and f.startswith("-")):
            want, reason = PAIR_EXEMPT[(marker, flag)]
            if any(_presence(h_cmds[c], flag)[0] or _presence(l_cmds[c], flag)[0] for c in shared):
                continue
            err(_exempt_unwatched(marker, flag, want, reason, "passes")
                + " (read over the commands both halves run)")
        for cmd in shared:
            h_inv, l_inv = h_cmds[cmd], l_cmds[cmd]
            for flag in sorted({f for inv in h_inv + l_inv for f in inv}):
                declared = PAIR_EXEMPT.get((marker, flag))
                h_any, h_all, h_vals = _presence(h_inv, flag)
                l_any, l_all, l_vals = _presence(l_inv, flag)
                if h_any and l_any:
                    if declared is not None:
                        # `both` declares the halves DISAGREE about the value,
                        # so agreement is its expiry — and an OPAQUE value
                        # reads as agreement here, where the declaration is
                        # what is being checked rather than the tree.
                        agree = OPAQUE in h_vals or OPAQUE in l_vals or h_vals == l_vals
                        msg = _exempt_present(marker, flag, declared[0], declared[1],
                                              True, True, agree, FLAG_VERB, f" on `{cmd}`")
                        if msg:
                            err(msg)
                        continue
                    if h_all != l_all:
                        side = "hosted" if h_all else "local"
                        err(f"the pair `{marker}` passes `{flag}` on EVERY `{cmd}` in the {side} half and "
                            "on only some of them in the other. Both halves name the flag, so a union "
                            "over the row would call that agreement — and it is how a flag dropped from "
                            "the row's own command hides behind a sibling that still carries it. Put it "
                            "on every invocation, or declare the pair in PAIR_EXEMPT")
                        continue
                    if OPAQUE in h_vals or OPAQUE in l_vals or h_vals == l_vals:
                        continue
                    err(f"the pair `{marker}` passes `{flag}` on `{cmd}` with different values — hosted "
                        f"{sorted(h_vals)}, local {sorted(l_vals)}. A selection that narrows on one side "
                        "runs fewer tests under the same row name, and nothing else here would say so: "
                        "mirror the value, or declare the pair in PAIR_EXEMPT with the reason it "
                        "differs (side `both`)")
                    continue
                if declared is not None:
                    msg = _exempt_present(marker, flag, declared[0], declared[1],
                                          h_any, l_any, False, FLAG_VERB, f" on `{cmd}`")
                    if msg:
                        err(msg)
                    continue
                side = "hosted" if h_any else "local"
                err(f"the pair `{marker}` passes `{flag}` on `{cmd}` in the {side} half only. This is one "
                    "of the flags that changes what a run MEANS rather than what it executes "
                    "(SEMANTIC_FLAGS), so the two halves now disagree about what a red run reports while "
                    "every roster claim above still says they run the same check — mirror it, or declare "
                    "the pair in PAIR_EXEMPT with the reason it is one-sided")

        # THE ENV ARM. Same table, same three expiry directions, and the
        # unwatched-absence one FIRST for the reason it is first above: an
        # entry that has stopped matching anything is a variable gone from
        # both halves with a confession sitting on top of it.
        for name in sorted(v for (m, v) in PAIR_EXEMPT if m == marker and not v.startswith("-")):
            want, reason = PAIR_EXEMPT[(marker, name)]
            if name in h_env or name in l_env or h_partial or l_partial:
                continue
            err(_exempt_unwatched(marker, name, want, reason, "sets"))
        for name in sorted(set(h_env) | set(l_env)):
            declared = PAIR_EXEMPT.get((marker, name))
            h_val, l_val = h_env.get(name), l_env.get(name)
            if h_val is not None and l_val is not None:
                agree = OPAQUE in (h_val, l_val) or h_val == l_val
                if declared is None:
                    if not agree:
                        err(f"the pair `{marker}` sets `{name}` to {h_val!r} hosted and {l_val!r} "
                            "locally. This is one of the variables that changes what a run MEANS "
                            "(SEMANTIC_ENV) — a cfg, a backend, a tolerance — so the two halves are "
                            "running different checks under one row name while every claim above says "
                            "they run the same one. Mirror the value, or declare the pair in "
                            "PAIR_EXEMPT with side `both` and the reason they differ")
                    continue
                msg = _exempt_present(marker, name, declared[0], declared[1],
                                      True, True, agree, ENV_VERB)
                if msg:
                    err(msg)
                continue
            side = "hosted" if h_val is not None else "local"
            # ONE-SIDEDNESS IS AN ABSENCE CLAIM, and an incomplete map cannot
            # support one: if the other half writes part of its environment as
            # a runner expression, this reader does not know whether that
            # expression sets this name. Reported at the claim rather than
            # guessed either way.
            if (h_partial if side == "local" else l_partial):
                continue
            if declared is not None:
                msg = _exempt_present(marker, name, declared[0], declared[1],
                                      h_val is not None, l_val is not None, False, ENV_VERB)
                if msg:
                    err(msg)
                continue
            err(f"the pair `{marker}` sets `{name}` in the {side} half only. This is one of the "
                "variables that changes what a run MEANS rather than what it costs (SEMANTIC_ENV), and "
                "a prefix dropped from one half is invisible to every other claim here: the row keeps "
                "its name, its flags and its citation. Set it on both halves, or declare the pair in "
                "PAIR_EXEMPT with the reason it is one-sided")

    for (marker, token), (want, reason) in sorted(PAIR_EXEMPT.items()):
        if marker not in markers:
            err(f"PAIR_EXEMPT declares `{token}` {want} for the pair `{marker}`, and "
                f"{LOCAL_HALF} carries no such HOSTED MIRROR marker. Either the pair is gone and the "
                f'entry should go with it, or ("{reason}") is describing a row that stopped running')

    # CLAIM 11 — the version literals under PIN_TREE against ci.yml's pins.
    # The argument is at PIN_FREE; the two arms are A (value) and B (name).
    pins = workflow_pins()
    values = set(pins.values())
    literals = pin_literals(root)
    declared: set[tuple[str, str]] = set()
    for (path, lit), reason in sorted(PIN_FREE.items()):
        if lit in values:
            err(f"PIN_FREE declares {lit} in {path} as a non-pin — {reason} — and {HOSTED_HALF} now "
                f"pins exactly {lit}. Whichever is true, arm A can no longer tell: a declared "
                "non-pin that equals a live pin is the one way a copy of that pin hides inside its "
                "own excuse. Rename the declaration's subject, or drop the entry if the literal was "
                "a pin copy all along")

    for path, lineno, lit, line in sorted(set(literals)):
        if (path, lit) in PIN_FREE:
            declared.add((path, lit))
            continue
        if lit in values:
            continue
        err(f"{path}:{lineno} names version {lit}, and {HOSTED_HALF} pins no such version — it pins "
            + ", ".join(f"{n}={v}" for n, v in sorted(pins.items()))
            + f". The line is `{line}`. If that literal is a copy of a pin, the pin has MOVED and "
            "the copy has not: a developer reading this line installs a version hosted CI no longer "
            "runs, and until now nothing said so. If it is not a pin at all, declare it in PIN_FREE "
            "with what it is — that table is the list of literals this tree is allowed to carry, and "
            "it runs that way round on purpose, so a new literal is an error until someone says "
            "what it is")

    # ONE REPORT PER LINE PER PIN. `literals` carries one entry per literal, so
    # a line with two versions on it would otherwise be reported twice for the
    # same pin — and ci-local.sh's prereq note is exactly such a line.
    by_line = {(path, lineno): line for path, lineno, _lit, line in literals}
    for name, value in sorted(pins.items()):
        # CASE-INSENSITIVE, AND `_` IS A BOUNDARY. `# Nextest 0.16.0` names the
        # tool as surely as `nextest` does, and `NEXTEST_VERSION=0.16.0` names
        # it twice; a case-sensitive matcher whose word boundary treats `_` as
        # a word character sees neither. Both were live escapes from this arm,
        # of exactly the shape this claim exists to catch. What the boundary
        # still refuses is a token INSIDE a longer word (`another` is not
        # `other`) and `nexte.st`, which is a hostname and not this token.
        named = re.compile("|".join(rf"(?<![A-Za-z0-9]){re.escape(n)}(?![A-Za-z0-9])"
                                    for n in tool_names(name)), re.IGNORECASE)
        for (path, lineno), line in sorted(by_line.items()):
            if not named.search(line):
                continue
            # A DECLARED NON-PIN IS NOT EVIDENCE ABOUT A TOOL'S VERSION, so it
            # is not counted here either. Without this, a PIN_FREE literal on a
            # line that happens to name a pinned tool reds with no declaration
            # path anywhere — arm A would excuse it and arm B could not.
            here = [x for x in VERSION_LITERAL_RE.findall(line) if (path, x) not in PIN_FREE]
            if not here or value in here:
                continue
            err(f"{path}:{lineno} names {tool_token(name)} and the version(s) {', '.join(here)}, and "
                f"none of them is {value} — the version {HOSTED_HALF} pins as {name} today. The line "
                f"is `{line}`. A line that names a tool and a version beside it is read as being "
                "about that tool's pin; if it is about something else, rewrite it so the two are not "
                "adjacent, because a reader will make the same inference this check does")

    for (path, lit), reason in sorted(PIN_FREE.items()):
        if (path, lit) not in declared:
            err(f"PIN_FREE declares {lit} in {path} as a non-pin, and that file no longer names it. "
                f'Either the line is gone and the entry should go with it, or ("{reason}") is '
                "describing a literal that has changed — a stale not-a-pin note is how a real pin "
                "copy comes to sit under cover")

    return errs


def _no_mirror_reasons(path: str, jobs: dict[str, "Job"]) -> dict[str, str]:
    """`# NO LOCAL MIRROR: <reason>` lines in a workflow, by the job each
    excuses.

    ONE PLACEMENT RULE, AND IT IS STRICT: the line must sit in the contiguous
    comment block that runs down to a job's key. Anywhere else raises `Bail`.
    The looser reading — walk up from the comment to whichever job came before
    it — silently attributes a reason written BETWEEN two jobs to the one
    ABOVE, so a confession meant for the job you are looking at excuses a
    different job entirely and both look fine. A rule that can only be
    satisfied one way is a rule a reader can check by eye.
    """
    with open(path, encoding="utf-8") as fh:
        raw = fh.read().splitlines()
    at_line = {j.line: j.name for j in jobs.values()}
    out: dict[str, str] = {}
    for i, line in enumerate(raw, 1):
        m = NO_MIRROR_RE.search(line)
        if not m:
            continue
        j = i + 1
        while j <= len(raw) and raw[j - 1].strip().startswith("#"):
            j += 1
        if j not in at_line:
            raise Bail(f"{path}:{i}: a `NO LOCAL MIRROR` line that is not against a job key — the "
                       "first non-comment line below it is not the start of a job. Write it in the "
                       "comment block that runs down to the key of the job it excuses, with no blank "
                       "line between, so the job it names is the job it is read against."
                       + NO_TEACH_TAIL)
        name = at_line[j]
        if name in out:
            raise Bail(f"{path}:{i}: job `{name}` carries two `NO LOCAL MIRROR` lines. Only one of "
                       "them would be read, and which one depends on the order they are written in "
                       "— so an empty line followed by a real one reads as excused while the same "
                       "two lines the other way round reads as a defect. A job has one reason or "
                       "none." + NO_TEACH_TAIL)
        out[name] = m.group(1)
    return out


# ---------------------------------------------------------------- self-test
#
# THE FIXTURE IS A MINIATURE REPO. Every case runs the checker AS A SUBPROCESS,
# not as an in-process call: the bash gates' shared harness runs its subject
# inside `if out=$(…)`, where bash suppresses errexit, and that is exactly the
# condition under which a `set -e` script dies before printing its own error.
# A self-test that cannot reproduce the real invocation cannot see that.
# CLAIM 11'S FIXTURE PINS. Two of them: one the local half restates correctly,
# and a second whose only job is to be a DIFFERENT pin's value, so the case
# where a literal drifts onto one can be planted. The names are what derive
# their arm-B tokens (`fixture`, `other`), and the values are chosen not to
# collide with any PIN_FREE literal the clean fixture plants below.
FIXTURE_PIN = ("FIXTURE_VERSION", "1.2.3")
# TWO WORDS, on purpose: its parts are `other` and `tool`, so a case can name
# it by its SECOND part and catch a derivation that only looks at the first.
FIXTURE_PIN_OTHER = ("OTHER_TOOL_VERSION", "4.5.6")
FIXTURE_PIN_LINE = f"# the fixture binary, pinned {FIXTURE_PIN[1]} to match hosted"


def plant_clean(t: str) -> None:
    for d in ("scripts/gates", "demos", "local-scripts", ".github/workflows", "tools/toolcrate"):
        os.makedirs(os.path.join(t, d), exist_ok=True)
    open(os.path.join(t, "tools/toolcrate/Cargo.toml"), "w").close()
    names = [f"check-{i}.sh" for i in range(FIXTURE_MIRRORED_ROWS)]
    for n in names:
        open(os.path.join(t, "scripts", n), "w").close()
    # Derived from TIER_BLIND, not listed again: a second spelling of that
    # tuple is a second roster, and this file's whole subject is rosters that
    # drift from what they describe. It is also what keeps this fixture from
    # breaking on a correct change: claim 3 requires every named check to exist
    # on disk, so a fixture that enumerated them would fail the moment the list
    # grew.
    for want in TIER_BLIND:
        open(os.path.join(t, want.split()[0]), "w").close()
    with open(os.path.join(t, HOSTED_HALF), "w") as fh:
        # CLAIM 11's population, and the PASSING shape of it: a workflow-level
        # `env:` block, which is what `scripts/ci-pin.py` anchors to. Written
        # before `jobs:` exactly as ci.yml writes it — content outside `jobs:`
        # is not read by any other claim here.
        fh.write("env:\n")
        for name, value in (FIXTURE_PIN, FIXTURE_PIN_OTHER):
            fh.write(f'  {name}: "{value}"\n')
        fh.write("jobs:\n")
        fh.write(f"  {SITING_JOB}:\n    steps:\n      - uses: actions/checkout@v4\n")
        for want in TIER_BLIND:
            fh.write(f"      - name: sited {want}\n        run: {want}\n")
        # A step the local half can cite without naming a TIER_BLIND path:
        # the citation marker must survive the plants that delete those rows.
        fh.write("      - name: sited rows\n        run: echo sited\n")
        fh.write("  discipline:\n    if: needs.filter.outputs.run_build == 'true'\n    steps:\n")
        fh.write("      - uses: actions/checkout@v4\n")
        fh.write("      - name: prune local-only tooling\n        run: rm -rf local-scripts .claude\n")
        for i, n in enumerate(names):
            fh.write(f"      - name: mirrored step {i}\n        run: scripts/{n}\n")
        fh.write("      - name: tools\n        run: cd tools/toolcrate && cargo test\n")
        # CLAIM 10'S TWO PAIR SHAPES, both agreeing. `cargo row` is a local row
        # written where the marker is; `cargo fn row` is a row whose argv lives
        # in a shell function the dispatch line names, which is how most of
        # this repo's local half is written.
        fh.write(f"      - name: {FIXTURE_CARGO_STEP}\n        run: {FIXTURE_CARGO_ROW}\n")
        fh.write(f"      - name: {FIXTURE_CARGO_FN_STEP}\n        run: {FIXTURE_CARGO_FN_ROW}\n")
        # A job with no local half, confessing at its own key — claim 9's
        # other branch, exercised by the CLEAN fixture so the passing shape is
        # covered as well as the failing ones.
        fh.write("  # NO LOCAL MIRROR: nothing is archived on one machine\n")
        fh.write("  archive:\n    steps:\n      - uses: actions/checkout@v4\n")
        fh.write("      - name: prune\n        run: rm -rf local-scripts .claude\n")
        fh.write("      - name: archived\n        run: echo hi\n")
        # EVERY hosted-only exemption, DERIVED, for the same reason the
        # local-only ones are written into the local half below: the orphan
        # check requires each MIRROR_EXEMPT path to be named by the side it is
        # exempted into. They ride the confessed job because a hosted-only row
        # is exactly a row with no local half.
        for i, path in enumerate(_exempt_side("hosted")):
            fh.write(f"      - name: hosted only {i}\n        run: {path}\n")
        # EVERY PAIR_EXEMPT pair, DERIVED, one job per cited job name. These
        # jobs check nothing out, so claim 6 passes over them; they are cited
        # by the markers the local half writes below, so claim 9 does too.
        # A hosted-side variable is written as a STEP `env:` block, which is
        # the spelling the real hosted half uses and the one that has to
        # compare equal to a local prefix.
        by_job: dict[str, list[tuple[str, str, dict[str, str]]]] = {}
        for marker, row in _pair_exempt_rows().items():
            job_name, step_name = marker.split(" / ", 1)
            by_job.setdefault(job_name, []).append((step_name, row.hosted_argv, row.hosted_env))
        for job_name, rows in sorted(by_job.items()):
            fh.write(f"  {job_name}:\n    steps:\n")
            for step_name, hosted_argv, hosted_env in rows:
                fh.write(f"      - name: {step_name}\n        run: {hosted_argv}\n")
                if hosted_env:
                    fh.write("        env:\n")
                    for k, v in sorted(hosted_env.items()):
                        fh.write(f"          {k}: {v}\n")
    with open(os.path.join(t, LOCAL_HALF), "w") as fh:
        fh.write("#!/usr/bin/env bash\n")
        fh.write(f"# HOSTED MIRROR: {SITING_JOB} / sited rows\n")
        for want in TIER_BLIND:
            fh.write(f"{want}\n")
        fh.write('if [ "$TIER" = docs ]; then\n  exit 0\nfi\n')
        for i, n in enumerate(names):
            fh.write(f"# HOSTED MIRROR: discipline / mirrored step {i}\nscripts/{n}\n")
        # EVERY local-only exemption, DERIVED. The orphan check requires each
        # MIRROR_EXEMPT path to be named by the side it is exempted into, so a
        # clean fixture that hardcodes one of them reds the whole selftest the
        # next time an entry is added — and reds it as "FAILED on a clean
        # fixture", which points at this builder rather than at the new entry.
        for path in _exempt_side("local"):
            fh.write(f"{path}\n")
        fh.write("cd tools/toolcrate && cargo test\n")
        fh.write(f"# HOSTED MIRROR: discipline / {FIXTURE_CARGO_STEP}\n{FIXTURE_CARGO_ROW}\n")
        fh.write(f"cargo_fn_row() {{\n  {FIXTURE_CARGO_FN_ROW}\n}}\n")
        fh.write(f"# HOSTED MIRROR: discipline / {FIXTURE_CARGO_FN_STEP}\nrun_fixture_row cargo_fn_row\n")
        for marker, row in _pair_exempt_rows().items():
            prefix = "".join(f"{k}={shlex.quote(v)} " for k, v in sorted(row.local_env.items()))
            fh.write(f"# HOSTED MIRROR: {marker}\n{prefix}{row.local_argv}\n")
        # A STEP NAME CAN CARRY A SCRIPT PATH — `compose (demos/render-uv.sh)`
        # — and the fixture plants that name in a workflow, where claims 2 and
        # 3 read it as an invocation. Naming it here too, and creating the
        # file below, is what the real repo does for the real row; the
        # alternative, scrubbing the path out of the fixture's copy of the
        # marker, would leave the table keyed on a pair the fixture does not
        # have.
        for path in _pair_exempt_paths():
            fh.write(f"{path}\n")
    for path in list(MIRROR_EXEMPT) + _pair_exempt_paths():
        os.makedirs(os.path.join(t, os.path.dirname(path)), exist_ok=True)
        open(os.path.join(t, path), "w").close()
    # CLAIM 11, the clean shape: one correctly-restated pin in the local half…
    with open(os.path.join(t, LOCAL_HALF), "a") as fh:
        fh.write(f"{FIXTURE_PIN_LINE}\n")
        # …and the two shapes this claim must NOT read, planted in the CLEAN
        # fixture because that is where an over-eager matcher shows up. An IP
        # address is not three versions overlapping (drop the lookbehind on
        # VERSION_LITERAL_RE and `127.0.0` becomes a literal naming no pin),
        # and a token inside a longer word is not the token (`another` is not
        # `other`, whose pin this line does not carry).
        fh.write("# binds on 127.0.0.1, which is an address and not a version\n")
        fh.write(f"# another restatement, of {FIXTURE_PIN[1]}\n")
    # …and every PIN_FREE literal where its entry says it is, DERIVED for the
    # reason `_exempt_side` is derived: an entry added to that table would
    # otherwise red the CLEAN fixture through its own expiry arm, reporting
    # this builder instead of the new entry.
    for path, lit in sorted(PIN_FREE):
        full = os.path.join(t, path)
        os.makedirs(os.path.dirname(full), exist_ok=True)
        with open(full, "a") as fh:
            fh.write(f"# {lit}\n")
    # THE INDEX IS THE POPULATION, so the fixture needs a real one — the same
    # thing `check-python-lint.py`'s end-to-end plants do, for the same reason.
    # LAST in this builder: a file created after it would not be listed.
    for argv in (["git", "init", "-q"], ["git", "add", "-A"]):
        done = subprocess.run(argv, cwd=t, capture_output=True, text=True, check=False)
        if done.returncode != 0:
            raise SystemExit(f"SELFTEST BROKEN: `{' '.join(argv)}` failed in the fixture "
                             f"({done.stderr.strip()}). Claim 11 reads `git ls-files`, so the "
                             "fixture has to be a git repo; this is a broken harness, not a "
                             "verdict on the tree.")


def _flag_spelling(flag: str, side: str | None = None) -> str:
    """How the fixture writes one allowlisted flag. A value-taking flag gets a
    literal, never an expansion: an expansion is OPAQUE, and a fixture whose
    values all compare equal to everything would pass every value case.

    `side` is for a `both` entry, whose two halves must carry DIFFERENT values
    — that is what such an entry declares, and a fixture that planted one
    value would satisfy the entry's expiry arm instead of the entry."""
    if not SEMANTIC_FLAGS[flag]:
        return flag
    return f"{flag} fixture-value" if side is None else f"{flag} fixture-{side}-value"


def _pair_exempt_paths() -> list[str]:
    """Every `scripts/` or `demos/` path named inside a PAIR_EXEMPT marker."""
    out = set()
    for marker in PAIR_EXEMPT:
        out.update(SCRIPT_RE.findall(marker[0]))
    return sorted(out)


def _env_spelling(name: str, side: str) -> str:
    """The value one exempted variable carries in the fixture.

    A `both` entry has to be planted with the halves DISAGREEING — that is
    what it declares — so the value carries the side that wrote it. The
    literals are never expansions: an expansion is OPAQUE and compares equal
    to anything, so a fixture built out of them would pass every value case.
    """
    return f"fixture-{side}-{name.lower()}"


class _ExemptRow:
    """One pair's fixture row: what each half's argv and environment must be
    for the PAIR_EXEMPT entries on it to be exactly satisfied."""

    def __init__(self, hosted_argv: str, local_argv: str) -> None:
        self.hosted_argv = hosted_argv
        self.local_argv = local_argv
        self.hosted_env: dict[str, str] = {}
        self.local_env: dict[str, str] = {}


def _pair_exempt_rows() -> dict[str, _ExemptRow]:
    """`marker -> _ExemptRow` for the PAIR_EXEMPT pairs, both token classes.

    DERIVED FROM THE TABLE, for the reason `_exempt_side` is: a fixture that
    named an entry would go red against a CLEAN fixture the day that entry
    expired for real, reporting the fixture where the finding is the entry.
    """
    flags: dict[str, tuple[list[str], list[str]]] = {}
    envs: dict[str, list[tuple[str, str]]] = {}
    for (marker, token), (side, _reason) in sorted(PAIR_EXEMPT.items()):
        is_flag = token.startswith("-")
        # `both` MEANS THE TWO HALVES CARRY DIFFERENT VALUES, so it is legal
        # for anything that HAS a value — every variable, and a value-taking
        # flag. It is refused for a boolean flag because there is no such
        # state to declare, and the refusal is written here rather than left
        # implicit: claim 10's own different-values error invites `both`, and
        # an invitation the table cannot honour is the defect this file exists
        # to catch, one class over.
        if side not in ("hosted", "local", "both"):
            raise Bail(f"PAIR_EXEMPT declares side={side!r} for `{token}` on the pair `{marker}`, "
                       "which is not one of hosted, local, both."
                       + teach("`_pair_exempt_rows`"))
        if side == "both" and is_flag and not SEMANTIC_FLAGS.get(token):
            raise Bail(f"PAIR_EXEMPT declares `{token}` `both` for the pair `{marker}`, and it is a "
                       "flag that takes no value. `both` says the halves carry DIFFERENT values; a "
                       "valueless flag is either passed or not, so what this entry describes cannot "
                       "exist. Use hosted or local." + teach("`_pair_exempt_rows`"))
        if " / " not in marker:
            raise Bail(f"PAIR_EXEMPT is keyed on the pair `{marker}`, which is not a "
                       "`<job> / <step name>` citation and so can never name a mirrored pair."
                       + teach("`_pair_exempt_rows`"))
        flags.setdefault(marker, ([], []))
        envs.setdefault(marker, [])
        if is_flag:
            h, ln = flags[marker]
            if side == "both":
                h.append(_flag_spelling(token, "hosted"))
                ln.append(_flag_spelling(token, "local"))
            else:
                (h if side == "hosted" else ln).append(_flag_spelling(token))
        else:
            envs[marker].append((token, side))
    # A DIFFERENT BASE FROM `FIXTURE_CARGO_ROW`, deliberately: the cases below
    # rewrite one row by its exact text, and two rows spelled identically make
    # every one of them edit both — which is how `flag_value_opaque` first
    # "failed" against a pair it had never touched.
    base = "cargo nextest run --archive-file fixture.tar.zst"
    out: dict[str, _ExemptRow] = {}
    for marker, (h, ln) in flags.items():
        row = _ExemptRow(" ".join([base, *h]), " ".join([base, *ln]))
        for name, side in envs[marker]:
            if side in ("hosted", "both"):
                row.hosted_env[name] = _env_spelling(name, "hosted")
            if side in ("local", "both"):
                row.local_env[name] = _env_spelling(name, "local")
        out[marker] = row
    return out


def _exempt_side(side: str) -> list[str]:
    """MIRROR_EXEMPT paths the named half is expected to name.

    A `want` this does not understand is raised rather than skipped: a
    silently-dropped exemption would leave the fixture unclean in a way whose
    error message names the fixture, which is the confusion this exists to
    prevent.
    """
    known = ("local", "hosted")
    out = []
    for path, (want, _reason) in sorted(MIRROR_EXEMPT.items()):
        if want not in known:
            raise Bail(f"{path}: MIRROR_EXEMPT declares want={want!r}, which the selftest's clean "
                       f"fixture does not know how to satisfy — it can name a path into {known}, and "
                       "nowhere else. An exemption the fixture cannot satisfy reds the whole selftest "
                       'as "FAILED on a clean fixture", naming the fixture instead of the entry,'
                       + teach("`_exempt_side`"))
        if want == side:
            out.append(path)
    return out


def _run(root: str, hosted: bool = False) -> tuple[int, str]:
    """One case's subprocess invocation, against the fixture.

    `GITHUB_ACTIONS` IS CLEARED unless a case asks for it. The children are
    being asked what they say about a miniature repo, not whether they are the
    gate of record — and `--root` is refused on the gate of record, so leaving
    an ambient `GITHUB_ACTIONS` in place would make every case below fail with
    that refusal whenever the self-test itself runs on hosted CI. Which is
    where it runs: the `mirror` job invokes `--selftest` before the real pass.
    """
    env = dict(os.environ)
    env.pop("GITHUB_ACTIONS", None)
    if hosted:
        env["GITHUB_ACTIONS"] = "true"
    r = subprocess.run([sys.executable, os.path.abspath(__file__), "--root", root],
                       capture_output=True, text=True, env=env)
    return r.returncode, r.stdout + r.stderr


def _case(want: str, plant) -> None:
    with tempfile.TemporaryDirectory() as t:
        plant_clean(t)
        plant(t)
        rc, out = _run(t)
        if rc == 0:
            raise SystemExit(f"SELFTEST FAILED: passed a planted violation ({plant.__name__})\n{out}")
        if want not in out:
            raise SystemExit(f"SELFTEST FAILED ({plant.__name__}): unexpected message\n{out}")


def _ok_case(plant) -> None:
    """A shape the checker must ACCEPT. An absence detector that reds on a
    correct tree gets routed around, so the shapes this one deliberately lets
    through are pinned as cases too, not left as prose."""
    with tempfile.TemporaryDirectory() as t:
        plant_clean(t)
        plant(t)
        rc, out = _run(t)
        if rc != 0:
            raise SystemExit(f"SELFTEST FAILED: refused a shape it must accept ({plant.__name__})\n{out}")


def _append(path: str, text: str):
    def go(t: str) -> None:
        with open(os.path.join(t, path), "a") as fh:
            fh.write(text)
    return go


def selftest_bail_messages() -> None:
    """EVERY `Bail` TELLS THE READER WHAT TO DO. A refusal that only says *no*
    is what sends the next person to the exemption list instead of to a
    two-minute fix, so each one either names the symbol to extend (`teach`) or
    says plainly that there is nothing to extend (`NO_TEACH`). Checked here
    rather than asserted in the header, because a convention about messages is
    exactly the kind that rots quietly."""
    with open(os.path.abspath(__file__), encoding="utf-8") as fh:
        src = fh.read()
    bad = []
    for i, chunk in enumerate(src.split("raise Bail(")[1:], 1):
        head = chunk[:900]
        stop = head.find("\n\n")
        body = head[:stop] if stop > 0 else head
        if "teach(" not in body and "NO_TEACH" not in body:
            bad.append(f"#{i}: {body.splitlines()[0].strip()[:90]}")
    if bad:
        raise SystemExit("SELFTEST FAILED: these Bail messages tell the reader nothing to do — add a "
                         "`teach(...)` pointer naming the symbol to extend, or `NO_TEACH` if there is "
                         "genuinely no shape to learn:\n  " + "\n  ".join(bad))


def selftest_both_is_reachable() -> None:
    """THE ADVICE HAS TO BE ADVICE THE TABLE CAN TAKE.

    Claim 10's different-values error tells the reader to declare the pair
    with side `both`. Before this was checked, `_pair_exempt_rows` refused
    exactly that entry for a flag — so a reader who followed the advice got a
    green run and a RED SELFTEST, which is a hosted row. That is the
    invited-then-refused defect this file's own header describes, re-created
    one token class over, in the diff that describes fixing it.

    Asserted here rather than through a fixture pair because the table is
    production data: there is no live `both` flag entry to derive one from,
    and inventing one in the table to test the table is a fiction. What the
    fixture DOES cover is the verdict side, and it covers it for both classes
    at once: `_exempt_present` is one function, called by both arms.
    """
    global PAIR_EXEMPT
    saved = PAIR_EXEMPT
    marker = sorted(PAIR_EXEMPT)[0][0]
    valued = sorted(f for f, takes in SEMANTIC_FLAGS.items() if takes)
    boolean = sorted(f for f, takes in SEMANTIC_FLAGS.items() if not takes)
    try:
        for flag in valued[:1]:
            PAIR_EXEMPT = {(marker, flag): ("both", "reachability probe")}
            try:
                row = _pair_exempt_rows()[marker]
            except Bail as exc:
                raise SystemExit(f"SELFTEST FAILED: claim 10 tells the reader to declare a pair with "
                                 f"side `both`, and the table refuses `{flag}` declared that way "
                                 f"({exc}). Advice the table cannot take reds this selftest on the "
                                 "hosted row for anyone who follows it.") from exc
            if row.hosted_argv == row.local_argv:
                raise SystemExit(f"SELFTEST FAILED: a `both` entry for `{flag}` planted the SAME "
                                 "value on both halves of the fixture. `both` declares that the two "
                                 "halves differ, so such a fixture satisfies the entry's expiry arm "
                                 "instead of the entry.")
        for flag in boolean[:1]:
            PAIR_EXEMPT = {(marker, flag): ("both", "reachability probe")}
            try:
                _pair_exempt_rows()
            except Bail:
                continue
            raise SystemExit(f"SELFTEST FAILED: `{flag}` takes no value and was accepted as a `both` "
                             "entry. `both` says the halves carry DIFFERENT values; for a valueless "
                             "flag there is no such state, so the entry describes nothing.")
    finally:
        PAIR_EXEMPT = saved


def selftest() -> None:
    selftest_bail_messages()
    selftest_both_is_reachable()
    with tempfile.TemporaryDirectory() as t:
        plant_clean(t)
        rc, out = _run(t)
        if rc != 0:
            raise SystemExit(f"SELFTEST FAILED: the checker FAILED on a clean fixture\n{out}")
        # `--root` carries the FIXTURE's marker floor, so on the gate of record
        # it is a way to run the real repo against a floor of three. The same
        # clean fixture, the same arguments, one environment variable the
        # runner sets and no repo edit can unset.
        rc, out = _run(t, hosted=True)
        if rc == 0 or "refused on the gate of record" not in out:
            raise SystemExit("SELFTEST FAILED: `--root` was accepted under GITHUB_ACTIONS, where it "
                             f"would lower this gate's own floor\n{out}")

    def hosted_only(t):
        _append(HOSTED_HALF, "      - name: new\n        run: scripts/check-new.sh\n")(t)
        open(os.path.join(t, "scripts/check-new.sh"), "w").close()
    def local_only(t):
        _append(LOCAL_HALF, "scripts/check-local.sh\n")(t)
        open(os.path.join(t, "scripts/check-local.sh"), "w").close()
    def gate_mode_one_side(t): _append(HOSTED_HALF, "      - name: cit\n        run: scripts/gates/probe-suite-census.sh --crates\n")(t)
    def ghost_path(t):
        _append(HOSTED_HALF, "      - name: g\n        run: scripts/gone.sh\n")(t)
        _append(LOCAL_HALF, "scripts/gone.sh\n")(t)
    def orphan(t):             open(os.path.join(t, "scripts/orphan.sh"), "w").close()
    # CLAIM 4'S SECOND ARM. `_declarer` plants a script that satisfies arm one
    # — both halves name it — and implements a `--selftest` mode. Whether that
    # mode has a caller is the only variable across the rows below.
    #
    # THE FILE CLASS IS A PARAMETER, not a constant. The first battery for this
    # arm planted a `.sh` declarer only, and dropping `.py` from the
    # population left the whole battery green while the real defect —
    # `scripts/opt-level-calibrate.py` — walked straight through. A fixture
    # that cannot see the file class of the live defect is not covering it.
    def _declarer(path: str, body: str = 'add_argument("--selftest")\n'):
        def go(t: str) -> None:
            full = os.path.join(t, path)
            os.makedirs(os.path.dirname(full), exist_ok=True)
            with open(full, "w") as fh:
                fh.write(body)
            _append(HOSTED_HALF, f"      - name: d\n        run: {path}\n")(t)
            _append(LOCAL_HALF, f"{path}\n")(t)
        # NAMED, because `_case` reports `plant.__name__` and a battery of
        # closures all called `go` says which claim failed and not which row.
        go.__name__ = f"declarer[{path}]"
        return go
    py_selftest_never_run = _declarer("scripts/declarer.py")
    sh_selftest_never_run = _declarer("scripts/declarer.sh", '[ "${1:-}" = --selftest ] && exit 0\n')
    demos_selftest_never_run = _declarer("demos/declarer.py")
    # A LOCAL CALLER IS NOT A CALLER, and this row is the one the first version
    # of this arm got backwards — it pinned this shape as a case the checker
    # must ACCEPT. Every hosted job deletes local-scripts/ at checkout, so a
    # selftest reachable only from there runs in no CI at all: the arm has to
    # RED here, not pass.
    def selftest_local_only(t):
        py_selftest_never_run(t)
        _append(LOCAL_HALF, "scripts/declarer.py --selftest\n")(t)
    # A CALLER THAT IS A COMMENT is a row someone deleted. `COMMENT_RE` is
    # full-line only and cannot see this inside a `run:` block, so `_shell_text`
    # is what has to.
    def selftest_caller_commented_out(t):
        py_selftest_never_run(t)
        _append(HOSTED_HALF, "      - name: st\n        run: true  # was: scripts/declarer.py --selftest\n")(t)
    # THE MODE NAMED IN ANOTHER SCRIPT'S TEXT. Not a caller either — scripts
    # are not the caller population — which is how the calibrator's selftest
    # came to look covered in three places while running in none.
    def selftest_named_in_a_script(t):
        py_selftest_never_run(t)
        _append("scripts/check-1.sh", "echo see scripts/declarer.py --selftest\n")(t)
    # THE SHAPES IT MUST ACCEPT. A hosted caller on one line; the same caller
    # written as a loop over two scripts in one `run:` block, which is the
    # plausible tidy-up that a line-at-a-time matcher reds with a message that
    # is simply false; and a `scripts/gates/` member, whose uninvoked selftest
    # is `gate-roster.sh`'s finding and must be silent here.
    def selftest_run_hosted(t):
        py_selftest_never_run(t)
        _append(HOSTED_HALF, "      - name: st\n        run: python3 scripts/declarer.py --selftest\n")(t)
    def selftest_run_in_a_loop(t):
        py_selftest_never_run(t)
        sh_selftest_never_run(t)
        _append(HOSTED_HALF, "      - name: st\n        run: |\n"
                             "          for s in scripts/declarer.py scripts/declarer.sh; do\n"
                             '            python3 "$s" --selftest\n'
                             "          done\n")(t)
    def selftest_continued_line(t):
        py_selftest_never_run(t)
        _append(HOSTED_HALF, "      - name: st\n        run: |\n"
                             "          python3 scripts/declarer.py \\\n"
                             "            --selftest\n")(t)
    # A MENTION IS NOT AN IMPLEMENTATION, in the other direction: a script
    # whose only `--selftest` is a full-line comment about ANOTHER script's
    # mode declares nothing. `demos/render-wild.sh` is exactly this on the
    # real tree, and reporting it would be a false red with no fix available.
    def selftest_only_a_comment(t):
        _declarer("scripts/commenter.py", "# see scripts/declarer.py --selftest\n")(t)
    def gates_selftest_uninvoked(t):
        full = os.path.join(t, "scripts/gates/quiet.sh")
        with open(full, "w") as fh:
            fh.write('[ "${1:-}" = --selftest ] && exit 0\n')
    def tools_one_side(t):     os.makedirs(os.path.join(t, "tools/lonely"))
    def unpruned_job(t):       _append(HOSTED_HALF, "  extra:\n    steps:\n      - uses: actions/checkout@v4\n      - run: echo hi\n")(t)
    def uppercase_job(t):      _append(HOSTED_HALF, "  buildXtra:\n    steps:\n      - uses: actions/checkout@v4\n      - run: cat local-scripts/ci-local.sh\n")(t)
    def second_workflow(t):
        with open(os.path.join(t, ".github/workflows/render.yml"), "w") as fh:
            fh.write("jobs:\n  tour:\n    steps:\n      - uses: actions/checkout@v4\n      - run: echo hi\n")
    def prune_after_read(t):   _append(HOSTED_HALF, "  late:\n    steps:\n      - uses: actions/checkout@v4\n      - run: cat local-scripts/ci-local.sh\n      - run: rm -rf local-scripts .claude\n")(t)
    def unparseable(t):        _append(HOSTED_HALF, "  not a job name\n")(t)
    # F1: the flush-style step sequence, ordinary YAML, that the previous
    # reader parsed as a key named `- uses` and dropped — the `buildXtra`
    # defect moved one line down.
    def flush_style_steps(t): _append(HOSTED_HALF, "  sneaky:\n    steps:\n    - uses: actions/checkout@v4\n    - run: cat local-scripts/ci-local.sh\n")(t)
    # F4c: a three-space job body. Valid YAML, and the reader used to see no
    # steps at all in it.
    def three_space_body(t): _append(HOSTED_HALF, "  spaced:\n   steps:\n     - uses: actions/checkout@v4\n     - run: echo hi\n")(t)
    # F2: the siting job waits on a job that skips. Its own `if:` is absent,
    # which is all claim 7 used to read.
    def siting_job_needs(t): _sub(t, HOSTED_HALF, f"  {SITING_JOB}:\n    steps:", f"  {SITING_JOB}:\n    needs: discipline\n    steps:")
    def siting_job_soft(t):  _sub(t, HOSTED_HALF, f"  {SITING_JOB}:\n    steps:", f"  {SITING_JOB}:\n    continue-on-error: true\n    steps:")
    def merge_key(t):        _append(HOSTED_HALF, "  merged:\n    <<: *anchor\n    steps:\n      - run: echo hi\n")(t)
    def unknown_job_key(t):  _append(HOSTED_HALF, "  odd:\n    stepz:\n      - run: echo hi\n")(t)
    def unknown_step_key(t): _append(HOSTED_HALF, "  odd:\n    steps:\n      - runn: echo hi\n")(t)
    def tabbed(t):           _append(HOSTED_HALF, "  tabbed:\n\tsteps:\n")(t)
    def bad_func_spelling(t): _append(LOCAL_HALF, "weird() (\n  echo hi\n)\n")(t)
    # BOTH DERIVED from MIRROR_EXEMPT rather than naming an entry. The pair
    # used to hardcode `demos/render-uv.sh`, and when that entry expired for
    # real (2026-08-22, the hosted population widening to every workflow file)
    # these two cases went red against a clean fixture — reporting the fixture
    # where the finding was the entry. An exemption's own list is the only
    # honest source for "an exemption".
    _one_local = _exempt_side("local")[0]
    _one_hosted = _exempt_side("hosted")[0]
    def exemption_expired(t):
        _append(HOSTED_HALF, f"      - name: x\n        run: {_one_local}\n")(t)
    def exemption_expired_hosted(t):
        _append(LOCAL_HALF, f"{_one_hosted}\n")(t)
    def exemption_orphaned(t):
        _sub(t, LOCAL_HALF, f"{_one_local}\n", "")
        os.remove(os.path.join(t, _one_local))
    def marker_wrong_job(t):   _sub(t, LOCAL_HALF, "# HOSTED MIRROR: discipline / mirrored step 0", "# HOSTED MIRROR: k-lint / mirrored step 0")
    def marker_step_renamed(t): _sub(t, HOSTED_HALF, "- name: mirrored step 0", "- name: mirrored step zero")
    def markers_deleted(t):    _sub(t, LOCAL_HALF, "# HOSTED MIRROR: ", "# was: ")
    def job_unmirrored(t):     _append(HOSTED_HALF, "  lonely:\n    steps:\n      - uses: actions/checkout@v4\n      - name: prune\n        run: rm -rf local-scripts .claude\n      - name: work\n        run: cargo test\n")(t)
    def job_reason_empty(t):   _append(HOSTED_HALF, "  # NO LOCAL MIRROR:\n  lonely:\n    steps:\n      - uses: actions/checkout@v4\n      - name: prune\n        run: rm -rf local-scripts .claude\n      - name: work\n        run: cargo test\n")(t)
    def confession_expired(t): _append(LOCAL_HALF, "# HOSTED MIRROR: archive / archived\n")(t)
    def reason_detached(t):    _sub(t, HOSTED_HALF, "  # NO LOCAL MIRROR: nothing is archived on one machine\n", "  # NO LOCAL MIRROR: detached\n\n  # NO LOCAL MIRROR: nothing is archived on one machine\n")
    # A reason that satisfies "not empty" and says what an empty one says.
    def reason_placeholder(t): _sub(t, HOSTED_HALF, "  # NO LOCAL MIRROR: nothing is archived on one machine\n", "  # NO LOCAL MIRROR: n/a\n")
    # TWO reasons in one block. Only one can be read, so any rule that picks
    # between them makes the verdict depend on writing order: the same two
    # lines reversed would say something different about the same job. Neither
    # order is accepted.
    def reason_twice(t):       _sub(t, HOSTED_HALF, "  # NO LOCAL MIRROR: nothing is archived on one machine\n", "  # NO LOCAL MIRROR:\n  # NO LOCAL MIRROR: nothing is archived on one machine\n")
    # A WHOLE SECOND FILE as the place to put an unmirrored job. This one
    # prunes correctly and parses cleanly, so every other claim passes it;
    # claim 9 reading ci.yml alone was what let it through.
    def second_file_job(t):
        with open(os.path.join(t, ".github/workflows/aside.yml"), "w") as fh:
            fh.write("jobs:\n  aside:\n    steps:\n      - uses: actions/checkout@v4\n"
                     "      - name: prune\n        run: rm -rf local-scripts .claude\n"
                     "      - name: work\n        run: cargo test\n")
    # One job name in two files. A `<job> / <step>` citation has no file half.
    def duplicate_job_name(t):
        with open(os.path.join(t, ".github/workflows/aside.yml"), "w") as fh:
            fh.write("jobs:\n  # NO LOCAL MIRROR: a second definition under one name, for this case\n"
                     "  archive:\n    steps:\n      - uses: actions/checkout@v4\n"
                     "      - name: prune\n        run: rm -rf local-scripts .claude\n"
                     "      - name: archived\n        run: echo hi\n")

    def exemption_inverted(t):
        _sub(t, LOCAL_HALF, f"{_one_local}\n", "")
        _append(HOSTED_HALF, f"      - name: x\n        run: {_one_local}\n")(t)

    # CLAIM 10. One case per allowlisted flag, DERIVED from `SEMANTIC_FLAGS`
    # rather than listed again: a hand-written list of cases beside a
    # hand-written list of flags is two rosters, and a flag added to one and
    # not the other is exactly the silence this whole file is about.
    def _flag_on_hosted_only(flag: str):
        def go(t: str) -> None:
            _sub(t, HOSTED_HALF, f"run: {FIXTURE_CARGO_ROW}\n",
                 f"run: {FIXTURE_CARGO_ROW} {_flag_spelling(flag)}\n")
        go.__name__ = f"flag_hosted_only_{flag.strip('-')}"
        return go

    def flag_local_only(t):
        _sub(t, LOCAL_HALF, f"{FIXTURE_CARGO_ROW}\n", f"{FIXTURE_CARGO_ROW} --no-fail-fast\n")

    # The row whose argv is in a function the dispatch line names. Without the
    # closure in `marker_row` this pair reads as "no cargo command locally",
    # which is a pass.
    def flag_through_function(t):
        _sub(t, LOCAL_HALF, f"  {FIXTURE_CARGO_FN_ROW}\n", "  cargo clippy --all-targets -- -D warnings\n")

    # Both halves pass the flag; the VALUES differ. Same row name, fewer tests.
    def flag_value_diverges(t):
        _sub(t, HOSTED_HALF, f"run: {FIXTURE_CARGO_ROW}\n", f"run: {FIXTURE_CARGO_ROW} --features one\n")
        _sub(t, LOCAL_HALF, f"{FIXTURE_CARGO_ROW}\n", f"{FIXTURE_CARGO_ROW} --features two\n")

    # An opaque value on either side is not a divergence: only a runner knows
    # what it expands to, and refusing there would red a correct tree.
    def flag_value_opaque(t):
        _sub(t, HOSTED_HALF, f"run: {FIXTURE_CARGO_ROW}\n",
             f"run: {FIXTURE_CARGO_ROW} --features ${{{{ matrix.feats }}}}\n")
        _sub(t, LOCAL_HALF, f"{FIXTURE_CARGO_ROW}\n", f"{FIXTURE_CARGO_ROW} --features two\n")

    # CLAIM 10'S ENV ARM. Every case below is written from the failure it has
    # to catch, and the two spellings are the subject of most of them: a
    # reader that saw only `env:` blocks, or only inline prefixes, would pass
    # exactly the divergence this arm was built for.
    _FX_RUSTFLAGS = "--cfg fixture_env"

    # The variable declared on hosted's STEP and nowhere locally. Catches the
    # arm not reading step blocks at all.
    def env_hosted_only(t):
        _sub(t, HOSTED_HALF, f"        run: {FIXTURE_CARGO_ROW}\n",
             f"        run: {FIXTURE_CARGO_ROW}\n        env:\n          RUSTFLAGS: {_FX_RUSTFLAGS}\n")

    # THE HOSTED HALF'S OTHER SPELLING: an inline prefix in a `run:` block,
    # which is how the wasm pair carries its backend cfg. A reader that took
    # the hosted environment from `env:` blocks alone would see nothing here.
    def env_hosted_prefix_only(t):
        _sub(t, HOSTED_HALF, f"        run: {FIXTURE_CARGO_ROW}\n",
             f"        run: RUSTFLAGS='{_FX_RUSTFLAGS}' {FIXTURE_CARGO_ROW}\n")

    # …and the mirror image: a prefix the hosted half does not carry.
    def env_local_only(t):
        _sub(t, LOCAL_HALF, f"{FIXTURE_CARGO_ROW}\n",
             f"RUSTFLAGS='{_FX_RUSTFLAGS}' {FIXTURE_CARGO_ROW}\n")

    # Both halves set it, to different cfgs. One row name, two builds.
    def env_value_diverges(t):
        _sub(t, HOSTED_HALF, f"        run: {FIXTURE_CARGO_ROW}\n",
             f"        run: {FIXTURE_CARGO_ROW}\n        env:\n          RUSTFLAGS: --cfg one\n")
        _sub(t, LOCAL_HALF, f"{FIXTURE_CARGO_ROW}\n", f"RUSTFLAGS='--cfg two' {FIXTURE_CARGO_ROW}\n")

    # THE CASE THE WHOLE ARM TURNS ON: a step `env:` block and an inline
    # prefix are the SAME FACT, and this must pass. A reader that compared one
    # spelling only would call this pair one-sided; one that compared the
    # values as written would trip over YAML's optional quotes.
    def env_block_equals_prefix(t):
        # QUOTED ON THE HOSTED SIDE, on purpose: YAML's quotes are optional
        # and the shell's are consumed by the shell, so a reader that compared
        # the two values as written would call these different.
        _sub(t, HOSTED_HALF, f"        run: {FIXTURE_CARGO_ROW}\n",
             f"        run: {FIXTURE_CARGO_ROW}\n        env:\n"
             f'          RUSTFLAGS: "{_FX_RUSTFLAGS}"\n')
        _sub(t, LOCAL_HALF, f"{FIXTURE_CARGO_ROW}\n",
             f'RUSTFLAGS="{_FX_RUSTFLAGS}" {FIXTURE_CARGO_ROW}\n')

    # A pair whose hosted half declares the variable at the JOB, forty lines
    # above the step the marker cites — and the two layers below it, so that
    # PRECEDENCE is a case rather than a sentence. Planted as its own job so
    # each case says one thing.
    def _env_job(name: str, local_prefix: str, job_value: str = _FX_RUSTFLAGS,
                 step_value: str | None = None, hosted_prefix: str = ""):
        def go(t: str) -> None:
            step = f"      - name: env row\n        run: {hosted_prefix}cargo check --workspace\n"
            if step_value is not None:
                step += f"        env:\n          RUSTFLAGS: {step_value}\n"
            _append(HOSTED_HALF,
                    f"  envjob:\n    env:\n      RUSTFLAGS: {job_value}\n    steps:\n" + step)(t)
            _append(LOCAL_HALF,
                    f"# HOSTED MIRROR: envjob / env row\n{local_prefix}cargo check --workspace\n")(t)
        go.__name__ = name
        return go

    _FX_PREFIX = f"RUSTFLAGS='{_FX_RUSTFLAGS}' "
    env_job_block_equals_prefix = _env_job("env_job_block_equals_prefix", _FX_PREFIX)
    env_job_block_hosted_only = _env_job("env_job_block_hosted_only", "")
    # THE LADDER, ONE RUNG AT A TIME. Each of these plants the RIGHT value on
    # the layer that must win and a wrong one on the layer below it, so the
    # case passes only if precedence runs workflow < job < step < prefix. A
    # reader that merged them in any other order reds on a correct pair.
    precedence_step_over_job = _env_job("precedence_step_over_job", _FX_PREFIX,
                                        job_value="--cfg outranked", step_value=_FX_RUSTFLAGS)
    precedence_prefix_over_block = _env_job("precedence_prefix_over_block", _FX_PREFIX,
                                            job_value="--cfg outranked",
                                            step_value="--cfg also-outranked",
                                            hosted_prefix=_FX_PREFIX)

    # THE BOTTOM RUNG, in a workflow file of its own so the case touches one
    # pair rather than every pair in the fixture. Without this layer, one line
    # added to ci.yml's top-level block changes every hosted row's environment
    # and nothing says a word.
    def _env_workflow(name: str, local_prefix: str):
        def go(t: str) -> None:
            with open(os.path.join(t, ".github/workflows/envfile.yml"), "w") as fh:
                fh.write(f"env:\n  RUSTFLAGS: {_FX_RUSTFLAGS}\njobs:\n  envfilejob:\n    steps:\n"
                         "      - name: env row\n        run: cargo check --workspace\n")
            _append(LOCAL_HALF, "# HOSTED MIRROR: envfilejob / env row\n"
                                f"{local_prefix}cargo check --workspace\n")(t)
        go.__name__ = name
        return go

    env_workflow_block_equals_prefix = _env_workflow("env_workflow_block_equals_prefix", _FX_PREFIX)
    env_workflow_block_hosted_only = _env_workflow("env_workflow_block_hosted_only", "")

    # THE LOCAL HALF'S OTHER SHAPES, each of which read as SETTING NOTHING
    # before the walk in `env_prefixes` learned about them — and reading as
    # nothing is indistinguishable from a half that never set the variable,
    # which is the silent pass this whole arm exists to close.
    def _env_local_shape(name: str, local_row: str, hosted_value: str = _FX_RUSTFLAGS):
        def go(t: str) -> None:
            _sub(t, HOSTED_HALF, f"        run: {FIXTURE_CARGO_ROW}\n",
                 f"        run: {FIXTURE_CARGO_ROW}\n        env:\n"
                 f"          RUSTFLAGS: '{hosted_value}'\n")
            _sub(t, LOCAL_HALF, f"{FIXTURE_CARGO_ROW}\n", local_row)
        go.__name__ = name
        return go

    # A ONE-LINE shell function: `f() { NAME=v cargo …; }`. Three live pairs
    # are written this way, one of them carrying three PAIR_EXEMPT entries.
    env_one_line_function = _env_local_shape(
        "env_one_line_function", f"fixture_one_liner() {{ {_FX_PREFIX}{FIXTURE_CARGO_ROW}; }}\n"
                                 "fixture_one_liner\n")
    # `env NAME=v cmd`, which the item's own class list names.
    env_wrapper_command = _env_local_shape(
        "env_wrapper_command", f"env {_FX_PREFIX}{FIXTURE_CARGO_ROW}\n")
    # A loop body, and a brace group — the same positional blindness.
    env_loop_body = _env_local_shape(
        "env_loop_body", f"for i in 1; do {_FX_PREFIX}{FIXTURE_CARGO_ROW}; done\n")
    # An opaque value on the LOCAL side. `env_value_opaque` plants on the
    # hosted side only, so a normaliser that read `$VAR` as text locally kept
    # passing every case in this file.
    env_value_opaque_local = _env_local_shape(
        "env_value_opaque_local", f'RUSTFLAGS="$SOME_LOCAL_VAR" {FIXTURE_CARGO_ROW}\n')
    # A value carrying INNER quotes — the real wasm cfg's shape. A normaliser
    # that stripped more than YAML's outer pair would break exactly here.
    env_inner_quotes = _env_local_shape(
        "env_inner_quotes", "RUSTFLAGS='--cfg backend=\"wasm_js\"' " + f"{FIXTURE_CARGO_ROW}\n",
        hosted_value='--cfg backend="wasm_js"')

    # THE ROW WHOSE ARGV LIVES IN A FUNCTION the dispatch line only names.
    # The flag arm has this case; without it here, a reader that took the
    # local environment from the marker's first code line passed every env
    # case in this file.
    def env_through_function(t):
        _sub(t, HOSTED_HALF, f"        run: {FIXTURE_CARGO_FN_ROW}\n",
             f"        run: {FIXTURE_CARGO_FN_ROW}\n        env:\n"
             f"          RUSTFLAGS: '{_FX_RUSTFLAGS}'\n")
        _sub(t, LOCAL_HALF, f"  {FIXTURE_CARGO_FN_ROW}\n", f"  {_FX_PREFIX}{FIXTURE_CARGO_FN_ROW}\n")

    # A THROUGHPUT KNOB ON THE LOCAL SIDE. The hosted-only case cannot see a
    # filter applied to one half and not the other.
    def env_throughput_knob_local(t):
        _sub(t, LOCAL_HALF, f"{FIXTURE_CARGO_ROW}\n",
             f"CARGO_PROFILE_TEST_DEBUG=line-tables-only {FIXTURE_CARGO_ROW}\n")

    # AN EXPRESSION WHERE A PREFIX WOULD BE. Only a runner knows whether it
    # sets this name, so the half is INCOMPLETE and a one-sided verdict
    # against it is withheld — the live shape at ci.yml's archived-test rows.
    def env_expression_prefix(t):
        _sub(t, HOSTED_HALF, f"        run: {FIXTURE_CARGO_ROW}\n",
             "        run: ${{ matrix.envprefix }} " + f"{FIXTURE_CARGO_ROW}\n")
        _sub(t, LOCAL_HALF, f"{FIXTURE_CARGO_ROW}\n", f"{_FX_PREFIX}{FIXTURE_CARGO_ROW}\n")

    # A MIRRORED PAIR WHOSE HOSTED HALF IS AN ACTION. There is no argv and no
    # environment to compare — what such a step does is its `with:` inputs —
    # so a pair cited at one is a pair checked by nothing.
    def env_uses_only_pair(t):
        _append(HOSTED_HALF, "  actionjob:\n    steps:\n      - name: action row\n"
                             "        uses: ./.github/actions/fixture\n        with:\n"
                             '          version: "1.2.3"\n')(t)
        _append(LOCAL_HALF, "# HOSTED MIRROR: actionjob / action row\n"
                            f"{FIXTURE_CARGO_ROW}\n")(t)

    # AN ASSIGNMENT THE WALK CANNOT ATTRIBUTE: after the command word, where
    # it is an argument and not a prefix — or in a construct not on the list.
    def env_unattributed(t):
        _sub(t, LOCAL_HALF, f"{FIXTURE_CARGO_ROW}\n",
             f"{FIXTURE_CARGO_ROW} && somecmd RUSTFLAGS=x\n")

    # A value only a runner can expand is not a divergence, on the flag arm's
    # own rule: refusing here reds a correct tree.
    def env_value_opaque(t):
        _sub(t, HOSTED_HALF, f"        run: {FIXTURE_CARGO_ROW}\n",
             f"        run: {FIXTURE_CARGO_ROW}\n        env:\n"
             "          RUSTFLAGS: ${{ matrix.flags }}\n")
        _sub(t, LOCAL_HALF, f"{FIXTURE_CARGO_ROW}\n", f"RUSTFLAGS='--cfg two' {FIXTURE_CARGO_ROW}\n")

    # A THROUGHPUT KNOB, HOSTED-ONLY, WHICH MUST NOT RED. The local half's
    # refusal to mirror `CARGO_PROFILE_*` is ratified and measured; an arm
    # that fired here would be demanding a change the repo decided against,
    # and the exemption table would become the place to put that decision.
    def env_throughput_knob(t):
        _sub(t, HOSTED_HALF, f"        run: {FIXTURE_CARGO_ROW}\n",
             f"        run: {FIXTURE_CARGO_ROW}\n        env:\n"
             "          CARGO_PROFILE_TEST_DEBUG: line-tables-only\n")

    # A pair with no cargo command on either half. The flag arm skips it by
    # construction; the env arm must not, because the pair this whole arm was
    # filed over is a render row.
    def env_on_a_row_without_cargo(t):
        _sub(t, HOSTED_HALF, "      - name: sited rows\n        run: echo sited\n",
             "      - name: sited rows\n        run: echo sited\n        env:\n"
             "          CAD_RENDER_ACCEPTOR: fixture\n")

    # THE TWO REFUSALS. A variable set for the rest of the shell is attributed
    # to no command, and would compare equal to a half that never set it.
    def env_standing_assignment(t):
        _sub(t, LOCAL_HALF, f"{FIXTURE_CARGO_ROW}\n",
             f"RUSTFLAGS='{_FX_RUSTFLAGS}'\n{FIXTURE_CARGO_ROW}\n")

    def env_exported(t):
        _sub(t, LOCAL_HALF, f"{FIXTURE_CARGO_ROW}\n",
             f"export RUSTFLAGS='{_FX_RUSTFLAGS}'\n{FIXTURE_CARGO_ROW}\n")

    # …and the hosted way to set a variable this reader cannot see: a step
    # that appends to $GITHUB_ENV sets it for every later step of the job.
    def env_via_github_env(t):
        _sub(t, HOSTED_HALF, f"        run: {FIXTURE_CARGO_ROW}\n",
             '        run: echo "RUSTFLAGS=x" >> $GITHUB_ENV\n')

    # The union's blind spot: the row's own command loses the flag and a
    # SECOND invocation beside it still carries one.
    def flag_on_only_some(t):
        _sub(t, HOSTED_HALF, f"run: {FIXTURE_CARGO_ROW}\n", f"run: {FIXTURE_CARGO_ROW} --no-fail-fast\n")
        _sub(t, LOCAL_HALF, f"{FIXTURE_CARGO_ROW}\n",
             f"{FIXTURE_CARGO_ROW} --no-fail-fast && {FIXTURE_CARGO_ROW}\n")

    # THE TOKENIZER, in the direction that fails silently: a redirection or a
    # substitution BEFORE the flags must not saw the argv in half. Both halves
    # carry the flag, so the only way these red is if one side lost it.
    def redirection_before_flags(t):
        _sub(t, HOSTED_HALF, f"run: {FIXTURE_CARGO_ROW}\n",
             f"run: {FIXTURE_CARGO_ROW} 2>&1 --no-fail-fast\n")
        _sub(t, LOCAL_HALF, f"{FIXTURE_CARGO_ROW}\n", f"{FIXTURE_CARGO_ROW} --no-fail-fast\n")

    def substitution_before_flags(t):
        _sub(t, HOSTED_HALF, f"run: {FIXTURE_CARGO_ROW}\n",
             f"run: {FIXTURE_CARGO_ROW} $(echo x) --no-fail-fast\n")
        _sub(t, LOCAL_HALF, f"{FIXTURE_CARGO_ROW}\n", f"{FIXTURE_CARGO_ROW} --no-fail-fast\n")

    # …and its other half: a masked substitution's CONTENTS are still read.
    # Dropping them instead is what made a correct pair look like two halves
    # running a different number of commands.
    def substitution_contents_read(t):
        _sub(t, HOSTED_HALF, f"run: {FIXTURE_CARGO_ROW}\n",
             f"run: echo $({FIXTURE_CARGO_ROW} --features one)\n")
        _sub(t, LOCAL_HALF, f"{FIXTURE_CARGO_ROW}\n", f"{FIXTURE_CARGO_ROW} --features two\n")

    # THE TWO REFUSALS. An argv this cannot read is never a pass.
    def flag_missing_value(t):
        _sub(t, HOSTED_HALF, f"run: {FIXTURE_CARGO_ROW}\n", f"run: {FIXTURE_CARGO_ROW} --features\n")

    def unclosed_quote(t):
        _sub(t, LOCAL_HALF, f"{FIXTURE_CARGO_ROW}\n", f"{FIXTURE_CARGO_ROW} -E 'test(/^a/)\n")

    # CLAIM 11. THE BUMP IS THE CASE THAT MATTERS: ci.yml's pin moves and the
    # local half's copy does not, which is the whole failure this claim exists
    # to catch and the one that used to happen in silence.
    def pin_bumped(t):
        _sub(t, HOSTED_HALF, f'{FIXTURE_PIN[0]}: "{FIXTURE_PIN[1]}"', f'{FIXTURE_PIN[0]}: "9.9.9"')

    # ARM A'S HOLE, WHICH IS ARM B'S SUBJECT: the copy drifts onto a value
    # ci.yml really does pin — for a different tool. Every literal in the tree
    # is then a pinned value and arm A has nothing to say.
    def pin_on_wrong_tool(t):
        _append(LOCAL_HALF, f"# fixture {FIXTURE_PIN_OTHER[1]}\n")(t)

    # A confession that has outlived its literal. Derived from the table for
    # the same reason the clean plant is.
    _pin_free_first = sorted(PIN_FREE)[0]

    def pin_free_expired(t):
        _sub(t, _pin_free_first[0], f"# {_pin_free_first[1]}\n", "#\n")

    # THE MOST LITERAL RESTATEMENT THERE IS: the pin's own KEY, set to another
    # pin's value. Arm A sees a value ci.yml really does pin; arm B saw nothing
    # at all until the key itself joined the spellings it looks for.
    def pin_key_restated(t):
        _append(LOCAL_HALF, f"{FIXTURE_PIN[0]}={FIXTURE_PIN_OTHER[1]}\n")(t)

    # …and the same escape through capitalisation alone.
    def pin_capitalised_tool(t):
        _append(LOCAL_HALF, f"# Fixture {FIXTURE_PIN_OTHER[1]}\n")(t)

    # A DECLARED NON-PIN ON A LINE THAT NAMES A PINNED TOOL. Arm A excuses it;
    # arm B must not then red it, because there would be no declaration path
    # left anywhere for a line the author has already explained.
    _pin_free_local = sorted(lit for (path, lit) in PIN_FREE if path == LOCAL_HALF)

    def pin_free_beside_a_tool(t):
        _append(LOCAL_HALF, f"# fixture, near {_pin_free_local[0]} which is declared\n")(t)

    # A CONFESSION THAT BECAME A PIN. The declared non-pin and a live pinned
    # value are the same string, so arm A excuses the very copy it exists to
    # find. Derived from the table, like every other case here.
    _pin_free_first_lit = sorted(PIN_FREE)[0][1]

    def pin_free_inverted(t):
        _sub(t, HOSTED_HALF, f'"{FIXTURE_PIN[1]}"', f'"{_pin_free_first_lit}"')

    # A MULTIWORD PIN NAMED BY ONE OF ITS PARTS. Derive the token by stripping
    # `_VERSION` alone and this line names nothing this arm looks for.
    def pin_multiword_part(t):
        _append(LOCAL_HALF, f"# tool {FIXTURE_PIN[1]}\n")(t)

    # THE POPULATION IS GIT'S, AND ITS ABSENCE IS A REFUSAL. Silently reading
    # an empty listing is a claim 11 that checks nothing and says OK.
    def pin_population_unlistable(t):
        shutil.rmtree(os.path.join(t, ".git"))

    _case("pins no such version", pin_bumped)
    _case("as a non-pin", pin_free_inverted)
    _case(f"none of them is {FIXTURE_PIN_OTHER[1]}", pin_multiword_part)
    _case("git ls-files", pin_population_unlistable)
    _case(f"none of them is {FIXTURE_PIN[1]}", pin_on_wrong_tool)
    _case(f"none of them is {FIXTURE_PIN[1]}", pin_key_restated)
    _case(f"none of them is {FIXTURE_PIN[1]}", pin_capitalised_tool)
    _case("no longer names it", pin_free_expired)
    if _pin_free_local:
        _ok_case(pin_free_beside_a_tool)

    _flag_exempt = sorted(k for k in PAIR_EXEMPT if k[1].startswith("-"))
    if _flag_exempt:
        _fx_marker, _fx_flag = _flag_exempt[0]
        _fx_side = PAIR_EXEMPT[(_fx_marker, _fx_flag)][0]
        _fx_row = _pair_exempt_rows()[_fx_marker]
        _fx_hosted, _fx_local = _fx_row.hosted_argv, _fx_row.local_argv

        def flag_exemption_expired(t):
            """The declared one-sided flag appears on the OTHER half too."""
            path, argv = ((LOCAL_HALF, _fx_local) if _fx_side == "hosted"
                          else (HOSTED_HALF, _fx_hosted))
            _sub(t, path, f"{argv}\n", f"{argv} {_flag_spelling(_fx_flag)}\n")

        def flag_exemption_inverted(t):
            """The flag moved to the half the entry says does not carry it."""
            keep, gain = ((_fx_hosted, _fx_local) if _fx_side == "hosted"
                          else (_fx_local, _fx_hosted))
            keep_path = HOSTED_HALF if _fx_side == "hosted" else LOCAL_HALF
            gain_path = LOCAL_HALF if _fx_side == "hosted" else HOSTED_HALF
            _sub(t, keep_path, f"{keep}\n", f"{FIXTURE_CARGO_ROW}\n")
            _sub(t, gain_path, f"{gain}\n", f"{gain} {_flag_spelling(_fx_flag)}\n")

        def flag_exemption_orphaned(t):
            _sub(t, LOCAL_HALF, f"# HOSTED MIRROR: {_fx_marker}\n", "# was a pair: \n")

        def flag_exemption_unwatched(t):
            """The declared flag DROPPED from the half that carried it. The
            entry then excuses nothing and hides the absence behind itself —
            MIRROR_EXEMPT's "NEITHER half names it" branch, which this table
            went without until a review measured what that cost."""
            path, argv = ((HOSTED_HALF, _fx_hosted) if _fx_side == "hosted"
                          else (LOCAL_HALF, _fx_local))
            _sub(t, path, f"{argv}\n", f"{argv.replace(' ' + _flag_spelling(_fx_flag), '')}\n")

    # THE ENV ARM'S EXEMPTION DIRECTIONS, derived from the table the same way
    # the flag ones are. Two subjects: a one-sided entry and a `both` entry,
    # because their expiry arms are different sentences.
    _env_exempt = sorted(k for k in PAIR_EXEMPT
                         if not k[1].startswith("-") and PAIR_EXEMPT[k][0] != "both")
    _env_both = sorted(k for k in PAIR_EXEMPT
                       if not k[1].startswith("-") and PAIR_EXEMPT[k][0] == "both")
    if _env_exempt:
        _ex_marker, _ex_name = _env_exempt[0]
        _ex_side = PAIR_EXEMPT[(_ex_marker, _ex_name)][0]
        _ex_row = _pair_exempt_rows()[_ex_marker]
        _ex_value = _env_spelling(_ex_name, _ex_side)

        def env_exemption_expired(t):
            """The declared one-sided variable appears on the OTHER half too."""
            if _ex_side == "hosted":
                _sub(t, LOCAL_HALF, f"{_ex_row.local_argv}\n",
                     f"{_ex_name}={_ex_value} {_ex_row.local_argv}\n")
            else:
                _sub(t, HOSTED_HALF, f"        run: {_ex_row.hosted_argv}\n",
                     f"        run: {_ex_row.hosted_argv}\n        env:\n"
                     f"          {_ex_name}: {_ex_value}\n")

        def env_exemption_inverted(t):
            """The variable moved to the half the entry says does not set it."""
            env_exemption_expired(t)
            env_exemption_unwatched(t)

        def env_exemption_unwatched(t):
            """The declared variable DROPPED from the half that set it. The
            entry then excuses nothing and hides the absence behind itself."""
            if _ex_side == "hosted":
                _sub(t, HOSTED_HALF, f"          {_ex_name}: {_ex_value}\n", "")
            else:
                _sub(t, LOCAL_HALF, f"{_ex_name}={_ex_value} ", "")

    if _env_both:
        _bo_marker, _bo_name = _env_both[0]
        _bo_row = _pair_exempt_rows()[_bo_marker]

        def env_exemption_both_one_sided(t):
            """A declared divergence where one half simply STOPPED setting the
            variable. The entry still matches — one side carries it — so an
            arm that only asked "is it declared?" would pass a real drop."""
            _sub(t, LOCAL_HALF, f"{_bo_name}={_env_spelling(_bo_name, 'local')} ", "")

        def env_exemption_agreed(t):
            """A declared divergence that CLOSED. The two halves now say the
            same thing and the entry is a fossil — the direction an exemption
            table rots in when nothing reads it back."""
            _sub(t, LOCAL_HALF, f"{_bo_name}={_env_spelling(_bo_name, 'local')} ",
                 f"{_bo_name}={_env_spelling(_bo_name, 'hosted')} ")

    _case("and local-scripts/ci-local.sh does not", hosted_only)
    _case("and no workflow in .github/workflows/ does", local_only)
    _case("is invoked by the hosted half only", gate_mode_one_side)
    _case("and no such file exists", ghost_path)
    _case("NEITHER half names", orphan)
    _case("scripts/declarer.py implements a `--selftest` mode", py_selftest_never_run)
    _case("scripts/declarer.sh implements a `--selftest` mode", sh_selftest_never_run)
    _case("demos/declarer.py implements a `--selftest` mode", demos_selftest_never_run)
    _case("is not a substitute and is not read here", selftest_local_only)
    _case("scripts/declarer.py implements a `--selftest` mode", selftest_caller_commented_out)
    _case("scripts/declarer.py implements a `--selftest` mode", selftest_named_in_a_script)
    _ok_case(selftest_run_hosted)
    _ok_case(selftest_run_in_a_loop)
    _ok_case(selftest_continued_line)
    _ok_case(gates_selftest_uninvoked)
    _ok_case(selftest_only_a_comment)
    _case("named by neither half", tools_one_side)
    _case("checks the repo out and does not delete", unpruned_job)
    _case("job `buildXtra` checks the repo out", uppercase_job)
    _case("render.yml job `tour` checks the repo out", second_workflow)
    _case("prunes it at step", prune_after_read)
    _case("but it is the one job", _resite_prune)
    _case("not a job name at job indent", unparseable)
    _case("job `sneaky` checks the repo out", flush_style_steps)
    _case("job `spaced` checks the repo out", three_space_body)
    _case("waits on `discipline` carries an `if:`", siting_job_needs)
    _case("cannot redden the PR", siting_job_soft)
    _case("not a `key:` line in job `merged`", merge_key)
    _case("carries the key `stepz`", unknown_job_key)
    _case("carries the key `runn`", unknown_step_key)
    _case("a TAB character", tabbed)
    _case("looks like a shell function definition", bad_func_spelling)
    _case("BOTH halves now name it", exemption_expired)
    # The same expiry from the other side: a hosted-only exemption whose row
    # came back locally. Symmetric because the widening above made
    # `want=hosted` a live value rather than a hypothetical one.
    _case("BOTH halves now name it", exemption_expired_hosted)
    _case("NEITHER half names it", exemption_orphaned)
    _case("has a step named", marker_wrong_job)
    _case("has a step named", marker_step_renamed)
    _case("below the", markers_deleted)
    _case("declared local-only in MIRROR_EXEMPT", exemption_inverted)
    # THE SITING RULE, both halves. These are the cases the reviewer's
    # experiment plants: hollow `mirror`, move the steps back into a job that
    # skips on docs tier; and, locally, move the row below the docs exit.
    _case("has no local mirror", job_unmirrored)
    _case("with no reason after it", job_reason_empty)
    _case("The confession has expired", confession_expired)
    _case("not against a job key", reason_detached)
    _case("which is under", reason_placeholder)
    _case("carries two `NO LOCAL MIRROR` lines", reason_twice)
    _case("aside.yml job `aside` has no local mirror", second_file_job)
    _case("is defined in both", duplicate_job_name)
    _case("carries an `if:`", _hollow_siting_job)
    _case("a definition above the exit is not a run", _local_row_below_exit)
    _case("never runs", _local_row_deleted)
    # CLAIM 10, every allowlisted flag one at a time.
    for _flag in sorted(SEMANTIC_FLAGS):
        _case(f"passes `{_flag}` on `cargo nextest run` in the hosted half only",
              _flag_on_hosted_only(_flag))
    _case("passes `--no-fail-fast` on `cargo nextest run` in the local half only", flag_local_only)
    _case("passes `--all-targets` on `cargo clippy` in the local half only", flag_through_function)
    _case("with different values", flag_value_diverges)
    _case("on only some of them in the other", flag_on_only_some)
    _case("with different values", substitution_contents_read)
    _case("takes a value", flag_missing_value)
    _case("never closes", unclosed_quote)
    _case("sets `RUSTFLAGS` in the hosted half only", env_hosted_only)
    _case("sets `RUSTFLAGS` in the hosted half only", env_hosted_prefix_only)
    _case("sets `RUSTFLAGS` in the local half only", env_local_only)
    _case("hosted and '--cfg two' locally", env_value_diverges)
    _case("sets `RUSTFLAGS` in the hosted half only", env_job_block_hosted_only)
    _case("sets `RUSTFLAGS` in the hosted half only", env_workflow_block_hosted_only)
    _case("cannot attribute it to a command", env_unattributed)
    _case("is a `uses:` step", env_uses_only_pair)
    _case("sets `CAD_RENDER_ACCEPTOR` in the hosted half only", env_on_a_row_without_cargo)
    _case("set for the rest of the shell here", env_standing_assignment)
    _case("set for the rest of the shell here", env_exported)
    _case("writes to $GITHUB_ENV", env_via_github_env)
    _ok_case(env_block_equals_prefix)
    _ok_case(env_job_block_equals_prefix)
    _ok_case(env_workflow_block_equals_prefix)
    _ok_case(precedence_step_over_job)
    _ok_case(precedence_prefix_over_block)
    _ok_case(env_one_line_function)
    _ok_case(env_wrapper_command)
    _ok_case(env_loop_body)
    _ok_case(env_value_opaque_local)
    _ok_case(env_inner_quotes)
    _ok_case(env_through_function)
    _ok_case(env_throughput_knob_local)
    _ok_case(env_expression_prefix)
    _ok_case(env_value_opaque)
    _ok_case(env_throughput_knob)
    if _env_exempt:
        _case("BOTH halves now set it", env_exemption_expired)
        _case("PAIR_EXEMPT and is set by the", env_exemption_inverted)
        _case("NEITHER half sets it", env_exemption_unwatched)
    if _env_both:
        _case("the two halves now agree", env_exemption_agreed)
        _case("only the hosted half sets it now", env_exemption_both_one_sided)
    _ok_case(flag_value_opaque)
    _ok_case(redirection_before_flags)
    _ok_case(substitution_before_flags)
    if _flag_exempt:
        _case("BOTH halves now pass it", flag_exemption_expired)
        _case("PAIR_EXEMPT and is passed by the", flag_exemption_inverted)
        _case("carries no such HOSTED MIRROR marker", flag_exemption_orphaned)
        _case("NEITHER half passes it", flag_exemption_unwatched)
    print("check-ci-mirror-parity selftest OK: every Bail names the symbol to extend or says there is "
          "none; passes a clean fixture, and refuses the fixture's own `--root` on the gate of "
          "record; fires on a one-sided row, a "
          "one-sided gate MODE, a path both halves name that does not exist, an orphan script, a "
          "`--selftest` mode no workflow invokes — as a .py, as a .sh, under demos/, called only "
          "from the local half, called on a line that is a COMMENT, or merely named in another "
          "script — a "
          "one-sided tools/ crate, a checked-out job that keeps either tree, an UPPERCASE job name doing "
          "the same, a second workflow file growing one, a prune that comes after the read, the siting job "
          "pruning, an unparseable workflow, a flush-style or three-space step block hiding a checked-out job, the siting job given a `needs:` onto a skipping job or `continue-on-error`, a merge key, an unknown job or step key, a tab, an unrecognised shell function spelling, an exemption that expired or was orphaned, a marker naming the wrong job or a renamed step, the markers "
          "deleted, an inverted exemption, the sited steps moved back into an `if:` job, the local "
          "half's rows wrapped in a function called below its docs-tier exit, or deleted, a hosted JOB "
          "with neither a citation nor a reason, a reason with nothing after it, a reason on a job the "
          "local half cites anyway, a reason separated from its job key by a blank line, a reason too "
          "short to be a sentence, two reasons against one job, a job in a SECOND workflow file with "
          "neither a citation nor a reason, one job name defined in two files, EVERY flag in "
          "SEMANTIC_FLAGS passed by one half of a mirrored pair and not the other (in each direction, "
          "and through a shell function the local row only names), a flag both halves pass with "
          "different values, a flag one half passes on every invocation and the other on only some, a "
          "flag read out of a command substitution, EVERY spelling of a semantics-bearing "
          "ENVIRONMENT variable set on one half of a pair and not the other — a workflow, job "
          "or step `env:` block and an inline prefix, on a pair that runs cargo and on one "
          "that runs none — the same variable set to two values, a standing, exported or "
          "unattributable assignment, a $GITHUB_ENV write, a mirrored pair cited at a `uses:` "
          "step, an argv whose flag has no value or whose quote never "
          "closes, a PAIR_EXEMPT entry that expired, inverted, lost its pair or lost the token it "
          "excused, a tool pin bumped in ci.yml while the local half went on naming the old version, "
          "a local literal that drifted onto a DIFFERENT pin's value beside the tool it is not, and a "
          "PIN_FREE entry whose literal is gone — while accepting a `--selftest` mode invoked by "
          "a workflow on one line, through a loop over two scripts in one `run:` block, or across a "
          "line continuation, a scripts/gates/ member's uninvoked one, a script whose only mention of the flag is a full-line COMMENT about another script, a value only a runner can "
          "expand, "
          "a hosted `env:` block and a local prefix that say the same thing in different "
          "spellings — including a value carrying inner quotes, a prefix inside a one-line "
          "shell function, a loop body or an `env NAME=v` wrapper, and one reached only "
          "through a function the local row names — every rung of the precedence ladder "
          "outranking the one below it, an opaque value on EITHER half, an expression "
          "standing where a prefix would, a throughput knob on either half, "
          "and a redirection or a substitution sitting between a cargo command and its flags")


def _sub(t: str, path: str, a: str, b: str) -> None:
    full = os.path.join(t, path)
    with open(full, encoding="utf-8") as fh:
        s = fh.read()
    with open(full, "w", encoding="utf-8") as fh:
        fh.write(s.replace(a, b))


def _resite_prune(t: str) -> None:
    _sub(t, HOSTED_HALF, f"  {SITING_JOB}:\n    steps:\n      - uses: actions/checkout@v4\n",
         f"  {SITING_JOB}:\n    steps:\n      - uses: actions/checkout@v4\n      - run: rm -rf local-scripts .claude\n")


def _hollow_siting_job(t: str) -> None:
    """The reviewer's experiment: hollow `mirror`, move its steps into the
    `if:`-guarded job."""
    moved = "".join(f"      - name: sited {w}\n        run: {w}\n" for w in TIER_BLIND)
    _sub(t, HOSTED_HALF, moved, "      - run: echo hollow\n")
    _sub(t, HOSTED_HALF, "      - name: prune local-only tooling\n        run: rm -rf local-scripts .claude\n",
         "      - name: prune local-only tooling\n        run: rm -rf local-scripts .claude\n" + moved)


def _local_row_below_exit(t: str) -> None:
    """The rows wrapped in a function DEFINED above the docs exit and CALLED
    below it — the shape that satisfies a "mentioned before the exit" check
    while running nothing on the tier the rows are about."""
    for w in TIER_BLIND:
        _sub(t, LOCAL_HALF, f"{w}\n", "")
    body = "".join(f"  {w}\n" for w in TIER_BLIND)
    _sub(t, LOCAL_HALF, 'if [ "$TIER" = docs ]; then\n  exit 0\nfi\n',
         f"tier_blind_rows() {{\n{body}}}\n" + 'if [ "$TIER" = docs ]; then\n  exit 0\nfi\n'
         "tier_blind_rows\n")


def _local_row_deleted(t: str) -> None:
    _sub(t, LOCAL_HALF, f"{TIER_BLIND[0]}\n", "")


def main() -> int:
    root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    args = sys.argv[1:]
    if "--selftest" in args:
        selftest()
        return 0
    # `--root` IS THE SELF-TEST'S AFFORDANCE and nothing else passes it, which
    # is why the fixture's floor rides on it rather than on a flag of its own:
    # a `--marker-floor` a real invocation could reach for would be a way to
    # lower a merge gate that reads as a flag rather than as a decision.
    #
    # AND IT IS REFUSED ON THE GATE OF RECORD, because giving `--root` the
    # power to lower the floor is precisely what makes it worth reaching for:
    # `--root .` would run every claim against this repo with a floor of three,
    # so thirty-five markers could go missing and the row would still pass. The
    # condition is `GITHUB_ACTIONS`, which the runner sets and no edit to this
    # repo can unset — the same reason `check-python-lint.py` reads it rather
    # than a flag of its own. Off the gate of record the self-test uses it
    # freely; that is what it is for.
    floor = MIRROR_MARKER_FLOOR
    if "--root" in args:
        if os.environ.get("GITHUB_ACTIONS"):
            print("ERROR: check-ci-mirror-parity: `--root` is the self-test's fixture affordance and "
                  "carries the fixture's marker floor with it, so it is refused on the gate of record: "
                  "it would run the real repo against a floor of "
                  f"{FIXTURE_MIRRORED_ROWS} instead of {MIRROR_MARKER_FLOOR}. Invoke the checker with "
                  "no arguments, or `--selftest`.", file=sys.stderr)
            return 1
        root = os.path.abspath(args[args.index("--root") + 1])
        floor = FIXTURE_MIRRORED_ROWS
    try:
        errs = check(root, floor)
    except Bail as exc:
        if os.environ.get("GITHUB_ACTIONS"):
            print(f"::error::check-ci-mirror-parity: {exc}")
        print(f"ERROR: check-ci-mirror-parity: {exc}", file=sys.stderr)
        return 1
    for e in errs:
        # BOTH FORMS. `::error::` is what puts the message on the failing step
        # in the Actions UI; the plain line on stderr is what a `gh run view
        # --log` and a piped local run carry. Neither subsumes the other, and
        # the cost of printing both is one line.
        if os.environ.get("GITHUB_ACTIONS"):
            print(f"::error::check-ci-mirror-parity: {e}")
        print(f"ERROR: check-ci-mirror-parity: {e}", file=sys.stderr)
    if errs:
        return 1
    print("check-ci-mirror-parity OK: both halves name the same checks and the same gate modes, no orphan "
          "or missing check under scripts/ or demos/, both tools/ crates' halves agree, every checked-out "
          f"job in {WORKFLOW_DIR}/ but `{SITING_JOB}` prunes local-only tooling before reading it, every "
          "tier-blind check is sited in a job that no `if:`, `needs:` chain or `continue-on-error` can "
          "skip and above the local half's docs exit, all hosted-mirror citations resolve against every "
          f"workflow in {WORKFLOW_DIR}/, every job in every one of them is either cited by the local "
          "half or says at its own key, in a sentence, why it has no local half, and no mirrored pair "
          "passes an undeclared semantics-bearing flag on one half only, or on only some of one half's "
          "invocations, of a cargo subcommand both halves run, nor sets a semantics-bearing "
          "ENVIRONMENT variable on one half only or to a different value on the two halves — "
          "however each half spells it, an `env:` block on the workflow, the job or the step, "
          "or an inline prefix on the command — and every version literal in the "
          f"tracked files under {PIN_TREE}/ is a version {HOSTED_HALF} pins today or is declared in "
          "PIN_FREE as something else, with every line that names a pinned tool carrying that "
          "tool\u2019s current pin, and every `--selftest` mode outside scripts/gates/ is passed that "
          f"flag by a workflow in {WORKFLOW_DIR}/")
    return 0


if __name__ == "__main__":
    sys.exit(main())
