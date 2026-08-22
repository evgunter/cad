#!/usr/bin/env python3
"""Every tracked Python file in this repo is linted by ruff, and the set that is
NOT linted is derived and named rather than assumed.

This is the ONLY invocation of ruff in the tree. Both halves of CI call this
script — `.github/workflows/ci.yml`'s `mirror` job and `local-scripts/
ci-local.sh`'s tier-blind rows — and neither calls `ruff` directly, because the
defect this whole change closes is a claim about a checker that nothing ran. A
second, differently-scoped invocation would be the same defect in a new place:
a config that says "the repo" and a CI row that reaches less of it.

WHAT THIS ADDS OVER `ruff check .`. Ruff's file walk is silent about what it
skipped: `exclude`, `.gitignore` and the default exclusion list all remove files
with no output at all. So the walk is reconciled here against git:

    linted   == `git ls-files '*.py' '*.pyi'`

exactly, in both directions. A tracked Python file ruff did not visit fails the
run (a hole in the gate, however it got there — an `exclude` entry, a stray
`.gitignore` line, a `per-file-ignores` block someone added). A file ruff DID
visit that git does not track also fails the run (ruff wandered into a
virtualenv, a build tree, or generated output). Neither can happen quietly, and
adding a Python file anywhere in the repo is all it takes to have it linted.

WHY `.pyi` IS IN THAT SET. `crates/pncad-py/pncad.pyi` is the typed façade the
`ty` row checks; it is Python source that ships inside the wheel, ruff reads it
by default, and leaving it out would make the reconciliation's own claim ("every
tracked Python file") false at exactly one path.

THE VERSION IS PINNED IN ci.yml, not here. `RUFF_VERSION` sits with
`NEXTEST_VERSION`, `MATURIN_VERSION` and `TY_VERSION` in the workflow's `env:`
block — one source of truth for every tool version, read from there by this
script so the two halves cannot run different linters and disagree.

DEGRADED BOXES. Hosted CI sets `REQUIRE_RUFF=1` and a missing or wrong-versioned
ruff is a hard failure there. Without it — a developer's box, the degraded local
gate — the row SKIPS LOUDLY, naming what it wanted, the same posture
`tests/test_ty.py` takes for `ty` and `scripts/check_step.sh` for `freecadcmd`.
A skip prints and returns 0; it never prints a pass.

    check-python-lint.py [--selftest]
"""

from __future__ import annotations

import os
import re
import subprocess
import sys
import tempfile

CI_YML = ".github/workflows/ci.yml"
CONFIG = "ruff.toml"
PIN_RE = re.compile(r'^\s*RUFF_VERSION:\s*"([0-9]+\.[0-9]+\.[0-9]+)"\s*$')
VERSION_RE = re.compile(r"^ruff\s+([0-9]+\.[0-9]+\.[0-9]+)")

# The file kinds ruff reads and this repo tracks. Kept as a pattern list handed
# to `git ls-files`, not a hand-written roster of paths: the population has to
# come from the tree, or this script is one more enumeration to fall behind it.
TRACKED_GLOBS = ("*.py", "*.pyi")


class Skip(Exception):
    """No usable ruff, and nothing demanded one. Loud, and not a pass."""


def repo_root() -> str:
    return subprocess.run(
        ["git", "rev-parse", "--show-toplevel"],
        check=True, capture_output=True, text=True,
    ).stdout.strip()


def pinned_version(root: str) -> str:
    """The single source of truth, read where it lives."""
    path = os.path.join(root, CI_YML)
    with open(path, encoding="utf-8") as fh:
        for line in fh:
            m = PIN_RE.match(line)
            if m:
                return m.group(1)
    sys.exit(
        f"check-python-lint: {CI_YML} has no `RUFF_VERSION: \"x.y.z\"` line. That pin is the one "
        "place this repo records which ruff it runs, and this script reads it from there so the "
        "hosted half and the local half cannot drift onto different linters. Restore the pin in "
        "ci.yml's `env:` block beside NEXTEST_VERSION/MATURIN_VERSION/TY_VERSION."
    )


