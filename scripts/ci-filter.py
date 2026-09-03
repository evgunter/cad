#!/usr/bin/env python3
"""Shared CI change filter — the SINGLE implementation of change
classification, used by BOTH .github/workflows/ci.yml (its `filter` job is
a thin YAML wrapper) and local-scripts/ci-local.sh. There is no second copy of
these rules anywhere; hosted and local runs are gated identically, and the
synthetic-diff tests exercise the one script both of them call.

Three tiers (Evan's ask: "changing a core crate runs everything, adding a
new crate only runs that new crate's tests" — dependency-AWARE, not naive
per-crate):

  TIER=docs     only NON-TRIGGERING paths changed — *.md anywhere,
                memories/, local-scripts/, .claude/ (the full list is
                `_is_docs`, and every entry past the first two is a tree
                hosted CI structurally cannot read). Nothing builds; the
                `docs-only ok` marker job is the whole gate. (Floor
                convention: floors apply to CODE PRs.)
  TIER=all      a workspace-level file changed — the change can move any
                crate's build, so everything runs, unscoped.
  TIER=closure  only crate sources changed. PKGS is the DEPENDENT CLOSURE:
                the changed members plus every member that transitively
                depends on them, dev-dependencies INCLUDED (a dev-dep edge
                is a real build edge for `cargo test`).

Classification is an ALLOWLIST, so it fails CLOSED by construction: a path
is scopable only if `_is_docs` recognises it as non-triggering or it lives
inside a known workspace member's directory. Anything unrecognised — a new
top-level file, a new excluded workspace, a renamed crate dir — is TIER=all.
Every error path (git failure, cargo-metadata failure, empty diff, a
crates/ subdirectory that is not a member) also lands in TIER=all.

Why no third-party tool. `determinator` (guppy) is the obvious ecosystem
answer and was evaluated: it is a LIBRARY with no binary (0.12.0, published
2023-06-26, no `bin_names`), so adopting it means writing and compiling a
bespoke Rust CLI here; it also wants `cargo metadata` for BOTH the base and
head revisions (it diffs package graphs), i.e. a second checkout; and its
headline feature — rules for mapping non-Cargo files onto packages — is
exactly the part this repo has to spell out by hand anyway (tier `all`
below). `cargo-nextest`'s `rdeps(pkg)` filterset expresses the closure
natively — and nextest WAS later adopted as the test runner (2026-08-03,
the build-once/archive restructure; its doc-test gap is covered by
explicit `cargo test --doc` rows). The filter still does not use
`rdeps()`: this script must also classify NON-test rows (clippy scope,
per-job roots, the docs/all tiers), so the ~40-line graph walk over
`cargo metadata --no-deps` remains the single implementation rather than
splitting the closure logic between two tools.

Usage:
  ci-filter.py --base <ref>        classify `git diff --name-only <ref>...HEAD`
  ci-filter.py --files <path|->    classify an explicit newline-separated list
  ci-filter.py --selftest          run the fixture battery below and exit
  ci-filter.py ... --seed <sha>    also SAMPLE the configuration matrix (below)
  ci-filter.py ... --config lane=interval eps=1e-12 klint=dev-probe
                                   REQUEST points instead of drawing them (below)
  ci-filter.py ... --config-from-message <file>
                                   read that same request out of a commit message
  ci-filter.py ... --notices <file>
                                   also write the human notices (a pin's
                                   reason, the interval advisory) to <file>,
                                   for a caller that relays them verbatim
  ci-filter.py --force-all         take no diff at all; return the `all` tier
  ci-filter.py --gated-set         print ONE nextest filterset expression
                                   selecting every gated suite in the tree
                                   (the nightly's ungated re-take), or
                                   `none()`; not the KEY=value stream

Output: KEY=value lines on stdout, one per line, safe to append to
$GITHUB_OUTPUT and to parse with `while IFS='=' read -r k v`.

  TIER=docs|all|closure
  PKGS=<comma-separated members, empty for docs, all members for `all`>
  CARGO_SCOPE=--workspace | -p a -p b ...
  RUN_BUILD=true|false          any cargo/grep row at all (false only for docs)
  RUN_EDITOR_CORE=true|false    the editor-core rows (see JOB_ROOTS)
  RUN_STL=true|false            watertight (admesh) row
  RUN_STEP_EXPORT=true|false    step import (freecad) row
  RUN_PNCAD_PY=true|false       python suite (wheel + unittest) row
  RUN_INTERVAL_BACKEND=true|false   interval-transcendentals' own workspace
  RUN_INTERVAL_ORACLE=true|false    its oracle-inari certification tier
  RUN_TOPO_RELEASE=true|false   corrupt input (release profile) row
  RUN_K_LINT=true|false         k-lint (gate) row
  LANE_ADVISORY=true|false      this diff touches `*interval*` files and this
                                run gates the DEFAULT lane, so if interval
                                semantics changed the author should ask for
                                the other one. Advisory: nothing reads it to
                                decide what runs (see below)
  LANE=default|interval|both    which COMPILE MODE this run gates (see below)
  EPS=default|<value>|all       which tolerance row this run gates
  KLINT_ROW=<unification>|all   which of `k-lint (gate)`'s five feature
                                unifications this run gates — drawn, or PINNED
                                by a `tools/` change to the row that RUNS that
                                crate's own suite (see below, and
                                `KLINT_PATH_ROWS`)
  SEEDS=<comma-separated members whose OWN files changed, empty for
                                docs and for `all`>
  CONFIG_SOURCE=lane:<src> eps:<src> klint:<src>
                                where each of the three values above came
                                from: `sampled` (drawn from --seed),
                                `unsampled` (no seed, so the whole matrix),
                                `pinned` (lane or klint — `_forces_interval` /
                                `_forces_klint` substituted it ahead of the
                                draw), `requested` (--config) or
                                `commit-trailer`
  TEST_FILTER=<nextest filterset expression>|<empty>
                                the GATED SUITES this run does not execute,
                                as one `-E` expression EXCLUDING them
                                (`not (A | B | ...)`), or empty for the
                                ordinary whole-suite run. Both run legs
                                append it; see THE PER-FILE TEST GATE below
  RUN_VIEWER_TOOLKIT=true|false the eframe/wgpu rows (`clippy -p viewer
                                --features app`, the doc gate's
                                --all-features pass over viewer) — keyed on
                                SEEDS, not on the closure; see
                                `VIEWER_TOOLKIT_SEEDS`

CONFIGURATION SAMPLING (2026-08-22, Evan's ask after the minutes audit).
The hosted gate used to run every point of {default, interval} x {default,
1e-6, 1e-12}. Those points almost always agree — that is the premise the
`interval` feature's additivity gate and the runtime-eps contract both
already assert — so the hosted gate now runs ONE point per run and lets
repetition cover the matrix: with 60 runs/hour during active work, a
break confined to one of the six points surfaces in minutes, and nothing
is shipped, so a briefly red main is affordable.

SEEDED FROM THE COMMIT, NOT FROM RANDOMNESS. `--seed` takes the head SHA
and the choice is `sha256(salt + seed) % len(choices)`. Two properties
follow, and both are the point rather than side effects:

  * A RE-RUN OF THE SAME COMMIT PICKS THE SAME POINT. True randomness
    would let a re-run of a red gate come back green on a different
    point, which reads as a flake and teaches a re-run habit that
    launders real failures. Here a red commit stays red until someone
    changes the tree.
  * THE POINT IS RECOVERABLE FROM THE SHA ALONE, so "which configuration
    gated this commit" is answerable after the fact, during a bisect,
    without the run's logs.

`hashlib`, not `hash()`: the builtin is salted per process (PYTHONHASHSEED)
and would break both properties on the first re-run.

THE THIRD SAMPLED DIMENSION (2026-08-22) is `k-lint (gate)`'s five FEATURE
UNIFICATIONS — see `KLINT_ROWS`. It is drawn under a salt of its own, like
lane and eps, so all thirty points of the matrix stay reachable.

AND IT IS THE SECOND DIMENSION WITH A PATH PIN (Evan's ruling, 2026-08-29). A
change under `tools/` does not draw its k-lint row: `_forces_klint` substitutes
the row that RUNS THAT CRATE'S OWN SUITE, from a mapping DERIVED off the job's
own steps (`KLINT_PATH_ROWS`) — which is not the same as the row that compiles
it, and the difference is written out there. `demos/` is deliberately not pinned and that scope
decision is argued at the same site. The residue is stated there too and it is
real: breakage in a path this mapping does not correlate still lands undrawn
and persists until a later draw finds it, which is the sampling design's own
argument. The pin narrows that hole; it does not close it.

NO SEED MEANS NO SAMPLING — LANE=both, EPS=all, KLINT_ROW=all. Fails OPEN into MORE work,
matching every other signal here. local-scripts/ci-local.sh passes no seed
and therefore still runs the whole matrix: it is not billed by the minute,
and with the hosted gate sampling, the local gate is now the only lane that
runs every point of the matrix on one tree.

A PIN IS ANNOUNCED TWICE, AND NEITHER HALF IS OPTIONAL. `LANE` is not always
drawn — `_forces_interval` pins it, ahead of the seed — nor is `KLINT_ROW`,
which `_forces_klint` pins on the same terms, and a pin no reader can see is
how a branch spends every run of its life on an axis nobody chose (#1122). Read
`lane` for `klint` throughout the two bullets below: one wording, two
dimensions, and the notice is composed once per pin in `main`.

  * `CONFIG_SOURCE=lane:pinned` on STDOUT. This is the half a machine and a
    reader-after-the-fact get: `LANE=interval` reads identically whether the
    seed chose it or the pin substituted it, so without this the run's own
    outputs answer "which configuration gated this commit" with `lane:sampled`
    over a lane no sample touched. It is a SOURCE, not a value — LANE stays
    `interval` and no job condition reads CONFIG_SOURCE.
  * THE REASON on STDERR, and into `--notices` when a caller asks for it. A
    path is not a matrix point and must not enter the KEY=value stream: both
    halves append stdout to $GITHUB_OUTPUT or read it with
    `IFS='=' read -r k v`, where one extra line would be one bogus output key.
    THE WORDING LIVES HERE AND ONLY HERE. ci.yml used to restate both notices
    in its own prose so it could print them where a reader looks, and the two
    copies drifted twice — one claimed the pin's reason always names a file
    (the fail-closed arm names none), the other said "DEFAULT LANE DRAWN" over
    a lane that had been requested. `--notices` is the relay that removed the
    second copy.

WHAT THE PIN NO LONGER COVERS, AND THE CONVENTION THAT REPLACED IT (Evan's
ruling, 2026-08-29, on #1122). `_forces_interval` used to pin on any changed
file whose BASENAME contained `interval`. That arm is gone: it could not tell
a rename from a semantic edit, and it gated a whole branch on the wrong axis
for its entire life because a type migration touched an interval-named test
file. The lane is now asked for, by the author, who is the only party that
knows: `CI-Config: lane=interval` on the head commit, or the dispatch input.
`LANE_ADVISORY=true` plus a stderr note is all this script does about a
name — it changes nothing about what runs. The rule itself lives in
`docs/prompts/implementer-discipline.md`, which every lane reads; this
message is a reminder of that rule, not a second copy of it.

REQUESTING A POINT INSTEAD OF DRAWING ONE (2026-08-28, Evan's ask). The draw
is a DEFAULT, not a lock: someone who wants this tree gated at 1e-12, or at
the k-lint row the draw keeps missing, says so and gets it. Two spellings,
one applier, and the only thing either does is replace a drawn value before
it is printed — no job condition, no matrix and no cache key reads anything
but the LANE / EPS / KLINT_ROW lines, so a requested point runs the identical
gate that point would have run had the SHA drawn it.

  --config lane=interval eps=1e-12 klint=dev-probe   THE INVOCATION says it.
      ci.yml's `workflow_dispatch` inputs land here, so a run can be aimed at
      a configuration with no commit and no push.
  --config-from-message <file>                       THE COMMIT says it, in a
      `CI-Config:` trailer line in the message (see `CONFIG_TRAILER`). ci.yml
      reads the PR's HEAD commit, not the merge ref it is checked out at, so
      what gates the run is what the author wrote.

WHY BOTH, when either alone answers the ask. They fail at opposite ends. A
dispatch cannot put its verdict on a pull request — its checks belong to the
run, not to the head commit's status — so it cannot be the thing a reviewer
looks at before merging. A trailer cannot re-gate a commit that is already
written, which is the whole of "that landed, now run it at 1e-12", because
this repo does not rewrite history. And the trailer keeps the property the
sampling was built around and a dispatch necessarily breaks: WHICH
CONFIGURATION GATED THIS COMMIT IS RECOVERABLE FROM THE COMMIT. A dispatch's
answer lives in one run's inputs, which is why CONFIG_SOURCE exists and why
ci.yml prints it in a step that always runs.

PRECEDENCE is invocation over trailer over draw, PER DIMENSION: the flag was
typed by whoever is standing here now, the trailer by whoever wrote the
commit. A dimension nobody named is still drawn, so `--config eps=1e-12`
means "1e-12, and surprise me twice".

A REQUEST THAT NAMES NO REAL POINT IS A HARD FAILURE, not a fallback to the
draw: an unknown key, an unknown value, a repeated key, a token that is not
`key=value`. This is the one place in this script that does not fail into
more work, and the asymmetry is the point — every other failure here is an
inability to classify, where running everything is the safe answer, while
this one is an INPUT ERROR whose author is standing there reading the result.
Failing open would hand them a green run over a configuration they did not
ask for, which is exactly the question they were asking.

`eps=all` IS NOT A LEGAL REQUEST, though the no-seed path above prints it: it
means "every row" to the local half, which loops over them, while the hosted
eps rows put the value straight into CAD_TOLERANCE_EPS, where `all` is a
parse error by design. `lane=both` and `klint=all` ARE legal — every job
condition already spells those as "run every row of that dimension".

THE PER-FILE TEST GATE (2026-09-02, S-TCOST lever 3; docs/TCOST-1-SPEC.md).
A suite that exercises the logic of a few named source files runs on a
pull-request gate only when one of those files, or the suite's own file, is
in the diff — rather than whenever any crate in its dependency closure moved,
which is what TIER=closure alone says. The suite names those paths ITSELF, in
a `test_utils::gated_to!` marker at the top of its file, and `TEST_FILTER`
above is the nextest expression that subtracts the untouched ones.

WHY A SUITE MAY BE SKIPPED AT ALL, since a skipped detector is normally the
thing this file refuses. `docs/CI-MINUTES-2026-08.md` §*What is NOT sampled*
draws the line at PERSISTENCE: skipping is sound for a detector whose subject
persists in the tree, and unsound for a detector of ABSENCE, which leaves no
future red behind. A gated suite's break persists — the code it was written
against is still wrong tomorrow — and the nightly's `--gated-set` row runs the
WHOLE gated set ungated on any day main moved, so the longest a break confined
to an unnamed path can hide is a day. This is the same argument the eps and
lane sampling rest on, at a longer period, and `memories/test-suite-cost.md`
already RULED the case for the first users: a fuzzer must be *"MARKED to run
only on changes to the code it was written to test"*, and one that is not
gated is a defect in the fuzzer.

RECORDED, NEVER SILENT, like every other skip here: one notice line per
skipped suite naming the suite and the paths that were not in the diff,
relayed by ci.yml's `the configuration this run gates` step, and both run legs
echo `TEST_FILTER` before invoking nextest.

The derivation, the marker's spelling and every fail-open arm are at THE
PER-FILE TEST GATE section further down this file, beside the code.

`--force-all` TAKES NO DIFF AT ALL and returns the `all` tier: everything
runs, unscoped. It is for the dispatch aimed at a ref whose diff against a
base is not the question — main after a merge, most often — where classifying
against the default branch comes back empty. It is not a workaround for a
base that is hard to name: with no file list the path-keyed signals fail
CLOSED, so such a run certifies the oracle and reads `lane` as `interval`
unless the request names a lane.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import subprocess
import sys

# Files that cannot move a hosted CI result.
#
# Documentation: deliberately narrow — only Markdown (anywhere) and the
# memories/ tree.
#
# EXCEPT THE MARKDOWN A CRATE COMPILES IN, and that exception is DERIVED, not
# listed. `crates/pncad/src/guide.rs` pulls `docs/GUIDE.md` and four pages
# under `docs/guide/` into rustdoc with `#![doc = include_str!(...)]`, which
# makes every Rust block in them a doctest, and `crates/pncad-py/tests/`
# executes the python blocks out of those same pages and out of that crate's
# README. An edit to any of them can turn a build red — so they are not docs.
#
# THE SET IS READ OFF THE SOURCES ON EVERY RUN (`_compiled_markdown` and
# `_markdown_read_by_python` below) and must stay that way. A sentence here
# naming today's pages would be a second roster, and this one is the file that
# decides whether anything runs: the last such sentence said `include_str!` was
# unused, went on saying it after five pages started being compiled in, and
# nothing could contradict it.
#
# local-scripts/: the LOCAL half of the tooling split (2026-08-11). No
# hosted job whose result is a build, a lint or a test may depend on
# anything in there, and that is enforced STRUCTURALLY rather than by
# convention — every workflow job that checks the repo out deletes the
# directory right after, so a workflow that grew a reference to it fails
# immediately and loudly instead of silently coupling the hosted gate to a
# developer's machine. Scripts hosted CI DOES run stay in scripts/ and keep
# forcing TIER=all, because a change to any of them can move a result.
#
# ONE JOB IS EXEMPT AND HAS TO BE: `mirror` reads local-scripts/ci-local.sh
# because its whole subject is whether the two halves of CI still run the
# same checks. That does not weaken the classification below — it is what
# makes it honest. `mirror` carries no `if:`, so it runs on EVERY tier
# including this one; a change under local-scripts/ therefore skips every
# build row and still runs the one job it can move. Before that job
# existed, a gate whose input was this tree argued it need not read the
# tree, BECAUSE a change to the tree classified docs and skipped the gate —
# a description of a hole offered as the reason not to close it.
# `scripts/check-ci-mirror-parity.py` fails if a second job stops pruning, or if
# `mirror` starts. That check is SITED IN THE JOB IT DESCRIBES, which is
# self-referential and is stated rather than hidden: deleting the job deletes
# the thing that would have complained. What limits the damage is that the same
# check runs in the local half above its docs exit, that the job is a required
# status check on this branch, and that a diff removing it is one line long in
# a file three tracks read.
#
# .claude/: agent session config (2026-08-15) — the SessionStart hook that
# provisions a Claude Code on the web container, and the settings.json that
# registers it. It is local-only tooling in exactly the sense above, just
# for an agent's container rather than a developer's laptop: it runs when a
# SESSION opens, never when a workflow does. It rides the SAME structural
# guard, and deliberately so — the prune step that deletes local-scripts/
# deletes this too, so the claim "hosted CI does not read .claude/" is
# checked on every run rather than trusted. Before that guard existed, a
# one-line hook edit cost a full 20-row gate including both render lanes.
#
# The distinction to keep hold of if this list grows again: a path belongs
# here only when hosted CI CANNOT read it (proven by the prune), not merely
# when it looks developer-ish. Anything hosted CI does read — scripts/,
# .github/, .cargo/ — stays out and keeps forcing TIER=all.
#
# Still an allowlist, still fails closed: a new top-level directory, or a
# new file directly under scripts/, is unrecognised and lands in TIER=all.
def _is_docs(path: str, consumed: frozenset[str] = frozenset()) -> bool:
    if path in consumed:
        return False
    return (
        path.startswith("memories/")
        or path.endswith(".md")
        or path.startswith("local-scripts/")
        or path.startswith(".claude/")
    )


# `include_str!("x")` / `include_bytes!` / `include!`, capturing the literal
# when there is one. A mention in prose (`\`include_str!\`ing the two lane
# files`) carries no `(` and is not a match.
_INCLUDE_RE = re.compile(r"\binclude(?:_str|_bytes)?!\s*\(\s*(\"[^\"\n]*\")?")
# Every Rust tree in the repo, workspace members and excluded workspaces
# alike: a page compiled into demos/tour's docs is compiled in just the same.
_RUST_TREES = ("crates", "demos", "tools", "interval-transcendentals")


def _compiled_markdown(root: str) -> frozenset[str]:
    """Repo-relative `.md` paths that some Rust source compiles into a build.

    FAILS CLOSED, twice over. An `include!` whose argument is not a plain
    string literal could name a `.md` and cannot be resolved by reading, so it
    raises `Bail` — TIER=all — rather than being skipped; and an unreadable
    source does the same. The scan is a regex over every `.rs` file outside
    `target/` — measured 0.43 s on this tree, against a whole classification
    of 0.65 s — and it runs before the docs branch is taken.

    BOTH SECONDS ARE ONE UNDATED LOCAL READING, re-taken by nothing, and
    they are here as a SHAPE rather than as a budget: the point is that the
    scan is a fraction of a classification that itself runs in under a
    second, so no tier's latency turns on it. Nothing asserts either, and a
    guard would be a wall-clock pin inside the filter that decides what CI
    runs — the one place a timing flake must not be able to change what a
    run gates. The figure that IS tracked, because it is the one anyone
    acts on, is the job's billed minute in docs/CI-MINUTES-2026-08.md.
    """
    out: set[str] = set()
    for tree in _RUST_TREES:
        base_dir = os.path.join(root, tree)
        if not os.path.isdir(base_dir):
            continue
        for base, dirs, names in os.walk(base_dir):
            dirs[:] = [d for d in dirs if d != "target"]
            for name in names:
                if not name.endswith(".rs"):
                    continue
                src = os.path.join(base, name)
                try:
                    with open(src, encoding="utf-8") as fh:
                        text = fh.read()
                except OSError as exc:
                    raise Bail(f"cannot read {src}: {exc}") from exc
                for m in _INCLUDE_RE.finditer(text):
                    lit = m.group(1)
                    if lit is None:
                        raise Bail(
                            f"{os.path.relpath(src, root)} has an `include!` whose argument is not a "
                            "string literal, so whether it compiles a .md into the build cannot be read "
                            "here — and that is what decides the docs tier"
                        )
                    target = lit[1:-1]
                    if not target.endswith(".md"):
                        continue
                    resolved = os.path.normpath(os.path.join(base, target))
                    out.add(os.path.relpath(resolved, root).replace(os.sep, "/"))
    return frozenset(out)


# `--no-renames` IS LOAD-BEARING, not a style flag. With rename detection on
# — git's default since 2.9 — `git diff --name-only` prints a rename as its
# DESTINATION PATH ONLY. So a source file moved out of a crate and into a
# `.md` arrives here as one path that `_is_docs` accepts, the whole change set
# classifies TIER=docs, and every build row is skipped over a deletion from a
# crate. Turning rename detection off makes the pair arrive as a delete and an
# add, and the delete side is unscopable, which is the answer the allowlist
# already knows how to give. The cost is that a pure rename inside one crate
# names two paths instead of one; both land in the same closure.
_DIFF_FLAGS = ("--name-only", "--no-renames")


def _markdown_read_by_python(root: str) -> frozenset[str]:
    """Repo-relative `.md` paths that a python test under `crates/` READS.

    The other half of the same fact. `crates/pncad-py/tests/test_guide.py`
    executes the python code blocks out of the guide pages and out of
    `crates/pncad-py/README.md`, so an edit to one of those can turn that
    suite red exactly as a Rust `include_str!` can turn a doctest red — and
    `_compiled_markdown` cannot see it, because there is no `include!` to
    match.

    The paths are built as `ROOT / "docs" / "guide" / "examples.md"`, so they
    are read the way they are written: an `ast` walk over `/`-chains of string
    constants, joined. Nor is the leading `Name` checked to BE the repo root: a
    chain rooted at a test directory resolves to a path that exists nowhere and
    simply matches no diff. The shape in use is the shape checked.

    FAILS CLOSED ON EVERY MENTION IT CAN SEE, which is the posture
    `_compiled_markdown` gets for free and this half has to buy. A `.md` STRING
    LITERAL anywhere in one of these sources that no resolved chain accounts for
    raises `Bail` — TIER=all — exactly as an unresolvable `include!` does on the
    Rust side. That is the guard against the failure mode that has no other
    tell: a page re-spelled some way this walk does not parse
    (`Path(__file__).parents[1] / "README.md"`), which does not error, does not
    shrink anything visibly, and simply drops that page into the docs tier where
    its suite stops running. The literal is still there to be seen, so it is
    read as uncertainty rather than as absence.

    THE RESIDUE, said plainly because a disclosed blind spot is a work order: a
    page whose name never appears as a literal at all — assembled from parts, a
    glob, an f-string, a name read from a fixture file. Nothing in the source
    ends in `.md`, so there is nothing to fail closed ON, and such a page stays
    in the docs tier with its suite skippable. Widening a mention this DOES see
    into a resolved path is a two-line change to `parts`; a page with no literal
    needs a different instrument.
    """
    import ast

    out: set[str] = set()

    def parts(node: "ast.AST") -> list[str] | None:
        if isinstance(node, ast.Constant) and isinstance(node.value, str):
            return [node.value]
        if isinstance(node, ast.BinOp) and isinstance(node.op, ast.Div):
            left, right = parts(node.left), parts(node.right)
            if left is None and isinstance(node.left, ast.Name):
                left = []
            if left is None or right is None:
                return None
            return left + right
        return None

    base_dir = os.path.join(root, "crates")
    if not os.path.isdir(base_dir):
        return frozenset()
    for base, dirs, names in os.walk(base_dir):
        dirs[:] = [d for d in dirs if d != "target"]
        for name in sorted(names):
            if not name.endswith(".py"):
                continue
            src = os.path.join(base, name)
            try:
                with open(src, encoding="utf-8") as fh:
                    tree = ast.parse(fh.read(), filename=src)
            except (OSError, SyntaxError) as exc:
                raise Bail(f"cannot read {src}: {exc}") from exc
            resolved: set[str] = set()
            for node in ast.walk(tree):
                if not isinstance(node, ast.BinOp) or not isinstance(node.op, ast.Div):
                    continue
                chain = parts(node)
                if chain and chain[-1].endswith(".md"):
                    resolved.add("/".join(chain))
            # Every `.md` literal in the file must be accounted for by one of
            # the chains above. A mention that is visible and unresolved is
            # uncertainty, and uncertainty is TIER=all.
            seen = {
                n.value
                for n in ast.walk(tree)
                if isinstance(n, ast.Constant)
                and isinstance(n.value, str)
                and n.value.endswith(".md")
            }
            for lit in sorted(seen):
                if any(r == lit or r.endswith("/" + lit) for r in resolved):
                    continue
                raise Bail(
                    f"{os.path.relpath(src, root)} names `{lit}` in a way this scan cannot "
                    "resolve to a repo path, so whether that page is executed by a suite "
                    "cannot be read here — and that is what decides the docs tier"
                )
            out |= resolved
    return frozenset(out)


def _consumed_markdown(root: str) -> frozenset[str]:
    """Markdown a BUILD OR A SUITE consumes, from both directions.

    Both halves fail closed on every consumption they can see, which is what
    makes the union safe to take: an `include!` that cannot be resolved and a
    `.md` literal that cannot be resolved each raise `Bail`, so a consumer
    re-spelled out of one parser's reach becomes TIER=all rather than becoming
    a page in the docs tier. Neither half can see a page whose name is never
    written down; that residue is `_markdown_read_by_python`'s to state and it
    does.
    """
    return _compiled_markdown(root) | _markdown_read_by_python(root)


def _run(cmd: list[str], cwd: str) -> str:
    return subprocess.run(
        cmd, cwd=cwd, check=True, capture_output=True, text=True
    ).stdout


class Bail(Exception):
    """Any uncertainty. Caught at top level and turned into TIER=all."""


def _repo_root() -> str:
    return os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


_MEMBERS_CACHE: dict[str, tuple[dict[str, str], dict[str, set[str]]]] = {}


def _members(root: str) -> tuple[dict[str, str], dict[str, set[str]]]:
    """Return (dir-name -> package name, package -> set of member deps).

    MEMOISED PER ROOT. Two callers now want the member map in one process —
    the closure, and the gated-suite terms' binary ids — and `cargo metadata`
    is a subprocess. The cache is keyed on the root and holds for the life of
    the process, which is one classification.

    `--no-deps` reads the workspace manifests only: no registry resolution,
    no network, no lockfile update. Dependency kinds are all kept — normal,
    build, AND dev — because `cargo test -p X` builds X's dev-dependencies,
    so a dev-dep edge propagates a change just as a normal one does.
    """
    if root in _MEMBERS_CACHE:
        return _MEMBERS_CACHE[root]
    meta = json.loads(
        _run(["cargo", "metadata", "--no-deps", "--format-version", "1"], root)
    )
    names = {p["name"] for p in meta["packages"]}
    dir_of: dict[str, str] = {}
    deps: dict[str, set[str]] = {}
    for pkg in meta["packages"]:
        manifest = os.path.abspath(pkg["manifest_path"])
        rel = os.path.relpath(os.path.dirname(manifest), root)
        parts = rel.split(os.sep)
        if len(parts) != 2 or parts[0] != "crates":
            # A member outside crates/<name>/ breaks the path mapping below.
            raise Bail(f"member {pkg['name']} lives at {rel}, not crates/<name>")
        dir_of[parts[1]] = pkg["name"]
        deps[pkg["name"]] = {
            d["name"] for d in pkg["dependencies"] if d["name"] in names
        }
    if not dir_of:
        raise Bail("no workspace members found")
    _MEMBERS_CACHE[root] = (dir_of, deps)
    return dir_of, deps


def _closure(seeds: set[str], deps: dict[str, set[str]]) -> list[str]:
    """Changed members + everything that transitively DEPENDS on them."""
    rdeps: dict[str, set[str]] = {p: set() for p in deps}
    for pkg, ds in deps.items():
        for d in ds:
            rdeps[d].add(pkg)
    out = set(seeds)
    stack = list(seeds)
    while stack:
        cur = stack.pop()
        for dependent in sorted(rdeps.get(cur, ())):
            if dependent not in out:
                out.add(dependent)
                stack.append(dependent)
    return sorted(out)


def classify(files: list[str], root: str) -> dict[str, str]:
    if not files:
        # No diff at all is not "nothing changed" as far as we can prove —
        # it is a base ref we failed to resolve. Fail closed.
        raise Bail("empty change set")

    consumed = _consumed_markdown(root)
    if all(_is_docs(f, consumed) for f in files):
        return {"TIER": "docs", "PKGS": "", "SEEDS": "", "CARGO_SCOPE": ""}

    dir_of, deps = _members(root)
    seeds: set[str] = set()
    for f in files:
        if _is_docs(f, consumed):
            continue
        parts = f.split("/")
        # ALLOWLIST: only a file inside a member's directory is scopable.
        # Everything else is workspace-level and forces TIER=all:
        #   root Cargo.toml / Cargo.lock / rust-toolchain.toml — resolution
        #     and toolchain move every crate;
        #   .cargo/**, .github/**, scripts/** — how everything is built/run;
        #   demos/**, tools/**, interval-transcendentals/** — the EXCLUDED
        #     workspaces; interval-transcendentals is a path dependency of
        #     geom-core, and demos/tour path-depends on nine members;
        #   docs/k-report-data/** — the k-lint job's committed input;
        #   anything new and unrecognised — by construction.
        if len(parts) < 3 or parts[0] != "crates" or parts[1] not in dir_of:
            raise Bail(f"workspace-level or unrecognised path: {f}")
        # A MEMBER's Cargo.toml is workspace-level too, even though it lives
        # under crates/: cargo unifies features across the whole workspace
        # build, so adding a feature or a dependency to one member can change
        # which features a SHARED dependency is compiled with for every other
        # member. There is no sound per-crate scoping of a manifest edit.
        if len(parts) == 3 and parts[2] == "Cargo.toml":
            raise Bail(f"member manifest changed (feature unification): {f}")
        seeds.add(dir_of[parts[1]])

    if not seeds:
        raise Bail("no member attributed")
    pkgs = _closure(seeds, deps)
    return {
        "TIER": "closure",
        "PKGS": ",".join(pkgs),
        # THE SEEDS, kept and reported alongside the closure they generate.
        # The closure answers "what must be rebuilt"; the seeds answer "what
        # did the author actually touch", and those are different questions
        # with different consumers. `pncad` is in almost every kernel change's
        # CLOSURE (it re-exports everything) and in almost none of their
        # SEEDS, which is exactly the distinction the viewer-toolkit axis in
        # `decorate` is keyed on. Reported rather than derived-and-discarded
        # so a reader of the filter's output can see the input to that
        # decision, not only its verdict.
        "SEEDS": ",".join(sorted(seeds)),
        "CARGO_SCOPE": " ".join(f"-p {p}" for p in pkgs),
    }


def _all_tier(root: str) -> dict[str, str]:
    try:
        dir_of, _ = _members(root)
        pkgs = ",".join(sorted(dir_of.values()))
    # Fail CLOSED, the same posture as the caller below and stated in the same
    # words on purpose. TIER is already "all" and CARGO_SCOPE already
    # "--workspace" by the time this runs, and `decorate` sets every job flag
    # true on TIER=all without consulting PKGS — so an unreadable member list
    # costs the ECHOED package names and nothing that is RUN. No job is skipped
    # by taking this branch.
    except Exception:  # noqa: BLE001 — fail CLOSED, like the caller below
        pkgs = ""
    # SEEDS is empty at tier `all` and that is not "nothing was
    # touched": the tier means the change is unscopable, so every
    # seed-keyed axis must read it as "all seeds", not as none. Each
    # such axis in `decorate` branches on the tier FIRST, before it
    # looks at this field.
    return {"TIER": "all", "PKGS": pkgs, "SEEDS": "", "CARGO_SCOPE": "--workspace"}


# Root packages per pipeline job. A job runs iff one of its roots is in the
# closure; the closure is already upward, so a root set is minimal — e.g.
# `stl` covers a geom-core change because stl transitively depends on it.
#
# watertight    builds bodies profile -> sweep -> topo -> mesh and writes
#               them with `cargo run -p stl --example export_acceptance`;
#               everything it touches is under stl's (dev-)dependency graph.
#               ITS HOSTED HALF MOVED TO nightly.yml (2026-08-22) and runs
#               there UNGATED — once a day is not a bill worth filtering — so
#               this signal's only remaining consumer is ci-local.sh. It is
#               kept rather than deleted because the local gate is the half
#               that pays for an unfiltered row, in a developer's wall clock.
# step-import   runs FreeCAD over the COMMITTED fixtures in
#               crates/step-export/tests/fixtures (no cargo build at all),
#               which are byte-golden against the step-export writer.
# editor-core   the named `cargo test -p editor-core --test ...` rows. THREE
#               OF THE FOUR WENT AWAY on 2026-08-22 and the signal survives on
#               the fourth: `persistence` and `band 4 corpus` were deleted
#               outright (their modules are ordinary tests in the archive, and
#               the jobs re-ran them at two fixed ε, defeating the ε sampling
#               for exactly those modules), and `rebuild latency` moved to
#               nightly.yml. What still reads this is ci.yml's `test-interval`
#               job — its two named interval rows — plus ci-local.sh.
# pncad-py      the python-suite row builds the wheel from pncad-py, whose
#               dependency graph is the whole façade stack (pncad ->
#               editor-core -> ... ), so `pncad-py in closure` is exactly
#               "something the wheel compiles moved"; the crate's own .py
#               test/stub files live under its member directory, so they
#               seed the same closure.
# topo          the release-profile corrupt-input row compiles
#               `-p topo --lib`, so topo's own closure membership is
#               exactly the condition under which anything it runs can
#               have moved. It is the one job whose root is the crate the
#               suite lives in rather than a downstream consumer.
JOB_ROOTS = {
    "RUN_EDITOR_CORE": {"editor-core"},
    "RUN_STL": {"stl"},
    "RUN_STEP_EXPORT": {"step-export"},
    "RUN_PNCAD_PY": {"pncad-py"},
    "RUN_TOPO_RELEASE": {"topo"},
}


# The oracle-inari certification tier is the ONE job keyed on paths rather
# than on TIER/PKGS, and it has to be.
#
# `interval-transcendentals` is its own workspace, so `classify`'s allowlist
# sends every change under it to TIER=all — and TIER=all is the majority
# verdict across merges, so `tier == "all"` (what RUN_INTERVAL_BACKEND uses)
# would fire this on most of them. That is affordable for the backend's
# oracle-free tier, which is seconds; it is not affordable here, because this
# job builds GMP and MPFR from C source: 234s of the ~250s it costs, measured
# on a hosted runner in #480, against 7s for the 4M certification cases
# themselves.
#
# Keyed on the paths, it fires when the certified code or its dependency
# pinning moves — 2 of the last 400 first-parent merges — for about eight
# runner-minutes a year.
ORACLE_PATHS: tuple[str, ...] = (
    "interval-transcendentals/src/",
    "interval-transcendentals/tests/",
    "interval-transcendentals/Cargo.toml",
    "interval-transcendentals/Cargo.lock",
)


def _touches_oracle(files: list[str] | None) -> bool:
    # Fail CLOSED, like everything else here: if we could not resolve a file
    # list at all, we cannot prove the certified code held still, so run it.
    #
    # An EMPTY list counts as unresolved, not as "nothing changed" — the same
    # reading `classify` already takes of an empty diff, and for the same
    # reason. Keeping the two consistent matters: otherwise the one input
    # that makes `classify` shout would make this signal go quiet.
    if not files:
        return True
    return any(f.startswith(ORACLE_PATHS) for f in files)


# THE SEEDS THAT BUY THE GUI TOOLKIT ROWS (Evan's viewer-CI-posture ruling,
# 2026-08-27; docs/GUI-LOG.md). SEEDS, not the closure — the argument is at
# `RUN_VIEWER_TOOLKIT` in `decorate`, and it is the whole of why this is a
# three-name set rather than "anything viewer depends on".
#
# Adding a name here is a decision about what can break the eframe/wgpu half
# without touching viewer's own sources; it is not a convenience. The nightly
# lane re-takes the whole row daily, which is what makes the set safe to keep
# small.
VIEWER_TOOLKIT_SEEDS: frozenset[str] = frozenset({"viewer", "pncad", "bvh"})

# THE SAMPLED MATRIX. Both lists are the full set the hosted gate used to
# run on every push; sampling picks one member of each per run.
#
# LANES are the two COMPILE MODES. `interval` is not a subset lane: when it
# is drawn it runs the WHOLE suite, not the interval-gated difference, because
# the default lane is not running that round and ~95% of the tests are shared.
# (That is the opposite of what the pre-sampling gate wanted, where both lanes
# ran and the overlap was pure re-execution.)
LANES: tuple[str, ...] = ("default", "interval")

# EPS rows straddle the compiled default (DEFAULT_EPS = 1e-9) three orders
# either side, and `default` means the variable genuinely UNSET — an empty
# CAD_TOLERANCE_EPS is a parse error by design (geom-core/src/tolerance.rs).
EPS_ROWS: tuple[str, ...] = ("default", "1e-6", "1e-12")

# `k-lint (gate)`'s FIVE FEATURE UNIFICATIONS, sampled one per run
# (2026-08-22). This comment used to say the job "bills 8-10 minutes", and so
# did ci.yml's `k-lint` header; both were quoting a PRE-SAMPLING column of
# docs/CI-MINUTES-2026-08.md as though it were current. It is not: that row is
# a one-shot reading of one reference run taken before this very ruling landed,
# the same document's 2026-08-22 section derives this sampling at −7 to −8
# billed minutes, and its 2026-08-31 addendum says a billed figure there is
# only true as of the measurement it names a run id for. NO RANGE IS RESTATED
# HERE — the argument below needs the SHAPE (five unifications sharing almost
# nothing, so the lever is running fewer of them), not a cost, and ci.yml's
# header carries the correction rather than a second copy of it. The reason
# this job is expensive at all is not one slow
# check: it compiles demos/tour and the kernel crates FIVE TIMES OVER, once
# per unification below, and those five share almost no artifacts —
# `--release` and dev are different profiles, and `budget` and `probe` are
# opt-in features gated at a module boundary, so each is its own fingerprint
# for every crate that sees it.
#
#   dev-default      demos/tour + demos/wild + the three tools/ crates,
#                    `cargo fmt --check` / `clippy --all-targets` / `cargo
#                    test`, dev profile, default features
#   release-default  the demos/tour suite: `cargo test --release` in
#                    demos/tour (the #99 ε pin plus the tour bin's own
#                    unit probes), default features
#   release-budget   the tessellation-budget sweep (`cargo run --release
#                    --features budget`) and the tess-lint gate over its CSV
#   dev-budget       `cargo clippy`/`cargo test -p mesh --features budget`,
#                    which is also where MIN-1's per-triangle certificate
#                    falsifier runs
#   dev-probe        the probe-gated test targets (compile + listing) and the
#                    K-telemetry sweep (`--features probe`, dev), and the
#                    large-K lint over the CSVs it writes
#
# THE PROFILE IS THE FIRST TOKEN, deliberately: ci.yml keys this job's
# `Swatinem/rust-cache` entry on it, so the two dev draws and the two release
# draws each share a cache lane instead of one lane thrashing between
# profiles. Renaming a row means reading that expression.
#
# SOUND FOR THE SAME REASON THE ε DRAW IS, and it was checked ROW BY ROW
# rather than assumed (2026-08-22). Sampling covers a detector whose subject
# PERSISTS in the tree — a clippy finding, a failed assertion, a grown
# triangle budget, a probe suite that stopped compiling all stay broken until
# someone fixes them, so a later draw finds them. It is unsound for a
# detector of ABSENCE, whose subject merges once and leaves no future red.
# None of the five is one: the census gate that would notice a probe suite
# DISAPPEARING (`probe-suite-census.sh` in its default mode, with its
# CENSUS_FLOOR) is sited in `discipline`, which is unconditional and not
# sampled — what rides here is only the behavioural half, and a suite that
# stops being built stays unbuilt.
#
# WHAT IT COSTS, said out loud because two ratified review outcomes named
# these rows as UNCONDITIONAL and this makes them 1-in-5: MIN-1's certificate
# falsifier (dev-budget) and `crates/sweep/tests/k_report.rs` +
# docs/K-REPORT.md's "on every building merge" (dev-probe). No gate reds on
# either — the census greps for the STEP NAME, not for how often it runs — so
# every correction here is written by hand, and all THREE sites are now
# corrected:
#
#   * `crates/sweep/tests/k_report.rs` says "1 in 5" and names the row it
#     rides.
#   * docs/K-REPORT.md names the row and the schedule at every sentence of
#     its that carried a frequency claim — including the one that turned out
#     to be TRUE, the census tally sited in `discipline`, which is marked as
#     unconditional rather than demoted with the rest.
#   * MIN-1's falsifier: its own step comment in ci.yml said the row "stays
#     unconditional" three lines above its own `if:`. It now says 1-in-5, and
#     says that no path pin restores it — `crates/mesh` is not a pinned root.
#     That site was missed on the first pass of this correction, which is the
#     discharge-by-line-number failure the correction itself is about.
#
# THE SCHEDULE THEY WERE CORRECTED TO IS THE ONE BELOW, both halves of it:
# drawn 1-in-5, and PINNED — not drawn at all — for the paths
# `KLINT_PATH_ROWS` names, which reach none of the three claims above.
KLINT_ROWS: tuple[str, ...] = (
    "dev-default", "release-default", "release-budget", "dev-budget", "dev-probe",
)

# WHEN THE K-LINT ROW IS NOT LEFT TO CHANCE (Evan's ruling, 2026-08-29). A
# change under `tools/` PINS the row that RUNS ITS SUITE, ahead of the draw.
#
# THE CASE, AND WHY IT IS NOT AN ARGUMENT AGAINST THE SAMPLING. All five rows
# above are persistence-detectors, so a break in one is found by a later draw
# — that is sound and it is not in question here. What it does not say is
# WHOSE merge finds it: for a tool crate the finder is whoever's PR next draws
# that row, so the break lands undrawn and detonates somewhere unrelated. THE
# MEASURED INSTANCE: `tools/tess-meter`'s `SPLIT_SCAN_DECADES` /
# `SPLIT_SCAN_SAMPLES` are boxed by a guard living in that crate's OWN tests,
# and the row that runs those tests is drawn 1-in-5, so the merge that retunes
# the constants is more likely than not the merge that does not run the guard.
# The pin measures nothing new and bills nothing new: it forces a row this run
# was going to spend anyway.
#
# THE SCOPE IS `tools/`, AND THE NUMBER THAT CHOSE IT IS NOT GUARDABLE. The
# ruling measured ~7% of code-shaped merges touching that tree over 14 days;
# re-measurements over other windows come out higher (9-11%). None of those is
# wrong: a FIRING RATE IS A PROPERTY OF MERGE TRAFFIC, NOT OF THIS TREE, so no
# gate here can hold it and none is written. The number's home is the ruling's
# own record in docs/S-QA-PLAN.md; what matters at this site is the qualitative
# claim it supports — `tools/` is a small enough slice that making this
# dimension deterministic on it leaves the dimension sampled.
#
# `demos/` IS DELIBERATELY NOT PINNED, and that is a decision rather than an
# omission — said here, because a scope that lists only what it covers reads as
# an oversight the next time someone asks. Two reasons, and the second is the
# one that decides it. It is several times the `tools/` slice (the ruling had
# ~29%), so pinning it would fix this dimension on something like a third of
# all runs, which is the sampling eroded rather than narrowed. And the demos
# failure shape that actually bit — a tour scene that stops compiling, a scene
# whose output moved — breaks every row that BUILDS the tour, which is 4 of the
# 5 (`dev-budget` is `-p mesh` and reaches no demo), so a draw finds it on the
# offending merge with probability 4/5 rather than 1/5. The `tools/` case is
# the opposite shape: one crate's own test suite, run by exactly one row.
#
# THE MAPPING IS DERIVED FROM WHICH ROW RUNS THE CRATE'S OWN SUITE — which is
# NOT the row that compiles it, and conflating the two is how this comment was
# wrong on its first writing. `demos/tour` takes `tess-meter` as a plain,
# un-feature-gated dependency (see its Cargo.toml), so `dev-default`,
# `release-default`, `release-budget` and `dev-probe` all COMPILE that crate
# through the tour; a syntax error in it reds four rows out of five. What only
# one row does is EXECUTE its tests, and a guard that lives in a test is
# invisible to the other four however thoroughly they type-check it. Read off
# `k-lint (gate)`'s steps and their `if:` conditions:
#
#   tools/k-lint/      `dev-default` runs its fmt + clippy + `cargo test` (the
#                      #99 litmus). `dev-probe` also builds it — `cargo run --`
#                      for the large-K lint — but runs none of its tests: that
#                      row is a CONSUMER of the binary, not the row that checks
#                      the crate.
#   tools/tess-lint/   the same shape. `dev-default` runs fmt + clippy + tests
#                      (the three exit voices); `release-budget` `cargo run --`s
#                      the binary as the tessellation-budget gate.
#   tools/tess-meter/  `dev-default` runs its suite, and it is the only row
#                      that does. Four rows compile it (above); one asserts
#                      anything about it, which is the case this pin was
#                      measured on.
#
# So every entry is `dev-default` today and the table is single-valued. It is a
# TABLE anyway for two reasons: the derivation is per-crate and the next tool
# need not land in the same row, and `_selftest_klint_premise` reds when a
# member of a pinned root has no entry — which turns "someone added a tool and
# nobody derived its row" from a silent inheritance into a failed self-test.
#
# THIS TABLE IS THE ONLY HOME OF THE SCOPE DECISION. `KLINT_PIN_ROOTS` below is
# derived from its keys rather than written beside them, so adding a `demos/…`
# entry here WIDENS the pin rather than sitting inert next to a `tools/` literal
# that ignores it — and `_selftest_klint_pin`'s `demos/`-must-DRAW case is then
# what reds, which is the ruling being enforced rather than a spelling.
KLINT_PATH_ROWS: tuple[tuple[str, str], ...] = (
    ("tools/k-lint/", "dev-default"),
    ("tools/tess-lint/", "dev-default"),
    ("tools/tess-meter/", "dev-default"),
)

# THE TREES THE PIN LOOKS AT, DERIVED FROM THE TABLE ABOVE. Two homes for one
# decision is one home too many: a literal `tools/` prefix test here would run
# BEFORE the table is consulted, so an entry naming any other tree would be
# silently inert — a widening that changes nothing and reds nothing.
KLINT_PIN_ROOTS: tuple[str, ...] = tuple(
    sorted({p.split("/", 1)[0] + "/" for p, _ in KLINT_PATH_ROWS})
)

# THE FALLBACK, AND WHY IT IS A ROW RATHER THAN THE DRAW. A path under a pinned
# root that the table does not name is a path whose row nobody has derived, and
# leaving that one to the draw is the state this pin exists to end.
# `dev-default` is the row that runs every tool crate's own suite today and is
# gated on more of this job's steps than any other row — both of which
# `_selftest_klint_workflow` reads off ci.yml rather than taking on trust — so
# it is the cheapest honest answer. It IS a guess, which is why the self-test
# reds on an unnamed member rather than letting the guess stand.
KLINT_PIN_FALLBACK = "dev-default"

# WHEN THE INTERVAL LANE IS NOT LEFT TO CHANCE — AND THE NAME-SHAPED HALF THAT
# NO LONGER IS (Evan's ruling, 2026-08-29, on #1122).
#
# This used to pin the lane on TWO signals. One was exact; the other guessed
# from a filename, and the guess is gone.
#
#   * `interval-transcendentals/` STAYS. It is exact by construction: that
#     tree is the interval backend's own workspace, so a change under it
#     cannot be about anything else, and the crate's own guard jobs sit
#     alongside this rather than depend on it.
#   * An unresolved file list STAYS, and stays for the reason every other
#     signal here fails closed: nothing can prove interval code held still
#     when nothing is known about what changed.
#   * `interval` ANYWHERE IN A BASENAME IS REMOVED. It matched a rename that
#     touched `extrude_interval.rs` for three identifiers of an
#     `EdgeGeometry` → `EdgeDescription` migration, and from then on every
#     push of that branch was pinned to a lane nobody chose — a re-push is a
#     fresh draw, but the pin ran first and short-circuited it, so the advice
#     "re-push until the default lane lands" looped forever. The branch's
#     whole subject was ~340 consumer sites, i.e. the default lane's battery,
#     and it spent its entire life on the other axis. The rule could not tell
#     a rename from a semantic edit, because a filename cannot.
#
# WHAT REPLACES IT IS A CONVENTION, NOT A HEURISTIC: whoever changed interval
# semantics knows they did, and asks for the lane with a
# `CI-Config: lane=interval` trailer on the head commit (or the dispatch
# door). `_advises_interval` below is the reminder, not the mechanism — the
# convention lives in `docs/prompts/implementer-discipline.md`, which every
# lane reads, because a convention only a filter message states is one nobody
# follows.
#
# AND THE PIN THAT REMAINS IS ANNOUNCED. It is not defeatable by re-pushing —
# it runs before the seeded draw and short-circuits it — so `main` prints it
# and its reason to stderr and `decorate` records `lane:pinned`, which is why
# this returns the VALUE AND THE REASON rather than a bool.
#
# THE `(value, why)` SHAPE IS SHARED WITH `_forces_klint` ON PURPOSE. Both pins
# feed one loop in `decorate` and one notice composer in `main`; a sibling that
# returned only a reason would need its value hardcoded at each of those sites,
# which is how a wording drifts from the thing it describes.
def _forces_interval(files: list[str] | None) -> tuple[str, str] | None:
    """`(lane, why)` when the lane is pinned, or `None` if it is drawn."""
    # Fail CLOSED like every other signal here: an unresolved file list cannot
    # prove interval code held still, so pin the lane rather than sample it.
    if not files:
        return ("interval", "the changed-file list could not be resolved")
    for f in files:
        if f.startswith("interval-transcendentals/"):
            return ("interval", f"{f} is under interval-transcendentals/")
    return None


# THE SAME SHAPE ONE DIMENSION OVER, and announced the same way: it runs before
# the seeded draw and short-circuits it, so `main` prints the reason to stderr
# and `decorate` records `klint:pinned`, which is why this returns the REASON
# alongside the row rather than the row alone. The scope, the derivation and
# the `demos/` exclusion are at `KLINT_PATH_ROWS`.
def _forces_klint(files: list[str] | None) -> tuple[str, str] | None:
    """`(row, why)` when a change pins the k-lint row, or `None` if it is drawn."""
    # UNRESOLVED FAILS CLOSED INTO EVERY ROW, not into the draw: nothing is
    # known about what changed, so nothing can prove `tools/` held still, and a
    # guarantee that lapses exactly where the evidence is missing is not one.
    # `all` is the expensive answer here — five compiles rather than one, which
    # is the whole bill the sampling removed — and it is still the right one,
    # because a run that could not resolve its own diff is already TIER=all and
    # running the entire workspace on precisely this argument.
    if not files:
        return ("all", "the changed-file list could not be resolved, so nothing here "
                       f"can prove {' or '.join(KLINT_PIN_ROOTS)} held still")
    # KEYED ON THE ROW, NOT THE FILE: a diff touching two members of one row
    # pins that row once, and the first file to reach it is the one named. The
    # files are sorted so which one that is does not depend on the order the
    # diff came out in — a reason that moves between two runs of the same tree
    # reads as a second pin.
    rows: dict[str, str] = {}
    for f in sorted(f for f in files if f.startswith(KLINT_PIN_ROOTS)):
        for prefix, row in KLINT_PATH_ROWS:
            if f.startswith(prefix):
                rows.setdefault(row, f"{f} is under {prefix}, whose own suite the "
                                     f"`{row}` row is the one that runs")
                break
        else:
            root = next(r for r in KLINT_PIN_ROOTS if f.startswith(r))
            rows.setdefault(KLINT_PIN_FALLBACK,
                            f"{f} is under {root} and no row is derived for it, so it "
                            f"falls back to `{KLINT_PIN_FALLBACK}` — the row gated on "
                            "the most of this job's steps, never the draw")
    if not rows:
        return None
    if len(rows) == 1:
        ((row, why),) = rows.items()
        return (row, why)
    # TWO ROWS ASKED FOR AT ONCE, so neither of them alone is honest and `all`
    # is the only value that runs both. Unreachable while the table is
    # single-valued, and written anyway because the alternative — first match
    # wins — is this unit's own defect one level down: a row quietly dropped
    # from the one run that needed it.
    detail = "; ".join(f"{row} ({why})" for row, why in sorted(rows.items()))
    return ("all", f"this diff needs {len(rows)} k-lint rows and `all` is the only "
                   f"value that runs them — {detail}")


# THE ADVICE THAT REPLACED THE PIN. Same name-shaped observation, stripped of
# the authority it should never have had: this changes NOTHING about what runs.
# It exists because the ruling that removed the pin removed a reminder along
# with it, and the case the pin was built for — someone edits interval
# semantics, the draw goes the other way, and they find out two runs later —
# is real even though the filename could not identify it. A name can raise the
# question; only the author can answer it.
def _advises_interval(files: list[str] | None) -> list[str]:
    """EVERY changed file whose basename carries `interval`; empty if none.

    Advisory only. Nothing reads this to decide what runs.

    ALL OF THEM, not the first. The reader's question is "did I change
    interval semantics", and one filename out of nine answers it for one file
    while implying it is the only one — the notice would then be quietly wrong
    about the size of what it is asking about.
    """
    return [f for f in (files or []) if "interval" in f.rsplit("/", 1)[-1]]


# ----------------------------------------------------- the per-file test gate
#
# WHAT A MARKER IS. A gated suite names, in its own file, the source paths it
# was written to exercise:
#
#     test_utils::gated_to!["crates/geom-core/src/ring.rs", "crates/geom-core/src/interval/"];
#
# The suite then runs on a pull-request gate only when one of those paths, or
# the suite's own file, is in the diff. `crates/test-utils/src/lib.rs` carries
# the macro and the rules a marker's AUTHOR needs; what follows is what the
# READER of this file needs, which is how the set becomes a nextest expression
# and every way that derivation is allowed to fail.
#
# WHY THIS IS READ FROM THE TEXT AND NOT FROM A ROSTER. The same argument
# `scripts/nightly-only-selection.py` makes for the demoted set and
# `check-ci-mirror-parity.py` makes for its citations: a central list of which
# suites are gated is a second copy of a fact the tree already holds, free to
# drift from it, while a mark at the test cannot. The set below is DERIVED on
# every run, from the tree that is about to be tested.
#
# WHY IT IS A MACRO AND NOT A COMMENT, given that this script reads text
# either way. A comment can be misspelt and stay a comment; the misspelling
# then reads as "this suite is not gated", which is the safe direction for
# COVERAGE and the wrong one for a reader who believes their fuzzer is gated.
# `gated_to!` is a real path into a real crate, so the same typo is a compile
# error. The macro expands to nothing and costs no build time.
#
# `crates/test-utils/` IS NOT SCANNED. It is the marker's home, its docs quote
# the spelling, and a diff touching it empties this filter outright (below) —
# so a marker there could never gate anything and would only make this scan's
# own fixtures ambiguous.

# `gated_to!` with any of the three delimiters a macro call may use, whether or
# not it is written through the `test_utils::` path. The literals are pulled
# out of the balanced text that follows, so a call rustfmt has wrapped across
# lines reads exactly like a one-line one.
_GATED_CALL_RE = re.compile(r"\bgated_to!\s*([\[({])")
_GATED_CLOSER = {"[": "]", "(": ")", "{": "}"}
_GATED_LITERAL_RE = re.compile(r'"([^"\n\\]*)"')
# `#[path = "curves/lt_r1_probes.rs"]` immediately above `mod curves_lt_r1_probes;`
# in a crate's aggregated `tests/all.rs`. That pair is the only place the
# module prefix a nextest test id carries is written down.
_ALL_RS_PATH_RE = re.compile(r'#\s*\[\s*path\s*=\s*"([^"\n]+)"\s*\]')
_ALL_RS_MOD_RE = re.compile(r"^\s*(?:pub\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*;")
# What may go into `test(/^<module>::/)` unquoted and unescaped. A Rust module
# path cannot contain anything else; a path that does is not silently emitted
# as a malformed filterset, it fails open (below).
_MODULE_PATH_RE = re.compile(r"^[A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_]*)*$")
_BINARY_ID_RE = re.compile(r"^[A-Za-z0-9_-]+(?:::[A-Za-z0-9_-]+)?$")

# A diff touching one of these empties the whole filter: they are the inputs
# the derivation itself is made of, and a run that changed how the gate is
# COMPUTED has no business trusting the gate's answer about itself.
#
#   crates/test-utils/ — the macro, and the harness every gated suite draws
#     its RNG and its EFFORT dial from. A change here can move what any of
#     them does.
#   scripts/ci-filter.py — this file.
#   */tests/all.rs — the aggregation file each `tests/` term's module prefix
#     is read out of. A suite renamed there is a term pointing at nothing, and
#     a term that matches no test EXCLUDES no test, which is silent.
#
# The first two are already unscopable by `classify` (a `scripts/` path forces
# TIER=all, a test-utils change seeds a closure that is nearly the workspace),
# so this is belt and braces for those; `tests/all.rs` is not, and is the one
# that earns the arm.
def _gate_fails_open(files: list[str] | None) -> str | None:
    for f in files or ():
        if f.startswith("crates/test-utils/"):
            return f
        if f == "scripts/ci-filter.py":
            return f
        if f.endswith("/tests/all.rs"):
            return f
    return None


class GatedSuite:
    """One marked file: what it declared, and the nextest term that selects it.

    `problem` is set when the term could not be derived or a declared path
    does not resolve. Such a suite is never excluded — it runs — and says so
    in a notice. The LOUD half of that is `scripts/gates/gated-suite-paths.sh`
    in the `discipline` row: this script's job is to keep the gate honest, and
    a gate that reds the run is the gate's job.
    """

    def __init__(self, path: str, paths: list[str], term: str | None, problem: str | None):
        self.path = path
        self.paths = paths
        self.term = term
        self.problem = problem

    def selected_by(self, changed: set[str]) -> bool:
        """Is one of this suite's paths — its OWN FILE INCLUDED — in the diff?"""
        if self.path in changed:
            return True
        for want in self.paths:
            if want.endswith("/"):
                if any(f.startswith(want) for f in changed):
                    return True
            elif want in changed:
                return True
        return False


