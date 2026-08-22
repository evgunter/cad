#!/usr/bin/env python3
"""The opt-level calibration lane: does `opt-level = 2` on the two archive
jobs still pay, ON THE BOX CI ACTUALLY RUNS ON?

WHY THIS EXISTS. ci.yml's `build` job carries a long OPT LEVEL note whose
verdict rests on one quantity: `r`, the opt-0/opt-2 EXECUTION ratio, quoted
there as 6.46 (default lane) and 7.08 (interval). Those numbers came from a
developer's box. A 2026-08-22 census re-measured the same ratio at **4.95 /
4.99** on a 4-core AVX-512 guest — about 30% less than the figure the verdict
is built on, enough that the note's "~2x and ~3x margins" would become 0.94x
and 0.91x, i.e. opt-0 winning outright. That is NOT a licence to flip: the
census box is not CI's 2-vCPU runner, and a ratio is exactly the kind of
number that does not transfer between machines. It IS a demonstration that the
quantity CI relies on has never been measured where CI runs.

So this lane measures it there, and keeps measuring it, because the
conclusion has expired once already — opt-2 (#449) was itself a reversal of an
earlier opt-0 verdict (#52/#53) whose premises went stale. A bare
`opt-level = 2` in a workflow tells the next reader nothing about when it
stopped being true.

THE VERDICT NEEDS NO MODEL. Both arms are a build plus a run, and the run
executes what the build produced:

    opt-2 wins  iff  a2 + E2  <  a0 + E0

where `a` is the archive/build step and `E` the suite's execution. Nothing is
extrapolated, no ratio is projected onto anything, and `r = E0/E2` is recorded
as a DERIVED figure — the thing to compare against the 6.46/7.08 the note
quotes — rather than as an input to the decision.

ASYMMETRIC, BECAUSE HALF OF IT IS ALREADY FREE. Arm A (opt-2) is not re-run:
every code-tier gate run already builds that archive and executes that suite,
and the jobs API reports each step's start and finish. So arm A is READ from
recent real gate runs — always current, never synthetic, and costing nothing.
Only arm B (opt-0) is a deliberate measurement, and it is the only reason this
lane costs minutes at all.

WHAT ARM B MUST NOT BE, and it is the one mistake that would produce a
confident wrong answer: arm B has to run the GATE'S test population. Building
it with `--cfg nightly_suite` would add the demoted tests, so `E0` would be
computed over a bigger suite than `E2` and the comparison would silently be
between two different questions. The workflow's arm-B step therefore does NOT
set that flag, and this script records the test COUNT of each arm so a reader
can check that they match.

REPORTING, NEVER GATING (memories/perf-measurement-lane.md, the standing rule
for every timing lane here). Nothing in this file exits non-zero on a
millisecond. A FLIP — the verdict disagreeing with the tree's current setting
— is surfaced loudly in the job summary and is the rare actionable event; a
margin that narrowed is a note for whoever reads next.

    opt-level-calibrate.py read-arm-a --out <json> [--runs N] [--window N]
    opt-level-calibrate.py decide --arm-a <json> --history <dir>
                                  [--max-age-days N] [--drift F]
    opt-level-calibrate.py record --arm-a <json> --arm-b <json>
                                  --history <dir> --sha <sha> [--summary <path>]
    opt-level-calibrate.py --selftest
"""

from __future__ import annotations

import argparse
import json
import os
import platform
import statistics
import subprocess
import sys
import time
import urllib.error
import urllib.request
from datetime import datetime, timezone

# The two step names arm A is read from, and the jobs they live in. THESE ARE
# LOAD-BEARING STRINGS: rename a step in ci.yml and this lane goes quiet
# rather than wrong — `read-arm-a` finds no samples and says so, and `decide`
# then refuses to run arm B against a missing arm A. That is the failure
# direction to prefer, but it is still a failure, so the names are here in one
# place with this sentence next to them.
ARCHIVE_JOB = "build + archive (default)"
ARCHIVE_STEP = "build test binaries + archive"
# The `test` job's name carries the sampled ε (`test (eps = 1e-6, 1/2)`), so
# the match is a prefix; the shards are summed, because what the verdict
# compares is the SUITE's execution and a run splits it two ways.
RUN_JOB_PREFIX = "test (eps = "
RUN_STEP = "run archived tests"