def resolve_ruff(root: str) -> str:
    """The ruff binary to use, or `Skip` with the reason."""
    want = pinned_version(root)
    exe = os.environ.get("RUFF", "ruff")
    try:
        got = subprocess.run([exe, "--version"], capture_output=True, text=True, check=True)
    except (OSError, subprocess.CalledProcessError) as exc:
        raise Skip(
            f"no ruff on PATH as `{exe}` ({exc}). Wanted {want} — the pin in {CI_YML}. "
            f"Install it with `pip install ruff=={want}`, or point $RUFF at the binary."
        ) from exc
    m = VERSION_RE.match(got.stdout.strip())
    if not m:
        raise Skip(f"`{exe} --version` printed {got.stdout.strip()!r}, which is not a ruff version")
    if m.group(1) != want:
        raise Skip(
            f"`{exe}` is ruff {m.group(1)} and {CI_YML} pins {want}. Rule sets and default "
            "behaviour move between ruff releases, so a different version here would report "
            "different findings from the ones that gate the merge. Install the pinned version, "
            "or set $RUFF to it."
        )
    return exe


def tracked(root: str, *globs: str) -> set[str]:
    out = subprocess.run(
        ["git", "ls-files", "-z", "--", *globs],
        cwd=root, check=True, capture_output=True, text=True,
    ).stdout
    return {p for p in out.split("\0") if p}


def ruff_files(exe: str, root: str) -> set[str]:
    """The files ruff's own walk would check, as repo-relative paths."""
    out = subprocess.run(
        [exe, "check", "--config", CONFIG, "--no-cache", "--show-files", "."],
        cwd=root, check=True, capture_output=True, text=True,
    ).stdout
    seen = set()
    for line in out.splitlines():
        line = line.strip()
        if line:
            seen.add(os.path.relpath(line, root))
    return seen


def reconcile(want: set[str], walked: set[str], all_tracked: set[str] | None = None) -> list[str]:
    """The whole point of this script. Pure, so `--selftest` can plant every
    failure without a repo that has them.

    `walked` is everything ruff's file walk resolved, which is not only Python:
    ruff reads `pyproject.toml` files too (RUF200 checks their `[project]`
    table) and its own `ruff.toml`. Those are split off and held to a weaker but
    still total claim — they must be tracked files — so the Python comparison
    stays exact and nothing in the walk goes unaccounted for.
    """
    problems = []
    linted = {p for p in walked if p.endswith((".py", ".pyi"))}
    other = walked - linted
    stray = sorted(other - (all_tracked if all_tracked is not None else other))
    if stray:
        problems.append(
            f"{len(stray)} non-Python file(s) in ruff's walk that git does not track: "
            f"{', '.join(stray[:10])}. Ruff also reads TOML (pyproject/ruff config); an untracked "
            "one means the walk left the repo's own files."
        )
    missing = sorted(want - linted)
    if missing:
        problems.append(
            f"{len(missing)} tracked Python file(s) that ruff DID NOT check: {', '.join(missing)}. "
            "Something is removing them from the walk — an `exclude` entry in ruff.toml, a "
            "`.gitignore` line, or a nested config — and the gate would have passed over them "
            "without a word. Every exclusion in this repo is written down with its reason; if "
            "one of these genuinely should not be linted, put it in ruff.toml's `exclude` AND "
            "here, so the two agree out loud."
        )
    extra = sorted(linted - want)
    if extra:
        problems.append(
            f"{len(extra)} file(s) ruff checked that git does not track: {', '.join(extra[:10])}"
            f"{' …' if len(extra) > 10 else ''}. Ruff walked outside the repo's own files — a "
            "virtualenv, a build tree, or generated output. Findings from there are not this "
            "repo's to fix and would red the gate on someone else's code."
        )
    return problems


