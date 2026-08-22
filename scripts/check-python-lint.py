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

DEGRADED BOXES, AND WHO MAY SKIP. On the gate of record a missing or
wrong-versioned ruff is a hard failure, and that does NOT rest on a flag a
four-character edit can remove: `GITHUB_ACTIONS` is set by the runner itself,
and it is what this script reads. `REQUIRE_RUFF: "1"` in ci.yml states the same
thing where a reader of the workflow meets it, and it is CHECKED — under
`GITHUB_ACTIONS` this script fails if that declaration is absent or not `"1"`,
so deleting or flipping it reds the row instead of silently disarming it. A
declaration nothing reads is precisely the defect this row exists to close, and
it does not get re-minted inside the fix for it.

Off the gate of record — a developer's box, the degraded local gate — the row
SKIPS LOUDLY, naming what it wanted, the same posture `tests/test_ty.py` takes
for `ty` and `scripts/check_step.sh` for `freecadcmd`. A skip prints and returns
0; it never prints a pass. Setting `REQUIRE_RUFF` on such a box promotes the
skip to a failure, which is what it is for there.

    check-python-lint.py [--selftest]
"""

from __future__ import annotations

import os
import re
import shutil
import subprocess
import sys
import tempfile
from collections.abc import Mapping
from typing import NoReturn

CI_YML = ".github/workflows/ci.yml"
CONFIG = "ruff.toml"
REQUIRE_ENV = "REQUIRE_RUFF"
PIN_RE = re.compile(r'^\s*RUFF_VERSION:\s*"([0-9]+\.[0-9]+\.[0-9]+)"\s*$')
VERSION_RE = re.compile(r"^ruff\s+([0-9]+\.[0-9]+\.[0-9]+)")
EXCLUDE_RE = re.compile(r"^exclude = \[", re.MULTILINE)

# The file kinds ruff reads and this repo tracks. Kept as a pattern list handed
# to `git ls-files`, not a hand-written roster of paths: the population has to
# come from the tree, or this script is one more enumeration to fall behind it.
TRACKED_GLOBS = ("*.py", "*.pyi")


class Skip(Exception):
    """No usable ruff, and nothing demanded one. Loud, and not a pass."""


def die(msg: str) -> NoReturn:
    """Every exit from this script wears the same frame. A bare traceback names
    a Python line; a reader of a CI log needs the row's name and what the run
    therefore did not decide."""
    sys.exit(f"check-python-lint: {msg}")


def run_or_die(argv: list[str], cwd: str | None, cause: str, undecided: str) -> str:
    """A subprocess whose failure is DIAGNOSED rather than raised.

    `check=True` on any of these would surface a malformed `ruff.toml` (ruff
    exits 2), a missing binary, or a non-repo cwd as an uncaught
    `CalledProcessError` traceback: no `check-python-lint:` framing, and no
    statement of what was left undecided. That is the shape
    `scripts/gates/lib.sh`'s `gate_error` was written to stop, in gates this
    script runs beside, so it is the shape used here.
    """
    printed = " ".join(argv)
    try:
        done = subprocess.run(argv, cwd=cwd, capture_output=True, text=True, check=False)
    except OSError as exc:
        die(f"could not run `{printed}` ({exc}). {cause} {undecided}")
    if done.returncode != 0:
        die(f"`{printed}` exited {done.returncode}. {cause}\n"
            f"--- its stderr ---\n{done.stderr.strip() or '(nothing)'}\n--- end ---\n{undecided}")
    return done.stdout


def gate_of_record(env: Mapping[str, str]) -> str | None:
    """Why a missing ruff must be fatal here, or `None` if a skip is allowed.

    The runner's own variable is the authority, not a step-level `env:` block:
    anyone editing ci.yml can delete that block, and nobody can delete the fact
    that the job is running on GitHub Actions.

    `env` is passed in rather than read from `os.environ` for the same reason
    `reconcile` takes its third set: on the gate of record BOTH conditions below
    hold, so an ambient read would leave the self-test unable to say which of
    them is doing the work — and the whole point of this function is that the
    first one is.
    """
    if env.get("GITHUB_ACTIONS"):
        return "this is hosted CI, the gate of record"
    if env.get(REQUIRE_ENV):
        return f"{REQUIRE_ENV} is set"
    return None


def check_required_declaration(env: Mapping[str, str]) -> None:
    """The `REQUIRE_RUFF: "1"` in ci.yml is a marker, so it is checked.

    Unchecked, it is a claim that a checker is mandatory with nothing enforcing
    it — this row's own defect, re-minted inside the fix for it. Checked, the
    workflow either says out loud that the linter is required here or the row
    reds. Runs before anything else, so both hosted invocations (`--selftest`
    and the real run) carry it.
    """
    if not env.get("GITHUB_ACTIONS"):
        return
    got = env.get(REQUIRE_ENV)
    if got == "1":
        return
    die(
        f"running under GITHUB_ACTIONS with {REQUIRE_ENV}={got!r}; on the gate of record it must "
        f'be "1". That line in {CI_YML}\'s python-lint step is the workflow saying out loud that '
        "a missing linter fails this job, and this check is what keeps it from becoming a "
        "sentence nothing enforces. Ruff is required here either way — the requirement is read "
        "from GITHUB_ACTIONS, not from this variable — so restore the declaration rather than "
        "working around it."
    )


def repo_root() -> str:
    return run_or_die(
        ["git", "rev-parse", "--show-toplevel"], None,
        "Every path this script reasons about is repo-relative, so a cwd outside a git work "
        "tree leaves it with no frame of reference.",
        "NOTHING WAS DECIDED about whether this repo's Python is linted.",
    ).strip()


def pinned_version(root: str) -> str:
    """The single source of truth, read where it lives."""
    path = os.path.join(root, CI_YML)
    try:
        with open(path, encoding="utf-8") as fh:
            for line in fh:
                m = PIN_RE.match(line)
                if m:
                    return m.group(1)
    except OSError as exc:
        die(f"cannot read {CI_YML} ({exc}), which is where the ruff version pin lives. NOTHING "
            "WAS DECIDED about whether this repo's Python is linted.")
    die(
        f"{CI_YML} has no `RUFF_VERSION: \"x.y.z\"` line. That pin is the one place this repo "
        "records which ruff it runs, and this script reads it from there so the hosted half and "
        "the local half cannot drift onto different linters. Restore the pin in ci.yml's `env:` "
        "block beside NEXTEST_VERSION/MATURIN_VERSION/TY_VERSION."
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
    out = run_or_die(
        ["git", "ls-files", "-z", "--", *globs], root,
        "That is where the linted population COMES FROM: this script keeps no roster of Python "
        "files, it asks git for one, and without an answer there is no set to reconcile against.",
        "NOTHING WAS DECIDED about whether this repo's Python is linted — not a pass, not a skip.",
    )
    return {p for p in out.split("\0") if p}


def ruff_files(exe: str, root: str) -> set[str]:
    """The files ruff's own walk would check, as repo-relative paths."""
    out = run_or_die(
        [exe, "check", "--config", CONFIG, "--no-cache", "--show-files", "."], root,
        f"Ruff would not resolve its own file list. A malformed or unreadable {CONFIG} exits 2 "
        "here, and so does an unknown rule code in its `select`/`ignore` lists.",
        "The reconciliation therefore never ran: NOTHING WAS DECIDED about which tracked Python "
        "files ruff would have checked, and no file in this repo was linted by this run.",
    )
    seen = set()
    for line in out.splitlines():
        line = line.strip()
        if line:
            seen.add(os.path.relpath(line, root))
    return seen


