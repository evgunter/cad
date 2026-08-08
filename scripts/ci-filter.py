#!/usr/bin/env python3
"""Shared CI change filter — the SINGLE implementation of change
classification, used by BOTH .github/workflows/ci.yml (its `filter` job is
a thin YAML wrapper) and scripts/ci-local.sh. There is no second copy of
these rules anywhere; hosted and local runs are gated identically, and the
synthetic-diff tests exercise the one script both of them call.

Three tiers (Evan's ask: "changing a core crate runs everything, adding a
new crate only runs that new crate's tests" — dependency-AWARE, not naive
per-crate):

  TIER=docs     only *.md files and memories/ changed. Nothing builds; the
                `docs-only ok` marker job is the whole gate. (Floor
                convention: floors apply to CODE PRs.)
  TIER=all      a workspace-level file changed — the change can move any
                crate's build, so everything runs, unscoped.
  TIER=closure  only crate sources changed. PKGS is the DEPENDENT CLOSURE:
                the changed members plus every member that transitively
                depends on them, dev-dependencies INCLUDED (a dev-dep edge
                is a real build edge for `cargo test`).

Classification is an ALLOWLIST, so it fails CLOSED by construction: a path
is scopable only if it is a *.md/memories docs path or lives inside a known
workspace member's directory. Anything unrecognised — a new top-level file,
a new excluded workspace, a renamed crate dir — lands in TIER=all. Every
error path (git failure, cargo-metadata failure, empty diff, a crates/
subdirectory that is not a member) also lands in TIER=all.

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

Output: KEY=value lines on stdout, one per line, safe to append to
$GITHUB_OUTPUT and to parse with `while IFS='=' read -r k v`.

  TIER=docs|all|closure
  PKGS=<comma-separated members, empty for docs, all members for `all`>
  CARGO_SCOPE=--workspace | -p a -p b ...
  RUN_BUILD=true|false          any cargo/grep row at all (false only for docs)
  RUN_EDITOR_CORE=true|false    persistence / corpus / rebuild-latency rows
  RUN_STL=true|false            watertight (admesh) row
  RUN_STEP_EXPORT=true|false    step import (freecad) row
  RUN_INTERVAL_BACKEND=true|false   interval-transcendentals' own workspace
  RUN_K_LINT=true|false         k-lint (gate) row
"""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys

# Files that are documentation and nothing else. Deliberately narrow: only
# Markdown (anywhere) and the memories/ tree. No crate includes a .md file
# into its docs (`include_str!` is unused), so a .md change cannot move a
# doc-test.
def _is_docs(path: str) -> bool:
    return path.startswith("memories/") or path.endswith(".md")


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

    if all(_is_docs(f) for f in files):
        return {"TIER": "docs", "PKGS": "", "CARGO_SCOPE": ""}

    dir_of, deps = _members(root)
    seeds: set[str] = set()
    for f in files:
        if _is_docs(f):
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
    except Exception:
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
JOB_ROOTS = {
    "RUN_EDITOR_CORE": {"editor-core"},
    "RUN_STL": {"stl"},
    "RUN_STEP_EXPORT": {"step-export"},
}


def decorate(res: dict[str, str]) -> dict[str, str]:
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
    # k-lint has no minimal root set: it is the only job that compiles
    # demos/tour (a path-dependent of NINE members) and tools/k-lint, and its
    # probe sweep records predicate margins from every kernel crate. Any
    # member change can break it, so it runs whenever anything builds.
    res["RUN_K_LINT"] = "false" if tier == "docs" else "true"
    return res


def main() -> int:
    ap = argparse.ArgumentParser()
    src = ap.add_mutually_exclusive_group(required=True)
    src.add_argument("--base", help="git ref/sha to diff HEAD against")
    src.add_argument("--files", help="file with a newline-separated list, or -")
    args = ap.parse_args()
    root = _repo_root()

    try:
        if args.files:
            raw = sys.stdin.read() if args.files == "-" else open(args.files).read()
        else:
            try:
                raw = _run(
                    ["git", "diff", "--name-only", f"{args.base}...HEAD"], root
                )
            except subprocess.CalledProcessError:
                # Unrelated histories / shallow clone: fall back to the
                # two-dot form rather than guessing.
                raw = _run(["git", "diff", "--name-only", args.base, "HEAD"], root)
        files = [ln.strip() for ln in raw.splitlines() if ln.strip()]
        res = classify(files, root)
    except Exception as exc:  # noqa: BLE001 — fail CLOSED on anything at all
        print(f"ci-filter: falling back to TIER=all: {exc}", file=sys.stderr)
        res = _all_tier(root)

    for key, val in decorate(res).items():
        print(f"{key}={val}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