def run(root: str) -> int:
    exe = resolve_ruff(root)
    want = tracked(root, *TRACKED_GLOBS)
    walked = ruff_files(exe, root)
    problems = reconcile(want, walked, tracked(root))
    checked = sorted(p for p in walked if p.endswith((".py", ".pyi")))
    done = subprocess.run(
        [exe, "check", "--config", CONFIG, "--no-cache", "."], cwd=root, check=False
    )
    # The verdict goes LAST, after ruff's own output, so a reader who sees
    # "All checks passed!" scroll by cannot mistake it for this row's result:
    # ruff passing on a walk that missed half the repo is exactly the failure
    # this script exists to catch.
    for problem in problems:
        print(f"check-python-lint: {problem}", file=sys.stderr)
    if problems or done.returncode != 0:
        print(f"check-python-lint FAILED: {len(checked)} of {len(want)} tracked Python files "
              f"checked, ruff exit {done.returncode}", file=sys.stderr)
        return 1
    print(f"check-python-lint OK: ruff {pinned_version(root)} checked all {len(want)} tracked "
          f"Python files, and checked nothing else")
    return 0


# --- selftest ---------------------------------------------------------------
# A GATE NEVER SHOWN TO FIRE IS NOT A GATE (scripts/gates/lib.sh). Each case
# plants the failure and asserts this file reds on it.

SELFTEST_CASES = (
    # (filename, source, rule that must appear)
    ("plain.py", "import os\n", "F401"),
    # The row's own defect: a marker for a check that reports nothing.
    ("stale.py", "import os  # noqa: F401\nprint(os.getcwd())\n", "RUF100"),
    # The five rules the pre-existing markers name, each shown to fire.
    ("e402.py", "x = 1\nimport os\nprint(os, x)\n", "E402"),
    ("e731.py", "f = lambda x: x\nprint(f(1))\n", "E731"),
    ("s102.py", "exec('x = 1')\n", "S102"),
    ("ble001.py", "try:\n    pass\nexcept Exception:\n    pass\n", "BLE001"),
)


def selftest(root: str) -> int:
    exe = resolve_ruff(root)
    bad = 0
    with tempfile.TemporaryDirectory() as tmp:
        for name, src, rule in SELFTEST_CASES:
            path = os.path.join(tmp, name)
            with open(path, "w", encoding="utf-8") as fh:
                fh.write(src)
            done = subprocess.run(
                [exe, "check", "--config", os.path.join(root, CONFIG), "--no-cache",
                 "--output-format", "concise", path],
                capture_output=True, text=True, check=False,
            )
            if done.returncode == 0 or rule not in done.stdout:
                bad += 1
                print(f"SELFTEST FAIL: planted {rule} in {name} and ruff reported "
                      f"{done.stdout.strip() or '(nothing)'}", file=sys.stderr)

    # The reconciliation, both directions, without a repo that is broken.
    if not reconcile({"a.py"}, {"a.py"}) == []:
        bad += 1
        print("SELFTEST FAIL: reconcile() flagged an exact match", file=sys.stderr)
    if not reconcile({"a.py", "b.py"}, {"a.py"}):
        bad += 1
        print("SELFTEST FAIL: reconcile() passed a tracked file ruff never checked — that is the "
              "silent hole this script exists to make loud", file=sys.stderr)
    if not reconcile({"a.py"}, {"a.py", "vendor/pyproject.toml"}, {"a.py"}):
        bad += 1
        print("SELFTEST FAIL: reconcile() passed an untracked TOML in ruff's walk", file=sys.stderr)
    if reconcile({"a.py"}, {"a.py", "ruff.toml"}, {"a.py", "ruff.toml"}):
        bad += 1
        print("SELFTEST FAIL: reconcile() flagged the tracked config ruff legitimately reads",
              file=sys.stderr)
    if not reconcile({"a.py"}, {"a.py", ".venv/lib/x.py"}):
        bad += 1
        print("SELFTEST FAIL: reconcile() passed a file ruff checked outside the tracked tree",
              file=sys.stderr)

    print(f"check-python-lint --selftest: {len(SELFTEST_CASES) + 5} cases, {bad} failed")
    return 1 if bad else 0


def main(argv: list[str]) -> int:
    root = repo_root()
    try:
        if argv[1:] == ["--selftest"]:
            return selftest(root)
        if argv[1:]:
            print(__doc__, file=sys.stderr)
            return 2
        return run(root)
    except Skip as why:
        if os.environ.get("REQUIRE_RUFF"):
            print(f"check-python-lint: REQUIRE_RUFF is set and {why}", file=sys.stderr)
            return 1
        print(f"check-python-lint: SKIPPED — {why}", file=sys.stderr)
        return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