def _marker_paths(text: str) -> list[str] | None:
    """The literals of the ONE `gated_to!` call in `text`, or None if there is
    no call. Raises `Bail` on a second call in one file, and on a call this
    reader cannot bracket-match — both of which mean the file says something
    this script would otherwise answer by ignoring half of it."""
    calls = list(_GATED_CALL_RE.finditer(text))
    if not calls:
        return None
    if len(calls) > 1:
        raise Bail(
            "more than one gated_to! call: one marker per file, so that the "
            "path set a reader sees at the top is the whole set"
        )
    match = calls[0]
    opener = match.group(1)
    closer = _GATED_CLOSER[opener]
    depth = 0
    end = None
    for i in range(match.end() - 1, len(text)):
        ch = text[i]
        if ch == opener:
            depth += 1
        elif ch == closer:
            depth -= 1
            if depth == 0:
                end = i
                break
    if end is None:
        raise Bail("a gated_to! call whose delimiters do not close")
    return [m.group(1) for m in _GATED_LITERAL_RE.finditer(text[match.end() : end])]


def _all_rs_modules(root: str, crate_dir: str) -> dict[str, str]:
    """`tests/<file>.rs` -> the module name `tests/all.rs` includes it under.

    The module name is what a nextest test id is prefixed with, and it is NOT
    derivable from the filename: `crates/geom/tests/all.rs` includes
    `curves/lt_r1_probes.rs` as `curves_lt_r1_probes`. Read the pair, never
    guess it.
    """
    all_rs = os.path.join(root, "crates", crate_dir, "tests", "all.rs")
    out: dict[str, str] = {}
    try:
        with open(all_rs, encoding="utf-8", errors="replace") as fh:
            lines = fh.read().splitlines()
    except OSError:
        return out
    pending: str | None = None
    for line in lines:
        found = _ALL_RS_PATH_RE.search(line)
        if found:
            pending = found.group(1)
            continue
        mod = _ALL_RS_MOD_RE.match(line)
        if mod and pending is not None:
            out[pending] = mod.group(1)
        if line.strip():
            pending = None
    return out


