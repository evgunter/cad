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
import subprocess
import sys

# Files that cannot move a hosted CI result.
#
# Documentation: deliberately narrow — only Markdown (anywhere) and the
# memories/ tree. No crate includes a .md file into its docs (`include_str!`
# is unused), so a .md change cannot move a doc-test.
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
def _is_docs(path: str) -> bool:
    return (
        path.startswith("memories/")
        or path.endswith(".md")
        or path.startswith("local-scripts/")
        or path.startswith(".claude/")
    )


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


def main() -> int:
    ap = argparse.ArgumentParser()
    src = ap.add_mutually_exclusive_group(required=True)
    src.add_argument("--base", help="git ref/sha to diff HEAD against")
    src.add_argument("--files", help="file with a newline-separated list, or -")
    args = ap.parse_args()
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
