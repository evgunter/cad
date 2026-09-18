#!/usr/bin/env python3
"""Does this change set reach the CI machinery the `mirror` job's selftests
are about?

    scripts/ci-reach.py --event pull_request --base "$BASE_SHA"
    # -> prints `selftests=true` or `selftests=false`

THE JOB THIS SERVES, AND THE DISTINCTION IT RESTS ON. `mirror` ("CI half
parity + gate wiring") carries no `if:` because its CHECKS are about prose,
`work/`, `docs/` and `local-scripts/` — the exact file classes that classify
TIER=docs and skip every `if: run_build` job. That is S61's siting rule (Ev,
2026-08-20) and nothing here touches it: every check in that job still runs on
every tier, on every event.

A SELFTEST IS NOT ITS CHECK. Each of those readers ships a `--selftest` that
builds a fixture tree under `--root` — a miniature repo, a scratch apt
archive, a synthetic viewer README — and asserts the reader refuses it. Not
one of them reads the real tree. So a selftest's input closure is ITS OWN
SCRIPT, and a change set that touches no script cannot make one fail. On the
2026-09-17 docs-tier run those fixtures were 129 of the job's 147 seconds of
work against 18 for the checks themselves: seven eighths of the job, and on a
documentation pull request every second of it a guaranteed no-op.

SO THE GATE IS THE SELFTESTS' OWN INPUTS, which is the same rule S61 states,
applied to the other half of each row. `mirror` keeps no `if:`; the selftest
steps inside it carry one, and it reads THIS script.

WHY A RAW PATH TEST AND NOT `scripts/ci-filter.py`. The filter's `docs` branch
fails OPEN — a change set wrongly read as documentation skips the gate and
reports green — and one of the selftests gated here is the filter's own. A
pull request widening `_is_docs` is classified by the WIDENED filter out of the
merge ref, so gating that selftest on the filter's own answer is the hole it
exists to catch, reintroduced one level up. A prefix match over
`git diff --name-only` cannot be fooled that way: the diff adding `scripts/` to
`_is_docs` is a diff touching `scripts/`, so its own selftest runs.

AND WHY THIS SCRIPT'S SELFTEST IS THE ONE THAT STAYS UNGATED. Everything here
decides whether other selftests run, so gating it on its own answer is the same
circularity from the inside: a bug that made this always print `false` would
skip the fixtures that would have caught it, once, silently. `mirror` runs
`--selftest` on every tier for the same reason `scripts/ci-filter.py
--selftest` is in `check-ci-mirror-parity.py`'s `TIER_BLIND` list. It costs
about a second.

EVERY REFUSAL FAILS OPEN TOWARDS RUNNING THEM. A missing base sha, a fetch that
did not land, a `git diff` that exits nonzero, an event this does not recognise
— each prints why and answers `true`. The cost of being wrong that way is the
two minutes this exists to save; the cost of the other way is a fixture that
stops running and nothing saying so.
"""

import argparse
import os
import subprocess
import sys
import tempfile

# The prefixes a change must touch for the fixtures to be able to notice.
#
# `scripts/` is the subject: every gated selftest lives there and reads nothing
# else. The other two are not needed for any selftest known today and are here
# because the cost of widening this is zero on the change class that matters —
# a documentation pull request touches none of the three — and the cost of
# narrowing it wrongly is a fixture that stops running. `.github/` covers the
# workflow files and the composite actions several readers parse; the fixtures
# build their own copies, but a reader taught a new workflow shape is a reader
# whose fixtures should re-run. `local-scripts/` is the local half of the pair
# this job compares, and the one tree that classifies TIER=docs.
REACH = ("scripts/", ".github/", "local-scripts/")

# The events on which everything runs regardless. A push to main, a merge group
# and a dispatch have no pull-request base to diff against; `push` also has no
# `mirror` skip to protect, since this job is cheap and unconditional there.
NARROWED_EVENT = "pull_request"


def _git(args: list[str], cwd: str) -> tuple[int, str]:
    p = subprocess.run(["git", *args], cwd=cwd, capture_output=True, text=True)
    return p.returncode, p.stdout


def changed_files(base: str, cwd: str = ".") -> tuple[list[str] | None, str]:
    """`(paths, why)` — `paths` is None when the diff could not be taken.

    The caller answers `true` on None. A diff this cannot take is a question it
    cannot answer, and the honest answer to that is to run the fixtures.
    """
    rc, out = _git(["diff", "--name-only", f"{base}..HEAD"], cwd)
    if rc != 0:
        return None, f"`git diff --name-only {base}..HEAD` exited {rc}"
    paths = [ln for ln in out.splitlines() if ln.strip()]
    if not paths:
        return None, f"the diff against {base} is empty"
    return paths, ""