def _suite_term(root: str, rel: str, dir_of: dict[str, str] | None) -> tuple[str | None, str | None]:
    """`(term, problem)` — the nextest filterset term selecting `rel`'s tests.

    Two shapes, because the tree has two. A `tests/` suite is one `#[path]`
    module of its crate's single aggregated test binary, so the term names
    that binary and the module prefix. A `src/` file's tests are in the
    crate's LIB test binary, whose binary id is the package name alone, and
    the module prefix is the file's own module path.
    """
    parts = rel.split("/")
    crate_dir = parts[1]
    pkg = (dir_of or {}).get(crate_dir, crate_dir)
    if not _BINARY_ID_RE.match(pkg):
        return None, f"package name {pkg!r} cannot go into a filterset unquoted"
    if parts[2] == "tests":
        inner = "/".join(parts[3:])
        modules = _all_rs_modules(root, crate_dir)
        mod = modules.get(inner)
        if mod is None:
            return None, (
                f"crates/{crate_dir}/tests/all.rs declares no `#[path = \"{inner}\"]` "
                "module, so the test-id prefix this suite's tests carry is unknown"
            )
        return f"(binary_id({pkg}::all) & test(/^{mod}::/))", None
    # src/: the module path IS the file path. `mod.rs` names its directory.
    inner = parts[3:]
    if inner[-1] == "mod.rs":
        inner = inner[:-1]
    else:
        inner[-1] = inner[-1][: -len(".rs")]
    mod = "::".join(inner)
    if not mod or not _MODULE_PATH_RE.match(mod):
        return None, f"module path {mod!r} is not a plain Rust path"
    return f"(binary_id({pkg}) & test(/^{mod}::/))", None