# Arm A's sample size. Five is a median over a working day's worth of gate
# runs — enough that one contended runner does not carry the figure, small
# enough that a real change in the tree shows up within a day rather than
# being averaged away over a week.
DEFAULT_RUNS = 5
# How many recent workflow runs to look through to find those five. Most runs
# in the window are docs-tier, superseded by `cancel-in-progress`, or drew the
# interval lane — none of which carry both steps.
DEFAULT_WINDOW = 60

# Recalibration cadence. WEEKLY, plus a DRIFT TRIGGER: arm A is free to read
# on every nightly, so "has the thing we are comparing against moved?" costs
# nothing to ask, and asking it is what stops a quiet week from letting the
# verdict go stale between calendar dates.
MAX_AGE_DAYS = 7.0
DRIFT = 0.20

SCHEMA = 1


# --------------------------------------------------------------- the API half


def _api(path: str) -> dict:
    """One GitHub REST call. Token and host come from the runner's own env."""
    base = os.environ.get("GITHUB_API_URL", "https://api.github.com")
    url = f"{base}{path}"
    if not url.startswith("https://"):
        raise SystemExit(f"opt-level-calibrate: refusing a non-https API URL: {url!r}")
    req = urllib.request.Request(url)  # noqa: S310 — scheme checked directly above
    req.add_header("Accept", "application/vnd.github+json")
    req.add_header("X-GitHub-Api-Version", "2022-11-28")
    token = os.environ.get("GITHUB_TOKEN") or os.environ.get("GH_TOKEN")
    if token:
        req.add_header("Authorization", f"Bearer {token}")
    with urllib.request.urlopen(req, timeout=60) as fh:  # noqa: S310 — as above
        return json.loads(fh.read().decode())


def _seconds(started: str | None, completed: str | None) -> float | None:
    """A step's wall clock, from the two ISO-8601 stamps the jobs API gives."""
    if not started or not completed:
        return None
    fmt = "%Y-%m-%dT%H:%M:%SZ"
    try:
        a = datetime.strptime(started, fmt).replace(tzinfo=timezone.utc)
        b = datetime.strptime(completed, fmt).replace(tzinfo=timezone.utc)
    except ValueError:
        return None
    d = (b - a).total_seconds()
    return d if d >= 0 else None


def _step_seconds(job: dict, name: str) -> float | None:
    for st in job.get("steps") or []:
        if st.get("name") == name:
            if st.get("conclusion") != "success":
                return None
            return _seconds(st.get("started_at"), st.get("completed_at"))
    return None


def sample_run(jobs: list[dict]) -> dict | None:
    """`{a2, E2, shards, labels}` for one workflow run, or None if this run is
    not a DEFAULT-LANE CODE-TIER run — a docs-tier run has no archive job, an
    interval draw has a differently-named one, and a cancelled run has steps
    that never completed. Every one of those is a skip, not an error."""
    a2 = None
    labels: list[str] = []
    for job in jobs:
        if job.get("name") == ARCHIVE_JOB:
            a2 = _step_seconds(job, ARCHIVE_STEP)
            labels = list(job.get("labels") or [])
    if a2 is None:
        return None
    shards = []
    for job in jobs:
        if str(job.get("name", "")).startswith(RUN_JOB_PREFIX):
            s = _step_seconds(job, RUN_STEP)
            if s is None:
                return None  # a partial row would understate E2
            shards.append(s)
    if not shards:
        return None
    return {"a2": a2, "E2": sum(shards), "shards": len(shards), "labels": labels}


def read_arm_a(runs: int, window: int) -> dict:
    repo = os.environ.get("GITHUB_REPOSITORY")
    if not repo:
        raise SystemExit("opt-level-calibrate: GITHUB_REPOSITORY is unset; this reads the "
                         "repository's own run history and has nothing to read without it")
    listing = _api(f"/repos/{repo}/actions/workflows/ci.yml/runs"
                   f"?event=pull_request&status=completed&per_page={window}")
    samples = []
    for run in listing.get("workflow_runs", []):
        if len(samples) >= runs:
            break
        try:
            jobs = _api(f"/repos/{repo}/actions/runs/{run['id']}/jobs?per_page=100")
        except (urllib.error.URLError, OSError, KeyError):
            continue
        got = sample_run(jobs.get("jobs", []))
        if got is None:
            continue
        got["run_id"] = run["id"]
        got["head_sha"] = run.get("head_sha", "")
        got["created_at"] = run.get("created_at", "")
        samples.append(got)
    return summarise_arm_a(samples)


