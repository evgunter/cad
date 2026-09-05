#!/usr/bin/env python3
"""EVERY JOB OF THIS WORKFLOW RUN REACHED A TERMINAL STATE, AND CONCLUDED GREEN.

This is the whole of what `ci.yml`'s `gate ok` job asserts, and `gate ok` is the
one check a merge queue or a branch protection is meant to require. Two claims,
both failing CLOSED:

  1. EVERY job of the run except the caller has reached a terminal state. A job
     still running when this reads the list is a job missing from `gate-ok`'s
     `needs:` — the omission the design is most exposed to — and it is a RED
     naming that job, not a pass.
  2. Every one of them concluded `success`, `skipped` or `neutral`. Anything
     else — `failure`, `cancelled`, `timed_out`, `action_required` — is a red.

WHY THE ACCEPT-LIST IS THOSE THREE, and it is not this repository's judgement:
GitHub defines a passing check that way. `content/pull-requests/how-tos/
merge-and-close-pull-requests/troubleshooting-required-status-checks.md`, under
*Required check needs to succeed against the latest commit SHA*: **"Successful
check statuses are `success`, `skipped`, and `neutral`."** Reading the run with
a narrower list than the thing that will read `gate ok` would make this job red
where the queue would have merged, which is a worse gate and not a stricter one.

`neutral` IS UNREACHABLE HERE AND STAYS ON THE LIST ANYWAY. A workflow JOB
cannot conclude `neutral` — job conclusions are success/failure/cancelled/
skipped/timed_out/action_required — and `.github/actions/rebaseline-lane/
action.yml` says so at its own key, which is why the render lanes' drift and
re-baseline signals are CHECK RUNS posted through `POST /check-runs` rather than
jobs. This reads `/actions/runs/{id}/jobs`, so those check runs are outside it
entirely: what `gate ok` summarises is JOBS and not check runs, and that is the
sentence the accept-list's third entry is really carrying. The entry stays
because the list is GitHub's definition of a pass and not an enumeration of what
this workflow happens to emit today — a `neutral` job conclusion added by GitHub
tomorrow is a pass by their own rule, and a red here would be this file's bug.

WHY THIS IS A SCRIPT AND NOT A `run:` BLOCK. It was 56 lines of inline python in
`ci.yml` when it landed, and exactly one of its decision paths executes on a real
run: the all-green one. The other six are a failed job, a job still running, a
paged job list, an empty population, `neutral`, and an unreadable API — none of
which a hosted run produces on demand, so nothing in the tree re-drove them and
the only required check in the design had six untested arms. Out here they are
`--selftest` cases against a stub `gh`, run in `discipline` on every code-tier
run and by the local half's own `discipline()`. That is the same argument
`scripts/base-test-listing.sh` carries for the same shape — a hosted-only
subject whose guards were the ones that fired on a real 403, with a selftest
written against stubs so they run anywhere.

NO LOCAL MIRROR FOR THE READING ITSELF. Its subject is a hosted run's own job
list and a developer box has no such list; `local-scripts/gate.sh` answers the
same question by being one process, where the exit status IS the summary this
reconstructs. What IS mirrored, and belongs in both halves, is the selftest.

    check-run-jobs.py --self "<this job's name>"
    check-run-jobs.py --selftest

Reads GITHUB_REPOSITORY, GITHUB_RUN_ID and (through `gh`) GH_TOKEN from the
environment. Needs `actions: read` and nothing else.
"""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
import tempfile

# Above the job count this workflow can produce — 33 on a code-tier run at the
# 2026-09-04 job set (17 single jobs + 6 `test` + 6 `test-interval` + 3 nested
# `render lanes / …` + `gate ok` itself, which the run 33894380300 receipt
# confirms as "32 jobs" plus the caller). Nothing computes on that number: the
# guard is `total_count`, checked below, so a run that outgrows this page is a
# RED that says to raise it and never a silent summary of a subset.
PER_PAGE = 100

# GitHub's own definition of a passing check status. See the module docstring.
PASSING = ("success", "skipped", "neutral")


def read_jobs(repo: str, run_id: str) -> tuple[dict | None, str]:
    """The run's job list, or (None, the reason there is none).

    `filter=latest` so a re-run of failed jobs is read as the run's CURRENT
    state rather than as its history.
    """
    url = (
        f"repos/{repo}/actions/runs/{run_id}/jobs"
        f"?filter=latest&per_page={PER_PAGE}"
    )
    proc = subprocess.run(
        ["gh", "api", url], capture_output=True, text=True, check=False
    )
    if proc.returncode != 0:
        return None, proc.stderr.strip() or f"gh api exited {proc.returncode}"
    try:
        doc = json.loads(proc.stdout)
    except ValueError as exc:
        # A zero exit with a body that is not the listing is the same failure as
        # a nonzero exit and is answered the same way. It has its own selftest
        # case because it is the shape a proxy error page takes.
        return None, f"the response was not JSON: {exc}"
    if not isinstance(doc, dict) or "jobs" not in doc:
        return None, "the response carries no `jobs` key"
    return doc, ""