def _scan_gated(root: str, dir_of: dict[str, str] | None = None) -> list[GatedSuite]:
    """Every marked file under `crates/`, in path order.

    Walks `crates/*/src` and `crates/*/tests` only: a marker anywhere else is
    not a suite this gate can name a binary for, and `gated-suite-paths.sh`
    reds the run on one.
    """
    out: list[GatedSuite] = []
    crates = os.path.join(root, "crates")
    for crate_dir in sorted(os.listdir(crates)):
        if crate_dir == "test-utils":
            continue
        for sub in ("src", "tests"):
            base = os.path.join(crates, crate_dir, sub)
            for dirpath, dirs, names in os.walk(base):
                dirs.sort()
                for name in sorted(names):
                    if not name.endswith(".rs"):
                        continue
                    full = os.path.join(dirpath, name)
                    rel = os.path.relpath(full, root).replace(os.sep, "/")
                    with open(full, encoding="utf-8", errors="replace") as fh:
                        text = fh.read()
                    if "gated_to!" not in text:
                        continue
                    paths = _marker_paths(text)
                    if paths is None:
                        continue
                    problem = None
                    if not paths:
                        problem = "the marker names no path at all"
                    for want in paths:
                        if want.startswith("/") or ".." in want.split("/"):
                            problem = f"{want!r} is not a repo-relative path"
                            break
                        target = os.path.join(root, want)
                        ok = os.path.isdir(target) if want.endswith("/") else os.path.isfile(target)
                        if not ok:
                            problem = f"{want!r} does not exist in the tree"
                            break
                    term, term_problem = _suite_term(root, rel, dir_of)
                    out.append(GatedSuite(rel, paths, term, problem or term_problem))
    return out


def gated_filter(
    root: str,
    files: list[str] | None,
    tier: str,
    dir_of: dict[str, str] | None = None,
) -> tuple[str, list[str]]:
    """`(TEST_FILTER, notices)` for this run.

    FAILS OPEN, ALWAYS TOWARD RUNNING, and the empty string is what that looks
    like: it is the ordinary whole-suite run, byte for byte what ran before
    this key existed. Every arm below returns it —
    tier `docs` (nothing runs at all), tier `all` (no diff to read, so nothing
    can be proven still), a diff with no file list, a diff touching the
    derivation's own inputs, and any exception anywhere in the scan.
    A suite whose own marker cannot be resolved fails open ALONE, so one
    broken marker cannot un-gate the rest.
    """
    if tier != "closure" or files is None:
        return "", []
    touched = _gate_fails_open(files)
    if touched is not None:
        return "", [
            f"gated suites: the whole gated set RUNS — this diff touches {touched}, "
            "which is an input to the gate's own derivation.\n"
            "  A change to the marker macro, to the fuzz harness, to this filter or to a "
            "crate's tests/all.rs can move what a gated suite DOES or which tests its term "
            "names, so the gate does not get to answer a question about itself."
        ]
    try:
        suites = _scan_gated(root, dir_of)
    except Exception as exc:  # noqa: BLE001 — fail OPEN, like every other arm here
        return "", [
            f"gated suites: the whole gated set RUNS — the marker scan failed ({exc}).\n"
            "  Nothing is excluded on a scan this script could not complete; "
            "scripts/gates/gated-suite-paths.sh is the row that reds for it."
        ]
    changed = set(files)
    notices: list[str] = []
    excluded: list[str] = []
    for suite in suites:
        if suite.selected_by(changed):
            continue
        if suite.problem is not None or suite.term is None:
            notices.append(
                f"gated: {suite.path} RUNS despite an untouched path set — {suite.problem}.\n"
                "  A marker this script cannot resolve never skips its suite. Fix the marker; "
                "scripts/gates/gated-suite-paths.sh reds the discipline row until it is fixed."
            )
            continue
        excluded.append(suite.term)
        shown = ", ".join(suite.paths) or "(no paths)"
        notices.append(f"gated: {suite.path} skipped — none of {shown} in the diff")
    if not excluded:
        return "", notices
    return "not (" + " | ".join(excluded) + ")", notices


def gated_set(root: str) -> int:
    """`--gated-set`: the union of EVERY gated suite's term, for the nightly.

    NOT the KEY=value stream — one filterset expression on stdout, the shape
    `nightly-only-selection.py` emits, because the caller passes it straight
    to `nextest -E`.

    AN EMPTY SET IS LEGITIMATE AND IS STILL NOT ACCEPTED BLINDLY, exactly as
    the demoted lane's is: a tree with no marker anywhere has no gated set,
    and `none()` is the honest answer. Markers PRESENT with nothing derivable
    is a broken rig — the shape that would report green having executed
    nothing, every night — and it exits 1.
    """
    try:
        dir_of, _ = _members(root)
    except Exception as exc:  # noqa: BLE001
        print(f"ci-filter: cargo metadata unavailable ({exc}); using directory names", file=sys.stderr)
        dir_of = None
    suites = _scan_gated(root, dir_of)
    if not suites:
        sys.stderr.write(
            "NOTE: no gated suites — not one file under crates/ carries a "
            "`test_utils::gated_to!` marker, so this tree HAS none and the nightly has "
            "nothing of this kind to re-take. Emitting `none()`.\n"
        )
        print("none()")
        return 0
    broken = [s for s in suites if s.problem is not None or s.term is None]
    if broken:
        for s in broken:
            sys.stderr.write(f"    {s.path}: {s.problem}\n")
        raise SystemExit(
            "error: {} of {} gated suite(s) could not be resolved to a nextest term. This lane "
            "exists to run the set the pull-request gate skipped, so emitting a filter that "
            "silently omits them would report green over exactly the suites nothing else "
            "runs. Fix the markers (scripts/gates/gated-suite-paths.sh names them "
            "individually).".format(len(broken), len(suites))
        )
    sys.stderr.write("gated suites: {} selected\n".format(len(suites)))
    for s in suites:
        sys.stderr.write("    {} -> {}\n".format(s.path, s.term))
    print(" | ".join(s.term for s in suites))
    return 0


def _sample(seed: str, salt: str, choices: tuple[str, ...]) -> str:
    """Deterministic choice from `choices`, keyed on (salt, seed).

    Salted per dimension so lane and eps are drawn independently — an
    unsalted second draw off the same seed would tie eps to lane and leave
    2 of the 6 matrix points unreachable forever.
    """
    digest = hashlib.sha256(f"{salt}\x00{seed}".encode()).digest()
    return choices[int.from_bytes(digest, "big") % len(choices)]


class ConfigError(Exception):
    """A configuration request that names no real point of the matrix."""


# THE DIMENSIONS A HUMAN CAN NAME: what they write -> (output key, legal
# values). The legal sets are NOT the sampled tuples: each is the sampled
# tuple plus whatever "every row of this dimension" is spelled as in the job
# conditions that read it — `both` for the lane, `all` for the k-lint row.
#
# EPS HAS NO SUCH MEMBER, and the asymmetry is real rather than an oversight.
# `EPS=all` is a LOCAL word: ci-local.sh loops the rows, while the hosted rows
# interpolate the value into CAD_TOLERANCE_EPS, where `all` is a parse error
# by design (geom-core/src/tolerance.rs). Requesting it hosted would ask for a
# run whose test rows cannot start, so it is not offered.
CONFIG_DIMENSIONS: dict[str, tuple[str, tuple[str, ...]]] = {
    "lane": ("LANE", (*LANES, "both")),
    "eps": ("EPS", EPS_ROWS),
    "klint": ("KLINT_ROW", (*KLINT_ROWS, "all")),
}

# THE COMMIT-MESSAGE SPELLING, deliberately shaped so it cannot happen by
# accident: a git trailer at the START of a line, which prose about this
# feature (indented, quoted, or mid-sentence) does not match. Case-insensitive
# on purpose — `CI-config:` is a typo, and a typo that reads as "no request"
# would put a sampled run in front of someone who asked for a chosen one, with
# nothing anywhere saying their line was ignored.
CONFIG_TRAILER = re.compile(r"^ci-config:[ \t]*(\S.*?)[ \t]*$", re.M | re.I)


def parse_config(tokens: list[str], source: str) -> dict[str, tuple[str, str]]:
    """`["lane=interval", ...]` -> `{"LANE": ("interval", source)}`, or raise.

    Raises rather than skipping: see the docstring's REQUEST section — an
    input error is the one failure here that must not fail open.
    """
    legal_keys = ", ".join(sorted(CONFIG_DIMENSIONS))
    out: dict[str, tuple[str, str]] = {}
    for token in tokens:
        key, sep, value = token.partition("=")
        if not sep or not key or not value:
            raise ConfigError(
                f"{source}: {token!r} is not `key=value` (keys: {legal_keys})"
            )
        if key not in CONFIG_DIMENSIONS:
            raise ConfigError(f"{source}: no configuration dimension {key!r} (keys: {legal_keys})")
        out_key, choices = CONFIG_DIMENSIONS[key]
        if value not in choices:
            raise ConfigError(
                f"{source}: {key}={value!r} is not one of {', '.join(choices)}"
            )
        if out_key in out:
            raise ConfigError(f"{source}: {key} named twice; say it once")
        out[out_key] = (value, source)
    return out


def config_from_message(message: str) -> dict[str, tuple[str, str]]:
    """The `CI-Config:` trailer(s) of one commit message, parsed like a flag.

    Several trailer lines are read as one request, so a repeated dimension is
    the same error across lines as within one.
    """
    tokens: list[str] = []
    for line in CONFIG_TRAILER.findall(message):
        tokens.extend(line.split())
    return parse_config(tokens, "commit-trailer")


def decorate(
    res: dict[str, str],
    files: list[str] | None = None,
    seed: str | None = None,
    config: dict[str, tuple[str, str]] | None = None,
) -> dict[str, str]:
    tier = res["TIER"]
    pkgs = set(p for p in res["PKGS"].split(",") if p)
    res["RUN_BUILD"] = "false" if tier == "docs" else "true"
    for key, roots in JOB_ROOTS.items():
        if tier == "docs":
            res[key] = "false"
        elif tier == "all":
            res[key] = "true"
        else:
            res[key] = "true" if pkgs & roots else "false"
    # interval-transcendentals is its OWN workspace, so no file under it can
    # appear in TIER=closure (any such change is TIER=all). Its job therefore
    # has nothing to verify in the closure tier.
    res["RUN_INTERVAL_BACKEND"] = "true" if tier == "all" else "false"
    res["RUN_INTERVAL_ORACLE"] = "true" if _touches_oracle(files) else "false"
    # k-lint has no minimal root set: it is the only job that compiles
    # demos/tour (a path-dependent of NINE members) and tools/k-lint, and its
    # probe sweep records predicate margins from every kernel crate. Any
    # member change can break it, so it runs whenever anything builds.
    res["RUN_K_LINT"] = "false" if tier == "docs" else "true"
    # THE VIEWER TOOLKIT AXIS — SEED-KEYED, NOT CLOSURE-KEYED (Evan,
    # 2026-08-27, ruling recorded in docs/GUI-LOG.md: "the GUI is treated as a
    # third-party consumer of the API").
    #
    # What it gates: the two rows that compile eframe + wgpu + naga + winit —
    # `clippy -p viewer --features app` and the rustdoc gate's `--all-features`
    # pass over `viewer`. Roughly 140 crates that no other row in this workflow
    # needs, and a permanent per-PR bill if every kernel change pays it.
    #
    # WHY SEEDS AND NOT THE CLOSURE, which is the whole substance of the
    # ruling. `viewer` sits downstream of `pncad`, which re-exports the entire
    # kernel — so `viewer` is in the dependent CLOSURE of nearly every kernel
    # change, and a closure-keyed test would be true almost always and would
    # gate nothing. The SEEDS are the members whose own files moved. `pncad` is
    # in every kernel change's closure and in almost none of their seeds, which
    # is exactly the difference that makes this axis mean something.
    #
    # WHY THESE THREE. `viewer` — its own code. `pncad` — the façade the
    # viewer's whole public-API path goes through, and the one crate whose own
    # source can break the app half without any kernel crate moving. `bvh` —
    # `Camera` speaks `bvh::Aabb` in its public signatures, the one direct
    # non-façade edge the crate has. A kernel crate that `pncad` merely
    # re-exports is deliberately NOT here: a breaking change to a re-exported
    # type still reaches viewer's DEFAULT-feature rows, which stay in the
    # ordinary closure below and put that breakage on the offending PR.
    #
    # WHAT THIS DOES NOT GATE, and the reason the ruling is affordable:
    # viewer's default-feature build and its headless suites — the camera,
    # input-mapping and scene rows, including the volume/winding tripwires —
    # ride the ordinary dependent closure like any other crate. This axis
    # skips the TOOLKIT only.
    #
    # THE COVERAGE THE SKIP GIVES UP is re-taken daily: nightly.yml runs the
    # app-feature clippy row ungated, so toolkit-dependency drift surfaces
    # within a day rather than at whichever unlucky PR next touches viewer.
    #
    # RECORDED, NEVER SILENT (the KLINT_ROW lesson): this is an output key, the
    # filter echoes it with the seeds it was computed from, and the workflow
    # prints the verdict in a step that always runs. A green job name over a
    # skipped step is the failure mode this shape exists to avoid.
    if tier == "docs":
        res["RUN_VIEWER_TOOLKIT"] = "false"
    elif tier == "all":
        # Unscopable: no seed information, so the axis fails OPEN like every
        # other signal here.
        res["RUN_VIEWER_TOOLKIT"] = "true"
    else:
        seeds = set(s for s in res.get("SEEDS", "").split(",") if s)
        res["RUN_VIEWER_TOOLKIT"] = "true" if seeds & VIEWER_TOOLKIT_SEEDS else "false"
    # Sampling is the LAST word and reads nothing above it: which point of
    # the matrix a run gates is independent of which rows the change filter
    # selected, and keeping the two apart is what lets the local gate consume
    # the same output while ignoring these two keys entirely.
    pins: dict[str, tuple[str, str] | None] = {"LANE": None, "KLINT_ROW": None}
    if seed is None:
        res["LANE"], res["EPS"], res["KLINT_ROW"] = "both", "all", "all"
    else:
        # The pin is held rather than re-derived: it decides the lane AND it is
        # what `CONFIG_SOURCE` reports below, and two calls could not disagree
        # only because `_forces_interval` happens to be pure today.
        pins["LANE"] = _forces_interval(files)
        res["LANE"] = (
            pins["LANE"][0] if pins["LANE"] is not None else _sample(seed, "lane", LANES)
        )
        res["EPS"] = _sample(seed, "eps", EPS_ROWS)
        # A THIRD SALT, drawn off the same seed and independent of the other
        # two. `_sample`'s docstring says why the salt is not optional: two
        # dimensions off one unsalted digest are the same number, which would
        # tie the k-lint row to the lane and leave 20 of the 30 points of this
        # matrix unreachable for the rest of the project's life.
        #
        # AND A PIN OVER IT, on the same terms as the lane's: held rather than
        # re-derived, because it decides the row AND it is what `CONFIG_SOURCE`
        # reports below.
        pins["KLINT_ROW"] = _forces_klint(files)
        res["KLINT_ROW"] = (
            pins["KLINT_ROW"][0] if pins["KLINT_ROW"] is not None
            else _sample(seed, "klint", KLINT_ROWS)
        )
    # THE REQUEST IS THE LAST WORD OF THE LAST WORD, and it is recorded in the
    # same breath. A run that gates a point nobody drew is only honest if the
    # output says so: CONFIG_SOURCE is per-dimension because the mixed case is
    # the common one — one dimension asked for, the other two still drawn.
    #
    # This deliberately also overrides `_forces_interval`'s pin. The pin
    # protects a SAMPLED run from skipping the lane the change is about;
    # someone typing `lane=default` over an interval change has answered that
    # question themselves, and CONFIG_SOURCE says `lane:requested` so the
    # answer is legible in the run rather than inferred from the tree.
    source = dict.fromkeys(
        (key for key, _ in CONFIG_DIMENSIONS.values()),
        "sampled" if seed is not None else "unsampled",
    )
    # A PIN IS NOT A DRAW, AND THE MACHINE-READABLE OUTPUT HAS TO SAY WHICH.
    # `LANE=interval` reads identically whether the seed chose it or
    # `_forces_interval` substituted it, so a reader answering "which
    # configuration gated this commit" off the outputs alone got `lane:sampled`
    # for a lane no sample ever touched — the same invisibility #1122 is about,
    # one level down from the stderr note. `pinned` is a SOURCE, not a value:
    # LANE is still `interval`, and every job condition reads LANE.
    # BOTH PINS ARE SOURCES, NOT VALUES, and both are overridden below by a
    # request that names their dimension — `klint=release-budget` over a
    # `tools/` diff is someone answering the pin's question themselves, and
    # `klint:requested` is how the run says so.
    for out_key, held in pins.items():
        if held is not None:
            source[out_key] = "pinned"
    for out_key, (value, src) in (config or {}).items():
        res[out_key] = value
        source[out_key] = src
    res["CONFIG_SOURCE"] = " ".join(
        f"{name}:{source[out_key]}" for name, (out_key, _) in CONFIG_DIMENSIONS.items()
    )
    # THE ADVISORY, AND WHY IT IS COMPUTED LAST. It fires only when this run is
    # NOT going to gate the interval lane, which is knowable only after the
    # pin, the draw and the request have all had their say — advising someone
    # to ask for a lane the run already gates is noise, and noise is how a real
    # notice stops being read. A BOOLEAN, not the reason: the reason is a path,
    # and a path has no business in a stream both halves parse as KEY=value.
    res["LANE_ADVISORY"] = (
        "true"
        if res["LANE"] == "default" and _advises_interval(files)
        else "false"
    )
    return res


# ---------------------------------------------------------------- self-test
#
# WHAT THIS IS AIMED AT. Every gate under `scripts/gates/` carries a
# `--selftest`, and `lib.sh` states the reason: a guard never shown to fire is
# not a guard. That sentence had never been applied one level up, to the script
# that decides whether any of those gates run at all.
#
# The fail-CLOSED direction is the cheap half to test and the less interesting
# one: `Bail` is caught in `main` and becomes TIER=all, so garbage runs
# everything. THE BRANCH THAT MATTERS IS `_is_docs`. It is taken before any of
# that, it is the one fail-OPEN path here, and when it is wrong the whole gate
# is skipped on a change that builds. So the battery below is weighted at it
# from both sides: change sets that MUST classify docs, and change sets that
# must NOT — a path one character off a docs prefix, a non-`.md` file under
# `docs/`, a `.md` beside a `.rs`, a rename, a deletion, an empty diff.
#
# THE FIXTURE IS A MINIATURE REPO and every case runs this script AS A
# SUBPROCESS, the way both halves invoke it. `--files` cases go through stdin;
# the `--base` cases run against a real git repo built in the fixture, because
# the rename and empty-diff shapes are properties of how the file list is
# OBTAINED and are invisible to a test that hands `classify` a list directly.
#
# The fixture ships a STUB `cargo` on PATH. The hosted job this runs in
# installs no toolchain at all (`mirror` is greps and stdlib python), so a
# self-test shelling out to the real cargo would be testing the runner image
# and would report TIER=all — the safe answer — for the wrong reason on every
# closure case. The stub also lets the closure cases state a dependency graph
# small enough to read.
_FIXTURE_PKGS = {
    # `stl` reaches `topo` by a DEV-dependency: `cargo test -p stl` builds it,
    # so the closure must propagate along that edge exactly like a normal one.
    "geom-core": [],
    "topo": [("geom-core", "normal")],
    "stl": [("topo", "dev")],
}


# A SECOND, SMALLER FIXTURE, for the seed-vs-closure axis only.
#
# It is separate from `_FIXTURE_PKGS` on purpose: adding `pncad` and `viewer`
# to that graph would move the expected closures of half the cases above, so
# the battery would be re-stating the fixture rather than the rule. Here the
# shape is the minimum that can tell the two questions apart —
#
#   viewer -> pncad -> topo      (and viewer -> bvh)
#
# so a `topo` change puts `pncad` and `viewer` in the CLOSURE while seeding
# neither, which is precisely the case a closure-keyed test would get wrong.
_VIEWER_FIXTURE_PKGS = {
    "topo": [],
    "bvh": [],
    "pncad": [("topo", "normal")],
    "viewer": [("pncad", "normal"), ("bvh", "normal")],
}


def _plant_viewer_fixture(t: str) -> str:
    """A minimal workspace exercising `RUN_VIEWER_TOOLKIT`'s seed keying."""
    import shutil

    for pkg in _VIEWER_FIXTURE_PKGS:
        os.makedirs(os.path.join(t, "crates", pkg, "src"), exist_ok=True)
        open(os.path.join(t, "crates", pkg, "Cargo.toml"), "w").close()
        open(os.path.join(t, "crates", pkg, "src", "lib.rs"), "w").close()
    os.makedirs(os.path.join(t, "scripts"), exist_ok=True)
    os.makedirs(os.path.join(t, "bin"), exist_ok=True)
    shutil.copy(os.path.abspath(__file__), os.path.join(t, "scripts", "ci-filter.py"))
    meta = {
        "packages": [
            {
                "name": pkg,
                "manifest_path": os.path.join(t, "crates", pkg, "Cargo.toml"),
                "dependencies": [{"name": d, "kind": k} for d, k in deps],
            }
            for pkg, deps in _VIEWER_FIXTURE_PKGS.items()
        ]
    }
    stub = os.path.join(t, "bin", "cargo")
    with open(stub, "w") as fh:
        fh.write("#!/bin/sh\n")
        fh.write('[ "$1" = metadata ] || { echo "stub cargo: $*" >&2; exit 1; }\n')
        fh.write("cat <<'JSON'\n" + json.dumps(meta) + "\nJSON\n")
    os.chmod(stub, 0o700)
    return t