def decide(event: str, base: str, cwd: str = ".") -> tuple[bool, str]:
    """`(run_selftests, why)`. The `why` is printed either way — a skipped
    fixture that says nothing about why it was skipped is the state this whole
    file exists to keep out of the log."""
    if event != NARROWED_EVENT:
        return True, f"event is `{event}`, not `{NARROWED_EVENT}` — nothing to diff against"
    if not base:
        return True, "no base sha was passed"
    paths, why = changed_files(base, cwd)
    if paths is None:
        return True, why
    hits = sorted({p for p in paths for pre in REACH if p.startswith(pre)})
    if hits:
        shown = ", ".join(hits[:5])
        more = f" (+{len(hits) - 5} more)" if len(hits) > 5 else ""
        return True, f"{len(hits)} of {len(paths)} changed path(s) reach the checkers: {shown}{more}"
    return False, (f"none of the {len(paths)} changed path(s) is under "
                   + ", ".join(REACH))


def emit(run: bool, why: str) -> None:
    value = "true" if run else "false"
    verb = "running" if run else "SKIPPING"
    print(f"ci-reach: {verb} the checker selftests — {why}")
    print(f"selftests={value}")
    out = os.environ.get("GITHUB_OUTPUT")
    if out:
        with open(out, "a", encoding="utf-8") as fh:
            fh.write(f"selftests={value}\n")


def _selftest() -> int:
    fails: list[str] = []

    def check(ok: bool, msg: str) -> None:
        if not ok:
            fails.append(msg)

    with tempfile.TemporaryDirectory() as t:
        def git(*args: str) -> None:
            subprocess.run(["git", *args], cwd=t, check=True, capture_output=True)

        def commit(path: str, body: str = "x") -> str:
            full = os.path.join(t, path)
            os.makedirs(os.path.dirname(full), exist_ok=True)
            with open(full, "w", encoding="utf-8") as fh:
                fh.write(body)
            git("add", "-A")
            git("commit", "-m", f"add {path}")
            rc, out = _git(["rev-parse", "HEAD"], t)
            assert rc == 0
            return out.strip()

        git("init", "-q", "-b", "main")
        git("config", "user.email", "t@example.invalid")
        git("config", "user.name", "t")
        base = commit("README.md", "base\n")

        # 1. A DOCUMENTATION CHANGE SET SKIPS THEM. The case this exists for.
        commit("docs/GUIDE.md")
        commit("work/ciw/item.md")
        run, why = decide("pull_request", base, t)
        check(not run, f"a docs-only diff wanted the fixtures: {why}")
        check("none of the" in why, f"the skip did not say what it looked at: {why}")

        # 2. EACH REACH PREFIX TURNS THEM BACK ON, one at a time — a list read
        # as a whole would pass with any single entry silently dropped.
        for pre in REACH:
            head = commit(f"{pre}thing.txt")
            run, why = decide("pull_request", base, t)
            check(run, f"a diff touching `{pre}` did not run the fixtures: {why}")
            check(pre in why or "reach" in why, f"the run did not name the reason: {why}")
            git("reset", "-q", "--hard", f"{head}~1")

        # 3. THE SELF-REFERENTIAL CASE, spelled out because it is the reason
        # this is a path test. A pull request that widens the filter's docs
        # branch IS a change to scripts/ci-filter.py, so it runs its own
        # selftest however that widened filter would classify it.
        commit("scripts/ci-filter.py", "def _is_docs(p): return True\n")
        run, why = decide("pull_request", base, t)
        check(run, f"a diff editing scripts/ci-filter.py skipped its own selftest: {why}")

        # 4. EVERY REFUSAL FAILS OPEN. A bad base, no base, an unknown event
        # and an empty diff all answer `true`.
        run, why = decide("pull_request", "0" * 40, t)
        check(run, f"an unreachable base sha skipped the fixtures: {why}")
        run, why = decide("pull_request", "", t)
        check(run, f"an empty base sha skipped the fixtures: {why}")
        for event in ("push", "merge_group", "workflow_dispatch", ""):
            run, why = decide(event, base, t)
            check(run, f"event `{event}` skipped the fixtures: {why}")
        rc, head = _git(["rev-parse", "HEAD"], t)
        assert rc == 0
        run, why = decide("pull_request", head.strip(), t)
        check(run, f"an empty diff skipped the fixtures: {why}")

        # 5. THE OUTPUT IS THE CONTRACT. The step reads `selftests=` out of
        # GITHUB_OUTPUT, so the spelling is asserted rather than assumed.
        out_file = os.path.join(t, "gh-out")
        os.environ["GITHUB_OUTPUT"] = out_file
        try:
            emit(False, "because")
            emit(True, "because")
        finally:
            del os.environ["GITHUB_OUTPUT"]
        with open(out_file, encoding="utf-8") as fh:
            wrote = fh.read().splitlines()
        check(wrote == ["selftests=false", "selftests=true"],
              f"GITHUB_OUTPUT got {wrote!r}")

    if fails:
        sys.stderr.write("ci-reach.py --selftest FAILED\n")
        for f in fails:
            sys.stderr.write(f"  - {f}\n")
        return 1
    print("ci-reach.py --selftest: ok")
    return 0


def main(argv: list[str]) -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--event", default="")
    ap.add_argument("--base", default="")
    ap.add_argument("--selftest", action="store_true")
    args = ap.parse_args(argv)
    if args.selftest:
        return _selftest()
    run, why = decide(args.event, args.base)
    emit(run, why)
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
