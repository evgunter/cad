#!/usr/bin/env python3
"""Shared CI change filter — the SINGLE implementation of change
classification, used by BOTH .github/workflows/ci.yml (its `filter` job is
a thin YAML wrapper) and local-scripts/ci-local.sh. There is no second copy of
these rules anywhere; hosted and local runs are gated identically, and the
synthetic-diff tests exercise the one script both of them call.

Three tiers (Ev's ask: "changing a core crate runs everything, adding a
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
  ci-filter.py ... --config lane=interval eps=1e-12 klint=dev-probe
                                   NARROW the run to those points (below)
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
  RUN_PNCAD_PY=true|false       python suite (wheel + unittest) row — keyed on
                                SEEDS, not on the closure, like
                                RUN_VIEWER_TOOLKIT below; see `PNCAD_PY_SEEDS`
  RUN_INTERVAL_BACKEND=true|false   interval-transcendentals' own workspace
  RUN_INTERVAL_ORACLE=true|false    its oracle-inari certification tier
  RUN_TOPO_RELEASE=true|false   corrupt input (release profile) row. LOCAL-ONLY
                                today — the hosted job moved to nightly.yml and
                                runs ungated there; see JOB_ROOTS
  RUN_K_LINT=true|false         k-lint (gate) row
  LANE_ADVISORY=true|false      this diff touches `*interval*` files and this
                                run was NARROWED to the default lane by a
                                request, so if interval semantics changed the
                                author narrowed away the axis they were
                                changing. Advisory: nothing reads it to
                                decide what runs (see below)
  LANE=default|interval|both    which COMPILE MODE this run gates. `both`
                                unless a request narrows it (see below)
  EPS=default|<value>|all       which tolerance row(s) this run gates. `all`
                                unless a request narrows it
  KLINT_ROW=<unification>|all   which of `k-lint (gate)`'s five feature
                                unifications this run gates. `all` unless a
                                request narrows it, and ci.yml fans `all` out
                                as five matrix legs the way it fans `EPS=all`
                                out as three
  SEEDS=<comma-separated members whose OWN files changed, empty for
                                docs and for `all`>
  CONFIG_SOURCE=lane:<src> eps:<src> klint:<src>
                                where each of the three values above came
                                from: `unsampled` (the whole dimension runs,
                                which is now every dimension's default) or
                                `requested` (--config, which is where ci.yml's
                                `workflow_dispatch` inputs land). There is no
                                third source: the vocabulary names what can
                                happen and nothing else
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

CONFIGURATION COVERAGE. NOTHING HERE IS SAMPLED (2026-09-04, two
authorisations from Ev in chat: "feel free to reinstate full runs instead of
sampling" for the lane and the eps row, then "you can un-sample k-lint" for the
row that was left).

THE LANE AND THE EPS ROW ARE NOT SAMPLED.

WHAT THAT UNDOES. From 2026-08-22 to 2026-09-04 a run drew ONE of those six
points from the head SHA and let repetition cover the rest. The premise was a
scarce billed resource — `docs/CI-MINUTES-2026-08.md` opens with the Actions
allowance being consumed faster than the work justified — and that premise
died when the repository went public on 2026-09-03: standard-runner minutes
are free and the runner is 4 vCPU / 16 GB (was 2 / 7). The sampling argument
was sound and it was never free: each point gated about one run in six, so a
break confined to one of them merged green and surfaced later, on a branch
belonging to whoever next drew it. Nothing about the soundness argument
changed; the thing it bought stopped having a price.

WHY THIS IS AFFORDABLE, AND IT IS NOT A 6x MULTIPLIER. The nextest archive is
built once per COMPILE MODE, and eps is RUNTIME env (CAD_TOLERANCE_EPS) read
by bit-identical binaries — so six points are TWO builds and TWELVE test jobs,
not six builds. Builds dominate; test legs are the cheap half. Measured on the
4-vCPU runner, over the 72 code-tier runs of one 3.9-hour window: the whole
matrix costs about **+15 job-minutes on a TIER=closure run and +19 on a
TIER=all one**, against medians of 22 and 31.

WALL CLOCK IS NOT FREE AND THE FIRST VERSION OF THIS NOTE SAID IT WAS. The eps
legs do start together behind an archive that was already being built, but a
run's wall follows their MAXIMUM, and the maximum of six legs is larger than
the maximum of two: measured, ~+20 s of critical path on a run that would have
drawn `interval` anyway, on top of the ~+172 s the interval archive adds to one
that would have drawn `default` — about +96 s in expectation on a TIER=all run.
The last job on that path is `test (interval, eps = default, 1/2)`, the first
eps row's shard 1, which also carries the two editor-core steps.

THE JOB-MINUTE FIGURES ARE FLOORS, NOT FORECASTS: three un-sampled runs came in
at 54.0 / 44.4 / 49.7 job-minutes against a 30.6-minute TIER=all median. The
population, the arithmetic and both corrections are in
`docs/CI-MINUTES-2026-08.md`, under the 2026-08-22 sampling section this
supersedes.

THE K-LINT ROW IS NOT SAMPLED EITHER, AND IT WAS THE LAST ONE (2026-09-04,
Ev in chat: "you can un-sample k-lint"). `k-lint (gate)`'s five FEATURE
UNIFICATIONS (see `KLINT_ROWS`) were drawn one per run under a salt of their
own. This script now prints `KLINT_ROW=all` on every run and ci.yml fans that
out as FIVE MATRIX LEGS, one per unification, exactly as it fans `EPS=all` out
into three.

THE COST SHAPE ARGUMENT WAS RIGHT AND IS NOT WHY THIS STAYED SAMPLED. Three eps
rows are ONE nextest archive replayed under a different env var; five
unifications are five COMPILES of demos/tour and the kernel crates that share
almost no artifacts, because `--release` and dev are different profiles and
`budget` and `probe` are opt-in features gated at a module boundary, so each is
its own fingerprint for every crate that sees it. That is a real difference and
it is why this dimension was scoped out of the lane/eps unit. What it never
was, until 2026-09-04, is a cost with something on the other side of it.

WHAT PUT SOMETHING ON THE OTHER SIDE: `#1756` -> `#1775`. On run 33834607784,
`k-lint (gate)` concluded SUCCESS with `demos tour fmt + clippy` SKIPPED — the
run drew `release-budget`, and 12 of that job's 14 row steps did not execute.
(14 is the row-gated set; 19 was the highest step NUMBER in that job's step
list, which also counts checkout, the prune, the toolchain and the cache.)
The clippy break that step would have caught reached main and stayed there
until a separate PR repaired it, and three lanes read the identical green in
three different ways, one of them concluding main had been fixed when it had
not. `demos/tour` and `demos/wild` are EXCLUDED workspaces, so no
`--workspace` check reaches them and the drawn row was their only gate.
A green job name over a skipped step is what the draw actually cost.

FIVE LEGS, NOT ONE JOB FIVE TIMES OVER. ci.yml's own header used to argue
against a matrix here on the grounds that it "would pay this job's setup and
cache restore five times to run one row's worth of work". That argument dies
with the draw and not before it: un-sampled, the work IS five rows' worth, so
five setups buy parallelism rather than paying for nothing. The rows are
self-contained — the two that consume a CSV consume one the step above them in
the SAME row wrote — which is what makes the row the matrix axis. The two cache
lanes survive and get sharper: the key is still the row's first token, so each
leg restores its own profile's lane instead of one lane thrashing between
profiles.

THE PATH PIN WENT WITH THE DRAW, the way `_forces_interval` went with the
lane's. `_forces_klint` substituted the row that RUNS a changed `tools/` crate's
own suite ahead of the seeded draw (Ev's ruling, 2026-08-29), and failed closed
into `all` when the file list could not be resolved. A run that gates every row
has nothing left to pin: the pin could only re-state the default or narrow it,
and narrowing on a path is the one thing this file will not do. So
`KLINT_PATH_ROWS`, `KLINT_PIN_ROOTS` and `KLINT_PIN_FALLBACK` are deleted with
it. The ruling's subject — a tool crate's guard living in that crate's own test
suite, reached by exactly one row — is now satisfied on every run rather than
on the runs a mapping remembered to cover.

NOTHING READS A SEED. `--seed` and `_sample` are gone: the lane and the eps row
stopped reading the seed on 2026-09-04 and the k-lint row was the only caller
left. A `--seed` on the command line is now an unrecognised option and reds,
which is this file's standing answer to an input it cannot make sense of. Both
properties the seeding bought — a re-run of the same commit picks the same row,
and the row is recoverable from the SHA alone — are answered instead by there
being no row to pick.

LOCAL AND HOSTED NOW GATE THE SAME CONFIGURATION SET, and this sentence has
moved twice in three days. `ci-local.sh` runs both lanes, all three eps rows and
all five k-lint unifications; so does a hosted run. What local still adds over
hosted is its opt-in `--nightly` row, and nothing else.

A NOTICE IS NOT A MATRIX POINT AND MUST NOT ENTER THE KEY=value STREAM. What is
left to announce here is the interval advisory and the gated-suite skips, and
both go to STDERR and into `--notices` when a caller asks for it — never to
stdout, where both halves append to $GITHUB_OUTPUT or read with
`IFS='=' read -r k v` and one extra line would be one bogus output key. THE
WORDING LIVES HERE AND ONLY HERE. ci.yml used to restate the notices in its own
prose so it could print them where a reader looks, and the two copies drifted
twice — one claimed a pin's reason always names a file (the fail-closed arm
named none), the other said "DEFAULT LANE DRAWN" over a lane that had been
requested. `--notices` is the relay that removed the second copy.

THE LANE IS NEITHER DRAWN NOR PINNED (2026-09-04). `_forces_interval` is gone
with the draw it existed to pre-empt: it pinned `LANE=interval` for a change
under `interval-transcendentals/` or an unresolvable file list, and a lane
that always runs both compile modes has nothing left to pin. `#1122`'s ruling
survives it in the only form that still has work to do — a filename is not
evidence about semantics — as `_advises_interval`, which now fires in exactly
one case: a run a REQUEST narrowed to `lane=default` over a diff touching
`*interval*` files. Someone who narrowed away the axis their diff touches is
the one reader that notice is still for. It changes nothing about what runs.

NARROWING A RUN TO ONE POINT (2026-08-28, Ev's ask; repurposed 2026-09-04 when
the draws went, and reduced to ONE SPELLING on 2026-09-04). While the draws
existed, this was how someone ASKED for the point a draw kept missing. There is
now exactly one way to say it:

  --config lane=interval eps=1e-12 klint=dev-probe   THE INVOCATION says it,
      and it MAY NARROW. ci.yml's `workflow_dispatch` inputs land here, so a
      run can be aimed at a configuration with no commit and no push — typed
      by whoever is standing there now, which is what a deliberate narrowing
      is.

THERE WAS A SECOND SPELLING AND IT IS DELETED (2026-09-04, Ev: *"i see in 1855
it's still talking about the ci config trailer; that code should be deleted
since it's no longer live"*). `--config-from-message` read a `CI-Config:`
trailer out of the head commit's message and was ADDITIVE-ONLY: it could not
gate less than an unmarked run, because a trailer is COPIED rather than typed
and rides one push with nobody standing over it. Once un-sampling reached the
last dimension, "additive-only" and "every dimension already runs whole" met:
the only values a trailer could legally name were `lane=both`, `eps=all` and
`klint=all`, each of which changed nothing, and every other value red the
classify step. NO INPUT MADE IT USEFUL — it could restate the default or fail —
so the path is gone rather than kept as a spelling whose entire legal
vocabulary is a no-op. What it was for survives in the dispatch input, which
is the deliberate act it was never a good shape for.

The request does nothing but replace a value before it is printed — no job
condition, no matrix and no cache key reads anything but the LANE / EPS /
KLINT_ROW lines, so a narrowed run runs the identical gate that point runs
inside a full one. A NARROWED RUN ANNOUNCES ITSELF ON THE RUN PAGE: ci.yml's
`the configuration this run gates` step emits a `::warning::` annotation
whenever LANE, EPS or KLINT_ROW is not the whole dimension, which is the one
channel that reaches a reader who never opens a job log.

WHICH CONFIGURATION GATED THIS COMMIT is recoverable from CONFIG_SOURCE, which
ci.yml prints in a step that always runs. A dispatch's answer lives in one
run's inputs and not in the commit, and that is now the only answer there is;
the trailer's one advantage over it died with the values that made it a no-op.

PRECEDENCE is the invocation over the default, PER DIMENSION. A dimension
nobody names keeps its default, and every default is now the WHOLE dimension —
every lane, every eps row and every k-lint unification — so `--config
eps=1e-12` narrows one axis and leaves the rest whole.

A REQUEST THAT NAMES NO REAL POINT IS A HARD FAILURE, not a fallback to the
default: an unknown key, an unknown value, a repeated key, a token that is not
`key=value`. This is the one place in this script that does not fail into
more work, and the asymmetry is the point — every other failure here is an
inability to classify, where running everything is the safe answer, while
this one is an INPUT ERROR whose author is standing there reading the result.
Failing open would hand them a green run over a configuration they did not
ask for, which is exactly the question they were asking.

`eps=all` IS A LEGAL REQUEST, AND WAS NOT BEFORE 2026-09-04. It used to mean
"every row" to the local half, which loops over them, while the hosted eps
rows put the value straight into CAD_TOLERANCE_EPS, where `all` is a parse
error by design. The hosted half now expands `all` into three matrix legs
before any value reaches that variable, and `all` is what this script prints
when nobody narrows the dimension — so refusing it as a request while emitting
it as the default would be incoherent. `lane=both` and `klint=all` are legal
on the same terms; all three spell "every row of that dimension".

THE PER-FILE TEST GATE (2026-09-02, S-TCOST lever 3; work/tcost/TCOST-1.md).
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
to an unnamed path can hide is a day. This is the same argument the k-lint
draw rests on, at a longer period, and `memories/test-suite-cost.md`
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
CLOSED, so such a run certifies the oracle. Neither the lane nor the k-lint row
is among those signals any more — every run gates both compile modes and all
five unifications, so there is nothing left for an unresolvable file list to
fail closed INTO.
"""