def _plant_fixture(t: str) -> str:
    import shutil

    for pkg in _FIXTURE_PKGS:
        os.makedirs(os.path.join(t, "crates", pkg, "src"), exist_ok=True)
        open(os.path.join(t, "crates", pkg, "Cargo.toml"), "w").close()
        open(os.path.join(t, "crates", pkg, "src", "lib.rs"), "w").close()
    # Four consumed pages and one that is only prose. The docs tier must
    # separate them, and the four are spelled deliberately unlike each other:
    # every arm of the derivation is the only thing holding one of them out of
    # the docs tier, so narrowing any one arm drops a page and reds a case.
    #
    # SUBDIRECTORIES ARE THE LIVE SHAPE. Four of this repo's five real consumed
    # pages sit under `docs/guide/`, so the fixture puts consumed pages there
    # too: a battery that only ever planted them at `docs/` top level would
    # pass with `docs/guide/` treated as a documentation prefix.
    os.makedirs(os.path.join(t, "docs", "guide"), exist_ok=True)
    for page in ("GUIDE.md", "TOURPAGE.md", "PROSE.md"):
        with open(os.path.join(t, "docs", page), "w") as fh:
            fh.write("prose\n")
    for page in ("PYPAGE.md", "ASSET.md"):
        with open(os.path.join(t, "docs", "guide", page), "w") as fh:
            fh.write("prose\n")
    # `include_str!` from a workspace member, and `include_bytes!` from the
    # same one: the regex spans the family, not one spelling of it.
    with open(os.path.join(t, "crates", "topo", "src", "guide.rs"), "w") as fh:
        fh.write('#![doc = include_str!("../../../docs/GUIDE.md")]\n')
        fh.write('const A: &[u8] = include_bytes!("../../../docs/guide/ASSET.md");\n')
    # An EXCLUDED workspace compiles one in too. `demos/tour` is a real crate
    # with real doctests; the docs tier does not stop at the workspace edge.
    os.makedirs(os.path.join(t, "demos", "tour", "src"), exist_ok=True)
    with open(os.path.join(t, "demos", "tour", "src", "lib.rs"), "w") as fh:
        fh.write('#![doc = include_str!("../../../docs/TOURPAGE.md")]\n')
    os.makedirs(os.path.join(t, "crates", "stl", "tests"), exist_ok=True)
    with open(os.path.join(t, "crates", "stl", "tests", "test_pages.py"), "w") as fh:
        fh.write('PAGE = ROOT / "docs" / "guide" / "PYPAGE.md"\n')
    os.makedirs(os.path.join(t, "scripts"), exist_ok=True)
    os.makedirs(os.path.join(t, "bin"), exist_ok=True)
    shutil.copy(os.path.abspath(__file__), os.path.join(t, "scripts", "ci-filter.py"))
    meta = {
        "packages": [
            {
                "name": pkg,
                "manifest_path": os.path.join(t, "crates", pkg, "Cargo.toml"),
                "dependencies": [{"name": d, "kind": k} for d, k in deps],
            }
            for pkg, deps in _FIXTURE_PKGS.items()
        ]
    }
    stub = os.path.join(t, "bin", "cargo")
    with open(stub, "w") as fh:
        fh.write("#!/bin/sh\n")
        fh.write('[ "$1" = metadata ] || { echo "stub cargo: $*" >&2; exit 1; }\n')
        fh.write("cat <<'JSON'\n" + json.dumps(meta) + "\nJSON\n")
    # Owner-only. The stub is executed by this process out of a tempdir it
    # owns; nothing else needs to read it, let alone run it.
    os.chmod(stub, 0o700)
    return t


def _selftest_run(t: str, argv: list[str], stdin: str = ""):
    """One invocation of this script as a SUBPROCESS, both streams kept.

    Separate from `_selftest_invoke` because stdout and stderr carry
    different contracts here — stdout is the machine-readable KEY=value
    stream, stderr is what a human reads — and the pin battery is the one
    case that has to look at the second.
    """
    env = dict(os.environ)
    env["PATH"] = os.path.join(t, "bin") + os.pathsep + env.get("PATH", "")
    r = subprocess.run(
        [sys.executable, os.path.join(t, "scripts", "ci-filter.py"), *argv],
        input=stdin, capture_output=True, text=True, env=env, cwd=t,
    )
    if r.returncode != 0:
        raise SystemExit(f"SELFTEST FAILED: {argv} exited {r.returncode}\n{r.stdout}{r.stderr}")
    return r


def _selftest_invoke(t: str, argv: list[str], stdin: str = "") -> dict[str, str]:
    r = _selftest_run(t, argv, stdin)
    out: dict[str, str] = {}
    for line in r.stdout.splitlines():
        k, _, v = line.partition("=")
        out[k] = v
    for key in ("TIER", "PKGS", "RUN_BUILD", "RUN_K_LINT", "RUN_INTERVAL_ORACLE"):
        if key not in out:
            raise SystemExit(f"SELFTEST FAILED: {argv} printed no {key} line\n{r.stdout}{r.stderr}")
    return out


def _selftest_invoke_must_fail(t: str, argv: list[str], stdin: str = "") -> str:
    """The other half of `_selftest_invoke`: an invocation that must NOT be
    served. Returns stderr, so the caller can require the message to name the
    thing that was wrong — a nonzero exit that says nothing is a worse gate
    than the one that fails open, because it fails in front of someone who now
    has to guess what to type instead."""
    env = dict(os.environ)
    env["PATH"] = os.path.join(t, "bin") + os.pathsep + env.get("PATH", "")
    r = subprocess.run(
        [sys.executable, os.path.join(t, "scripts", "ci-filter.py"), *argv],
        input=stdin, capture_output=True, text=True, env=env, cwd=t,
    )
    if r.returncode == 0:
        raise SystemExit(
            f"SELFTEST FAILED: {argv} was served (exit 0) — a configuration request "
            f"that names no real point must red the step, not fall back to the draw\n{r.stdout}"
        )
    return r.stderr


def _expect(what: str, got: dict[str, str], want: dict[str, str]) -> None:
    bad = {k: (v, got.get(k)) for k, v in want.items() if got.get(k) != v}
    if bad:
        detail = "; ".join(f"{k}: want {w!r}, got {g!r}" for k, (w, g) in sorted(bad.items()))
        raise SystemExit(f"SELFTEST FAILED: {what} — {detail}\nfull output: {got}")


def _files_case(t: str, what: str, files: list[str], **want: str) -> None:
    _expect(what, _selftest_invoke(t, ["--files", "-"], "\n".join(files) + "\n"), want)


def _git(t: str, *args: str) -> None:
    env = dict(os.environ, GIT_AUTHOR_NAME="s", GIT_AUTHOR_EMAIL="s@e",
               GIT_COMMITTER_NAME="s", GIT_COMMITTER_EMAIL="s@e")
    subprocess.run(["git", "-C", t, *args], check=True, capture_output=True, env=env)


def selftest() -> None:
    import tempfile

    with tempfile.TemporaryDirectory() as t:
        _plant_fixture(t)

        # --- the docs branch, the direction it is ALLOWED to take.
        for what, files in (
            ("a design doc", ["docs/DESIGN.md"]),
            ("prose anywhere", ["README.md", "crates/topo/src/NOTES.md"]),
            ("the memories tree", ["memories/MEMORY.md", "memories/evan-profile.md"]),
            ("the local half", ["local-scripts/ci-local.sh"]),
            ("agent session config", [".claude/settings.json"]),
            # The other side of the two rows below: a page NOTHING consumes
            # stays in the docs tier. Widening `_is_docs`'s exception to all
            # of `docs/` would pass those and fail this.
            ("a page nothing consumes", ["docs/PROSE.md"]),
        ):
            _files_case(t, f"{what} must classify docs", files,
                        TIER="docs", PKGS="", RUN_BUILD="false", RUN_K_LINT="false",
                        RUN_INTERVAL_ORACLE="false")

        # --- THE FAIL-OPEN FAMILY. Each of these is a change set that BUILDS,
        # and a docs verdict on any of them skips every gate in the pipeline.
        for what, files, tier in (
            # One kernel source file in a change set of prose. `all()` is the
            # whole guard against this, and `all()` over a list nobody tests
            # is a claim.
            ("a .rs beside a .md", ["docs/DESIGN.md", "crates/topo/src/lib.rs"], "closure"),
            # NOT WORKSPACE-LEVEL, so these two are not in that allowlist —
            # they sit inside a member and must SCOPE. They are here because
            # the docs tier is about what a build reads, not about what a human
            # reads, and both of these are read by a build: this repo carries
            # `crates/topo/proptest-regressions/*.txt` (the seeds a proptest
            # replays) and `.step`/`.expect` pairs under `tests/fixtures/` (the
            # goldens a comparison asserts against). Treating either as prose
            # skips the suite whose input just changed.
            ("a proptest regression seed", ["crates/topo/proptest-regressions/seq.txt"], "closure"),
            ("a golden test fixture", ["crates/topo/tests/fixtures/cube.step"], "closure"),
            # `docs/` is not a docs PREFIX here and must not become one: the
            # k-lint job's committed input lives under it.
            ("a non-.md file under docs/", ["docs/k-report-data/margins.json"], "all"),
            # One character off a docs prefix. `startswith("local-scripts/")`
            # keeps the slash for exactly this reason.
            ("a near-miss on the local-scripts prefix", ["local-scriptsy/tool.rs"], "all"),
            ("a near-miss on the .claude prefix", [".claude-old/hook.sh"], "all"),
            # The self-referential case: a diff that edits THIS FILE. If it
            # ever classified docs, the run that could have caught the edit is
            # the run the edit skips.
            ("an edit to the filter itself", ["scripts/ci-filter.py"], "all"),
            ("a gate script", ["scripts/gates/lib.sh"], "all"),
            ("the hosted half", [".github/workflows/ci.yml"], "all"),
            ("the lockfile", ["Cargo.lock"], "all"),
            # A member manifest: feature unification has no per-crate scoping.
            ("a member manifest", ["crates/topo/Cargo.toml"], "all"),
            ("an unrecognised crate directory", ["crates/brand-new/src/lib.rs"], "all"),
            ("an unrecognised top-level file", ["deny.toml"], "all"),
            # THE REST OF THE ALLOWLIST COMMENT IN `classify`, one case each.
            # That comment enumerates what is workspace-level and therefore
            # unscopable; an entry named there with no case here is a rule
            # stated and not held, and widening `_is_docs` over it stays green.
            ("the workspace manifest", ["Cargo.toml"], "all"),
            ("the toolchain pin", ["rust-toolchain.toml"], "all"),
            ("cargo configuration", [".cargo/config.toml"], "all"),
            # `k-lint` is the only job that compiles these two, so a docs
            # verdict on either skips the only build that would have seen it.
            ("the excluded demos workspace", ["demos/tour/src/main.rs"], "all"),
            ("the excluded tools workspace", ["tools/k-lint/src/main.rs"], "all"),
            ("the excluded interval workspace", ["interval-transcendentals/src/lib.rs"], "all"),
            # A .md rustdoc COMPILES IN: every Rust block in it is a doctest,
            # so an edit to it can turn a build red. This is the live shape —
            # `crates/pncad/src/guide.rs` does exactly this to `docs/GUIDE.md`
            # and four pages under `docs/guide/`.
            ("a page compiled into rustdoc", ["docs/GUIDE.md"], "all"),
            # The same fact through the other two arms of the same derivation.
            # Without these, `_INCLUDE_RE` could narrow to `include_str!` alone
            # and `_RUST_TREES` could shrink to `("crates",)` with every case
            # green — two claims in the header that nothing checked.
            ("a page embedded as bytes", ["docs/guide/ASSET.md"], "all"),
            ("a page compiled into an excluded workspace", ["docs/TOURPAGE.md"], "all"),
            # And the other consumer: a page a python suite executes, IN A
            # SUBDIRECTORY, which is where the real ones live.
            ("a page a python suite reads", ["docs/guide/PYPAGE.md"], "all"),
        ):
            _files_case(t, f"{what} must NOT classify docs", files,
                        TIER=tier, RUN_BUILD="true", RUN_K_LINT="true")

        # --- an empty change set is UNRESOLVED, never "nothing changed".
        _expect("an empty change set must run everything",
                _selftest_invoke(t, ["--files", "-"], ""),
                {"TIER": "all", "RUN_BUILD": "true", "RUN_INTERVAL_ORACLE": "true"})

        # --- the dependent closure, including the dev-dependency edge.
        _files_case(t, "a leaf crate seeds its dependents", ["crates/geom-core/src/lib.rs"],
                    TIER="closure", PKGS="geom-core,stl,topo", RUN_STL="true",
                    CARGO_SCOPE="-p geom-core -p stl -p topo")
        _files_case(t, "a dependent crate does not seed its dependencies",
                    ["crates/stl/src/lib.rs"], TIER="closure", PKGS="stl",
                    RUN_STL="true", RUN_TOPO_RELEASE="false")

        # --- the oracle signal, which is keyed on PATHS and not on the tier.
        _files_case(t, "certified sources re-certify",
                    ["interval-transcendentals/src/pad.rs"], RUN_INTERVAL_ORACLE="true")
        _files_case(t, "the backend lockfile re-certifies",
                    ["interval-transcendentals/Cargo.lock"], RUN_INTERVAL_ORACLE="true")
        _files_case(t, "the derivation prose does not re-certify",
                    ["interval-transcendentals/docs/pads.md"], RUN_INTERVAL_ORACLE="false")
        _files_case(t, "a kernel change does not re-certify",
                    ["crates/topo/src/lib.rs"], RUN_INTERVAL_ORACLE="false")

        # --- THE `--base` CASES. Everything above hands the script a file
        # list; these make it derive one, which is where the rename shape
        # lives.
        _git(t, "init", "-q", ".")
        _git(t, "add", "-A")
        _git(t, "commit", "-qm", "base")

        os.makedirs(os.path.join(t, "docs"), exist_ok=True)
        with open(os.path.join(t, "docs", "PLAN.md"), "w") as fh:
            fh.write("prose\n")
        _git(t, "add", "-A")
        _git(t, "commit", "-qm", "prose")
        _expect("a real docs-only commit classifies docs",
                _selftest_invoke(t, ["--base", "HEAD~1"]), {"TIER": "docs"})

        # THE RENAME. `git diff --name-only` reports a rename as its
        # DESTINATION only, so a crate source moved to a .md arrives as one
        # docs path and the deletion is invisible — TIER=docs over a change
        # that empties a crate. `--no-renames` is what makes both sides
        # visible; delete that flag and this case goes red.
        _git(t, "mv", "crates/topo/src/lib.rs", "docs/moved.md")
        _git(t, "commit", "-qm", "rename out of a crate")
        _expect("a crate source renamed to a .md must not classify docs",
                _selftest_invoke(t, ["--base", "HEAD~1"]),
                {"TIER": "closure", "PKGS": "stl,topo", "RUN_BUILD": "true"})

        _git(t, "rm", "-q", "crates/geom-core/src/lib.rs")
        _git(t, "commit", "-qm", "delete")
        _expect("a deleted crate source is still a crate change",
                _selftest_invoke(t, ["--base", "HEAD~1"]),
                {"TIER": "closure", "PKGS": "geom-core,stl,topo"})

        _expect("a base that does not resolve runs everything",
                _selftest_invoke(t, ["--base", "0000000000000000000000000000000000000000"]),
                {"TIER": "all", "RUN_BUILD": "true", "RUN_INTERVAL_ORACLE": "true"})
        _expect("a base equal to HEAD is an empty diff, not a docs change",
                _selftest_invoke(t, ["--base", "HEAD"]),
                {"TIER": "all", "RUN_BUILD": "true"})

    # --- THE VIEWER TOOLKIT AXIS, on its own fixture (`_VIEWER_FIXTURE_PKGS`).
    #
    # The rule under test is SEED keying, and the only way to see it is a case
    # where the seeds and the closure disagree. Both directions are here: a
    # crate that seeds the axis, and a crate that reaches it only through the
    # closure. Without the second case a closure-keyed implementation passes
    # this battery, which would make the ruling unenforced.
    with tempfile.TemporaryDirectory() as t:
        _plant_viewer_fixture(t)
        _files_case(t, "viewer's own sources buy the toolkit rows",
                    ["crates/viewer/src/app.rs"],
                    TIER="closure", SEEDS="viewer", RUN_VIEWER_TOOLKIT="true")
        _files_case(t, "the facade's own sources buy them",
                    ["crates/pncad/src/lib.rs"],
                    TIER="closure", SEEDS="pncad", RUN_VIEWER_TOOLKIT="true")
        _files_case(t, "bvh's own sources buy them (Camera speaks bvh::Aabb)",
                    ["crates/bvh/src/aabb.rs"],
                    TIER="closure", SEEDS="bvh", RUN_VIEWER_TOOLKIT="true")
        # THE CASE THAT MATTERS. `topo` is under `pncad`, so `pncad` and
        # `viewer` are both in the closure — and neither is a seed. A
        # closure-keyed axis would say true here and gate nothing, which is
        # the whole reason the ruling says "seeds".
        _files_case(t, "a kernel crate reaching viewer only through the closure does NOT",
                    ["crates/topo/src/lib.rs"],
                    TIER="closure", PKGS="pncad,topo,viewer", SEEDS="topo",
                    RUN_VIEWER_TOOLKIT="false")
        # Fails OPEN with the rest of the filter: an unscopable change has no
        # seeds to read, and "no seeds" must not read as "no toolkit".
        _files_case(t, "an unscopable change runs the toolkit rows",
                    ["Cargo.toml"], TIER="all", SEEDS="", RUN_VIEWER_TOOLKIT="true")
        _files_case(t, "a docs-only change runs nothing, toolkit included",
                    ["README.md"], TIER="docs", SEEDS="", RUN_VIEWER_TOOLKIT="false")

    # An `include!` this reader cannot resolve could name a .md, so it takes
    # the whole change set to TIER=all rather than guessing. Its own fixture:
    # one unreadable include poisons every other verdict, which is the point.
    with tempfile.TemporaryDirectory() as t:
        _plant_fixture(t)
        with open(os.path.join(t, "crates", "topo", "src", "gen.rs"), "w") as fh:
            fh.write('const X: &str = include_str!(concat!(env!("OUT_DIR"), "/x.md"));\n')
        _expect("an include! that cannot be read must not leave the docs tier open",
                _selftest_invoke(t, ["--files", "-"], "docs/PROSE.md\n"),
                {"TIER": "all", "RUN_BUILD": "true"})

    # --- THE DERIVATION ITSELF FAILING TO PARSE, on the python side. Its own
    # fixture, for the same reason the `include!` case above has one: this is
    # supposed to poison every verdict, so it cannot share a tree with cases
    # that expect a clean derivation.
    #
    # A page named by any spelling the `/`-chain walk does not parse is the one
    # failure with no other tell — the suite still reads the page, the set
    # silently loses it, and the page falls into the docs tier where that suite
    # stops running. `_selftest_docs_premise` cannot catch it: "everything in
    # the set is non-docs" is true of a set that lost a member. The visible
    # `.md` literal is what is left to fail closed on.
    with tempfile.TemporaryDirectory() as t:
        _plant_fixture(t)
        with open(os.path.join(t, "crates", "stl", "tests", "test_pages.py"), "w") as fh:
            fh.write('PAGE = Path(__file__).resolve().parents[2] / "docs" / "guide" / "PYPAGE.md"\n')
        _expect("a page named by a spelling the scan cannot resolve must not leave the docs tier open",
                _selftest_invoke(t, ["--files", "-"], "docs/guide/PYPAGE.md\n"),
                {"TIER": "all", "RUN_BUILD": "true"})

    with tempfile.TemporaryDirectory() as t:
        _plant_fixture(t)
        _selftest_lane_pin(t)
        _selftest_klint_pin(t)
    # --- THE REQUEST PATH THROUGH THE CLI. `_selftest_config` covers the
    # applier as a function; what only a subprocess can show is the wiring —
    # that the flags reach it, that a bad request exits NONZERO rather than
    # printing a fallback, and that `--force-all` returns a tier without
    # touching a diff. All three are what ci.yml actually invokes.
    with tempfile.TemporaryDirectory() as t:
        _plant_fixture(t)
        _expect("a requested point must reach the output through the flag",
                _selftest_invoke(t, ["--files", "-", "--seed", "deadbeef",
                                     "--config", "lane=interval", "eps=1e-12"],
                                 "crates/geom-core/src/lib.rs\n"),
                {"LANE": "interval", "EPS": "1e-12",
                 "CONFIG_SOURCE": "lane:requested eps:requested klint:sampled"})
        with open(os.path.join(t, "msg.txt"), "w") as fh:
            fh.write("topo: a commit\n\nCI-Config: lane=both\n")
        _expect("a requested point must reach the output through the commit trailer",
                _selftest_invoke(t, ["--files", "-", "--seed", "deadbeef",
                                     "--config-from-message", "msg.txt"],
                                 "crates/geom-core/src/lib.rs\n"),
                {"LANE": "both", "CONFIG_SOURCE": "lane:commit-trailer eps:sampled klint:sampled"})
        err = _selftest_invoke_must_fail(
            t, ["--files", "-", "--seed", "deadbeef", "--config", "eps=1e-13"],
            "crates/geom-core/src/lib.rs\n")
        if "1e-13" not in err:
            raise SystemExit(f"SELFTEST FAILED: the refusal must name the value refused: {err!r}")
        _expect("--force-all must return the all tier with no diff taken",
                _selftest_invoke(t, ["--force-all", "--seed", "deadbeef"]),
                {"TIER": "all", "RUN_BUILD": "true", "RUN_INTERVAL_ORACLE": "true",
                 "LANE": "interval"})

    _selftest_docs_premise()
    _selftest_klint_premise()
    _selftest_klint_workflow()
    _selftest_sampling()
    _selftest_config()
    _selftest_gated()
    print(
        "ci-filter selftest OK: the docs tier is reached by prose, memories/, "
        "local-scripts/ and .claude/ and by nothing else here — not a .rs beside a .md, "
        "not a non-.md file under docs/, not a path one character off a docs prefix, "
        "not an edit to this script, a gate, the workflow, the lockfile or a member "
        "manifest, not an unrecognised crate directory or top-level file, not the "
        "workspace manifest, toolchain pin or cargo config, not an excluded workspace, "
        "not a proptest seed or a golden fixture, not an empty "
        "diff, not a crate source renamed to a .md, and not a page that rustdoc compiles "
        "in — by any include! spelling, from any rust tree — or that a python suite "
        "executes, including one named by a spelling the scan cannot resolve; "
        "the closure follows dev-dependency "
        "edges upward only; the oracle signal fires on certified sources and lockfile and "
        "not on their prose; the three sampled dimensions fail open with no seed, "
        "repeat under the same seed, and are drawn independently enough that every one "
        "of the 30 matrix points is reachable; the interval-transcendentals/ lane pin "
        "beats a draw that went the other way, says so on stderr naming the file that "
        "pinned it, is recorded as `lane:pinned` in CONFIG_SOURCE so the outputs alone "
        "tell a pin from a draw, keeps the reason off stdout, stays out of unpinned and "
        "unseeded runs, and goes quiet in both channels when a request overrides it, "
        "while a merely interval-NAMED file draws its lane like anything else and "
        "raises LANE_ADVISORY with the spelling of the request instead — naming every "
        "such file rather than the first, saying whether the lane was drawn or "
        "requested, and staying silent on a run already gating interval, on an "
        "unseeded run, and on a diff naming no such file; the tools/ k-lint-row pin "
        "substitutes the row DERIVED as RUNNING THE SUITE of what changed — every "
        "member of every pinned root has such an entry, every entry names a real "
        "directory and a real row, and the derivation is re-run against ci.yml itself, "
        "where each mapped row must still hold a `cargo test` step for its crate, the "
        "job's own `if:` lists must name exactly KLINT_ROWS, and the fallback row must "
        "still be gated on the most steps — announces itself as `klint:pinned` and on "
        "stderr naming the file, falls "
        "back to the most-testing row rather than to the draw on an unmapped tools/ "
        "path and says that it did, fails closed into every row when the change set "
        "cannot be resolved, yields to a requested row from either spelling, and "
        "leaves demos/ and every ordinary diff to DRAW; --notices carries all three "
        "notices to a relay file and is truncated when there is none; and a "
        "configuration REQUESTED by hand "
        "— by flag or by `CI-Config:` commit trailer — reaches the dimension it names "
        "and only that one, beats the interval pin, is recorded in CONFIG_SOURCE, and "
        "reds the step rather than falling back to the draw when it names no real point; "
        "and the per-file test gate excludes a gated suite whose named paths and own file "
        "are all untouched — reading the module prefix out of the crate\'s tests/all.rs "
        "rather than off the filename, and the src/ shape off the module path — while "
        "running it on any of them, on tier all, on tier docs, on a change to the "
        "test-utils harness or to a tests/all.rs, and on a marker whose path is not in "
        "the tree, which is named in a notice and does not un-gate its healthy "
        "siblings; --gated-set prints every marked suite for the nightly, `none()` for a "
        "tree that has none, and reds rather than a short filter when a marker cannot be "
        "resolved"
    )