def summarise_arm_a(samples: list[dict]) -> dict:
    if not samples:
        return {"n": 0, "samples": []}
    return {
        "n": len(samples),
        # MEDIAN, not mean: a hosted 2-vCPU runner has a fat tail (the same
        # sentence docs/perf-data/rebuild-latency/README.md carries), and one
        # contended run would drag a mean far enough to move the verdict.
        "a2": statistics.median(s["a2"] for s in samples),
        "E2": statistics.median(s["E2"] for s in samples),
        "samples": samples,
    }


# ------------------------------------------------------------ the cadence half


def _history(path: str) -> list[str]:
    if not os.path.isdir(path):
        return []
    return sorted(f for f in os.listdir(path) if f.endswith(".json"))


def newest_sample(path: str) -> dict | None:
    names = _history(path)
    if not names:
        return None
    with open(os.path.join(path, names[-1]), encoding="utf-8") as fh:
        return json.load(fh)


def decide(arm_a: dict, history: str, max_age_days: float, drift: float,
           now: float | None = None) -> tuple[bool, str]:
    """Does arm B have to run tonight? Returns `(run, reason)`.

    FAILS TOWARDS *NOT* SPENDING THE MINUTES, with one exception. No arm A
    means no comparison is possible, so arm B would produce half a sample and
    a number nobody can read — skip. An empty history means the lane has never
    calibrated — run. Between those, the two triggers are the calendar and
    drift in the free half."""
    if not arm_a.get("n"):
        return False, ("no arm-A samples: no recent code-tier run carried both "
                       f"`{ARCHIVE_JOB} / {ARCHIVE_STEP}` and `{RUN_JOB_PREFIX}… / {RUN_STEP}`. "
                       "Arm B alone measures nothing, so it is not run")
    prev = newest_sample(history)
    if prev is None:
        return True, "no previous calibration in the history: this is the first sample"
    now = time.time() if now is None else now
    age_days = (now - float(prev.get("measured_at_epoch_s", 0))) / 86400.0
    if age_days >= max_age_days:
        return True, f"the last calibration is {age_days:.1f} days old (cadence: {max_age_days:g} days)"
    then = float(prev.get("arm_a", {}).get("E2") or 0.0)
    if then > 0:
        moved = abs(arm_a["E2"] - then) / then
        if moved > drift:
            return True, (f"E2 has moved {moved * 100:.0f}% since the last calibration "
                          f"({then:.0f} s -> {arm_a['E2']:.0f} s), past the {drift * 100:.0f}% "
                          "drift trigger")
    return False, (f"the last calibration is {age_days:.1f} days old and E2 has held "
                   f"({then:.0f} s -> {arm_a['E2']:.0f} s); nothing to re-measure")


# ------------------------------------------------------------- the record half


def _run(cmd: list[str]) -> str:
    try:
        return subprocess.run(cmd, capture_output=True, text=True, timeout=60).stdout.strip()
    except (OSError, subprocess.SubprocessError):
        return ""


def environment() -> dict:
    """The block that makes two samples comparable rather than arguable — the
    same fields ci.yml's rebuild-latency measurement records, for the same
    reason (memories/perf-measurement-lane.md: a committed timing is only
    worth anything if you know which box produced it)."""
    mem = ""
    try:
        with open("/proc/meminfo", encoding="utf-8") as fh:
            mem = fh.readline().split()[1]
    except (OSError, IndexError):
        pass
    return {
        "runner": os.environ.get("RUNNER_ENV_LABEL", "")
                  or f"{platform.system()}/{platform.machine()} {os.environ.get('RUNNER_IMAGE', 'ubuntu-latest')}",
        "os": platform.system().lower(),
        "arch": platform.machine(),
        "nproc": os.cpu_count(),
        "mem_total_kb": int(mem) if mem.isdigit() else None,
        "rustc": _run(["rustc", "-V"]),
        "nextest": _run(["cargo-nextest", "nextest", "--version"]),
        "rustflags": os.environ.get("RUSTFLAGS", ""),
        "rustc_wrapper": os.environ.get("RUSTC_WRAPPER", ""),
        # Every CARGO_PROFILE_* knob in scope, not a hand-picked few: the
        # opt-level pair is the subject, and the debug/strip ones move the
        # archive step too.
        "cargo_profile": {k: v for k, v in sorted(os.environ.items())
                          if k.startswith("CARGO_PROFILE_")},
        # dev/test default debug-assertions ON and nothing here turns them
        # off; recorded as what it is rather than assumed by a reader.
        "debug_assertions": os.environ.get("CARGO_PROFILE_TEST_DEBUG_ASSERTIONS", "default(on)"),
        "tolerance_eps": os.environ.get("CAD_TOLERANCE_EPS", ""),
        "nightly_suite_cfg": "--cfg nightly_suite" in os.environ.get("RUSTFLAGS", ""),
    }