def reconcile(want: set[str], walked: set[str], all_tracked: set[str]) -> list[str]:
    """The whole point of this script. Pure, so `--selftest` can plant every
    failure without a repo that has them.

    `walked` is everything ruff's file walk resolved, which is not only Python:
    ruff reads `pyproject.toml` files too (RUF200 checks their `[project]`
    table) and its own `ruff.toml`. Those are split off and held to a weaker but
    still total claim — they must be tracked files — so the Python comparison
    stays exact and nothing in the walk goes unaccounted for.

    `all_tracked` HAS NO DEFAULT, deliberately. The obvious one — fall back to
    `other` — makes the first claim below empty by construction: a caller that
    omits the argument silently gets two of the three checks, with no sign the
    third was dropped. A knob whose default turns a check off, inside a guard,
    is the shape this row exists to remove.
    """
    problems = []
    linted = {p for p in walked if p.endswith((".py", ".pyi"))}
    other = walked - linted
    stray = sorted(other - all_tracked)
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
# plants the failure and asserts this file reds on it. A demonstration run once
# by hand and written into a PR description is NOT a guard — nothing re-runs it
# — so every mechanism this row rests on is planted here: the rule set, the
# reconciliation, a REAL `exclude` entry hiding a REAL file, the version pin,
# and the gate-of-record posture.