# A THIRD FIXTURE, for the per-file test gate. Separate from the two above for
# the reason the viewer one is separate from the docs one: this one needs a
# `tests/all.rs`, a `#[path]` module whose name is NOT its filename, a `src/`
# file whose module path is its file path, and a marker that names a directory
# — none of which the other fixtures have any use for, and all of which would
# move the closures those cases assert.
_GATED_FIXTURE_PKGS = {
    "geom-core": [],
    "stl": [("geom-core", "normal")],
    "topo": [("geom-core", "normal")],
}

# The two terms the fixture's healthy markers derive to, written out here
# rather than rebuilt by the cases: a case that computed its own expectation
# from the same rule the code uses would assert that the rule is applied
# consistently, not that it is right.
_GATED_TERM_RING = "(binary_id(geom-core::all) & test(/^ring_fuzz::/))"
_GATED_TERM_PROBE = "(binary_id(topo) & test(/^review_probe::/))"


def _plant_gated_fixture(t: str) -> str:
    """A miniature workspace carrying three markers: a healthy `tests/` suite,
    a healthy `src/` one, and one naming a path that is not there."""
    import shutil

    for pkg in _GATED_FIXTURE_PKGS:
        os.makedirs(os.path.join(t, "crates", pkg, "src"), exist_ok=True)
        os.makedirs(os.path.join(t, "crates", pkg, "tests"), exist_ok=True)
        open(os.path.join(t, "crates", pkg, "Cargo.toml"), "w").close()
        open(os.path.join(t, "crates", pkg, "src", "lib.rs"), "w").close()
    # The code the gated suites are about.
    open(os.path.join(t, "crates", "geom-core", "src", "ring.rs"), "w").close()
    os.makedirs(os.path.join(t, "crates", "geom-core", "src", "interval"), exist_ok=True)
    open(os.path.join(t, "crates", "geom-core", "src", "interval", "scalar.rs"), "w").close()
    open(os.path.join(t, "crates", "topo", "src", "euler.rs"), "w").close()
    # THE MODULE NAME IS NOT THE FILENAME, deliberately: `geom`'s real
    # `tests/all.rs` includes `curves/lt_r1_probes.rs` as
    # `curves_lt_r1_probes`, so a fixture whose two agreed would pass with the
    # `#[path]` pair unread.
    with open(os.path.join(t, "crates", "geom-core", "tests", "all.rs"), "w") as fh:
        fh.write('#[path = "ring_fuzz.rs"]\nmod ring_fuzz;\n')
        fh.write('#[path = "sub/orphan_fuzz.rs"]\nmod sub_orphan_fuzz;\n')
    with open(os.path.join(t, "crates", "geom-core", "tests", "ring_fuzz.rs"), "w") as fh:
        fh.write(
            'test_utils::gated_to![\n'
            '    "crates/geom-core/src/ring.rs",\n'
            '    "crates/geom-core/src/interval/",\n'
            '];\n'
        )
    # THE MARKER THAT CANNOT RESOLVE, and it is a live shape rather than an
    # invented one: a suite whose subject was renamed away under it.
    os.makedirs(os.path.join(t, "crates", "geom-core", "tests", "sub"), exist_ok=True)
    with open(os.path.join(t, "crates", "geom-core", "tests", "sub", "orphan_fuzz.rs"), "w") as fh:
        fh.write('test_utils::gated_to!["crates/geom-core/src/renamed_away.rs"];\n')
    with open(os.path.join(t, "crates", "topo", "src", "review_probe.rs"), "w") as fh:
        fh.write('test_utils::gated_to!["crates/topo/src/euler.rs"];\n')
    os.makedirs(os.path.join(t, "crates", "test-utils", "src"), exist_ok=True)
    open(os.path.join(t, "crates", "test-utils", "Cargo.toml"), "w").close()
    with open(os.path.join(t, "crates", "test-utils", "src", "lib.rs"), "w") as fh:
        fh.write("// the marker's home; never scanned\n")
    os.makedirs(os.path.join(t, "scripts"), exist_ok=True)
    os.makedirs(os.path.join(t, "bin"), exist_ok=True)
    shutil.copy(os.path.abspath(__file__), os.path.join(t, "scripts", "ci-filter.py"))
    meta = {
        "packages": [
            {
                "name": pkg,
                "manifest_path": os.path.join(t, "crates", pkg, "Cargo.toml"),
                "dependencies": [{"name": d, "kind": k} for d, k in deps],
            }
            for pkg, deps in _GATED_FIXTURE_PKGS.items()
        ]
    }
    stub = os.path.join(t, "bin", "cargo")
    with open(stub, "w") as fh:
        fh.write("#!/bin/sh\n")
        fh.write('[ "$1" = metadata ] || { echo "stub cargo: $*" >&2; exit 1; }\n')
        fh.write("cat <<'JSON'\n" + json.dumps(meta) + "\nJSON\n")
    os.chmod(stub, 0o700)
    return t


def _selftest_gated() -> None:
    """The per-file test gate, from both directions.

    WEIGHTED AT THE DIRECTION THAT LOSES COVERAGE, the way the docs battery
    above is. Emitting an expression that excludes a suite the diff SHOULD
    have run is the failure that ships a break; emitting nothing is the
    ordinary whole-suite run and costs only minutes. So every case that must
    RUN a suite asserts the term is absent, and the one case that skips
    asserts the exact expression and the notice a reader will look for.
    """
    import tempfile

    with tempfile.TemporaryDirectory() as t:
        _plant_gated_fixture(t)

        def case(what: str, files: list[str], **want: str) -> None:
            _expect(
                what,
                _selftest_invoke(t, ["--files", "-"], "\n".join(files) + "\n"),
                want,
            )

        # UNTOUCHED: both healthy suites are excluded, the unresolvable one is
        # not. Full-string, because the shape of the expression is the
        # contract with nextest and a substring test would pass on a filter
        # that had lost its `not`.
        case(
            "a change touching neither suite's paths",
            ["crates/stl/src/lib.rs"],
            TIER="closure",
            TEST_FILTER=f"not ({_GATED_TERM_RING} | {_GATED_TERM_PROBE})",
        )
        # A NAMED FILE moved: that suite runs, the other stays skipped.
        case(
            "a named file in the diff",
            ["crates/geom-core/src/ring.rs"],
            TEST_FILTER=f"not ({_GATED_TERM_PROBE})",
        )
        # A NAMED DIRECTORY's descendant moved. `crates/geom-core/src/interval/`
        # means anything under it, at any depth.
        case(
            "a named directory's descendant in the diff",
            ["crates/geom-core/src/interval/scalar.rs"],
            TEST_FILTER=f"not ({_GATED_TERM_PROBE})",
        )
        # THE SUITE'S OWN FILE is an implicit member of its own path set —
        # editing a fuzzer is the one change certain to be about it.
        case(
            "the suite's own file in the diff",
            ["crates/geom-core/tests/ring_fuzz.rs"],
            TEST_FILTER=f"not ({_GATED_TERM_PROBE})",
        )
        # The `src/`-module shape, from its own side.
        case(
            "a src/ marker's named file in the diff",
            ["crates/topo/src/euler.rs"],
            TEST_FILTER=f"not ({_GATED_TERM_RING})",
        )
        # TIER=all — no diff that can prove anything held still.
        case(
            "an unscopable change",
            ["Cargo.lock"],
            TIER="all",
            TEST_FILTER="",
        )
        # THE DERIVATION'S OWN INPUTS. A test-utils change can move what every
        # gated suite DOES; a tests/all.rs change can move which tests a term
        # NAMES, and a term that names nothing excludes nothing, silently.
        case(
            "a test-utils change",
            ["crates/test-utils/src/fuzz.rs"],
            TEST_FILTER="",
        )
        case(
            "a tests/all.rs change",
            ["crates/geom-core/tests/all.rs", "crates/stl/src/lib.rs"],
            TEST_FILTER="",
        )
        # A DOCS-TIER RUN RUNS NO TESTS AT ALL, so the key is empty rather
        # than an expression nothing will consume.
        case("a docs-tier change", ["README.md"], TIER="docs", TEST_FILTER="")

        # THE UNRESOLVABLE MARKER, and both halves of what it must do: never
        # skip its own suite, and say why in the notices a reader of the run
        # is handed. The healthy sibling is still skipped in the same run —
        # one broken marker does not un-gate the tree.
        run = _selftest_run(
            t, ["--files", "-"], "crates/stl/src/lib.rs\n"
        )
        for want in (
            "crates/geom-core/tests/sub/orphan_fuzz.rs RUNS despite an untouched path set",
            "'crates/geom-core/src/renamed_away.rs' does not exist in the tree",
            "gated: crates/topo/src/review_probe.rs skipped — none of "
            "crates/topo/src/euler.rs in the diff",
        ):
            if want not in run.stderr:
                raise SystemExit(
                    f"SELFTEST FAILED: the gate's notices did not carry {want!r}\n{run.stderr}"
                )

        # --gated-set: EVERY marked suite, and a tree whose markers cannot all
        # be resolved is a RED step rather than a short filter — the nightly
        # runs only what this prints.
        broken = _selftest_invoke_must_fail(t, ["--gated-set"])
        if "could not be resolved to a nextest term" not in broken:
            raise SystemExit(f"SELFTEST FAILED: --gated-set was not loud about a broken marker\n{broken}")
        os.remove(os.path.join(t, "crates", "geom-core", "tests", "sub", "orphan_fuzz.rs"))
        run = _selftest_run(t, ["--gated-set"])
        want_set = f"{_GATED_TERM_RING} | {_GATED_TERM_PROBE}"
        if run.stdout.strip() != want_set:
            raise SystemExit(
                f"SELFTEST FAILED: --gated-set printed {run.stdout.strip()!r}, wanted {want_set!r}"
            )

    # AND THE EMPTY TREE, which is legitimate and is still not accepted
    # blindly — the same distinction `nightly-only-selection.py` draws. Here
    # `none()` is provable from the source: no marker anywhere under crates/.
    with tempfile.TemporaryDirectory() as t:
        _plant_fixture(t)
        run = _selftest_run(t, ["--gated-set"])
        if run.stdout.strip() != "none()":
            raise SystemExit(
                f"SELFTEST FAILED: --gated-set on a marker-free tree printed "
                f"{run.stdout.strip()!r}, wanted 'none()'"
            )
        if "no gated suites" not in run.stderr:
            raise SystemExit(
                f"SELFTEST FAILED: --gated-set said nothing about the empty case\n{run.stderr}"
            )


def _selftest_sampling() -> None:
    """THE THREE DRAWS, and the one property that cannot be seen by reading a
    single run's output: INDEPENDENCE.

    A wrong salt here does not fail loudly. Every run still prints a lane, an
    ε and a k-lint row, every one of them is a legal value, and the gate goes
    green for as long as anyone cares to look — but the same digest feeding
    two dimensions makes them one dimension, and the points that pairing
    excludes are then unreachable FOREVER rather than rare. That is the exact
    shape of failure the sampling premise cannot survive: "repetition covers
    the matrix" is false about a point no seed can draw.

    So this walks synthetic seeds and requires the whole product to appear.
    It is deterministic — the seeds are counted, not random — so it cannot
    flake, and it is in-process because `decorate` is a pure function of
    (result, files, seed) and a subprocess would only be slower.
    """
    # A file list that does not PIN the lane: `_forces_interval` is a floor
    # over the sampling and would make every draw here `interval`.
    files = ["crates/geom-core/src/lib.rs"]
    base = {"TIER": "closure", "PKGS": "geom-core", "CARGO_SCOPE": "-p geom-core"}

    got = decorate(dict(base), files, None)
    for key, want in (("LANE", "both"), ("EPS", "all"), ("KLINT_ROW", "all")):
        if got[key] != want:
            raise SystemExit(f"SELFTEST FAILED: no seed must fail open into more work — "
                             f"{key} is {got[key]!r}, want {want!r}")

    one = decorate(dict(base), files, "deadbeef")
    two = decorate(dict(base), files, "deadbeef")
    if (one["LANE"], one["EPS"], one["KLINT_ROW"]) != (two["LANE"], two["EPS"], two["KLINT_ROW"]):
        raise SystemExit("SELFTEST FAILED: the same seed drew two different points — the draw is "
                         "not a function of the SHA, so a re-run of a red gate can come back green")

    seen: set[tuple[str, str, str]] = set()
    for i in range(4000):
        d = decorate(dict(base), files, f"{i:040x}")
        seen.add((d["LANE"], d["EPS"], d["KLINT_ROW"]))
    want_points = {(ln, e, k) for ln in LANES for e in EPS_ROWS for k in KLINT_ROWS}
    missing = want_points - seen
    if missing:
        raise SystemExit(
            "SELFTEST FAILED: over 4000 seeds these matrix points were never drawn: "
            f"{sorted(missing)}. Two dimensions sharing a salt (or one drawn without "
            "one) collapse into a single number and make part of the product "
            "unreachable — check `_sample`'s salt argument at every call site in "
            "`decorate`")