def derive(arm_a: dict, arm_b: dict) -> dict:
    a2, e2 = float(arm_a["a2"]), float(arm_a["E2"])
    a0, e0 = float(arm_b["a0"]), float(arm_b["E0"])
    total2, total0 = a2 + e2, a0 + e0
    return {
        "total_opt2_s": total2,
        "total_opt0_s": total0,
        "verdict": "opt-2" if total2 < total0 else "opt-0",
        # The margin as the reader wants it BOTH ways: seconds say how much
        # slack there is, the ratio says how far the inputs would have to move.
        "margin_s": abs(total0 - total2),
        "margin_ratio": (total0 / total2) if total2 else None,
        # THE INPUTS, kept beside the verdict so the next reader can tell when
        # the conclusion expired rather than inheriting a bare `opt-level = 2`.
        "r_execution_ratio": (e0 / e2) if e2 else None,
        "archive_delta_s": a2 - a0,
        "L_over_T_opt2": (a2 / total2) if total2 else None,
        "L_over_T_opt0": (a0 / total0) if total0 else None,
        # The figure ci.yml's OPT LEVEL note quotes for this lane. Written
        # here so a diff of two samples shows the drift against it directly.
        "r_quoted_by_ci_yml_default_lane": 6.46,
    }


def record(arm_a: dict, arm_b: dict, history: str, sha: str) -> tuple[dict, str, str]:
    now = int(time.time())
    sample = {
        "schema": SCHEMA,
        "commit": sha,
        "measured_at_epoch_s": now,
        "measured_at_utc": datetime.fromtimestamp(now, tz=timezone.utc).isoformat(),
        "arm_a": {k: v for k, v in arm_a.items()},
        "arm_b": arm_b,
        "derived": derive(arm_a, arm_b),
        "environment": environment(),
        "method": (
            "arm A (opt-2) READ from the step durations of recent code-tier gate runs via the "
            "jobs API — real gate data, never re-run; arm B (opt-0) measured here on one clean "
            "target directory, in the GATE test population (NO --cfg nightly_suite). Verdict is "
            "the direct comparison a2+E2 < a0+E0; REPORTING ONLY, never a gate."
        ),
    }
    name = f"{now}-{sha[:7]}.json"
    os.makedirs(history, exist_ok=True)
    dest = os.path.join(history, name)
    with open(dest, "w", encoding="utf-8") as fh:
        json.dump(sample, fh, indent=1, sort_keys=True)
        fh.write("\n")
    return sample, dest, summary(sample)


def summary(sample: dict) -> str:
    d, a, b = sample["derived"], sample["arm_a"], sample["arm_b"]
    tree = os.environ.get("CI_TREE_OPT_LEVEL", "2")
    flip = d["verdict"] != f"opt-{tree}"
    out = [
        "### opt-level calibration",
        "",
        f"**Verdict: {d['verdict']} wins** — {d['margin_s']:.0f} s of margin "
        f"({d['margin_ratio']:.2f}x). The tree is set to opt-level {tree}.",
        "",
        ("> **THIS IS A FLIP.** The tree's setting and this measurement disagree. That is the "
         "rare actionable event this lane exists for — read the inputs below, then "
         "ci.yml's OPT LEVEL note, before changing anything. Nothing here gates; no row is red."
         if flip else
         "The measurement agrees with the tree's setting. Nothing to do."),
        "",
        "| | archive (a) | execution (E) | total |",
        "|---|---|---|---|",
        f"| opt-2 (arm A, read from {a.get('n', 0)} gate runs) | {a['a2']:.0f} s | {a['E2']:.0f} s "
        f"| {d['total_opt2_s']:.0f} s |",
        f"| opt-0 (arm B, measured here) | {b['a0']:.0f} s | {b['E0']:.0f} s "
        f"| {d['total_opt0_s']:.0f} s |",
        "",
        f"* `r` (opt-0/opt-2 execution ratio) = **{d['r_execution_ratio']:.2f}** — "
        f"ci.yml's OPT LEVEL note quotes {d['r_quoted_by_ci_yml_default_lane']} for this lane.",
        f"* `a2 - a0` (what opt-2 costs to build) = **{d['archive_delta_s']:.0f} s**.",
        f"* build share of the total: {d['L_over_T_opt2'] * 100:.0f}% at opt-2, "
        f"{d['L_over_T_opt0'] * 100:.0f}% at opt-0.",
        f"* tests executed: {a.get('tests', 'n/a')} (arm A) / {b.get('tests', 'n/a')} (arm B) — "
        "these must match, or the two arms measured different suites.",
        "",
    ]
    return "\n".join(out)