from __future__ import annotations

import argparse
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
# pncad-py      NO LONGER HERE. `RUN_PNCAD_PY` is computed in `decorate`
#               off the SEEDS, beside `RUN_VIEWER_TOOLKIT` and for the
#               same reason; the argument is at that site. What this
#               table said, and why it stopped being the right condition:
#               the wheel compiles pncad-py's whole dependency graph —
#               the entire façade stack — so `pncad-py in closure` is
#               "something the wheel compiles moved", which is true of
#               nearly every kernel change and therefore gates almost
#               nothing while costing a second kernel compile under the
#               `python` feature on almost every code-tier run.
# topo          the release-profile corrupt-input row compiles
#               `-p topo --lib`, so topo's own closure membership is
#               exactly the condition under which anything it runs can
#               have moved. It is the one root whose crate is where the
#               suite lives rather than a downstream consumer.
#
#               THE HOSTED HALF OF THIS ROW IS GONE (S-TCOST C1,
#               2026-09-03): `corrupt input (release profile)` moved to
#               nightly.yml, where it runs UNGATED once a day, so no job
#               in ci.yml reads this key any more and ci.yml's `filter`
#               publishes no `run_topo_release` output. THE KEY STAYS
#               because `local-scripts/ci-local.sh` still consumes it —
#               nothing bills the local gate by the minute, so the row
#               keeps its per-change scoping there. Deleting the key
#               would silently promote a scoped local row to
#               unconditional, which is the opposite of what the demotion
#               decided. The soundness argument for the demotion is at
#               the job in nightly.yml, per row, against
#               docs/CI-MINUTES-2026-08.md's absence rule.
JOB_ROOTS = {
    "RUN_EDITOR_CORE": {"editor-core"},
    "RUN_STL": {"stl"},
    "RUN_STEP_EXPORT": {"step-export"},
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


# THE SEEDS THAT BUY THE GUI TOOLKIT ROWS (Ev's viewer-CI-posture ruling,
# 2026-08-27; docs/GUI-LOG.md). SEEDS, not the closure — the argument is at
# `RUN_VIEWER_TOOLKIT` in `decorate`, and it is the whole of why this is a
# three-name set rather than "anything viewer depends on".
#
# Adding a name here is a decision about what can break the eframe/wgpu half
# without touching viewer's own sources; it is not a convenience. The nightly
# lane re-takes the whole row daily, which is what makes the set safe to keep
# small.
VIEWER_TOOLKIT_SEEDS: frozenset[str] = frozenset({"viewer", "pncad", "bvh"})

# THE SEEDS THAT BUY THE PYTHON SUITE (Ev's approval in chat, 2026-09-03;
# S-TCOST unit C3). SEEDS, not the closure, and the argument is at
# `RUN_PNCAD_PY` in `decorate`.
#
# `pncad-py` sits downstream of `pncad`, which re-exports the whole kernel, so
# it is in the dependent CLOSURE of nearly every kernel change — a
# closure-keyed test is true almost always and gates almost nothing, while the
# row it gates is a SECOND compile of the kernel under the `python` feature.
# That is the identical shape the viewer axis was ruled on, one crate over.
#
# WHY THESE THREE. `pncad-py` — the binding layer's own Rust and the suite's
# own .py/.pyi files, which live under that member directory. `pncad` — the
# façade every binding call goes through, and the one crate whose own source
# can change what the suite sees without any other crate moving. `editor-core`
# — the document model the suite drives through the façade (the guide and
# north-star tests build documents), and the one non-façade edge the bindings
# have. A kernel crate that `pncad` merely re-exports is deliberately NOT here:
# a breaking change to a re-exported type reddens the ordinary closure rows on
# the offending PR, and a change in its NUMBERS is what the nightly re-take is
# for.
#
# Adding a name here is a decision about what can change the wheel's observable
# behaviour without touching the three above; it is not a convenience. The
# nightly lane re-takes the whole suite daily, which is what makes the set safe
# to keep small.
PNCAD_PY_SEEDS: frozenset[str] = frozenset({"pncad-py", "pncad", "editor-core"})

# THE MATRIX. Every point of LANES x EPS_ROWS runs on every hosted code-tier
# run (2026-09-04); these two lists are also the legal values a request may
# narrow a run to, and ci.yml's eps matrix legs are the same three rows.
#
# LANES are the two COMPILE MODES. `interval` is not a subset lane here: it
# runs the WHOLE suite, not the interval-gated difference that
# `scripts/interval-only-selection.py` computes for the local half. That
# subtraction was correct when both lanes ran on every hosted push and the
# overlap was pure re-execution; it was reverted for hosted on 2026-08-22
# because a sampled run drew ONE lane and the subtracted ~93% would then have
# been gated by nothing. Both lanes run again — so the subtraction's original
# premise holds again and its reversal's does not. It stays reverted here:
# restoring it would REDUCE what a run gates, which is a cost lever with its
# own argument to make and not part of un-sampling. Filed as
# work/ciw/interval-only-selection-premise-restored.md.
LANES: tuple[str, ...] = ("default", "interval")

# EPS rows straddle the compiled default (DEFAULT_EPS = 1e-9) three orders
# either side, and `default` means the variable genuinely UNSET — an empty
# CAD_TOLERANCE_EPS is a parse error by design (geom-core/src/tolerance.rs).
# All three run on every hosted run, as three matrix legs over ONE archive:
# eps is runtime env, so they execute bit-identical binaries.
EPS_ROWS: tuple[str, ...] = ("default", "1e-6", "1e-12")

# `k-lint (gate)`'s FIVE FEATURE UNIFICATIONS. ALL FIVE RUN ON EVERY RUN
# (2026-09-04); until that day one was drawn per run from the head SHA. ci.yml
# expands `KLINT_ROW=all` into five matrix legs, one per row below, and the
# CONFIGURATION COVERAGE note at the top of this file carries the argument and
# the miss that ended the draw.
#
# NO COST FIGURE IS RESTATED HERE. This comment used to say the job "bills 8-10
# minutes", and so did ci.yml's `k-lint` header; both were quoting a
# PRE-SAMPLING column of docs/CI-MINUTES-2026-08.md as though it were current,
# and that document's 2026-08-31 addendum states the rule that settles it — a
# billed-minute figure there is maintained BY HAND and is only true as of the
# measurement it names a run id for. What matters at this site is the SHAPE:
# five unifications that share almost no artifacts, because `--release` and dev
# are different profiles and `budget` and `probe` are opt-in features gated at a
# module boundary, so each is its own fingerprint for every crate that sees it.
# That shape is why the rows are five parallel matrix legs rather than five
# passes through one job, and why no cache configuration collapses them.
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
# `Swatinem/rust-cache` entry on it, so the two dev legs and the two release
# legs each share a cache lane instead of one lane thrashing between profiles.
# Renaming a row means reading that expression.
#
# WHAT SAMPLING THEM COST, kept because it is the argument against putting the
# draw back. The five are all PERSISTENCE-detectors — a clippy finding, a
# failed assertion, a grown triangle budget, a probe suite that stopped
# compiling all stay broken until someone fixes them — which is what licensed
# drawing one, and each was audited against that rule individually (2026-08-22)
# rather than as a group. The rule is sound and it was never the whole story:
# what a draw gives up is WHOSE merge finds the break, and the answer was
# whoever next drew the row. `#1756` -> `#1775` is that bill paid in full, and
# `demos/` is where it landed, because the demo roots are excluded workspaces
# that no `--workspace` check reaches and the drawn row was their only gate.
#
# TWO RATIFIED REVIEW OUTCOMES NAMED ROWS HERE AS UNCONDITIONAL, and from
# 2026-08-22 to 2026-09-04 they were not. Both are unconditional again: MIN-1's
# per-triangle certificate falsifier (`dev-budget`), and
# `crates/sweep/tests/k_report.rs` + docs/K-REPORT.md's "on every building
# merge" (`dev-probe`). No gate reds on a frequency claim — the census greps for
# the STEP NAME, not for how often it runs — so every one of those sites is
# corrected by hand, in the same PR that makes the correction true.
KLINT_ROWS: tuple[str, ...] = (
    "dev-default", "release-default", "release-budget", "dev-budget", "dev-probe",
)


# THE `tools/` PATH PIN STOOD HERE AND IS DELETED (2026-09-04), with
# `KLINT_PATH_ROWS`, `KLINT_PIN_ROOTS`, `KLINT_PIN_FALLBACK` and
# `_forces_klint`. It substituted the row that RUNS a changed tool crate's own
# suite ahead of the seeded draw (Ev's ruling, 2026-08-29) and failed closed
# into `all` on an unresolvable file list. A run that gates every row has
# nothing left for it to do: a pin over an un-sampled dimension can only
# re-state the default or narrow it, and no path in this file narrows a gate.
# The ruling's subject — a guard living in a tool crate's own tests, executed
# by exactly one of the five rows — now holds on every run instead of on the
# runs a hand-derived mapping remembered to cover. This is `_forces_interval`'s
# tombstone one dimension over, and for the same reason.

# THE LANE'S PIN STOOD HERE AND IS DELETED (2026-09-04). `_forces_interval`
# substituted `LANE=interval` ahead of the seeded draw for a change under
# `interval-transcendentals/`, or for a file list nothing could resolve. It
# existed to stop a DRAW from missing the axis a change was about, and the
# draw is gone: every run gates both compile modes, so the pin could only ever
# re-state what the default already says, and a request naming `lane` beat it
# anyway. What it can no longer do is what #1122 caught it doing — gate a
# whole branch on one axis for its entire life, invisibly. `_advises_interval`
# below is what survives of that ruling.

# THE ADVICE THAT REPLACED THE PIN. Same name-shaped observation, stripped of
# the authority it should never have had: this changes NOTHING about what runs.
# ITS POPULATION NARROWED TO ONE CASE WHEN THE LANE STOPPED BEING SAMPLED
# (2026-09-04) and it is kept for that case rather than retired with the draw.
# A default run gates both compile modes, so nobody can miss the interval lane
# by accident any more; what remains is missing it ON PURPOSE — a dispatch
# aimed at `lane: default` over a diff whose filenames say interval, which is
# the only spelling of that left. That is someone narrowing away the
# axis their own diff touches, and it is worth one line of stderr. A name can
# raise the question; only the author can answer it.
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


def _gated_code_only(text: str) -> str:
    """`text` with `//`-to-end-of-line comments removed.

    A MARKER IS AN ITEM AND NEVER A COMMENT, and the difference has to be
    drawn or the macro's own documentation reads as a marker: three files here
    quote the spelling in prose without calling it (the macro's docs, the gate
    that describes it, the reader census that subtracts what a marker
    declares). This is a line cut, not a lexer — a `//` inside a string
    literal truncates the line early, which can only LOSE a call and never
    invent one, and losing one leaves its suite running.
    """
    return "\n".join(
        line if (i := line.find("//")) < 0 else line[:i] for line in text.splitlines()
    )


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
                    paths = _marker_paths(_gated_code_only(text))
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
                        if want.endswith("/"):
                            if not os.path.isdir(target):
                                problem = f"{want!r} does not exist in the tree as a directory"
                                break
                        elif not os.path.isfile(target):
                            # A DIRECTORY WRITTEN WITHOUT ITS TRAILING SLASH is
                            # the one near-miss that would otherwise pass a
                            # bare existence test and then match no changed
                            # file ever — the path is compared for EQUALITY
                            # unless it ends in `/`, and no changed file equals
                            # a directory.
                            extra = (
                                " — it is a DIRECTORY: name it with a trailing `/`, which is "
                                "what makes it match everything under it"
                                if os.path.isdir(target)
                                else ""
                            )
                            problem = f"{want!r} does not exist in the tree{extra}"
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


def gated_check(root: str) -> int:
    """`--gated-check`: every marker in the tree resolves. The LOUD half.

    THE FILTER FAILS OPEN AND THIS DOES NOT, and that division is the whole
    design. A marker whose paths have been renamed out from under it still
    RUNS its suite — nothing is lost, only minutes are spent — which means
    nothing about the run says the marker is broken. Left there, the marker
    would stay broken, and the next reader would believe a gate that had
    quietly become "always runs". So the state is made loud somewhere it stops
    a merge instead: `scripts/gates/gated-suite-paths.sh`, in the `discipline`
    row of both halves.

    It is also what stops a rename going the OTHER way. A marker naming a path
    that no longer exists is one edit away from naming a path that never runs,
    and `--gated-set`'s nightly row refuses such a tree outright — so without
    this row a renamed source file would red the NIGHTLY, hours later and in
    someone else's lane.
    """
    problems: list[str] = []
    try:
        dir_of, _ = _members(root)
    except Exception as exc:  # noqa: BLE001 — the directory name stands in
        print(f"ci-filter: gated-check: no member map ({exc})", file=sys.stderr)
        dir_of = None

    # MARKERS SITED WHERE NOTHING WILL EVER READ THEM. `_scan_gated` walks
    # `crates/*/{src,tests}` because those are the two shapes a nextest term
    # can be derived for; a marker anywhere else is not a narrower gate, it is
    # a comment, and it reads exactly like the real thing to its author.
    for dirpath, dirs, names in os.walk(root):
        dirs[:] = sorted(d for d in dirs if d not in (".git", "target", "node_modules"))
        for name in sorted(names):
            if not name.endswith(".rs"):
                continue
            full = os.path.join(dirpath, name)
            rel = os.path.relpath(full, root).replace(os.sep, "/")
            parts = rel.split("/")
            sited = (
                len(parts) > 3
                and parts[0] == "crates"
                and parts[1] != "test-utils"
                and parts[2] in ("src", "tests")
            )
            if sited:
                continue
            with open(full, encoding="utf-8", errors="replace") as fh:
                text = fh.read()
            # THE SAME RECOGNISER THE SCANNER USES, and it has to be. A bare
            # substring test fires on the macro's own NAME, which is written
            # without being called in three legitimate places — the macro's
            # docs, its definition, and `reader_census.rs`, which counts the
            # paths a marker declares so it can subtract them. A gate that
            # reds on a file for saying the word is a gate authors route
            # around.
            if not _GATED_CALL_RE.search(_gated_code_only(text)):
                continue
            if rel.startswith("crates/test-utils/"):
                problems.append(
                    f"{rel}: a gated_to! call inside crates/test-utils/, which is the "
                    "marker's own home and is never scanned — nothing would ever read it"
                )
            else:
                problems.append(
                    f"{rel}: a gated_to! call outside crates/<crate>/src/ and "
                    "crates/<crate>/tests/. Those are the two shapes a nextest term can "
                    "be derived for, so a marker here gates nothing while reading like "
                    "one that does"
                )

    suites = _scan_gated(root, dir_of)
    for suite in suites:
        if suite.problem is not None:
            problems.append(f"{suite.path}: {suite.problem}")
        # A MARKER ON A FILE THAT HOLDS NO TEST gates an empty set. Harmless to
        # the run and misleading to every reader of it, which is the class this
        # whole unit is about.
        with open(os.path.join(root, suite.path), encoding="utf-8", errors="replace") as fh:
            body = fh.read()
        if "#[test]" not in body and "#[cfg(test)]" not in body:
            problems.append(
                f"{suite.path}: carries a marker and no `#[test]` or `#[cfg(test)]`. "
                "The term it derives selects nothing, so the marker gates nothing and "
                "says otherwise"
            )

    if problems:
        for p in problems:
            sys.stderr.write(f"    {p}\n")
        sys.stderr.write(
            "\nA marker names the source paths its suite is SPECIFIC TO, repo-relative, "
            "files or directories with a trailing `/`; the suite's own file is implicit "
            "and is never listed. Fix the path, or delete the marker deliberately — an "
            "unresolvable one leaves the suite running on every pull request while "
            "reading as gated, and reds the nightly's ungated re-take.\n"
        )
        raise SystemExit(
            "error: {} problem(s) in {} gated-suite marker(s)".format(len(problems), len(suites))
        )
    print(
        "gated-suite markers OK: {} suite(s), every named path resolves in the tree "
        "and every term derives".format(len(suites))
    )
    return 0


# `_sample` STOOD HERE AND IS DELETED (2026-09-04). It was the seeded draw —
# `sha256(salt + seed) % len(choices)`, salted per dimension so that two draws
# off one seed were not the same number — and the k-lint row was its last
# caller. Nothing in this file is chosen for anyone any more, so there is no
# salt to keep independent and no seed to keep. What the salt cost when it was
# missing is written into the record rather than into a helper nobody calls:
# lane, eps and k-lint off one unsalted digest would have been the SAME number,
# making 20 of those 30 points unreachable forever. Whoever adds a draw back
# owes that argument again, from scratch.


class ConfigError(Exception):
    """A configuration request that names no real point of the matrix."""


# THE DIMENSIONS A HUMAN CAN NAME: what they write -> (output key, legal
# values). The legal sets are NOT the sampled tuples: each is the sampled
# tuple plus whatever "every row of this dimension" is spelled as in the job
# conditions that read it — `both` for the lane, `all` for the k-lint row.
#
# EPS GAINED ITS MEMBER ON 2026-09-04, and the asymmetry it used to have is
# worth recording because it was real rather than an oversight. `EPS=all` was
# a LOCAL word: ci-local.sh loops the rows, while the hosted rows interpolated
# the value straight into CAD_TOLERANCE_EPS, where `all` is a parse error by
# design (geom-core/src/tolerance.rs) — so requesting it hosted asked for a run
# whose test rows could not start. ci.yml now expands `all` into three matrix
# legs and interpolates one ROW per leg, so nothing ever puts the word in the
# variable, and `all` is what an un-narrowed run prints.
CONFIG_DIMENSIONS: dict[str, tuple[str, tuple[str, ...]]] = {
    "lane": ("LANE", (*LANES, "both")),
    "eps": ("EPS", (*EPS_ROWS, "all")),
    "klint": ("KLINT_ROW", (*KLINT_ROWS, "all")),
}


def parse_config(tokens: list[str], source: str) -> dict[str, tuple[str, str]]:
    """`["lane=interval", ...]` -> `{"LANE": ("interval", source)}`, or raise.

    Raises rather than skipping: see the docstring's REQUEST section — an
    input error is the one failure here that must not fail open.

    ONE CALLER, ONE AUTHORITY (2026-09-04). This used to take `additive_only`
    for the commit-trailer spelling, which could add but never narrow; that
    spelling is deleted, so every request reaching here was typed by whoever is
    standing there now and MAY narrow.
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


def decorate(
    res: dict[str, str],
    files: list[str] | None = None,
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
    # THE VIEWER TOOLKIT AXIS — SEED-KEYED, NOT CLOSURE-KEYED (Ev,
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
    # THE PYTHON SUITE — SEED-KEYED FOR THE SAME REASON, AND ARGUED THE SAME
    # WAY (Ev, in chat 2026-09-03; S-TCOST unit C3). It sat in JOB_ROOTS
    # above until then, keyed on `pncad-py in the dependent closure`.
    #
    # WHY THE CLOSURE WAS THE WRONG CONDITION. `pncad-py` depends on `pncad`,
    # which re-exports the entire kernel, so almost every kernel change puts
    # `pncad-py` in its closure. The condition was therefore true on nearly
    # every code-tier run, which is a gate that selects nothing — and what it
    # bought on each of those runs was a SECOND compile of the kernel, under
    # the non-default `python` feature (pyo3 plus four more crates, its own
    # cache lane), to run a suite whose subject had not moved.
    #
    # WHY THESE SEEDS ARE ENOUGH. The suite exercises the bindings' own
    # surface: the .pyi lattice, the guide and north-star scripts, and the
    # façade calls they make. `pncad-py` is that code; `pncad` is the façade it
    # calls; `editor-core` is the document model those scripts drive. A change
    # in any OTHER kernel crate reaches the suite only through `pncad`'s
    # re-exports — and a breaking one there reddens that crate's ordinary
    # closure rows on the offending PR, in the same run, because the Rust side
    # of the façade is compiled by the ordinary build. What the seeds give up
    # is a change in kernel NUMBERS that the python suite's own assertions
    # would have caught first, and that is what the nightly re-take exists for.
    #
    # RECORDED, NEVER SILENT (the KLINT_ROW lesson, and the viewer axis's own
    # rule): this is an output key, the filter echoes it with the seeds it was
    # computed from, and ci.yml prints the verdict in a step that always runs.
    # A green job name over a skipped job is the failure mode this shape exists
    # to avoid — and it is worse here than for the viewer rows, because a
    # SKIPPED job shows no steps at all.
    if tier == "docs":
        res["RUN_PNCAD_PY"] = "false"
    elif tier == "all":
        # Unscopable: no seed information, so the axis fails OPEN like every
        # other signal here.
        res["RUN_PNCAD_PY"] = "true"
    else:
        seeds = set(s for s in res.get("SEEDS", "").split(",") if s)
        res["RUN_PNCAD_PY"] = "true" if seeds & PNCAD_PY_SEEDS else "false"
    # THE CONFIGURATION IS THE LAST WORD AND READS NOTHING ABOVE IT: which
    # points of the matrix a run gates is independent of which rows the change
    # filter selected, and keeping the two apart is what lets the local gate
    # consume the same output while ignoring these keys entirely.
    #
    # NOTHING HERE READS A SEED (2026-09-04). Every run gates every compile
    # mode, every tolerance row and every k-lint unification; a request below is
    # the only thing that narrows any of them, and ci.yml expands the two `all`s
    # into matrix legs.
    res["LANE"], res["EPS"], res["KLINT_ROW"] = "both", "all", "all"
    # THE REQUEST IS THE LAST WORD OF THE LAST WORD, and it is recorded in the
    # same breath. A run that gates less than the whole matrix is only honest
    # if the output says so: CONFIG_SOURCE is per-dimension because the mixed
    # case is the common one — one dimension narrowed, the others whole.
    #
    # `unsampled` IS THE WORD FOR "THE WHOLE DIMENSION RUNS", and it is now the
    # standing value for lane and eps on every hosted run, not just the local
    # half's seedless one. It was already that word before 2026-09-04 and is
    # not re-spelled: a reader who learned it on a `ci-local.sh` run reads the
    # same thing here, and the value beside it (`LANE=both`, `EPS=all`) says
    # the same in the machine-readable half.
    source = dict.fromkeys(
        (key for key, _ in CONFIG_DIMENSIONS.values()), "unsampled"
    )
    for out_key, (value, src) in (config or {}).items():
        res[out_key] = value
        source[out_key] = src
    res["CONFIG_SOURCE"] = " ".join(
        f"{name}:{source[out_key]}" for name, (out_key, _) in CONFIG_DIMENSIONS.items()
    )
    # THE ADVISORY, AND WHY IT IS COMPUTED LAST. It fires only when this run is
    # NOT going to gate the interval lane, which is knowable only after the
    # request has had its say — advising someone to ask for a lane the run
    # already gates is noise, and noise is how a real notice stops being read.
    # Since 2026-09-04 `LANE` is `default` only when a request made it so, so
    # this condition now selects exactly the narrowed run; it is left as a test
    # of the VALUE rather than of the source, because what makes the notice
    # worth printing is that the interval lane is not running.
    # A BOOLEAN, not the reason: the reason is a path, and a path has no
    # business in a stream both halves parse as KEY=value.
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


# THE PYTHON SUITE'S FIXTURE, same shape and same argument one crate over:
#
#   pncad-py -> pncad -> editor-core -> topo
#
# so a `topo` change puts `pncad-py` in the CLOSURE while seeding neither it
# nor either crate between — precisely what a closure-keyed test gets wrong,
# and precisely the population this axis is meant to stop paying a second
# kernel compile for.
_PY_FIXTURE_PKGS = {
    "topo": [],
    "editor-core": [("topo", "normal")],
    "pncad": [("editor-core", "normal")],
    "pncad-py": [("pncad", "normal")],
}


def _plant_seed_axis_fixture(t: str, pkgs: dict[str, list] = _VIEWER_FIXTURE_PKGS) -> str:
    """A minimal workspace exercising a SEED-keyed axis — `RUN_VIEWER_TOOLKIT`
    by default, `RUN_PNCAD_PY` with `_PY_FIXTURE_PKGS`.

    ONE PLANTER, TWO GRAPHS, and they stay separate graphs deliberately: the
    viewer cases assert an exact PKGS closure, so growing one fixture to serve
    both axes would make every one of those expectations a statement about the
    other axis's dependency edges.
    """
    import shutil

    for pkg in pkgs:
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
            for pkg, deps in pkgs.items()
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


def _selftest_run(t: str, argv: list[str], stdin: str = "", allow_fail: bool = False):
    """One invocation of this script as a SUBPROCESS, both streams kept.

    Separate from `_selftest_invoke` because stdout and stderr carry
    different contracts here — stdout is the machine-readable KEY=value
    stream, stderr is what a human reads — and the advisory battery is the one
    case that has to look at the second.

    `allow_fail` returns the completed process instead of raising, for the one
    kind of case that is ABOUT a refusal: a retired option, whose whole point
    is that it exits non-zero rather than being ignored.
    """
    env = dict(os.environ)
    env["PATH"] = os.path.join(t, "bin") + os.pathsep + env.get("PATH", "")
    r = subprocess.run(
        [sys.executable, os.path.join(t, "scripts", "ci-filter.py"), *argv],
        input=stdin, capture_output=True, text=True, env=env, cwd=t,
    )
    if r.returncode != 0 and not allow_fail:
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
        _plant_seed_axis_fixture(t)
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

    # --- THE PYTHON SUITE AXIS, on `_PY_FIXTURE_PKGS`. Same rule, same two
    # directions: without the closure-only case a closure-keyed implementation
    # passes this battery, and the axis would be unenforced.
    with tempfile.TemporaryDirectory() as t:
        _plant_seed_axis_fixture(t, _PY_FIXTURE_PKGS)
        _files_case(t, "the binding crate's own sources buy the python suite",
                    ["crates/pncad-py/src/lib.rs"],
                    TIER="closure", SEEDS="pncad-py", RUN_PNCAD_PY="true")
        # THE SUITE'S OWN FILES. They are .py and .pyi under the member
        # directory, so they are neither docs nor Rust — and they seed the
        # member like any other source, which is the arm that keeps a
        # test-only edit from skipping the job that runs it.
        _files_case(t, "the suite's own .py files buy it",
                    ["crates/pncad-py/tests/test_guide.py"],
                    TIER="closure", SEEDS="pncad-py", RUN_PNCAD_PY="true")
        _files_case(t, "the facade's own sources buy it",
                    ["crates/pncad/src/lib.rs"],
                    TIER="closure", SEEDS="pncad", RUN_PNCAD_PY="true")
        _files_case(t, "the document model's own sources buy it",
                    ["crates/editor-core/src/doc.rs"],
                    TIER="closure", SEEDS="editor-core", RUN_PNCAD_PY="true")
        # THE CASE THAT MATTERS, and the whole reason this key left JOB_ROOTS:
        # `topo` is under `pncad-py` through two crates, so `pncad-py` is in
        # the closure and is not a seed. A closure-keyed axis says true here —
        # on nearly every kernel change — and buys a second kernel compile
        # under the `python` feature for a suite whose subject held still.
        _files_case(t, "a kernel crate reaching pncad-py only through the closure does NOT",
                    ["crates/topo/src/lib.rs"],
                    TIER="closure", PKGS="editor-core,pncad,pncad-py,topo",
                    SEEDS="topo", RUN_PNCAD_PY="false")
        # Fails OPEN with the rest of the filter.
        _files_case(t, "an unscopable change runs the python suite",
                    ["Cargo.toml"], TIER="all", SEEDS="", RUN_PNCAD_PY="true")
        _files_case(t, "a docs-only change runs nothing, the python suite included",
                    ["README.md"], TIER="docs", SEEDS="", RUN_PNCAD_PY="false")

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
        _selftest_lane_unsampled(t)
    # --- THE REQUEST PATH THROUGH THE CLI. `_selftest_config` covers the
    # applier as a function; what only a subprocess can show is the wiring —
    # that the flag reaches it, that a bad request exits NONZERO rather than
    # printing a fallback, that the DELETED second spelling reds rather than
    # being quietly ignored, and that `--force-all` returns a tier without
    # touching a diff. All of it is what ci.yml actually invokes.
    with tempfile.TemporaryDirectory() as t:
        _plant_fixture(t)
        _expect("a requested point must reach the output through the flag",
                _selftest_invoke(t, ["--files", "-",
                                     "--config", "lane=interval", "eps=1e-12"],
                                 "crates/geom-core/src/lib.rs\n"),
                {"LANE": "interval", "EPS": "1e-12",
                 "CONFIG_SOURCE": "lane:requested eps:requested klint:unsampled"})
        # `--config-from-message` IS AN ERROR NOW, on the same terms as
        # `--seed`: the trailer spelling is deleted, and an option that took a
        # commit message and ignored it would read, to every caller copied from
        # an older brief or an older ci.yml, as a trailer that still configures
        # the run. There is nothing left for it to mean.
        with open(os.path.join(t, "msg.txt"), "w") as fh:
            fh.write("topo: a commit\n\nCI-Config: lane=both\n")
        stale = _selftest_run(t, ["--files", "-", "--config-from-message", "msg.txt"],
                              "crates/geom-core/src/lib.rs\n", allow_fail=True)
        if stale.returncode == 0:
            raise SystemExit("SELFTEST FAILED: `--config-from-message` was accepted. The commit "
                             "trailer is deleted; accepting the flag tells a caller their "
                             f"trailer still configures the run\n{stale.stdout}")
        err = _selftest_invoke_must_fail(
            t, ["--files", "-", "--config", "eps=1e-13"],
            "crates/geom-core/src/lib.rs\n")
        if "1e-13" not in err:
            raise SystemExit(f"SELFTEST FAILED: the refusal must name the value refused: {err!r}")
        # `--force-all` takes no diff, so the path-keyed signals fail CLOSED:
        # the oracle tier runs. Neither LANE nor KLINT_ROW is one of those
        # signals any more — they are `both` and `all` because they always are,
        # which is what the old `interval` and pinned-`all` expectations here
        # were standing in for.
        _expect("--force-all must return the all tier with no diff taken",
                _selftest_invoke(t, ["--force-all"]),
                {"TIER": "all", "RUN_BUILD": "true", "RUN_INTERVAL_ORACLE": "true",
                 "LANE": "both", "KLINT_ROW": "all"})

    _selftest_docs_premise()
    _selftest_eps_rows_workflow()
    _selftest_klint_workflow()
    _selftest_unsampled()
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
        "not on their prose; NO CONFIGURATION DIMENSION IS SAMPLED OR PINNED — LANE=both, "
        "EPS=all and KLINT_ROW=all over an ordinary diff, over the two file lists that used "
        "to pin the lane or the k-lint row, over the demo roots the k-lint pin left alone "
        "and over an unresolvable change set, each recorded as "
        "`lane:unsampled eps:unsampled klint:unsampled` so the value and the source agree, "
        "and `--seed` is refused rather than ignored; ci.yml's eps and k-lint matrix "
        "literals are both re-derived against EPS_ROWS and KLINT_ROWS rather than kept in "
        "step by a comment, and the k-lint job's step conditions are required to name every "
        "row of that matrix and no others — read off `if:` keys alone, so a row named only "
        "in a comment does not count as gating anything — while the job's matrix axis is "
        "required to read `needs.filter.outputs.klint_rows`, so the literal re-derived is "
        "the list the job expands and a leg cannot report green over no steps; a "
        "request NARROWS any one dimension, leaves the rest whole, is recorded as "
        "`requested`, and `eps=all` / `klint=all` are legal because they are what an "
        "un-narrowed run prints; LANE_ADVISORY fires on exactly the run a request "
        "narrowed to the default lane over interval-named files — naming every such "
        "file rather than the first and saying it was narrowed — and stays silent on "
        "an un-narrowed run and a diff naming no such file, while --notices carries it "
        "to a relay file and is truncated when there is none; and a "
        "configuration REQUESTED by hand — by the one spelling left, `--config`, where "
        "ci.yml's workflow_dispatch inputs land — reaches the dimension it names "
        "and only that one, is recorded in CONFIG_SOURCE, and "
        "reds the step rather than falling back when it names no real point, "
        "while `--config-from-message` is refused rather than ignored; "
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


def _selftest_unsampled() -> None:
    """THAT NOTHING HERE IS CHOSEN FOR YOU, which is the claim no single run's
    output can support.

    A RE-INTRODUCED DRAW WOULD NOT FAIL LOUDLY. Every run would still print a
    legal `LANE=`, `EPS=` and `KLINT_ROW=`, every job condition would still read
    them, and the gate would stay green while covering a fraction of what it
    says it covers — one point in six for the lane and eps, one row in five for
    k-lint. That is what this walks: the classification is required to come back
    whole over a spread of file lists, including the two that used to PIN a
    dimension and the tools/ path whose pin was deleted on 2026-09-04.

    THE SEED IS NOT PASSED BECAUSE THERE IS NOWHERE TO PASS IT. `decorate` no
    longer takes one, so a draw could only come back by someone adding a
    parameter — and the values below are what would catch it if they did.

    Deterministic and in-process: `decorate` is a pure function of (result,
    files, config), so a subprocess would only be slower."""
    base = {"TIER": "closure", "PKGS": "geom-core", "CARGO_SCOPE": "-p geom-core"}
    lists: list[tuple[str, list[str] | None]] = [
        ("an ordinary crate change", ["crates/geom-core/src/lib.rs"]),
        ("the tree the lane pin used to read", ["interval-transcendentals/src/lib.rs"]),
        ("an interval-NAMED source", ["crates/topo/src/ring_interval.rs"]),
        ("the tree the k-lint pin used to read", ["tools/tess-meter/src/main.rs"]),
        ("a tools/ path no mapping ever named", ["tools/notyet/src/main.rs"]),
        ("the demo roots the pin deliberately left alone", ["demos/tour/src/main.rs"]),
        ("an unresolvable change set", None),
    ]
    for label, files in lists:
        got = decorate(dict(base), files)
        for key, want in (("LANE", "both"), ("EPS", "all"), ("KLINT_ROW", "all")):
            if got[key] != want:
                raise SystemExit(
                    f"SELFTEST FAILED: {label} gated {key}={got[key]!r}, want {want!r}. No "
                    "dimension here is sampled or pinned (2026-09-04, Ev's two authorisations): "
                    "every run gates every compile mode, every tolerance row and all five k-lint "
                    "unifications, and a draw or a pin returning here would silently hand back a "
                    "gate that covers a fraction of what its job names say")
        if got["CONFIG_SOURCE"] != "lane:unsampled eps:unsampled klint:unsampled":
            raise SystemExit(
                f"SELFTEST FAILED: {label} reported {got['CONFIG_SOURCE']!r}. The value and the "
                "source have to agree, or a reader answering `which configuration gated this "
                "commit` off the outputs gets two different answers")


def _selftest_lane_unsampled(t: str) -> None:
    """THAT THE LANE AND THE EPS ROW ARE NOT CHOSEN FOR YOU, AND THE ONE NOTICE
    THAT SURVIVED THEIR DRAW.

    THE PIN AND THE DRAW ARE BOTH GONE (2026-09-04). `_forces_interval` used to
    run BEFORE the seeded draw and short-circuit it, so a branch that tripped
    it was on the interval lane for every push it ever made; #1122 is what that
    cost when the trip was a filename. Both are now unreachable states rather
    than removed code paths, and an unreachable state is exactly what a reading
    of this file cannot confirm — every run still prints a `LANE=` line, and a
    restored pin or draw would print a legal one. So the cases below assert the
    NEGATIVE, on the two file lists that used to trip the pin and the advisory:
    `interval-transcendentals/` and an interval-named source.

    WHAT SURVIVES IS THE ADVISORY, on a population of one: a run someone
    NARROWED to `lane=default` over a diff whose filenames say interval. It
    changes nothing about what runs and never did; what changed is that it can
    no longer be talking to someone who did not choose the lane.
    """
    # NO SEED IS PASSED AND NONE CAN BE: `--seed` was deleted with the last
    # draw (2026-09-04) and is now an unrecognised option. These cases run the
    # CLI exactly as ci.yml does.
    exact = _selftest_run(t, ["--files", "-"],
                          "interval-transcendentals/src/lib.rs\n")
    if "LANE=both" not in exact.stdout.splitlines():
        raise SystemExit("SELFTEST FAILED: a change under interval-transcendentals/ did not gate "
                         f"both lanes — this run gates one\n{exact.stdout}")
    if "PINNED" in exact.stderr or "lane:pinned" in exact.stdout:
        raise SystemExit("SELFTEST FAILED: the lane was PINNED. `_forces_interval` was deleted "
                         "with the draw it pre-empted; a pin here means a run gating one compile "
                         f"mode again\n{exact.stdout}\nstderr: {exact.stderr!r}")
    if "CONFIG_SOURCE=lane:unsampled eps:unsampled klint:unsampled" not in exact.stdout.splitlines():
        raise SystemExit("SELFTEST FAILED: a run reported a dimension as sampled or pinned — "
                         f"nothing here is either\n{exact.stdout}")

    # THE BASENAME CASE, ASSERTED TWICE OVER. `ring_interval.rs` is the shape
    # the deleted arm matched — an interval-named source, not a rename victim —
    # so if anything ever pins on a basename again, it pins here. And the
    # advisory must stay QUIET: this run gates the interval lane, and an
    # advisory that fires where it has nothing to say is one nobody reads where
    # it does.
    named = _selftest_run(t, ["--files", "-"],
                          "crates/topo/src/ring_interval.rs\n")
    if "LANE=both" not in named.stdout.splitlines() or "lane:pinned" in named.stdout:
        raise SystemExit("SELFTEST FAILED: a basename carrying `interval` moved the lane — that "
                         f"arm was REMOVED by the #1122 ruling and the draw is gone\n{named.stdout}")
    if "LANE_ADVISORY=false" not in named.stdout.splitlines():
        raise SystemExit("SELFTEST FAILED: the advisory fired on a run already gating the interval "
                         f"lane\n{named.stdout}")

    # THE ONE POPULATION THE ADVISORY HAS LEFT: a narrowed run. It fires, it
    # names EVERY interval-named file rather than the first, and it says how to
    # stop narrowing — all three are things only this script can see, which is
    # why the wording lives here and not in ci.yml's relay.
    notes = os.path.join(t, "notices.txt")
    narrowed = _selftest_run(
        t, ["--files", "-", "--config", "lane=default", "--notices", notes],
        "crates/topo/src/ring_interval.rs\ncrates/sweep/tests/extrude_interval.rs\n")
    if "LANE=default" not in narrowed.stdout.splitlines():
        raise SystemExit("SELFTEST FAILED: a requested lane did not narrow the run\n"
                         f"{narrowed.stdout}")
    if "LANE_ADVISORY=true" not in narrowed.stdout.splitlines():
        raise SystemExit("SELFTEST FAILED: a run narrowed to the default lane over a diff of "
                         f"interval-named files raised no advisory\n{narrowed.stdout}")
    if ("ring_interval.rs" not in narrowed.stderr or "extrude_interval.rs" not in narrowed.stderr
            or "2 file(s)" not in narrowed.stderr):
        raise SystemExit("SELFTEST FAILED: the advisory named fewer than all the interval files "
                         f"it matched\nstderr: {narrowed.stderr!r}")
    if "NARROWED" not in narrowed.stderr or "lane=default" not in narrowed.stderr:
        raise SystemExit("SELFTEST FAILED: the advisory did not say the lane was NARROWED BY A "
                         "REQUEST, which is the only way this run can be on one lane\n"
                         f"stderr: {narrowed.stderr!r}")

    # THE RELAY FILE, which is the only reason ci.yml no longer restates these
    # notices in its own prose. Two properties, and the second is the one a
    # reader would never think to check: the file CARRIES the notice, and it is
    # TRUNCATED when there is none — a relay that leaves the previous run's
    # notice in place announces something this run does not have, and the
    # consumer `cat`s it unconditionally.
    with open(notes) as fh:
        relayed = fh.read()
    if "ring_interval.rs" not in relayed:
        raise SystemExit("SELFTEST FAILED: --notices did not carry the advisory, so ci.yml's "
                         f"relay would print nothing where it used to print prose\n{relayed!r}")
    _selftest_run(t, ["--files", "-", "--notices", notes],
                  "crates/topo/src/lib.rs\n")
    with open(notes) as fh:
        if fh.read() != "":
            raise SystemExit("SELFTEST FAILED: --notices was not truncated on a run with no "
                             "notice — the relay would announce the PREVIOUS run's advisory")

    # THE EPS ROW, ON THE SAME TERMS. It is the dimension with no pin and no
    # advisory, so nothing else in this file would notice a draw returning to
    # it; `_selftest_unsampled` walks the file lists for that, and this is the
    # request half — narrowing works, and says it was a request.
    one_row = _selftest_run(t, ["--files", "-", "--config", "eps=1e-12"],
                            "crates/topo/src/lib.rs\n")
    if "EPS=1e-12" not in one_row.stdout.splitlines() or "eps:requested" not in one_row.stdout:
        raise SystemExit("SELFTEST FAILED: a requested eps row did not narrow the run, or did not "
                         f"say it was requested\n{one_row.stdout}")
    every_row = _selftest_run(t, ["--files", "-", "--config", "eps=all"],
                              "crates/topo/src/lib.rs\n")
    if "EPS=all" not in every_row.stdout.splitlines():
        raise SystemExit("SELFTEST FAILED: `eps=all` was refused. It became legal when the hosted "
                         "half started expanding it into three matrix legs, and refusing a value "
                         f"this script PRINTS as its own default is incoherent\n{every_row.stdout}")

    # `--seed` IS AN ERROR NOW, and that is deliberate rather than a leftover:
    # an option that accepted the head SHA and ignored it would read, to every
    # caller copied from an older brief, as a run that still draws.
    stale = _selftest_run(t, ["--files", "-", "--seed", "deadbeef"],
                          "crates/topo/src/lib.rs\n", allow_fail=True)
    if stale.returncode == 0:
        raise SystemExit("SELFTEST FAILED: `--seed` was accepted. It was deleted with the last "
                         "draw; silently ignoring it tells a caller their run still draws a "
                         f"k-lint row\n{stale.stdout}")


# THE JOB WHOSE ROWS THIS TUPLE FEEDS, and the one string in this file that
# names it. `_selftest_klint_workflow` reads the workflow's TEXT — the census
# gate reads ci.yml the same way, for the same reason: a claim nobody re-runs
# against its source is a transcription with a date on it.
KLINT_JOB_KEY = "k-lint"
KLINT_WORKFLOW = ".github/workflows/ci.yml"
# ONE ROW PER MATRIX LEG SINCE 2026-09-04, so a step names its row with a plain
# equality against `matrix.row` rather than with a `contains(fromJSON([...]))`
# list that had to carry `all` as its escape hatch. The set a step is gated on
# is still what this returns, so everything below reads the same way.
#
# ANCHORED TO `if:`, because the row set is read off CONDITIONS and not off
# mentions. An unanchored scan over a step's whole text counts a row named in
# a COMMENT inside that step, so a careless edit that deletes a real `if:` and
# leaves the comment behind still satisfies the union check below — which is
# the one failure this case exists to catch.
_KLINT_IF_RE = re.compile(r"matrix\.row\s*==\s*'([a-z0-9-]+)'")
_KLINT_MATRIX_RE = re.compile(r"^\s*klint_rows:.*?'(\[[^\]]*\])'", re.M)
# THE WIRE BETWEEN THE TWO ENDS. The matrix literal is checked above and the
# step conditions below, and neither of them reads `strategy.matrix.row` — so
# without this, rewriting line 3807 to a hand-typed `['dev-default']` leaves
# both halves green while three rows silently run nothing.
_KLINT_AXIS_RE = re.compile(
    r"^\s*row:\s*\$\{\{\s*fromJSON\(\s*needs\.filter\.outputs\.klint_rows\s*\)\s*\}\}\s*$",
    re.M)


def _klint_job_block(text: str) -> str:
    """The `k-lint` job's own block of ci.yml, and nothing else.

    Every job in this workflow indents identically, so a scan that is not
    bounded to one block attributes a neighbour's text to this job and the
    assertions built on it are then about the wrong file.
    """
    lines = text.split("\n")
    try:
        start = next(i for i, ln in enumerate(lines) if ln == f"  {KLINT_JOB_KEY}:")
    except StopIteration:
        raise SystemExit(
            f"SELFTEST FAILED: {KLINT_WORKFLOW} has no `{KLINT_JOB_KEY}:` job. KLINT_ROWS is "
            "the matrix that job fans out over; if it was renamed, re-derive against whatever "
            "replaced it rather than repointing this name"
        ) from None
    end = next(
        (i for i in range(start + 1, len(lines)) if re.match(r"^  [A-Za-z0-9_-]+:\s*$", lines[i])),
        len(lines),
    )
    return "\n".join(lines[start:end])


def _klint_job_steps(text: str) -> list[tuple[frozenset[str], str]]:
    """`(rows this step is gated on, the step's text)` for the k-lint job.

    Bounded to that job's own block: every other job in this workflow indents
    its steps identically, so an unbounded scan would attribute a neighbour's
    row to this one and the assertions below would be about the wrong file.
    A step with no `klint_row` condition comes back with an EMPTY row set
    rather than being dropped — "gated on nothing" is a real answer here (the
    checkout and cache steps are), and dropping it would let a row condition
    that was DELETED read as a step that never had one.

    ONLY the step's `if:` key is read for rows, never its comments or its
    `run:` body: a row is what GATES a step, and a mention of one is not.
    """
    steps: list[list[str]] = []
    for ln in _klint_job_block(text).split("\n"):
        if re.match(r"^      - \S", ln):
            steps.append([])
        if steps:
            steps[-1].append(ln)
    out: list[tuple[frozenset[str], str]] = []
    for body in steps:
        blob = "\n".join(body)
        out.append((frozenset(_KLINT_IF_RE.findall(_step_conditions(body))), blob))
    return out


def _step_conditions(body: list[str]) -> str:
    """The text of a step's `if:` key(s) and nothing else.

    A block or folded `if:` continues onto the lines indented deeper than the
    key itself, so those are taken too; anything at the key's indent or
    shallower ends it. Everything outside — comments, `name:`, `run:` — is
    dropped, so a row named anywhere but in a condition does not count as
    gating a step.
    """
    out: list[str] = []
    depth: int | None = None
    for ln in body:
        stripped = ln.lstrip()
        indent = len(ln) - len(stripped)
        if depth is not None:
            if stripped and indent <= depth:
                depth = None
            else:
                out.append(ln)
                continue
        if re.match(r"if:\s", stripped) or stripped == "if:":
            out.append(ln)
            depth = indent
    return "\n".join(out)


def _selftest_klint_workflow() -> None:
    """THE FIVE ROWS, CHECKED AGAINST THE JOB THAT RUNS THEM.

    `KLINT_ROWS` is printed by this script and validated against by every
    request; ci.yml carries the same five AGAIN, as the JSON array it expands
    `KLINT_ROW=all` into, because a matrix dimension has to be a list and this
    script's output is a stream of words. `EPS_ROWS` has exactly this problem
    and exactly this answer — read the workflow's TEXT and re-derive.

    THREE CLAIMS, and each is a sentence written elsewhere in this file that
    would otherwise be true only on the day it was typed:

      * THE MATRIX. ci.yml's `klint_rows` literal names exactly `KLINT_ROWS`.
        A row in the tuple and not in the literal is a row this script will
        accept as a request and the workflow will expand into nothing; a row in
        the literal and not in the tuple is a leg no request can name and no
        `--selftest` here has ever seen.
      * THE WIRE. The k-lint job's matrix axis reads
        `fromJSON(needs.filter.outputs.klint_rows)` — i.e. the literal checked
        above is the literal the job actually expands. Without this the other
        two claims hold over a job whose axis was rewritten to a hand-typed
        list, and three rows run nothing while both ends read green.
      * THE STEPS. The set of rows the job's own step conditions name is
        exactly `KLINT_ROWS` too. This is the half that catches the failure the
        draw used to hide in plain sight: a leg that runs with no step gated on
        it is a green job reporting on nothing, and a row named by a step but
        absent from the matrix is a step that can never run — which is what
        `demos tour fmt + clippy` effectively was on four runs in five.

    WHAT IT STILL CANNOT SEE: whether the steps gated on a row are the RIGHT
    steps for it. That is the roster at `KLINT_ROWS` and the job's own
    comments; this is the mechanical shadow of it."""
    path = os.path.join(_repo_root(), KLINT_WORKFLOW)
    try:
        with open(path) as fh:
            text = fh.read()
    except OSError as exc:
        raise SystemExit(f"SELFTEST FAILED: {KLINT_WORKFLOW} cannot be read ({exc}); the k-lint "
                         "rows have no source to be re-derived against") from exc

    found = _KLINT_MATRIX_RE.findall(text)
    if len(found) != 1:
        raise SystemExit(
            f"SELFTEST FAILED: expected exactly ONE `klint_rows:` output carrying a JSON array "
            f"literal in {KLINT_WORKFLOW}; found {len(found)}. Two would be two lists to keep in "
            "step with KLINT_ROWS, which is the thing this case exists to prevent")
    try:
        rows = json.loads(found[0])
    except ValueError as exc:
        raise SystemExit(f"SELFTEST FAILED: {KLINT_WORKFLOW}'s k-lint matrix literal {found[0]!r} "
                         f"is not JSON ({exc}); the matrix would expand to nothing") from exc
    if tuple(rows) != KLINT_ROWS:
        raise SystemExit(
            f"SELFTEST FAILED: {KLINT_WORKFLOW} expands KLINT_ROW=all into {rows} and this "
            f"script's KLINT_ROWS is {list(KLINT_ROWS)}. One of them gates unifications the other "
            "does not name — change both, in the same commit")

    block = _klint_job_block(text)
    if not _KLINT_AXIS_RE.search(block):
        raise SystemExit(
            f"SELFTEST FAILED: the `{KLINT_JOB_KEY}` job in {KLINT_WORKFLOW} does not take its "
            "matrix axis from `${{ fromJSON(needs.filter.outputs.klint_rows) }}`. The literal "
            "re-derived above is then not the list the job expands, and a dispatch narrowing "
            "the `klint` input reaches nothing")

    steps = _klint_job_steps(text)
    in_workflow = frozenset().union(*(rows for rows, _ in steps)) if steps else frozenset()
    if in_workflow != frozenset(KLINT_ROWS):
        raise SystemExit(
            f"SELFTEST FAILED: KLINT_ROWS is {sorted(KLINT_ROWS)} and the `{KLINT_JOB_KEY}` job's "
            f"own step conditions name {sorted(in_workflow)}. A row in the matrix with no step "
            "gated on it is a leg that reports green over nothing; a row named by a step and not "
            "in the matrix is a step that can never run")

    gated = {row: sum(1 for rows, _ in steps if row in rows) for row in KLINT_ROWS}
    if min(gated.values()) < 1:
        raise SystemExit(
            f"SELFTEST FAILED: this job gates {gated}; a row with no steps is a matrix leg whose "
            "whole product is a green name")
    print(f"ci-filter selftest: {KLINT_WORKFLOW}'s k-lint matrix literal re-derives against "
          f"KLINT_ROWS — {', '.join(KLINT_ROWS)}; steps gated per row {gated}")


# THE ε ROWS HAVE TWO SPELLINGS AND THIS IS THE ONE THAT RECONCILES THEM.
# `EPS_ROWS` above is the tuple this script prints and validates requests
# against; ci.yml's `filter` job carries the same three rows AGAIN, as a JSON
# array literal, because a matrix dimension has to be a list and this script's
# output is a stream of words. A comment saying "keep these in sync" is what
# that arrangement usually gets, and a comment has never stopped a list
# drifting. `_selftest_klint_workflow` does the same thing one dimension over,
# against the same file, for the same reason.
#
# WHAT DRIFT WOULD LOOK LIKE WITHOUT IT: adding a fourth ε row here would print
# `EPS=all`, accept `eps=<new row>` as a request, and run three legs; deleting
# one would leave the workflow expanding a row this script refuses to name. Both
# are green runs gating a matrix nobody wrote down.
EPS_ROWS_WORKFLOW = ".github/workflows/ci.yml"
_EPS_ROWS_RE = re.compile(r"^\s*eps_rows:.*?'(\[[^\]]*\])'", re.M)


def _selftest_eps_rows_workflow() -> None:
    """ci.yml's eps matrix literal must name exactly `EPS_ROWS`."""
    path = os.path.join(_repo_root(), EPS_ROWS_WORKFLOW)
    try:
        with open(path, encoding="utf-8") as fh:
            text = fh.read()
    except OSError as exc:
        raise SystemExit(f"SELFTEST FAILED: {EPS_ROWS_WORKFLOW} cannot be read ({exc}); the eps "
                         "matrix literal is derived from that file and cannot be checked") from exc
    found = _EPS_ROWS_RE.findall(text)
    if len(found) != 1:
        raise SystemExit(
            f"SELFTEST FAILED: expected exactly ONE `eps_rows:` output carrying a JSON array "
            f"literal in {EPS_ROWS_WORKFLOW}; found {len(found)}. Two would be two lists to keep "
            "in step with EPS_ROWS, which is the thing this case exists to prevent")
    try:
        rows = json.loads(found[0])
    except ValueError as exc:
        raise SystemExit(f"SELFTEST FAILED: {EPS_ROWS_WORKFLOW}'s eps matrix literal {found[0]!r} "
                         f"is not JSON ({exc}); the matrix would expand to nothing") from exc
    if tuple(rows) != EPS_ROWS:
        raise SystemExit(
            f"SELFTEST FAILED: {EPS_ROWS_WORKFLOW} expands EPS=all into {rows} and this script's "
            f"EPS_ROWS is {list(EPS_ROWS)}. One of them gates rows the other does not name — "
            "change both, in the same commit")
    print(f"ci-filter selftest: {EPS_ROWS_WORKFLOW}'s eps matrix literal re-derives against "
          f"EPS_ROWS — {', '.join(EPS_ROWS)}")


def _selftest_config() -> None:
    """THE REQUEST PATH, in-process where it is a pure function and through the
    CLI where the wiring is.

    WHAT IS ACTUALLY AT RISK HERE, and it is not "does an override override".
    It is the SILENT failure a request can have: one that is READ BUT NOT
    APPLIED, or applied to the wrong dimension. Nothing reds; the run gates
    the wrong point and reports a green that answers a question nobody asked.

    So every legal value of every dimension is requested and checked, and the
    dimensions nobody named are required to still be the whole dimension.

    THE COMMIT-TRAILER SPELLING'S CASES STOOD HERE AND ARE DELETED WITH IT
    (2026-09-04): the regex near-misses, the case-insensitivity, the
    precedence pair and the additive-only refusals all asserted properties of
    a path that no longer exists. Its wiring is now covered by the one thing
    left to say about it — `--config-from-message` reds — in `selftest`."""
    files = ["crates/geom-core/src/lib.rs"]
    base = {"TIER": "closure", "PKGS": "geom-core", "CARGO_SCOPE": "-p geom-core"}
    unasked = decorate(dict(base), files)
    keys = [out_key for out_key, _ in CONFIG_DIMENSIONS.values()]

    if unasked["CONFIG_SOURCE"] != "lane:unsampled eps:unsampled klint:unsampled":
        raise SystemExit("SELFTEST FAILED: a run nobody narrowed must record every dimension as "
                         f"unsampled — CONFIG_SOURCE is {unasked['CONFIG_SOURCE']!r}")

    for name, (out_key, choices) in CONFIG_DIMENSIONS.items():
        for value in choices:
            got = decorate(dict(base), files, parse_config([f"{name}={value}"], "requested"))
            if got[out_key] != value:
                raise SystemExit(f"SELFTEST FAILED: {name}={value} was requested and {out_key} came "
                                 f"back {got[out_key]!r} — the request is being read and dropped")
            if f"{name}:requested" not in got["CONFIG_SOURCE"]:
                raise SystemExit(f"SELFTEST FAILED: {name}={value} was requested and CONFIG_SOURCE "
                                 f"says {got['CONFIG_SOURCE']!r} — an unrecorded override is a run "
                                 "that cannot be read back")
            for other in keys:
                if other != out_key and got[other] != unasked[other]:
                    raise SystemExit(f"SELFTEST FAILED: requesting {name} moved {other} as well "
                                     "— the dimensions nobody named must keep their default")

    # LOUD ON EVERY MALFORMED REQUEST.
    # `eps=all` is NOT in this list any more (2026-09-04): it became legal
    # when the hosted half started expanding it into three matrix legs. What
    # replaces it here is `eps=every`, a value that is still not one.
    for bad in (["lane"], ["lane="], ["=interval"], ["mode=interval"], ["lane=fast"],
                ["eps=every"], ["lane=default", "lane=interval"]):
        try:
            parse_config(bad, "requested")
        except ConfigError:
            continue
        raise SystemExit(f"SELFTEST FAILED: {bad} was accepted as a configuration request")


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
        "--gated-check",
        nargs="?",
        const="",
        metavar="DIR",
        help="check every gated-suite marker in DIR (default: this repo) — "
        "paths resolve, terms derive, markers are sited where they are read; "
        "reds on the first problem. scripts/gates/gated-suite-paths.sh is its "
        "wrapper under lib.sh's two-mode contract",
    )
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
    # `--seed` STOOD HERE AND IS DELETED (2026-09-04). It carried the head SHA
    # the k-lint row was drawn from; nothing is drawn any more, and an option
    # that accepted a value and ignored it would be worse than one that reds.
    ap.add_argument(
        "--config",
        action="extend",
        nargs="+",
        metavar="KEY=VALUE",
        help="narrow this run to a named point, e.g. "
        "`--config lane=interval eps=1e-12 klint=dev-probe`; unnamed "
        "dimensions keep their default, which is the WHOLE dimension (every "
        "lane, every eps row, every k-lint unification)",
    )
    # `--config-from-message` STOOD HERE AND IS DELETED (2026-09-04), on the
    # same terms as `--seed`. It read a `CI-Config:` trailer out of the head
    # commit's message, and once every dimension ran whole by default the
    # additive-only rule left it no value it could name that changed anything.
    # An option that read a message and ignored it would tell every caller
    # copied from an older brief that their trailer still configures the run.
    ap.add_argument(
        "--notices",
        metavar="FILE",
        help="also write the human notices (the interval advisory, the gated "
        "suites this run skips) to FILE, so a caller can relay them verbatim "
        "instead of restating them; truncated to empty when there are none",
    )
    args = ap.parse_args()
    if args.selftest:
        selftest()
        return 0

    if args.gated_check is not None:
        # Outside the fail-closed wrapper, like `--gated-set` and for the twin
        # reason: this mode's whole product is a verdict about the markers, so
        # a failure to reach one is the verdict, not a fallback.
        return gated_check(args.gated_check or _repo_root())

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
    out = decorate(res, files, config)

    # THE NOTICES ARE COMPOSED HERE AND WRITTEN TWICE, TO ONE WORDING. They go
    # to stderr, where the local half and anyone running this by hand sees
    # them, and — when `--notices` names a file — to that file, which ci.yml's
    # always-run configuration step relays VERBATIM. Before that relay existed
    # ci.yml restated the notices in its own prose, and the two copies had
    # already drifted twice: one said a pin's reason names a file, which the
    # fail-closed arm could not, and the other said "DEFAULT LANE DRAWN" over a
    # lane that had been requested. There is one wording now, and it is the one
    # that can see the values it is describing.
    #
    # THE PIN NOTICE STOOD HERE AND IS DELETED (2026-09-04) with `_forces_klint`
    # — the last pinned dimension — and with the `CONFIG_DIMENSIONS` loop that
    # composed it. `decorate` stays free of I/O either way, which is what let
    # `_selftest_sampling` call it thousands of times in-process.
    notices: list[str] = []

    # THE ADVISORY. It names EVERY interval-named file, and it fires only on a
    # run someone NARROWED to the default lane — an un-narrowed run gates both
    # compile modes and has nothing to advise about. It used to also report
    # whether that lane was drawn or asked for; there is no draw left, so the
    # word is gone rather than kept as a branch that can only take one arm.
    if out["LANE_ADVISORY"] == "true":
        hits = _advises_interval(files)
        shown = ", ".join(hits[:5]) + (f" (+{len(hits) - 5} more)" if len(hits) > 5 else "")
        notices.append(
            f"This diff touches {len(hits)} file(s) whose basenames carry `interval` "
            f"— {shown} — and this run was NARROWED to LANE=default by a request.\n"
            "  An un-narrowed run gates both compile modes, so this is the one way "
            "left to miss the interval lane, and you chose it.\n"
            "  IF INTERVAL SEMANTICS CHANGED, DROP THE NARROWING: re-run the "
            "workflow_dispatch without the `lane=default` input, or aim it at "
            "`lane: both`.\n"
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
