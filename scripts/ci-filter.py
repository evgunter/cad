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

Output: KEY=value lines on stdout, one per line, safe to append to
$GITHUB_OUTPUT and to parse with `while IFS='=' read -r k v`.

  TIER=docs|all|closure
  PKGS=<comma-separated members, empty for docs, all members for `all`>
  CARGO_SCOPE=--workspace | -p a -p b ...
  RUN_BUILD=true|false          any cargo/grep row at all (false only for docs)
  RUN_EDITOR_CORE=true|false    persistence / corpus / rebuild-latency rows
  RUN_STL=true|false            watertight (admesh) row
  RUN_STEP_EXPORT=true|false    step import (freecad) row
  RUN_PNCAD_PY=true|false       python suite (wheel + unittest) row
  RUN_INTERVAL_BACKEND=true|false   interval-transcendentals' own workspace
  RUN_INTERVAL_ORACLE=true|false    its oracle-inari certification tier
  RUN_TOPO_RELEASE=true|false   corrupt input (release profile) row
  RUN_K_LINT=true|false         k-lint (gate) row
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
    constants, joined. WHAT THIS DOES NOT SEE, said plainly because a
    disclosed blind spot is a work order: a page named any other way — an
    f-string, a glob, a name assembled at runtime, a path read from a fixture
    file. Those stay in the docs tier and their suite stays skippable. Nor is
    the leading `Name` checked to BE the repo root: a chain rooted at a test
    directory resolves to a path that exists nowhere and simply matches no
    diff. The shape in use is the shape checked.
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
            for node in ast.walk(tree):
                if not isinstance(node, ast.BinOp) or not isinstance(node.op, ast.Div):
                    continue
                chain = parts(node)
                if chain and chain[-1].endswith(".md"):
                    out.add("/".join(chain))
    return frozenset(out)


def _consumed_markdown(root: str) -> frozenset[str]:
    """Markdown a BUILD OR A SUITE consumes, from both directions."""
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
# step-import   runs FreeCAD over the COMMITTED fixtures in
#               crates/step-export/tests/fixtures (no cargo build at all),
#               which are byte-golden against the step-export writer.
# editor-core   persistence (D6.*), band-4 corpus (D1), rebuild latency —
#               all `cargo test -p editor-core --test ...`.
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


def decorate(res: dict[str, str], files: list[str] | None = None) -> dict[str, str]:
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
    # A page rustdoc compiles in, a page a python suite reads, and a page
    # that is only prose. The docs tier must separate these three.
    os.makedirs(os.path.join(t, "docs"), exist_ok=True)
    for page in ("GUIDE.md", "PYPAGE.md", "PROSE.md"):
        with open(os.path.join(t, "docs", page), "w") as fh:
            fh.write("prose\n")
    with open(os.path.join(t, "crates", "topo", "src", "guide.rs"), "w") as fh:
        fh.write('#![doc = include_str!("../../../docs/GUIDE.md")]\n')
    os.makedirs(os.path.join(t, "crates", "stl", "tests"), exist_ok=True)
    with open(os.path.join(t, "crates", "stl", "tests", "test_pages.py"), "w") as fh:
        fh.write('PAGE = ROOT / "docs" / "PYPAGE.md"\n')
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
    os.chmod(stub, 0o755)
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
            ("the excluded interval workspace", ["interval-transcendentals/src/lib.rs"], "all"),
            # A .md rustdoc COMPILES IN: every Rust block in it is a doctest,
            # so an edit to it can turn a build red. This is the live shape —
            # `crates/pncad/src/guide.rs` does exactly this to `docs/GUIDE.md`
            # and four pages under `docs/guide/`.
            ("a page compiled into rustdoc", ["docs/GUIDE.md"], "all"),
            # And the other consumer: a page a python suite executes.
            ("a page a python suite reads", ["docs/PYPAGE.md"], "all"),
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

    _selftest_docs_premise()
    print(
        "ci-filter selftest OK: the docs tier is reached by prose, memories/, "
        "local-scripts/ and .claude/ and by nothing else here — not a .rs beside a .md, "
        "not a non-.md file under docs/, not a path one character off a docs prefix, "
        "not an edit to this script, a gate, the workflow, the lockfile or a member "
        "manifest, not an unrecognised crate directory or top-level file, not an empty "
        "diff, not a crate source renamed to a .md, and not a page that rustdoc compiles "
        "in or a python suite executes; the closure follows dev-dependency "
        "edges upward only; the oracle signal fires on certified sources and lockfile and "
        "not on their prose"
    )


def _selftest_docs_premise() -> None:
    """THE PREMISE THE DOCS TIER RESTS ON, read off the REAL tree rather than
    asserted in a header. `_consumed_markdown` fails closed — an unreadable
    source or an `include!` it cannot resolve raises `Bail`, which makes every
    change set TIER=all — and the only trace of that in a real run is one line
    on stderr inside the filter job. Here it is a red self-test instead."""
    root = _repo_root()
    consumed = _consumed_markdown(root)
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
    for key, val in decorate(res, files).items():
        print(f"{key}={val}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
