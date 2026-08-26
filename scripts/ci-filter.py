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
  LANE=default|interval|both    which COMPILE MODE this run gates (see below)
  EPS=default|<value>|all       which tolerance row this run gates
  KLINT_ROW=<unification>|all   which of `k-lint (gate)`'s five feature
                                unifications this run gates (see below)

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

NO SEED MEANS NO SAMPLING — LANE=both, EPS=all, KLINT_ROW=all. Fails OPEN into MORE work,
matching every other signal here. local-scripts/ci-local.sh passes no seed
and therefore still runs the whole matrix: it is not billed by the minute,
and with the hosted gate sampling, the local gate is now the only lane that
runs every point of the matrix on one tree.
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


def _members(root: str) -> tuple[dict[str, str], dict[str, set[str]]]:
    """Return (dir-name -> package name, package -> set of member deps).

    `--no-deps` reads the workspace manifests only: no registry resolution,
    no network, no lockfile update. Dependency kinds are all kept — normal,
    build, AND dev — because `cargo test -p X` builds X's dev-dependencies,
    so a dev-dep edge propagates a change just as a normal one does.
    """
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
        return {"TIER": "docs", "PKGS": "", "CARGO_SCOPE": ""}

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
    return {"TIER": "all", "PKGS": pkgs, "CARGO_SCOPE": "--workspace"}


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
# (2026-08-22). That job bills 8-10 minutes and the reason is not one slow
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
# WHAT IT COSTS, said out loud because two ratified review outcomes name
# these rows as UNCONDITIONAL and this makes them 1-in-5: MIN-1's certificate
# falsifier (dev-budget) and `crates/sweep/tests/k_report.rs` +
# docs/K-REPORT.md's "on every building merge" (dev-probe). Neither claim is
# checked by any gate — the census greps for the STEP NAME, not for how often
# it runs — so nothing goes red; the sentences simply become false in that
# one word, and are owed a correction.
KLINT_ROWS: tuple[str, ...] = (
    "dev-default", "release-default", "release-budget", "dev-budget", "dev-probe",
)

# WHEN THE INTERVAL LANE IS NOT LEFT TO CHANCE. A change to interval code is
# exactly the change whose interval lane a sampled run must not skip, and
# waiting an expected two runs to find that out is the one case where the
# sampling's latency lands on the author who could have been told immediately.
#
# The rule is PATH-SHAPED and rests on a naming convention this repo already
# keeps: every interval-specific test file in crates/*/tests carries
# `interval` in its basename (28 files at the time of writing), the two
# interval sources in geom-core are `interval.rs` and `ring_interval.rs`, and
# the backend is its own workspace root. It is a HEURISTIC over names, not a
# proof over the feature graph: a change to an interval-gated block inside a
# file with an ordinary name is not matched, and falls back to the sampling
# like anything else. That residue is acceptable precisely because the
# sampling is the floor — the rule only ever ADDS certainty, never removes it.
def _forces_interval(files: list[str] | None) -> bool:
    # Fail CLOSED like every other signal here: an unresolved file list cannot
    # prove interval code held still, so pin the lane rather than sample it.
    if not files:
        return True
    for f in files:
        if f.startswith("interval-transcendentals/"):
            return True
        if "interval" in f.rsplit("/", 1)[-1]:
            return True
    return False


def _sample(seed: str, salt: str, choices: tuple[str, ...]) -> str:
    """Deterministic choice from `choices`, keyed on (salt, seed).

    Salted per dimension so lane and eps are drawn independently — an
    unsalted second draw off the same seed would tie eps to lane and leave
    2 of the 6 matrix points unreachable forever.
    """
    digest = hashlib.sha256(f"{salt}\x00{seed}".encode()).digest()
    return choices[int.from_bytes(digest, "big") % len(choices)]


def decorate(
    res: dict[str, str],
    files: list[str] | None = None,
    seed: str | None = None,
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
    # Sampling is the LAST word and reads nothing above it: which point of
    # the matrix a run gates is independent of which rows the change filter
    # selected, and keeping the two apart is what lets the local gate consume
    # the same output while ignoring these two keys entirely.
    if seed is None:
        res["LANE"], res["EPS"], res["KLINT_ROW"] = "both", "all", "all"
    else:
        res["LANE"] = (
            "interval" if _forces_interval(files) else _sample(seed, "lane", LANES)
        )
        res["EPS"] = _sample(seed, "eps", EPS_ROWS)
        # A THIRD SALT, drawn off the same seed and independent of the other
        # two. `_sample`'s docstring says why the salt is not optional: two
        # dimensions off one unsalted digest are the same number, which would
        # tie the k-lint row to the lane and leave 20 of the 30 points of this
        # matrix unreachable for the rest of the project's life.
        res["KLINT_ROW"] = _sample(seed, "klint", KLINT_ROWS)
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


def _selftest_invoke(t: str, argv: list[str], stdin: str = "") -> dict[str, str]:
    env = dict(os.environ)
    env["PATH"] = os.path.join(t, "bin") + os.pathsep + env.get("PATH", "")
    r = subprocess.run(
        [sys.executable, os.path.join(t, "scripts", "ci-filter.py"), *argv],
        input=stdin, capture_output=True, text=True, env=env, cwd=t,
    )
    if r.returncode != 0:
        raise SystemExit(f"SELFTEST FAILED: {argv} exited {r.returncode}\n{r.stdout}{r.stderr}")
    out: dict[str, str] = {}
    for line in r.stdout.splitlines():
        k, _, v = line.partition("=")
        out[k] = v
    for key in ("TIER", "PKGS", "RUN_BUILD", "RUN_K_LINT", "RUN_INTERVAL_ORACLE"):
        if key not in out:
            raise SystemExit(f"SELFTEST FAILED: {argv} printed no {key} line\n{r.stdout}{r.stderr}")
    return out


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

    _selftest_docs_premise()
    _selftest_sampling()
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
        "not on their prose; and the three sampled dimensions fail open with no seed, "
        "repeat under the same seed, and are drawn independently enough that every one "
        "of the 30 matrix points is reachable"
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
    # NOT in the mutually-exclusive `src` group: the seed selects a matrix
    # point and is orthogonal to how the file list was obtained, so it rides
    # alongside --base or --files rather than instead of them.
    ap.add_argument(
        "--seed",
        help="head SHA to key the configuration sample on; omit to run the "
        "whole matrix (LANE=both, EPS=all, KLINT_ROW=all)",
    )
    args = ap.parse_args()
    if args.selftest:
        selftest()
        return 0
    root = _repo_root()

    # `None` until a file list is actually in hand, so that a failure ANYWHERE
    # below — including one that happens before `files` is ever bound — still
    # reaches the path-keyed signals as "unknown", which they read as run.
    files: list[str] | None = None
    try:
        if args.files:
            raw = sys.stdin.read() if args.files == "-" else open(args.files).read()
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
    for key, val in decorate(res, files, args.seed).items():
        print(f"{key}={val}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