def check(doc: dict, self_name: str, tier: str, run_build: str, out) -> int:
    jobs = doc.get("jobs", [])
    total = doc.get("total_count")
    if total is not None and total > len(jobs):
        print(
            f"FAILED: the jobs API reports {total} jobs and this page carried "
            f"{len(jobs)}.",
            file=out,
        )
        print(
            "A paged job list would let this gate summarise a subset of the run "
            "and call it",
            file=out,
        )
        print("the run. Raise PER_PAGE (or page) in this script.", file=out)
        return 1

    others = [j for j in jobs if j.get("name") != self_name]
    if not others:
        print("FAILED: this run's job list names no job but this one.", file=out)
        print(
            "Every claim below is about a population, and an empty one passes "
            "for the wrong",
            file=out,
        )
        print("reason.", file=out)
        return 1

    running = [j for j in others if j.get("status") != "completed"]
    bad = [
        j
        for j in others
        if j.get("status") == "completed" and j.get("conclusion") not in PASSING
    ]

    width = max(len(j.get("name", "")) for j in others)
    for j in sorted(others, key=lambda j: j.get("name", "")):
        state = j.get("conclusion") or j.get("status")
        print(f"  {j.get('name', ''):<{width}}  {state}", file=out)

    if running:
        print(file=out)
        print("FAILED: these jobs had not finished when this gate ran:", file=out)
        for j in running:
            print(f"  {j.get('name')} ({j.get('status')})", file=out)
        print(file=out)
        print(
            "This gate runs last because `needs:` names every other job in "
            "ci.yml. A job",
            file=out,
        )
        print(
            "that is still going is one this gate did not wait for — add it to "
            "`needs:`.",
            file=out,
        )
        print(
            "Reported as a failure and not as a pass: a check that summarises a "
            "run it",
            file=out,
        )
        print(
            "did not see the end of is the exact thing this job exists to "
            "prevent.",
            file=out,
        )
        return 1

    if bad:
        print(file=out)
        print("FAILED: these jobs did not pass:", file=out)
        for j in bad:
            print(f"  {j.get('name')} ({j.get('conclusion')})", file=out)
        print(file=out)
        print(
            "Open the named job. This gate reports nothing of its own — it is "
            "one check",
            file=out,
        )
        print(
            "name standing for the whole run, so that a branch protection or a "
            "merge queue",
            file=out,
        )
        print(
            "can require one name that is present on every tier.",
            file=out,
        )
        return 1

    print(file=out)
    plural = "" if len(others) == 1 else "s"
    print(
        f"gate ok: {len(others)} job{plural}, all success, skipped or neutral.",
        file=out,
    )
    # WHAT THE ONE GREEN NAME GATED, and not only how many rows it counted.
    # Once `gate ok` is the only required check its one line is the whole
    # reading surface for "what did this run gate?", and a count answers a
    # different question: this file elsewhere calls a green job name over a
    # silent skip "the failure mode to avoid". So the tier and the build flag
    # are printed beside the tally, and a run whose build rows were all skipped
    # says so in a sentence rather than leaving the reader to count `skipped`s.
    counts = {}
    for j in others:
        counts[j.get("conclusion")] = counts.get(j.get("conclusion"), 0) + 1
    tally = ", ".join(f"{counts[k]} {k}" for k in PASSING if counts.get(k))
    print(
        f"  tier: {tier or 'unreported'}   run_build: "
        f"{run_build or 'unreported'}   ({tally})",
        file=out,
    )
    if run_build and run_build != "true":
        print(file=out)
        print(
            "NOTE: run_build was not true on this run, so every build, test and "
            "lint row",
            file=out,
        )
        print(
            "above is `skipped`. This green says that nothing which RAN failed. "
            "On a",
            file=out,
        )
        print(
            "documentation-tier change or a draft pull request that is the "
            "intended state",
            file=out,
        )
        print("and the tier line says which; it is not a claim about the tree.", file=out)
    return 0


def run(self_name: str, tier: str = "", run_build: str = "", out=sys.stdout) -> int:
    repo = os.environ.get("GITHUB_REPOSITORY", "")
    run_id = os.environ.get("GITHUB_RUN_ID", "")
    if not repo or not run_id:
        print(
            "FAILED: $GITHUB_REPOSITORY and $GITHUB_RUN_ID name the run to "
            "summarise, and one",
            file=out,
        )
        print(
            "of them is unset. This check only means anything inside a GitHub "
            "Actions run.",
            file=out,
        )
        return 1
    doc, reason = read_jobs(repo, run_id)
    if doc is None:
        print("FAILED: could not read this run's job list.", file=out)
        print(
            "This gate summarises the run through the jobs API, so an "
            "unreadable list is a",
            file=out,
        )
        print(
            "RED and never a pass: with no list there is nothing to assert "
            "about.",
            file=out,
        )
        for line in reason.splitlines():
            print(f"  {line}", file=out)
        return 1
    return check(doc, self_name, tier, run_build, out)