def _selftest_lane_pin(t: str) -> None:
    """THE PIN THAT REMAINS, THE ONE THAT WAS REMOVED, AND THE ADVICE IN ITS PLACE.

    `_forces_interval` runs BEFORE the seeded draw and short-circuits it, so a
    branch that trips it is on the interval lane for every push it ever makes.
    For `interval-transcendentals/` that is right — the tree is the backend's
    own workspace. For a BASENAME it was not, and Evan's ruling on #1122
    removed that arm: it gated a type migration's whole branch on the wrong
    axis because the rename touched an interval-named test file.

    So the cases below fix BOTH directions of that ruling, because only one of
    them is testable by the code that replaced it: that the exact arm still
    pins and still says so, and that a basename now DRAWS and merely ADVISES.
    Without the second case, restoring the deleted arm would pass this file.
    """
    # The seed is FOUND, not hardcoded: the pinned case only tests the pin if
    # the draw it overrode was `default`, and a literal SHA here would stop
    # being that the moment `LANES` or the salt moved.
    seed = next(
        s for s in (f"{i:040x}" for i in range(1000))
        if _sample(s, "lane", LANES) == "default"
    )

    pinned = _selftest_run(t, ["--files", "-", "--seed", seed],
                           "interval-transcendentals/src/lib.rs\n")
    if "LANE=interval" not in pinned.stdout.splitlines():
        raise SystemExit("SELFTEST FAILED: a change under interval-transcendentals/ did not pin "
                         f"the lane\n{pinned.stdout}")
    if "PINNED" not in pinned.stderr or "interval-transcendentals/src/lib.rs" not in pinned.stderr:
        raise SystemExit("SELFTEST FAILED: the lane was pinned and the run did not say so, or "
                         f"did not name the file that pinned it\nstderr: {pinned.stderr!r}")
    if "PINNED" in pinned.stdout:
        raise SystemExit("SELFTEST FAILED: the pin note reached STDOUT, where both halves read "
                         f"KEY=value lines\n{pinned.stdout}")
    if "CONFIG_SOURCE=lane:pinned eps:sampled klint:sampled" not in pinned.stdout.splitlines():
        raise SystemExit("SELFTEST FAILED: a pinned lane was recorded as something other than "
                         f"`lane:pinned` — the outputs cannot tell a pin from a draw\n{pinned.stdout}")

    # THE REMOVED ARM, ASSERTED AS REMOVED. `ring_interval.rs` is the shape the
    # old rule matched — an interval-named source, not a rename victim — so if
    # anything ever pins on a basename again, it pins here.
    advised = _selftest_run(t, ["--files", "-", "--seed", seed],
                            "crates/topo/src/ring_interval.rs\n")
    if "LANE=default" not in advised.stdout.splitlines():
        raise SystemExit("SELFTEST FAILED: a basename carrying `interval` pinned the lane — that "
                         f"arm was REMOVED by the #1122 ruling; the lane is asked for\n{advised.stdout}")
    if "lane:pinned" in advised.stdout or "PINNED" in advised.stderr:
        raise SystemExit("SELFTEST FAILED: a basename-only match was announced as a pin\n"
                         f"{advised.stdout}\nstderr: {advised.stderr!r}")
    if "LANE_ADVISORY=true" not in advised.stdout.splitlines():
        raise SystemExit("SELFTEST FAILED: a diff touching *interval* files under a default lane "
                         f"raised no advisory — the ruling replaced the pin with one\n{advised.stdout}")
    if "CI-Config: lane=interval" not in advised.stderr:
        raise SystemExit("SELFTEST FAILED: the advisory did not say HOW to ask for the lane, which "
                         f"is the whole of the convention it points at\nstderr: {advised.stderr!r}")

    # THE ADVISORY GOES QUIET WHERE IT WOULD BE NOISE — a run already gating
    # `interval` needs no advice to ask for it, and an advisory that fires on
    # runs it has nothing to say to is one nobody reads on the run it does.
    interval_seed = next(
        s for s in (f"{i:040x}" for i in range(1000))
        if _sample(s, "lane", LANES) == "interval"
    )
    quiet = _selftest_run(t, ["--files", "-", "--seed", interval_seed],
                          "crates/topo/src/ring_interval.rs\n")
    if "LANE_ADVISORY=false" not in quiet.stdout.splitlines():
        raise SystemExit("SELFTEST FAILED: the advisory fired on a run already gating the interval "
                         f"lane\n{quiet.stdout}")

    drawn = _selftest_run(t, ["--files", "-", "--seed", seed],
                          "crates/topo/src/lib.rs\n")
    if "LANE=default" not in drawn.stdout.splitlines():
        raise SystemExit("SELFTEST FAILED: an ordinary basename did not fall through to the "
                         f"draw\n{drawn.stdout}")
    if "PINNED" in drawn.stderr:
        raise SystemExit("SELFTEST FAILED: an unpinned run announced a pin — the note would then "
                         f"say nothing\nstderr: {drawn.stderr!r}")
    if "lane:pinned" in drawn.stdout:
        raise SystemExit("SELFTEST FAILED: a drawn lane was recorded as pinned; `lane:pinned` then "
                         f"says nothing about any run\n{drawn.stdout}")
    if "LANE_ADVISORY=false" not in drawn.stdout.splitlines():
        raise SystemExit("SELFTEST FAILED: an advisory fired on a diff with no interval-named file "
                         f"at all\n{drawn.stdout}")

    # A REQUEST OVERRIDES THE PIN, so the note must go quiet. `decorate` lets a
    # requested lane beat `_forces_interval` deliberately; if the announcement
    # did not know that, a run gating `default` by request would print that it
    # was pinned to `interval` — naming a lane the run is not on. This case is
    # the seam between the pin and the request path, and neither one's own
    # cases cover it.
    overridden = _selftest_run(
        t, ["--files", "-", "--seed", seed, "--config", "lane=default"],
        "interval-transcendentals/src/lib.rs\n")
    if "LANE=default" not in overridden.stdout.splitlines():
        raise SystemExit("SELFTEST FAILED: a requested lane did not beat the pin\n"
                         f"{overridden.stdout}")
    if "PINNED" in overridden.stderr:
        raise SystemExit("SELFTEST FAILED: the pin note fired over a lane the request "
                         f"overrode\nstderr: {overridden.stderr!r}")
    if "lane:requested" not in overridden.stdout:
        raise SystemExit("SELFTEST FAILED: a request that beat the pin was recorded as the pin; "
                         f"the run would credit its lane to a file nobody chose\n{overridden.stdout}")

    # THE RELAY FILE, which is the only reason ci.yml no longer restates these
    # notices in its own prose. Two properties, and the second is the one a
    # reader would never think to check: the file CARRIES the notice, and it is
    # TRUNCATED when there is none — a relay that leaves yesterday's pin in
    # place announces a pin the run does not have, and the consumer `cat`s it
    # unconditionally.
    notes = os.path.join(t, "notices.txt")
    _selftest_run(t, ["--files", "-", "--seed", seed, "--notices", notes],
                  "interval-transcendentals/src/lib.rs\n")
    with open(notes) as fh:
        relayed = fh.read()
    if "PINNED" not in relayed or "interval-transcendentals/src/lib.rs" not in relayed:
        raise SystemExit("SELFTEST FAILED: --notices did not carry the pin's reason, so ci.yml's "
                         f"relay would print nothing where it used to print prose\n{relayed!r}")
    _selftest_run(t, ["--files", "-", "--seed", seed, "--notices", notes],
                  "crates/topo/src/lib.rs\n")
    with open(notes) as fh:
        if fh.read() != "":
            raise SystemExit("SELFTEST FAILED: --notices was not truncated on a run with no "
                             "notice — the relay would announce the PREVIOUS run's pin")

    # EVERY interval-named file, not the first, and the word for how the lane
    # was arrived at. Both are things the relay cannot re-derive, which is why
    # the wording moved into this script.
    many = _selftest_run(
        t, ["--files", "-", "--seed", seed, "--notices", notes],
        "crates/topo/src/ring_interval.rs\ncrates/sweep/tests/extrude_interval.rs\n")
    if ("ring_interval.rs" not in many.stderr or "extrude_interval.rs" not in many.stderr
            or "2 file(s)" not in many.stderr):
        raise SystemExit("SELFTEST FAILED: the advisory named fewer than all the interval files "
                         f"it matched\nstderr: {many.stderr!r}")
    if "LANE=default (drawn)" not in many.stderr:
        raise SystemExit("SELFTEST FAILED: the advisory did not say the lane was DRAWN\n"
                         f"stderr: {many.stderr!r}")
    asked = _selftest_run(
        t, ["--files", "-", "--seed", interval_seed, "--config", "lane=default"],
        "crates/topo/src/ring_interval.rs\n")
    if "LANE=default (REQUESTED)" not in asked.stderr:
        raise SystemExit("SELFTEST FAILED: the advisory called a REQUESTED lane drawn — the run "
                         f"would credit a choice to a die nobody rolled\nstderr: {asked.stderr!r}")

    # No seed: nothing is drawn, so there is nothing to pin OR to advise.
    # LANE=both already runs both compile modes.
    unseeded = _selftest_run(t, ["--files", "-"], "interval-transcendentals/src/lib.rs\n")
    if ("LANE=both" not in unseeded.stdout.splitlines() or "PINNED" in unseeded.stderr
            or "lane:pinned" in unseeded.stdout
            or "LANE_ADVISORY=false" not in unseeded.stdout.splitlines()):
        raise SystemExit("SELFTEST FAILED: an unseeded run must be LANE=both and announce neither "
                         f"a pin nor advice\n{unseeded.stdout}\nstderr: {unseeded.stderr!r}")


def _selftest_klint_pin(t: str) -> None:
    """THE `tools/` PIN ON THE K-LINT ROW, and the scope it deliberately stops at.

    THE CASE THIS EXISTS FOR IS THE ONE THAT LOOKS GREEN. Delete
    `_forces_klint`'s call site and every run still prints a legal
    `KLINT_ROW=`, every job condition still reads it, and the gate is green for
    as long as anyone looks — while a `tools/` change is back to being gated by
    whichever row a hash picked, which is the case this pin exists for. So the
    first case below is `decorate` restoring the DRAW over a `tools/` diff, and
    it must red.

    AND THE SCOPE IS TESTED FROM BOTH SIDES, because a pin that quietly grew is
    the #1122 failure one dimension over: `demos/` is required to DRAW, so
    widening the prefix to the other excluded workspace cannot pass this file.
    """
    # SEEDS FOUND, NOT HARDCODED. The pinned case only tests the pin if the
    # draw it overrode went somewhere else, and a literal SHA stops being that
    # the moment `KLINT_ROWS` or the salt moves.
    seed = next(
        s for s in (f"{i:040x}" for i in range(1000))
        if _sample(s, "klint", KLINT_ROWS) != "dev-default"
    )

    pinned = _selftest_run(t, ["--files", "-", "--seed", seed],
                           "tools/tess-meter/src/main.rs\n")
    if "KLINT_ROW=dev-default" not in pinned.stdout.splitlines():
        raise SystemExit("SELFTEST FAILED: a change under tools/tess-meter/ did not pin the k-lint "
                         f"row that runs its suite — the pin is gone\n{pinned.stdout}")
    if "PINNED" not in pinned.stderr or "tools/tess-meter/src/main.rs" not in pinned.stderr:
        raise SystemExit("SELFTEST FAILED: the k-lint row was pinned and the run did not say so, "
                         f"or did not name the file that pinned it\nstderr: {pinned.stderr!r}")
    if "PINNED" in pinned.stdout:
        raise SystemExit("SELFTEST FAILED: the k-lint pin note reached STDOUT, where both halves "
                         f"read KEY=value lines\n{pinned.stdout}")
    if "CONFIG_SOURCE=lane:sampled eps:sampled klint:pinned" not in pinned.stdout.splitlines():
        raise SystemExit("SELFTEST FAILED: a pinned k-lint row was recorded as something other "
                         f"than `klint:pinned` — the outputs cannot tell a pin from a draw\n"
                         f"{pinned.stdout}")

    # THE OTHER TWO MEMBERS, so the mapping is exercised rather than one entry
    # of it. Both derive to `dev-default` today; if a future derivation moves
    # one, this reads the table rather than a literal.
    for prefix, row in KLINT_PATH_ROWS:
        got = _selftest_run(t, ["--files", "-", "--seed", seed], f"{prefix}src/lib.rs\n")
        if f"KLINT_ROW={row}" not in got.stdout.splitlines():
            raise SystemExit(f"SELFTEST FAILED: {prefix} is mapped to `{row}` and a change under "
                             f"it did not pin that row\n{got.stdout}")

    # THE FALLBACK ARM: a `tools/` path the table does not name pins the row
    # that runs the most tests and SAYS it was not derived, rather than
    # inheriting an entry it never earned.
    unmapped = _selftest_run(t, ["--files", "-", "--seed", seed], "tools/notyet/src/main.rs\n")
    if f"KLINT_ROW={KLINT_PIN_FALLBACK}" not in unmapped.stdout.splitlines():
        raise SystemExit("SELFTEST FAILED: an unmapped tools/ path fell through to the DRAW — the "
                         f"fallback is a row, never the draw\n{unmapped.stdout}")
    if "no row is derived" not in unmapped.stderr:
        raise SystemExit("SELFTEST FAILED: the fallback did not say the row was a fallback, so a "
                         f"guess reads as a derivation\nstderr: {unmapped.stderr!r}")

    # `demos/` DRAWS, AND THAT IS THE RULING RATHER THAN AN OMISSION. It is the
    # other excluded workspace and the obvious next prefix; this is what stops
    # it being added without the argument at `KLINT_PATH_ROWS` being reopened.
    demos = _selftest_run(t, ["--files", "-", "--seed", seed],
                          "demos/tour/src/main.rs\ndemos/wild/src/main.rs\n")
    if f"KLINT_ROW={_sample(seed, 'klint', KLINT_ROWS)}" not in demos.stdout.splitlines():
        raise SystemExit("SELFTEST FAILED: a demos/-only diff did not DRAW its k-lint row — "
                         f"`demos/` is ruled OUT of the pin\n{demos.stdout}")
    if "klint:pinned" in demos.stdout or "KLINT_ROW" in demos.stderr:
        raise SystemExit("SELFTEST FAILED: a demos/-only diff announced a k-lint pin\n"
                         f"{demos.stdout}\nstderr: {demos.stderr!r}")

    drawn = _selftest_run(t, ["--files", "-", "--seed", seed], "crates/topo/src/lib.rs\n")
    if "klint:pinned" in drawn.stdout or "KLINT_ROW" in drawn.stderr:
        raise SystemExit("SELFTEST FAILED: an ordinary diff announced a k-lint pin — the note "
                         f"would then say nothing about any run\n{drawn.stdout}")

    # PRECEDENCE, and it is the seam neither the request path's cases nor the
    # pin's own cover: a REQUEST beats the pin, and the note must go quiet with
    # it or a run gating `dev-probe` prints that it is pinned to `dev-default`.
    asked = _selftest_run(
        t, ["--files", "-", "--seed", seed, "--config", "klint=dev-probe"],
        "tools/tess-meter/src/main.rs\n")
    if "KLINT_ROW=dev-probe" not in asked.stdout.splitlines():
        raise SystemExit(f"SELFTEST FAILED: a requested k-lint row did not beat the pin\n"
                         f"{asked.stdout}")
    if "klint:requested" not in asked.stdout:
        raise SystemExit("SELFTEST FAILED: a request that beat the pin was recorded as the pin; "
                         f"the run would credit its row to a file nobody chose\n{asked.stdout}")
    if "KLINT_ROW" in asked.stderr:
        raise SystemExit("SELFTEST FAILED: the pin note fired over a row the request overrode\n"
                         f"stderr: {asked.stderr!r}")
    with open(os.path.join(t, "klint-msg.txt"), "w") as fh:
        fh.write("tools: retune the split scan\n\nCI-Config: klint=all\n")
    trailered = _selftest_run(
        t, ["--files", "-", "--seed", seed, "--config-from-message", "klint-msg.txt"],
        "tools/tess-meter/src/main.rs\n")
    if ("KLINT_ROW=all" not in trailered.stdout.splitlines()
            or "klint:commit-trailer" not in trailered.stdout):
        raise SystemExit("SELFTEST FAILED: a `CI-Config:` trailer must beat the pin the same way "
                         f"the flag does\n{trailered.stdout}")

    # THE RELAY FILE carries this notice too — ci.yml restates neither.
    notes = os.path.join(t, "klint-notices.txt")
    _selftest_run(t, ["--files", "-", "--seed", seed, "--notices", notes],
                  "tools/tess-meter/src/main.rs\n")
    with open(notes) as fh:
        relayed = fh.read()
    if "KLINT_ROW=dev-default is PINNED" not in relayed or "tess-meter" not in relayed:
        raise SystemExit("SELFTEST FAILED: --notices did not carry the k-lint pin's reason, so "
                         f"ci.yml's relay would print nothing about it\n{relayed!r}")

    # UNRESOLVED FAILS CLOSED INTO EVERY ROW. This is the arm that costs five
    # compiles, so it is also the one most likely to be "optimised" back into
    # the draw by someone reading the bill and not the argument.
    empty = _selftest_run(t, ["--files", "-", "--seed", seed], "\n")
    if "KLINT_ROW=all" not in empty.stdout.splitlines() or "klint:pinned" not in empty.stdout:
        raise SystemExit("SELFTEST FAILED: an unresolvable change set drew a k-lint row — nothing "
                         f"there can prove tools/ held still\n{empty.stdout}")

    # No seed: nothing is drawn, so there is nothing to pin. KLINT_ROW=all
    # already runs every unification.
    unseeded = _selftest_run(t, ["--files", "-"], "tools/tess-meter/src/main.rs\n")
    if ("KLINT_ROW=all" not in unseeded.stdout.splitlines()
            or "klint:pinned" in unseeded.stdout or "KLINT_ROW" in unseeded.stderr):
        raise SystemExit("SELFTEST FAILED: an unseeded run must be KLINT_ROW=all and announce no "
                         f"pin\n{unseeded.stdout}\nstderr: {unseeded.stderr!r}")


# THE JOB WHOSE ROWS THIS MAPPING IS DERIVED FROM, and the one string in this
# file that names it. `_selftest_klint_workflow` reads the workflow's TEXT — the
# census gate reads ci.yml the same way, for the same reason: a derivation
# nobody re-runs against its source is a transcription with a date on it.
KLINT_JOB_KEY = "k-lint"
KLINT_WORKFLOW = ".github/workflows/ci.yml"
_KLINT_IF_RE = re.compile(
    r"contains\(fromJSON\('(\[[^\]]*\])'\)\s*,\s*needs\.filter\.outputs\.klint_row\)"
)


def _klint_job_steps(text: str) -> list[tuple[frozenset[str], str]]:
    """`(rows this step is gated on, the step's text)` for the k-lint job.

    Bounded to that job's own block: every other job in this workflow indents
    its steps identically, so an unbounded scan would attribute a neighbour's
    row to this one and the assertions below would be about the wrong file.
    A step with no `klint_row` condition comes back with an EMPTY row set
    rather than being dropped — "gated on nothing" is a real answer here (the
    checkout and cache steps are), and dropping it would let a row condition
    that was DELETED read as a step that never had one.
    """
    lines = text.split("\n")
    try:
        start = next(i for i, ln in enumerate(lines) if ln == f"  {KLINT_JOB_KEY}:")
    except StopIteration:
        raise SystemExit(
            f"SELFTEST FAILED: {KLINT_WORKFLOW} has no `{KLINT_JOB_KEY}:` job. KLINT_PATH_ROWS is "
            "derived from that job's steps; if it was renamed, re-derive the mapping against "
            "whatever replaced it rather than repointing this name"
        ) from None
    end = next(
        (i for i in range(start + 1, len(lines)) if re.match(r"^  [A-Za-z0-9_-]+:\s*$", lines[i])),
        len(lines),
    )
    steps: list[list[str]] = []
    for ln in lines[start:end]:
        if re.match(r"^      - \S", ln):
            steps.append([])
        if steps:
            steps[-1].append(ln)
    out: list[tuple[frozenset[str], str]] = []
    for body in steps:
        blob = "\n".join(body)
        m = _KLINT_IF_RE.search(blob)
        out.append((frozenset(json.loads(m.group(1))) - {"all"} if m else frozenset(), blob))
    return out


def _selftest_klint_workflow() -> None:
    """THE MAPPING, CHECKED AGAINST THE JOB IT IS DERIVED FROM.

    `_selftest_klint_premise` reads the TREE and can only say that every tool
    crate has an entry. The wrong half is the other one: an entry that names a
    real directory and a real row, and a workflow where that crate's suite has
    since moved to a different row. Nothing reds — the pin substitutes a row
    that no longer runs the thing it was chosen for, and the run is green about
    a suite it never executed. That is this unit's own defect class arriving in
    its own instrument, so the derivation is re-run here rather than dated.

    THREE CLAIMS, and each is a sentence written elsewhere in this file that
    would otherwise be true only on the day it was typed:

      * THE MAPPING. Every `(prefix, row)` has a step gated on `row` that
        `cd`s into `prefix` and runs `cargo test`. "Runs the crate's suite" is
        the whole basis for choosing that row over the ones that merely
        COMPILE the crate, so it is the thing asserted, not mere mention.
      * THE ROSTER. `KLINT_ROWS` equals the set of row names the job's own
        `if:` lists carry. The literal survives as the ORDER (the draw's
        indices) and as a change-detector; what is checked is the membership,
        which is where a row added to the workflow and not here would leave
        `_sample` unable ever to select it.
      * THE FALLBACK'S SUPERLATIVE. `KLINT_PIN_FALLBACK` is gated on at least
        as many of this job's steps as any other row. HONEST ABOUT ITS UNIT:
        this counts STEPS, not tests, because a step count is what the file
        can see — a row of one step that runs a thousand tests would defeat
        it. It is a floor under "the row that runs the most", not a proof of
        it, and the argument at `KLINT_PIN_FALLBACK` is still the reason.

    WHAT IT STILL CANNOT SEE: whether a step gated on the right row actually
    exercises the guard someone cares about. `cargo test` in the right
    directory is the mechanical shadow of that; the rest is the derivation
    written at `KLINT_PATH_ROWS`."""
    path = os.path.join(_repo_root(), KLINT_WORKFLOW)
    try:
        with open(path) as fh:
            text = fh.read()
    except OSError as exc:
        raise SystemExit(f"SELFTEST FAILED: {KLINT_WORKFLOW} cannot be read ({exc}); the k-lint "
                         "mapping has no source to be derived from") from exc
    steps = _klint_job_steps(text)

    for prefix, row in KLINT_PATH_ROWS:
        crate = prefix.rstrip("/")
        if not any(
            row in rows and f"cd {crate}" in blob and "cargo test" in blob
            for rows, blob in steps
        ):
            raise SystemExit(
                f"SELFTEST FAILED: KLINT_PATH_ROWS maps {prefix} to `{row}`, and the "
                f"`{KLINT_JOB_KEY}` job has no step gated on `{row}` that enters {crate} and runs "
                "`cargo test`. The pin would substitute a row that does not run that crate's "
                "suite — which is the whole reason that row was chosen over the ones that only "
                "compile it. Re-derive the mapping from the job as it stands now"
            )

    in_workflow = frozenset().union(*(rows for rows, _ in steps)) if steps else frozenset()
    if in_workflow != frozenset(KLINT_ROWS):
        raise SystemExit(
            f"SELFTEST FAILED: KLINT_ROWS is {sorted(KLINT_ROWS)} and the `{KLINT_JOB_KEY}` job's "
            f"own `if:` conditions name {sorted(in_workflow)}. A row in the job and not in the "
            "tuple can never be drawn; a row in the tuple and not in the job is a draw that gates "
            "nothing and reports green"
        )

    gated = {row: sum(1 for rows, _ in steps if row in rows) for row in KLINT_ROWS}
    if gated[KLINT_PIN_FALLBACK] < max(gated.values()):
        raise SystemExit(
            f"SELFTEST FAILED: the unmapped-path fallback is `{KLINT_PIN_FALLBACK}`, justified as "
            f"the row that runs the most, and this job now gates {gated}. Failing closed into a "
            "row that is no longer the largest is a guess that stopped being the cheapest honest "
            "one — re-derive the fallback, or say at KLINT_PIN_FALLBACK why the step count is not "
            "the right reading"
        )
    print(f"ci-filter selftest: the k-lint mapping re-derives against {KLINT_WORKFLOW} — "
          + ", ".join(f"{p} -> {r}" for p, r in KLINT_PATH_ROWS)
          + f"; rows gated per step {gated}")