# ------------------------------------------------------------------- self-test


_FAKE_JOBS = {
    "jobs": [
        {"name": ARCHIVE_JOB, "labels": ["ubuntu-latest"], "steps": [
            {"name": "checkout", "conclusion": "success",
             "started_at": "2026-08-22T00:00:00Z", "completed_at": "2026-08-22T00:00:10Z"},
            {"name": ARCHIVE_STEP, "conclusion": "success",
             "started_at": "2026-08-22T00:00:10Z", "completed_at": "2026-08-22T00:07:22Z"},
        ]},
        {"name": "test (eps = 1e-6, 1/2)", "steps": [
            {"name": RUN_STEP, "conclusion": "success",
             "started_at": "2026-08-22T00:08:00Z", "completed_at": "2026-08-22T00:09:03Z"}]},
        {"name": "test (eps = 1e-6, 2/2)", "steps": [
            {"name": RUN_STEP, "conclusion": "success",
             "started_at": "2026-08-22T00:08:00Z", "completed_at": "2026-08-22T00:08:56Z"}]},
        {"name": "k-lint (gate)", "steps": []},
    ]
}


def selftest() -> None:
    import tempfile

    got = sample_run(_FAKE_JOBS["jobs"])
    assert got is not None and got["shards"] == 2, got
    assert abs(got["a2"] - 432.0) < 0.5, got
    assert abs(got["E2"] - 119.0) < 0.5, got

    # A DOCS-TIER RUN, and the shape that matters: no archive job at all. It
    # must be a skip, not a zero — a zero would enter the median and drag the
    # figure the verdict rests on towards nothing.
    assert sample_run([j for j in _FAKE_JOBS["jobs"] if j["name"] != ARCHIVE_JOB]) is None
    # A CANCELLED SHARD. Half a row understates E2, which biases the verdict
    # towards opt-2 — the direction nobody would question. So it is refused.
    partial = json.loads(json.dumps(_FAKE_JOBS["jobs"]))
    partial[1]["steps"][0]["conclusion"] = "cancelled"
    assert sample_run(partial) is None
    # A RENAMED STEP goes quiet, never wrong.
    renamed = json.loads(json.dumps(_FAKE_JOBS["jobs"]))
    renamed[0]["steps"][1]["name"] = "build test binaries + archive (v2)"
    assert sample_run(renamed) is None

    arm_a = summarise_arm_a([dict(got, run_id=i) for i in range(3)])
    assert arm_a["n"] == 3 and abs(arm_a["E2"] - 119.0) < 0.5

    with tempfile.TemporaryDirectory() as t:
        run, why = decide(arm_a, t, MAX_AGE_DAYS, DRIFT)
        assert run and "first sample" in why, why
        assert decide({"n": 0, "samples": []}, t, MAX_AGE_DAYS, DRIFT)[0] is False

        # opt-0 is 4x slower to execute and 3x cheaper to build here, which on
        # these numbers makes opt-2 win — the arithmetic the whole lane is.
        arm_b = {"a0": 127.0, "E0": 476.0, "tests": 2791}
        sample, dest, text = record(arm_a, arm_b, t, "0123456789abcdef")
        assert sample["derived"]["verdict"] == "opt-2", sample["derived"]
        assert abs(sample["derived"]["r_execution_ratio"] - 4.0) < 0.01
        assert os.path.basename(dest).endswith("-0123456.json")
        assert "opt-2 wins" in text and "| opt-0 (arm B" in text

        # FRESH HISTORY: neither trigger fires.
        run, why = decide(arm_a, t, MAX_AGE_DAYS, DRIFT)
        assert not run and "held" in why, why
        # THE CALENDAR TRIGGER.
        run, why = decide(arm_a, t, MAX_AGE_DAYS, DRIFT, now=time.time() + 8 * 86400)
        assert run and "days old" in why, why
        # THE DRIFT TRIGGER — E2 moved 30% with the calendar still fresh. This
        # is the half that costs nothing to ask and is the reason the cadence
        # is not just a date.
        drifted = dict(arm_a, E2=arm_a["E2"] * 1.3)
        run, why = decide(drifted, t, MAX_AGE_DAYS, DRIFT)
        assert run and "drift trigger" in why, why
        # ...and 10% is inside it.
        run, _ = decide(dict(arm_a, E2=arm_a["E2"] * 1.1), t, MAX_AGE_DAYS, DRIFT)
        assert not run

        # THE FLIP, which is the one event this lane is for: arm B faster
        # overall must say so, loudly, and still not fail.
        flipped, _, text = record(arm_a, {"a0": 127.0, "E0": 300.0, "tests": 2791}, t, "f" * 40)
        assert flipped["derived"]["verdict"] == "opt-0"
        assert "THIS IS A FLIP" in text

    env = environment()
    for key in ("runner", "nproc", "rustflags", "cargo_profile", "debug_assertions",
                "tolerance_eps", "nightly_suite_cfg"):
        assert key in env, key

    print("opt-level-calibrate selftest OK: a run's arm-A sample is read from the archive step "
          "and the summed shards; a docs-tier run, a cancelled shard and a renamed step are "
          "SKIPS rather than zeroes; the verdict is the direct a+E comparison; the cadence "
          "fires on a first sample, on the calendar and on >20% drift in the free half, and "
          "holds otherwise; no arm A means arm B is not spent; and a flip is reported loudly "
          "without failing anything")


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("mode", nargs="?", choices=("read-arm-a", "decide", "record"))
    ap.add_argument("--selftest", action="store_true")
    ap.add_argument("--out")
    ap.add_argument("--arm-a")
    ap.add_argument("--arm-b")
    ap.add_argument("--history", default="docs/perf-data/opt-level")
    ap.add_argument("--sha", default="")
    ap.add_argument("--summary")
    ap.add_argument("--runs", type=int, default=DEFAULT_RUNS)
    ap.add_argument("--window", type=int, default=DEFAULT_WINDOW)
    ap.add_argument("--max-age-days", type=float, default=MAX_AGE_DAYS)
    ap.add_argument("--drift", type=float, default=DRIFT)
    args = ap.parse_args()

    if args.selftest:
        selftest()
        return 0
    if args.mode is None:
        ap.error("a mode is required unless --selftest is given")

    if args.mode == "read-arm-a":
        arm_a = read_arm_a(args.runs, args.window)
        with open(args.out, "w", encoding="utf-8") as fh:
            json.dump(arm_a, fh, indent=1, sort_keys=True)
        print(f"arm A: {arm_a['n']} sample(s)"
              + (f", a2 = {arm_a['a2']:.0f} s, E2 = {arm_a['E2']:.0f} s" if arm_a["n"] else ""))
        return 0

    with open(args.arm_a, encoding="utf-8") as fh:
        arm_a = json.load(fh)

    if args.mode == "decide":
        run, why = decide(arm_a, args.history, args.max_age_days, args.drift)
        print(f"arm B: {'RUN' if run else 'skip'} — {why}")
        out = os.environ.get("GITHUB_OUTPUT")
        if out:
            with open(out, "a", encoding="utf-8") as fh:
                fh.write(f"run_arm_b={'true' if run else 'false'}\n")
                fh.write(f"reason={why}\n")
        return 0

    with open(args.arm_b, encoding="utf-8") as fh:
        arm_b = json.load(fh)
    _, dest, text = record(arm_a, arm_b, args.history, args.sha)
    print(f"wrote {dest}")
    print(text)
    if args.summary:
        with open(args.summary, "a", encoding="utf-8") as fh:
            fh.write(text + "\n")
    return 0


if __name__ == "__main__":
    sys.exit(main())