PLANTED_RULES = (
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

# Source ruff is clean on, so a planted `exclude` is the ONLY reason a file
# written from it could go missing from the walk.
CLEAN_SRC = "VALUE = 1\n"


class Tally:
    """Cases run and cases failed, COUNTED — so the number this file prints
    comes from the cases it actually ran, not from a total kept by hand
    somewhere below them."""

    def __init__(self) -> None:
        self.ran = 0
        self.bad = 0

    def check(self, ok: bool, msg: str) -> None:
        self.ran += 1
        if not ok:
            self.bad += 1
            print(f"SELFTEST FAIL: {msg}", file=sys.stderr)


def _plant_repo(tmp: str, root: str, extra_exclude: str | None) -> None:
    """A real git repo carrying the repo's REAL ruff.toml and ci.yml, holding
    two lint-clean Python files — one of them under `hidden/`."""
    os.makedirs(os.path.join(tmp, "hidden"))
    os.makedirs(os.path.join(tmp, os.path.dirname(CI_YML)))
    for rel in ("kept.py", "hidden/tucked.py"):
        with open(os.path.join(tmp, rel), "w", encoding="utf-8") as fh:
            fh.write(CLEAN_SRC)
    with open(os.path.join(root, CONFIG), encoding="utf-8") as fh:
        config = fh.read()
    if extra_exclude is not None:
        config, n = EXCLUDE_RE.subn(f'exclude = ["{extra_exclude}", ', config)
        if n != 1:
            raise SystemExit(
                f"SELFTEST BROKEN: {CONFIG} no longer opens an `exclude = [` list at the start of "
                "a line, so this case cannot plant the exclusion it is about. Fix EXCLUDE_RE."
            )
    with open(os.path.join(tmp, CONFIG), "w", encoding="utf-8") as fh:
        fh.write(config)
    shutil.copyfile(os.path.join(root, CI_YML), os.path.join(tmp, CI_YML))
    for argv in (["git", "init", "-q"], ["git", "add", "-A"]):
        run_or_die(argv, tmp, "The end-to-end cases need a real index for `git ls-files` to read.",
                   "The self-test did not run: this is a broken harness, not a verdict on the "
                   "repo's Python.")


def _end_to_end(exe: str, tmp: str) -> list[str]:
    """The real pipeline — git's list, ruff's own walk, the reconciliation —
    against a planted repo."""
    return reconcile(tracked(tmp, *TRACKED_GLOBS), ruff_files(exe, tmp), tracked(tmp))


def _self_run(root: str, env: dict[str, str]) -> tuple[int, str]:
    """This script, invoked the way CI invokes it, under a planted environment.
    An empty value means UNSET, which is the edit each case is about."""
    done = subprocess.run(
        [sys.executable, os.path.abspath(__file__)], cwd=root, check=False,
        capture_output=True, text=True,
        env={k: v for k, v in {**os.environ, **env}.items() if v != ""},
    )
    return done.returncode, done.stdout + done.stderr


def selftest(root: str) -> int:
    exe = resolve_ruff(root)
    t = Tally()

    # 1. The rules the config selects fire on real source.
    with tempfile.TemporaryDirectory() as tmp:
        for name, src, rule in PLANTED_RULES:
            path = os.path.join(tmp, name)
            with open(path, "w", encoding="utf-8") as fh:
                fh.write(src)
            done = subprocess.run(
                [exe, "check", "--config", os.path.join(root, CONFIG), "--no-cache",
                 "--output-format", "concise", path],
                capture_output=True, text=True, check=False,
            )
            t.check(done.returncode != 0 and rule in done.stdout,
                    f"planted {rule} in {name} and ruff reported "
                    f"{done.stdout.strip() or '(nothing)'}")

    # 2. The reconciliation, every direction, without a repo that is broken.
    t.check(reconcile({"a.py"}, {"a.py"}, {"a.py"}) == [],
            "reconcile() flagged an exact match")
    t.check(bool(reconcile({"a.py", "b.py"}, {"a.py"}, {"a.py", "b.py"})),
            "reconcile() passed a tracked file ruff never checked — that is the silent hole "
            "this script exists to make loud")
    t.check(bool(reconcile({"a.py"}, {"a.py", "vendor/pyproject.toml"}, {"a.py"})),
            "reconcile() passed an untracked TOML in ruff's walk")
    t.check(not reconcile({"a.py"}, {"a.py", "ruff.toml"}, {"a.py", "ruff.toml"}),
            "reconcile() flagged the tracked config ruff legitimately reads")
    t.check(bool(reconcile({"a.py"}, {"a.py", ".venv/lib/x.py"}, {"a.py"})),
            "reconcile() passed a file ruff checked outside the tracked tree")

    # 3. THE TRAP'S OWN GUARD, END TO END: a real `exclude` entry in a real
    #    ruff.toml hiding a real tracked file from ruff's real walk. The clean
    #    plant is the negative control — without it, a reconciliation that
    #    fired on everything would pass the case below.
    with tempfile.TemporaryDirectory() as tmp:
        _plant_repo(tmp, root, None)
        t.check(_end_to_end(exe, tmp) == [],
                "the end-to-end plant is not clean BEFORE the exclusion is added, so the case "
                "after it proves nothing about the exclusion")
    with tempfile.TemporaryDirectory() as tmp:
        _plant_repo(tmp, root, "hidden")
        problems = _end_to_end(exe, tmp)
        t.check(any("hidden/tucked.py" in p and "DID NOT check" in p for p in problems),
                'an `exclude = ["hidden", …]` entry hid a tracked Python file from ruff and the '
                f"reconciliation reported {problems or '(nothing)'} — ruff's own run prints "
                "`All checks passed!` on that tree, which is the whole reason this check exists")

    # 4. The version pin: a wrong version SKIPS rather than lints, and a
    #    deleted pin is a hard failure rather than a default.
    with tempfile.TemporaryDirectory() as tmp:
        os.makedirs(os.path.join(tmp, os.path.dirname(CI_YML)))
        with open(os.path.join(tmp, CI_YML), "w", encoding="utf-8") as fh:
            fh.write('env:\n  RUFF_VERSION: "0.0.1"\n')
        try:
            resolve_ruff(tmp)
            t.check(False, "a ci.yml pinning ruff 0.0.1 did not stop the real binary being used")
        except Skip as why:
            t.check("0.0.1" in str(why), f"the version mismatch did not name the pin: {why}")
        with open(os.path.join(tmp, CI_YML), "w", encoding="utf-8") as fh:
            fh.write('env:\n  NEXTEST_VERSION: "0.9.0"\n')
        try:
            pinned_version(tmp)
            t.check(False, "a ci.yml with no RUFF_VERSION line returned a version anyway")
        except SystemExit as why:
            t.check("RUFF_VERSION" in str(why), f"the missing pin did not name it: {why}")

    # 5. THE GATE-OF-RECORD POSTURE, first as a truth table on the two
    #    functions that decide it. On hosted CI both conditions hold at once,
    #    so only a direct call can show that the RUNNER'S variable — not the
    #    step-level flag — is what forbids a skip there.
    t.check(gate_of_record({"GITHUB_ACTIONS": "true"}) is not None,
            "gate_of_record allowed a skip on hosted CI with no REQUIRE_RUFF — the flag would "
            "then be the only thing between the gate of record and a silent no-op")
    t.check(gate_of_record({REQUIRE_ENV: "1"}) is not None,
            f"gate_of_record allowed a skip with {REQUIRE_ENV} set")
    t.check(gate_of_record({}) is None,
            "gate_of_record forbade a skip on a plain developer's box, which would block a "
            "whole local battery on a tool nobody there installed")
    check_required_declaration({})
    check_required_declaration({"GITHUB_ACTIONS": "true", REQUIRE_ENV: "1"})
    for planted in ({"GITHUB_ACTIONS": "true"}, {"GITHUB_ACTIONS": "true", REQUIRE_ENV: "0"}):
        try:
            check_required_declaration(planted)
            t.check(False, f"the ci.yml declaration went unchecked under {planted}")
        except SystemExit as why:
            t.check(REQUIRE_ENV in str(why), f"the declaration check did not name it: {why}")

    #    Then end to end, as this script is actually invoked: each case below is
    #    an edit to that one step-level line that would otherwise have left the
    #    row printing SKIPPED and returning 0.
    gone = {"RUFF": "/nonexistent/ruff", "GITHUB_ACTIONS": "", REQUIRE_ENV: ""}
    rc, out = _self_run(root, gone)
    t.check(rc == 0 and "SKIPPED" in out,
            f"a developer's box without ruff did not skip loudly: rc={rc}\n{out}")
    rc, out = _self_run(root, {**gone, REQUIRE_ENV: "1"})
    t.check(rc != 0 and "SKIPPED" not in out,
            f"{REQUIRE_ENV}=1 with no ruff did not fail: rc={rc}\n{out}")
    rc, out = _self_run(root, {**gone, "GITHUB_ACTIONS": "true", REQUIRE_ENV: "1"})
    t.check(rc != 0 and "SKIPPED" not in out,
            f"hosted CI with no ruff did not fail: rc={rc}\n{out}")
    rc, out = _self_run(root, {**gone, "GITHUB_ACTIONS": "true"})
    t.check(rc != 0 and REQUIRE_ENV in out,
            f"{REQUIRE_ENV} deleted from ci.yml's step went unnoticed on the gate of record: "
            f"rc={rc}\n{out}")
    rc, out = _self_run(root, {"GITHUB_ACTIONS": "true", REQUIRE_ENV: "0"})
    t.check(rc != 0 and REQUIRE_ENV in out,
            f"{REQUIRE_ENV} flipped to 0 went unnoticed on the gate of record: rc={rc}\n{out}")

    print(f"check-python-lint --selftest: {t.ran} cases, {t.bad} failed")
    return 1 if t.bad else 0


def main(argv: list[str]) -> int:
    check_required_declaration(os.environ)
    root = repo_root()
    try:
        if argv[1:] == ["--selftest"]:
            return selftest(root)
        if argv[1:]:
            print(__doc__, file=sys.stderr)
            return 2
        return run(root)
    except Skip as why:
        required = gate_of_record(os.environ)
        if required:
            print(f"check-python-lint: {required}, and {why}", file=sys.stderr)
            return 1
        print(f"check-python-lint: SKIPPED — {why}", file=sys.stderr)
        return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