def _selftest_klint_premise() -> None:
    """THE MAPPING, CHECKED AGAINST THE TREE IT CLAIMS TO DESCRIBE.

    `KLINT_PATH_ROWS` is a DERIVATION, and a derivation nobody re-runs is a
    transcription. Two ways it goes quietly wrong, and neither reds anything
    else: a crate is added under a pinned root and inherits the fallback while
    nobody derives which row runs its suite, or an entry outlives the directory
    it names and the table reads as covering ground that is gone.

    THE ROOTS ARE THE TABLE'S OWN, not a literal: this walks `KLINT_PIN_ROOTS`,
    which is derived from the keys, so widening the pin to another tree brings
    that tree's members under this requirement in the same edit.

    The ci.yml half — that each mapped row still runs the crate's suite — is
    `_selftest_klint_workflow`. This one reads the DIRECTORY listing only."""
    root = _repo_root()
    mapped = {prefix for prefix, _ in KLINT_PATH_ROWS}
    for pinned_root in KLINT_PIN_ROOTS:
        base = os.path.join(root, pinned_root.rstrip("/"))
        members = sorted(
            d for d in os.listdir(base) if os.path.isdir(os.path.join(base, d))
        ) if os.path.isdir(base) else []
        for name in members:
            if f"{pinned_root}{name}/" not in mapped:
                raise SystemExit(
                    f"SELFTEST FAILED: {pinned_root}{name}/ has no entry in KLINT_PATH_ROWS, so a "
                    "change under it would fall back to a row nobody derived. Read which k-lint "
                    f"row runs its suite (the job's steps and their `if:`) and add "
                    f"{pinned_root}{name}/ with that row"
                )
    for prefix, row in KLINT_PATH_ROWS:
        if "/" not in prefix.rstrip("/") or not prefix.endswith("/"):
            raise SystemExit(f"SELFTEST FAILED: {prefix!r} is not a `<root>/<member>/` directory "
                             "prefix; a bare-name entry matches by accident or not at all")
        if not os.path.isdir(os.path.join(root, prefix)):
            raise SystemExit(f"SELFTEST FAILED: KLINT_PATH_ROWS names {prefix}, which is not a "
                             "directory in this tree — the mapping describes a tree that moved")
        if row not in KLINT_ROWS:
            raise SystemExit(f"SELFTEST FAILED: {prefix} is mapped to {row!r}, which is not one of "
                             f"the k-lint rows ({', '.join(KLINT_ROWS)}); the job's `if:` "
                             "conditions would match it against nothing and the row would be SKIPPED")
    if KLINT_PIN_FALLBACK not in KLINT_ROWS:
        raise SystemExit(f"SELFTEST FAILED: the fallback row {KLINT_PIN_FALLBACK!r} is not a "
                         "k-lint row, so an unmapped path would pin a row that never runs")
    print("ci-filter selftest: every member of " + ", ".join(KLINT_PIN_ROOTS)
          + " has a derived k-lint row: "
          + ", ".join(f"{p} -> {r}" for p, r in KLINT_PATH_ROWS))


def _selftest_config() -> None:
    """THE REQUEST PATH, in-process where it is a pure function and through the
    CLI where the wiring is.

    WHAT IS ACTUALLY AT RISK HERE, and it is not "does an override override".
    It is the two SILENT failures either spelling can have:

      * a request that is READ BUT NOT APPLIED, or applied to the wrong
        dimension. Nothing reds; the run gates the drawn point and reports a
        green that answers a question nobody asked.
      * a request that is NOT READ — the trailer regex drifting, most likely,
        since it is the half nobody types twice. Same green, same wrong
        question, and the author's line sits in the commit forever looking
        like it did something.

    So every legal value of every dimension is requested and checked, the
    dimensions nobody named are required to still match the draw, and the
    trailer's near-misses (indented, mid-sentence) are required NOT to be read
    while the typo case (wrong case) is required to be."""
    files = ["crates/geom-core/src/lib.rs"]
    base = {"TIER": "closure", "PKGS": "geom-core", "CARGO_SCOPE": "-p geom-core"}
    seed = "deadbeef"
    drawn = decorate(dict(base), files, seed)
    keys = [out_key for out_key, _ in CONFIG_DIMENSIONS.values()]

    if drawn["CONFIG_SOURCE"] != "lane:sampled eps:sampled klint:sampled":
        raise SystemExit("SELFTEST FAILED: an unrequested run must record every dimension as "
                         f"sampled — CONFIG_SOURCE is {drawn['CONFIG_SOURCE']!r}")
    if decorate(dict(base), files, None)["CONFIG_SOURCE"] != (
        "lane:unsampled eps:unsampled klint:unsampled"
    ):
        raise SystemExit("SELFTEST FAILED: the no-seed path must record itself as unsampled")

    for name, (out_key, choices) in CONFIG_DIMENSIONS.items():
        for value in choices:
            got = decorate(dict(base), files, seed, parse_config([f"{name}={value}"], "requested"))
            if got[out_key] != value:
                raise SystemExit(f"SELFTEST FAILED: {name}={value} was requested and {out_key} came "
                                 f"back {got[out_key]!r} — the request is being read and dropped")
            if f"{name}:requested" not in got["CONFIG_SOURCE"]:
                raise SystemExit(f"SELFTEST FAILED: {name}={value} was requested and CONFIG_SOURCE "
                                 f"says {got['CONFIG_SOURCE']!r} — an unrecorded override is a run "
                                 "that cannot be read back")
            for other in keys:
                if other != out_key and got[other] != drawn[other]:
                    raise SystemExit(f"SELFTEST FAILED: requesting {name} moved {other} as well "
                                     "— the dimensions nobody named must still be drawn")

    # THE INTERVAL PIN IS A FLOOR OVER THE DRAW, NOT OVER A PERSON. Requesting
    # `lane=default` on a change `_forces_interval` pins must win, and must say
    # in CONFIG_SOURCE that it did.
    pinned = ["crates/geom-core/src/interval.rs"]
    if decorate(dict(base), pinned, seed)["LANE"] != "interval":
        raise SystemExit("SELFTEST FAILED: the interval pin no longer fires — this case is "
                         "checking nothing")
    got = decorate(dict(base), pinned, seed, parse_config(["lane=default"], "requested"))
    if got["LANE"] != "default" or "lane:requested" not in got["CONFIG_SOURCE"]:
        raise SystemExit("SELFTEST FAILED: a requested lane must override the interval pin and "
                         f"record it — got {got['LANE']!r}, {got['CONFIG_SOURCE']!r}")

    # PRECEDENCE, per dimension: the invocation over the trailer over the draw.
    merged = dict(config_from_message("t\n\nCI-Config: lane=both eps=1e-6\n"))
    merged.update(parse_config(["lane=interval"], "requested"))
    got = decorate(dict(base), files, seed, merged)
    if (got["LANE"], got["EPS"], got["KLINT_ROW"]) != ("interval", "1e-6", drawn["KLINT_ROW"]):
        raise SystemExit(f"SELFTEST FAILED: precedence is invocation > trailer > draw; got {got}")
    if got["CONFIG_SOURCE"] != "lane:requested eps:commit-trailer klint:sampled":
        raise SystemExit(f"SELFTEST FAILED: a mixed run must say which dimension came from where "
                         f"— got {got['CONFIG_SOURCE']!r}")

    # THE TRAILER, read out of a message shaped like a real one.
    real = (
        "topo: split the half-edge link\n"
        "\n"
        "Body prose that mentions ci-config: lane=both in passing.\n"
        "    CI-Config: klint=all\n"
        "\n"
        "CI-Config: eps=1e-12 klint=dev-probe\n"
    )
    if config_from_message(real) != {
        "EPS": ("1e-12", "commit-trailer"),
        "KLINT_ROW": ("dev-probe", "commit-trailer"),
    }:
        raise SystemExit("SELFTEST FAILED: the trailer must be read at the start of a line and "
                         f"nowhere else — got {config_from_message(real)}")
    if config_from_message("no request here\n\nSigned-off-by: someone\n"):
        raise SystemExit("SELFTEST FAILED: a message with no trailer must request nothing")
    # A TYPO IN THE CASE IS STILL A REQUEST. The alternative reading — silence
    # — is the failure this whole function is about.
    if config_from_message("t\n\nci-Config: eps=1e-6\n") != {"EPS": ("1e-6", "commit-trailer")}:
        raise SystemExit("SELFTEST FAILED: the trailer must be case-insensitive; a miscased line "
                         "that reads as `no request` gates the wrong point silently")

    # LOUD ON EVERY MALFORMED REQUEST, from either spelling.
    for bad in (["lane"], ["lane="], ["=interval"], ["mode=interval"], ["lane=fast"],
                ["eps=all"], ["lane=default", "lane=interval"]):
        try:
            parse_config(bad, "requested")
        except ConfigError:
            continue
        raise SystemExit(f"SELFTEST FAILED: {bad} was accepted as a configuration request")
    for bad_msg in ("t\n\nCI-Config: eps=1e-13\n", "t\n\nCI-Config: lane=x eps=1e-6\n",
                    "t\n\nCI-Config: lane=both\nCI-Config: lane=interval\n"):
        try:
            config_from_message(bad_msg)
        except ConfigError:
            continue
        raise SystemExit(f"SELFTEST FAILED: {bad_msg!r} was accepted as a configuration request")


def _selftest_docs_premise() -> None:
    """THE PREMISE THE DOCS TIER RESTS ON, read off the REAL tree rather than
    asserted in a header.

    WHAT THIS PROVES AND WHAT IT DOES NOT, because the difference is the whole
    reason this function is not enough on its own. It proves the union comes
    back derivable on this tree and that nothing in it is misfiled as
    documentation. It does NOT prove the union is COMPLETE: "every member of
    this set is non-docs" is vacuously true of a set that has quietly lost
    members, and a derivation that stopped recognising a spelling loses them
    without erroring. Completeness rests on `_markdown_read_by_python` and
    `_compiled_markdown` failing closed on every consumption they can see — a
    mention neither can resolve raises `Bail` rather than going missing — and
    that is what turns a re-spelled page into the red below instead of into a
    quietly smaller set.

    The `Bail` is caught here for one reason: in a real run its only trace is a
    line on stderr inside the filter job, under a TIER=all that looks like any
    other conservative verdict. Here it is a named self-test failure."""
    root = _repo_root()
    try:
        consumed = _consumed_markdown(root)
    except Bail as exc:
        raise SystemExit(
            f"SELFTEST FAILED: this tree's consumed-markdown set cannot be derived: {exc}"
        ) from exc
    for path in sorted(consumed):
        if _is_docs(path, consumed):
            raise SystemExit(f"SELFTEST FAILED: {path} is consumed by a build or a suite and still "
                             "classifies as documentation")
    print("ci-filter selftest: markdown this tree compiles or executes, and so keeps out of the docs "
          "tier: " + (", ".join(sorted(consumed)) or "(none)"))


def main() -> int:
    ap = argparse.ArgumentParser()
    src = ap.add_mutually_exclusive_group(required=True)
    src.add_argument("--base", help="git ref/sha to diff HEAD against")
    src.add_argument("--files", help="file with a newline-separated list, or -")
    src.add_argument("--selftest", action="store_true", help="run the fixture battery")
    # In the `src` group for the same reason `--force-all` is: it takes no
    # diff. The gated SET is a property of the tree alone — every marked
    # suite, whether or not anything changed — which is exactly the question
    # the nightly re-take asks.
    src.add_argument(
        "--gated-set",
        action="store_true",
        help="print one nextest filterset expression selecting EVERY gated "
        "suite in the tree (the nightly's ungated re-take), or `none()` when "
        "the tree carries no marker; not the KEY=value stream",
    )
    # In the `src` group because it is the third way to answer "what changed":
    # by declining to ask. It takes no diff, so it cannot be combined with one.
    src.add_argument(
        "--force-all",
        action="store_true",
        help="take no diff at all and return the `all` tier (everything runs, "
        "unscoped; the path-keyed signals then fail closed)",
    )
    # NOT in the mutually-exclusive `src` group: the seed selects a matrix
    # point and is orthogonal to how the file list was obtained, so it rides
    # alongside --base or --files rather than instead of them.
    ap.add_argument(
        "--seed",
        help="head SHA to key the configuration sample on; omit to run the "
        "whole matrix (LANE=both, EPS=all, KLINT_ROW=all)",
    )
    ap.add_argument(
        "--config",
        action="extend",
        nargs="+",
        metavar="KEY=VALUE",
        help="request a matrix point rather than drawing it, e.g. "
        "`--config lane=interval eps=1e-12 klint=dev-probe`; unnamed "
        "dimensions are still drawn",
    )
    ap.add_argument(
        "--config-from-message",
        metavar="FILE",
        help="read the same request from the `CI-Config:` trailer of a commit "
        "message in FILE; --config wins per dimension",
    )
    ap.add_argument(
        "--notices",
        metavar="FILE",
        help="also write the human notices (either pin's reason, the interval "
        "advisory) to FILE, so a caller can relay them verbatim instead of "
        "restating them; truncated to empty when there are none",
    )
    args = ap.parse_args()
    if args.selftest:
        selftest()
        return 0

    if args.gated_set:
        # NOT under the fail-closed wrapper below, and that is the whole
        # difference between this mode and the filter. The filter's failure
        # answer is "run everything", which is safe because everything then
        # runs; this mode's caller runs ONLY what it prints, so its failure
        # answer has to be a red step. An empty answer it cannot prove is the
        # silent-zero-coverage shape `nightly-only-selection.py` was written
        # against, one lane over.
        return gated_set(_repo_root())

    # BEFORE ANY WORK, AND OUTSIDE THE FAIL-CLOSED WRAPPER BELOW. A malformed
    # request is not a classification that could not be made, so it does not
    # become TIER=all — it becomes a red step under the person who typed it.
    config: dict[str, tuple[str, str]] = {}
    try:
        if args.config_from_message:
            with open(args.config_from_message) as fh:
                config.update(config_from_message(fh.read()))
        if args.config:
            config.update(parse_config(args.config, "requested"))
    except (ConfigError, OSError) as exc:
        print(f"ci-filter: {exc}", file=sys.stderr)
        return 2

    root = _repo_root()

    # `None` until a file list is actually in hand, so that a failure ANYWHERE
    # below — including one that happens before `files` is ever bound — still
    # reaches the path-keyed signals as "unknown", which they read as run.
    files: list[str] | None = None
    try:
        if args.force_all:
            # `files` stays None, and that is the honest reading: nothing was
            # diffed, so nothing here can prove the oracle sources or the
            # interval lane held still. Both signals run.
            res = _all_tier(root)
        elif args.files:
            raw = sys.stdin.read() if args.files == "-" else open(args.files).read()
            files = [ln.strip() for ln in raw.splitlines() if ln.strip()]
            res = classify(files, root)
        else:
            try:
                raw = _run(["git", "diff", *_DIFF_FLAGS, f"{args.base}...HEAD"], root)
            except subprocess.CalledProcessError:
                # Unrelated histories / shallow clone: fall back to the
                # two-dot form rather than guessing.
                raw = _run(["git", "diff", *_DIFF_FLAGS, args.base, "HEAD"], root)
            files = [ln.strip() for ln in raw.splitlines() if ln.strip()]
            res = classify(files, root)
    except Exception as exc:  # noqa: BLE001 — fail CLOSED on anything at all
        print(f"ci-filter: falling back to TIER=all: {exc}", file=sys.stderr)
        res = _all_tier(root)

    # `files` deliberately survives the `except` above. A Bail out of
    # `classify` is the NORMAL route for this signal, not a breakdown:
    # every interval-transcendentals path is workspace-level by the
    # allowlist, so the very changes the oracle cares about arrive here as
    # TIER=all with a perfectly good file list. Only a failure to resolve
    # the diff at all leaves `files` None, and that is the case that runs.
    out = decorate(res, files, args.seed, config)

    # THE PIN, SAID OUT LOUD, TWICE OVER. `CONFIG_SOURCE` now carries
    # `lane:pinned` — that is the machine-readable half, and it is what ci.yml's
    # always-run "the configuration this run gates" step reads. This is the
    # human half: the REASON, which is a filename and belongs nowhere near a
    # KEY=value stream. `decorate` stays free of I/O — `_selftest_sampling`
    # calls it 4000 times in-process — so it is printed here.
    #
    # ONE GUARD, NOT THREE. `lane:pinned` is emitted only under a seed and only
    # when no request overrode the pin, so both of the conditions that used to
    # be spelled out here are already inside it; re-deriving them would let the
    # note and the output key disagree.
    #
    # THE NOTICES ARE COMPOSED HERE AND WRITTEN TWICE, TO ONE WORDING. They go
    # to stderr, where the local half and anyone running this by hand sees
    # them, and — when `--notices` names a file — to that file, which ci.yml's
    # always-run configuration step relays VERBATIM. Before that relay existed
    # ci.yml restated both notices in its own prose, and the two copies had
    # already drifted twice: one said the pin's reason names a file, which the
    # fail-closed arm cannot, and the other said "DEFAULT LANE DRAWN" over a
    # lane that had been requested. There is one wording now, and it is the
    # one that can see the values it is describing.
    notices: list[str] = []

    # ONE COMPOSER, TWO DIMENSIONS. The two pins' notices were written out by
    # hand side by side and differed in wording where they did not differ in
    # meaning — which is the drift recorded three paragraphs up, arriving inside
    # the very function that was supposed to have ended it. The skeleton (what
    # is pinned, that re-pushing cannot change it, how to ask for something
    # else) is shared; what differs per dimension is one middle paragraph, and
    # that is all this table holds. The legal values in the closing line come
    # from `CONFIG_DIMENSIONS`, so a new row of either dimension cannot leave
    # the advice naming a set the parser no longer accepts.
    pin_bodies = {
        "LANE": (
            "  This is not a coverage gap. Both lanes archive the same scope and "
            "the interval lane only adds `--features interval`, so a pinned run "
            "executes the same rows in a stricter compile; what it does not reach "
            "is the loud-skip marker rows gated `cfg(not(feature = \"interval\"))` "
            "under crates/*/tests."
        ),
        "KLINT_ROW": (
            "  This is not a coverage gap, it is the opposite: the row that RUNS "
            "the suite of what changed is the row that runs, instead of the row a "
            "hash picked. (Four of the five rows COMPILE tools/tess-meter, through "
            "demos/tour's plain dependency on it; one executes its tests, and a "
            "guard that lives in a test is invisible to a type-check.) What this "
            "run does NOT gate is the other four unifications, exactly as a drawn "
            "run does not gate the other four."
        ),
    }
    for name, (out_key, choices) in CONFIG_DIMENSIONS.items():
        if f"{name}:pinned" not in out["CONFIG_SOURCE"]:
            continue
        # The forcers are pure, so this re-derives exactly what `decorate`
        # pinned on; the output key above is the guard, never this call. The
        # `or` arm is unreachable through that key and says so rather than
        # interpolating a `None` into a sentence a reader would have to decode.
        forced = (_forces_interval if out_key == "LANE" else _forces_klint)(files)
        _, why = forced or ("", "the pin's reason could not be re-derived")
        notices.append(
            f"{out_key}={out[out_key]} is PINNED, not drawn: {why}.\n"
            "  Re-pushing cannot change it — the pin runs before the seeded draw "
            "and short-circuits it.\n"
            f"{pin_bodies[out_key]}\n"
            f"  To gate a different {name} instead, say so: a `CI-Config: "
            f"{name}=<value>` trailer on the head commit beats the pin "
            f"({', '.join(choices)})."
        )

    # THE ADVISORY. It names EVERY interval-named file, and it says whether the
    # lane it is advising about was drawn or asked for — both are things only
    # this function can see, which is why the wording lives here.
    if out["LANE_ADVISORY"] == "true":
        hits = _advises_interval(files)
        shown = ", ".join(hits[:5]) + (f" (+{len(hits) - 5} more)" if len(hits) > 5 else "")
        how = "REQUESTED" if "lane:sampled" not in out["CONFIG_SOURCE"] else "drawn"
        notices.append(
            f"This diff touches {len(hits)} file(s) whose basenames carry `interval` "
            f"— {shown} — and this run gates LANE=default ({how}).\n"
            "  The filename is NOT taken as evidence any more: it once pinned the "
            "lane, and it pinned a rename that touched an interval-named file for "
            "three identifiers (#1122).\n"
            "  SO IF INTERVAL SEMANTICS CHANGED, ASK FOR THE LANE: put "
            "`CI-Config: lane=interval` in the head commit's message, or run the "
            "workflow_dispatch with lane=interval. Say in the PR which lane gated.\n"
            "  If they did not change, this notice is noise and you can ignore it. "
            "The convention is in docs/prompts/implementer-discipline.md."
        )

    # THE GATED-SUITE FILTER, COMPUTED LAST AND FAILING OPEN INTO THE EMPTY
    # STRING. It reads the tier `decorate` has already settled and the same
    # file list every other path-keyed signal here reads, and it is the one
    # output key whose value is a nextest expression rather than a word — so
    # it is composed here, beside the notices that explain it, rather than in
    # `decorate`, which does no I/O and cannot open a source file.
    #
    # THE MEMBER MAP IS BEST-EFFORT. It supplies the PACKAGE name for a term's
    # binary id; without it the directory name stands in, which is the same
    # string for every member of this workspace and is checked by the local
    # verification the gate's own `--selftest` cannot do (a term that matches
    # no test excludes no test). A cargo that cannot run is therefore not a
    # reason to skip the gate, and not a reason to trust it either: the
    # `gated: … skipped` notices name every suite the run did not execute.
    try:
        gate_dir_of, _ = _members(root)
    except Exception as exc:  # noqa: BLE001 — the directory name stands in
        print(f"ci-filter: gated suites: no member map ({exc})", file=sys.stderr)
        gate_dir_of = None
    test_filter, gate_notices = gated_filter(root, files, out["TIER"], gate_dir_of)
    out["TEST_FILTER"] = test_filter
    notices.extend(gate_notices)

    for note in notices:
        print(f"ci-filter: {note}", file=sys.stderr)
    if args.notices:
        # Truncated even when empty: the relay `cat`s this file unconditionally,
        # and a stale one from an earlier invocation would announce a pin this
        # run does not have.
        with open(args.notices, "w") as fh:
            for note in notices:
                fh.write(f"ci-filter: {note}\n")

    for key, val in out.items():
        print(f"{key}={val}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