# ---------------------------------------------------------------- selftest

_GH_STUB = """#!/bin/sh
# `gh api <path>` answers $STUB_BODY_FILE and exits $STUB_STATUS. An error body
# on STDERR with a nonzero exit is how the real `gh` reports a 403.
if [ "${STUB_STATUS:-0}" != "0" ]; then
  printf '%s\\n' "${STUB_STDERR:-gh: refused}" >&2
  exit "${STUB_STATUS}"
fi
cat "${STUB_BODY_FILE}"
"""


def _job(name, status="completed", conclusion="success"):
    return {"name": name, "status": status, "conclusion": conclusion}


def selftest() -> int:
    """The seven decision paths, each driven against a stub `gh`.

    They are the seven the lane drove by hand against fixture job lists before
    the logic went into `ci.yml`, and nothing re-drove them once it was there.
    The unreadable-API path carries two cases (a nonzero exit and a zero exit
    with a body that is not the listing) because they are one disposition
    reached two ways.
    """
    failures = []

    def want(needle, text, case):
        if needle not in text:
            failures.append(f"{case}: expected {needle!r} in output:\n{text}")

    def reject(needle, text, case):
        if needle in text:
            failures.append(f"{case}: did NOT expect {needle!r} in output:\n{text}")

    with tempfile.TemporaryDirectory() as tmp:
        binpath = os.path.join(tmp, "bin")
        os.makedirs(binpath)
        stub = os.path.join(binpath, "gh")
        with open(stub, "w", encoding="utf-8") as fh:
            fh.write(_GH_STUB)
        os.chmod(stub, 0o755)  # noqa: S103 -- a stub on a private PATH in a temp dir
        body = os.path.join(tmp, "body.json")

        def case(doc_text, *, status=0, stderr="", tier="code", run_build="true"):
            with open(body, "w", encoding="utf-8") as fh:
                fh.write(doc_text)
            env = dict(os.environ)
            env["PATH"] = binpath + os.pathsep + env.get("PATH", "")
            env["GITHUB_REPOSITORY"] = "owner/repo"
            env["GITHUB_RUN_ID"] = "1"
            env["STUB_BODY_FILE"] = body
            env["STUB_STATUS"] = str(status)
            env["STUB_STDERR"] = stderr
            proc = subprocess.run(
                [
                    sys.executable,
                    os.path.abspath(__file__),
                    "--self",
                    "gate ok",
                    "--tier",
                    tier,
                    "--run-build",
                    run_build,
                ],
                capture_output=True,
                text=True,
                env=env,
                check=False,
            )
            return proc.returncode, proc.stdout + proc.stderr

        def listing(jobs, total=None):
            return json.dumps(
                {"total_count": len(jobs) if total is None else total, "jobs": jobs}
            )

        # 1 — GREEN. Every job terminal and acceptable, the caller excluded from
        # the population and from the count.
        rc, out = case(
            listing(
                [
                    _job("gate ok"),
                    _job("filter"),
                    _job("test (eps = 1e-12, 1/2)"),
                    _job("docs-only ok", conclusion="skipped"),
                ]
            )
        )
        if rc != 0:
            failures.append(f"case 1 (green) exited {rc}:\n{out}")
        want("gate ok: 3 jobs, all success, skipped or neutral.", out, "case 1")
        want("tier: code   run_build: true   (2 success, 1 skipped)", out, "case 1")
        reject("FAILED", out, "case 1")
        reject("NOTE: run_build", out, "case 1")

        # 1b — THE SAME GREEN ON A RUN THAT BUILT NOTHING. It is still a pass
        # and it must not read as one about the tree: this is the "green job
        # name over a silent skip" case, and the one required check's one line
        # is the whole reading surface for it.
        rc, out = case(
            listing(
                [
                    _job("gate ok"),
                    _job("filter"),
                    _job("clippy", conclusion="skipped"),
                ]
            ),
            tier="docs",
            run_build="false",
        )
        if rc != 0:
            failures.append(f"case 1b (green, run_build false) exited {rc}:\n{out}")
        want("tier: docs   run_build: false", out, "case 1b")
        want("NOTE: run_build was not true on this run", out, "case 1b")

        # 2 — A FAILED JOB. Red, and it names the job and its conclusion.
        rc, out = case(
            listing([_job("gate ok"), _job("clippy", conclusion="failure")])
        )
        if rc != 1:
            failures.append(f"case 2 (a failed job) exited {rc}:\n{out}")
        want("FAILED: these jobs did not pass:", out, "case 2")
        want("clippy (failure)", out, "case 2")

        # 3 — A JOB MISSING FROM `needs:`, STILL RUNNING. The omission this
        # design is most exposed to, and it is a red naming the job to add.
        rc, out = case(
            listing(
                [
                    _job("gate ok"),
                    _job("fmt"),
                    _job("k-lint (gate)", status="in_progress", conclusion=None),
                ]
            )
        )
        if rc != 1:
            failures.append(f"case 3 (a job still running) exited {rc}:\n{out}")
        want("FAILED: these jobs had not finished", out, "case 3")
        want("k-lint (gate) (in_progress)", out, "case 3")
        want("add it to `needs:`", out, "case 3")

        # 4 — A PAGED JOB LIST. `total_count` above the page: red before any
        # claim is made about the population, because the population is partial.
        rc, out = case(listing([_job("gate ok"), _job("fmt")], total=140))
        if rc != 1:
            failures.append(f"case 4 (a paged list) exited {rc}:\n{out}")
        want("the jobs API reports 140 jobs and this page carried 2", out, "case 4")

        # 5 — AN EMPTY POPULATION. A run whose job list names nobody but the
        # caller: every claim below is about a population, and an empty one
        # passes for the wrong reason.
        rc, out = case(listing([_job("gate ok")]))
        if rc != 1:
            failures.append(f"case 5 (an empty population) exited {rc}:\n{out}")
        want("names no job but this one", out, "case 5")

        # 6 — `neutral` IS A PASS. Unreachable for a workflow job today (see the
        # docstring); on the list because GitHub's definition of a passing check
        # is what this reads by.
        rc, out = case(
            listing([_job("gate ok"), _job("renders", conclusion="neutral")])
        )
        if rc != 0:
            failures.append(f"case 6 (neutral) exited {rc}:\n{out}")
        want("gate ok: 1 job, all success, skipped or neutral.", out, "case 6")
        want("1 neutral", out, "case 6")

        # 7a — AN UNREADABLE API. A 403 on the jobs endpoint is a RED: with no
        # list there is nothing to assert about.
        rc, out = case("", status=1, stderr="HTTP 403: Resource not accessible")
        if rc != 1:
            failures.append(f"case 7a (gh failed) exited {rc}:\n{out}")
        want("could not read this run's job list", out, "case 7a")
        want("HTTP 403", out, "case 7a")

        # 7b — THE SAME DISPOSITION, REACHED BY A ZERO EXIT. A body that is not
        # the listing (a proxy error page) must not be read as an empty run.
        rc, out = case("<html>nope</html>")
        if rc != 1:
            failures.append(f"case 7b (a non-JSON body) exited {rc}:\n{out}")
        want("could not read this run's job list", out, "case 7b")
        want("was not JSON", out, "case 7b")

        # AND THE ENVIRONMENT ITSELF. Not one of the seven: this is the path
        # that says the check only means anything inside a run.
        env = dict(os.environ)
        env["PATH"] = binpath + os.pathsep + env.get("PATH", "")
        env.pop("GITHUB_RUN_ID", None)
        env["GITHUB_REPOSITORY"] = "owner/repo"
        proc = subprocess.run(
            [sys.executable, os.path.abspath(__file__), "--self", "gate ok"],
            capture_output=True,
            text=True,
            env=env,
            check=False,
        )
        if proc.returncode != 1:
            failures.append(f"case 8 (no run id) exited {proc.returncode}")
        want("name the run to summarise", proc.stdout + proc.stderr, "case 8")

    for f in failures:
        print(f"SELFTEST FAILURE — {f}", file=sys.stderr)
    if failures:
        print(f"check-run-jobs.py --selftest: {len(failures)} failure(s)")
        return 1
    print("check-run-jobs.py --selftest: 10 cases over 7 decision paths, all pass")
    return 0


def main(argv: list[str]) -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument(
        "--self",
        dest="self_name",
        default="",
        help="the name of the job running this, excluded from the population",
    )
    ap.add_argument(
        "--tier",
        default="",
        help="the `filter` job's TIER, printed beside the tally so the one "
             "green name says what it gated",
    )
    ap.add_argument(
        "--run-build",
        dest="run_build",
        default="",
        help="the `filter` job's RUN_BUILD, so a run whose build rows all "
             "skipped says so",
    )
    ap.add_argument("--selftest", action="store_true")
    args = ap.parse_args(argv)
    if args.selftest:
        return selftest()
    if not args.self_name:
        print("FAILED: --self <this job's name> is required.", file=sys.stdout)
        print(
            "Without it this job would summarise itself as a job that has not "
            "finished.",
            file=sys.stdout,
        )
        return 1
    return run(args.self_name, args.tier, args.run_build)


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
