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

THE VERDICT NEEDS NO MODEL. Every arm is a build plus a run, and the run
executes what the build produced, so the verdict is an argmin and nothing more:

    the winner is  argmin over the measured levels of  (a + E)

where `a` is the archive/build step and `E` the suite's execution. Nothing is
extrapolated, no ratio is projected onto anything, and `r = E0/E2` is recorded
as a DERIVED figure — the thing to compare against the 6.46/7.08 the note
quotes — rather than as an input to the decision.

THREE ARMS SINCE 2026-08-25, AND WHY THE PAIR WAS THE WRONG SHAPE. Every
artifact in this decision's history — #52/#53, #449, the census, the note in
ci.yml, this lane's first two samples — compares opt-0 against opt-2 and
nothing else. `opt-level = 1` had never been measured, proposed or rejected
anywhere in the repository. That is not a considered omission, it is the
question never having been asked: `a + E` is being minimised over a knob with
four settings, the two arms sit at opposite extremes of BOTH terms, and the
build penalty opt-2 swallows to buy its execution win (`a2 - a0`, 499 s in the
2026-08-25 sample) is more than twice the margin it wins by (220 s). An
interior point has room, and arm C is that point.

A three-arm sweep on a 4-core AVX-512 guest (2026-08-25, 3489 tests, all three
green) says the shape is real: opt-0 143 + 289 = 432 s, opt-1 307 + 60 =
**367 s**, opt-2 427 + 58 = 485 s. opt-1 came within 3% of opt-2's execution
for 58% of its build penalty, and won outright. THAT BOX IS NOT THIS LANE'S
BOX — it is the same 4-core class the census used, and its own ratio is
precisely the number `scripts/check-ci-mirror-parity.py` declares this lane
exists to distrust. The sweep is why arm C is here; only arm C's own samples,
taken on the runner, can say what the runner does.

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
    opt-level-calibrate.py record --arm-a <json> --arm-b <json> [--arm-c <json>]
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

# SCHEMA 2 (2026-08-25) ADDED THE THIRD ARM. A schema-1 sample has `arm_a`
# and `arm_b` only, and `derived.verdict` there was a choice between two
# settings; from 2 it is an argmin over every arm the sample carries, and
# `margin_s`/`margin_ratio` are the winner against the RUNNER-UP rather than
# always opt-0 against opt-2. The historical orientation is kept beside them
# as `pair_opt0_over_opt2_ratio`, so nothing a schema-1 reader wanted is gone.
SCHEMA = 2


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
        #
        # READ THE OPT-LEVEL PAIR HERE AS THE JOB'S BASELINE, NOT AS THE
        # SAMPLE'S. This runs in the record step, so it sees the job-level
        # environment — which is arm B's. Each arm carries its own
        # `opt_level`, and that is the field to read: a three-arm sample whose
        # `cargo_profile` says 0 has still measured 0, 1 and 2.
        "cargo_profile": {k: v for k, v in sorted(os.environ.items())
                          if k.startswith("CARGO_PROFILE_")},
        "opt_level_varied_per_arm": True,
        # dev/test default debug-assertions ON and nothing here turns them
        # off; recorded as what it is rather than assumed by a reader.
        "debug_assertions": os.environ.get("CARGO_PROFILE_TEST_DEBUG_ASSERTIONS", "default(on)"),
        "tolerance_eps": os.environ.get("CAD_TOLERANCE_EPS", ""),
        "nightly_suite_cfg": "--cfg nightly_suite" in os.environ.get("RUSTFLAGS", ""),
    }


def derive(arm_a: dict, arm_b: dict, arm_c: dict | None = None) -> dict:
    """The verdict, as an ARGMIN over the arms this sample carries.

    WHY IT IS NOT A PAIR ANY MORE (2026-08-25). The two-arm version could only
    answer "opt-2 or opt-0", and nobody had ever checked that this was the
    right question: `a + E` is being minimised over a knob with more than two
    settings, and the two arms sat at opposite extremes of BOTH terms. opt-1
    is the interior point — rustc's optimiser on, its most expensive passes
    off — and it had never appeared in any measurement, note or decision in
    this repository.

    Arm C is OPTIONAL. A sample carrying only the original pair derives what
    it always derived, including the tie (a dead heat still reads opt-0, the
    cheaper thing to build), so the two committed schema-1 samples stay
    readable against this code."""
    a2, e2 = float(arm_a["a2"]), float(arm_a["E2"])
    a0, e0 = float(arm_b["a0"]), float(arm_b["E0"])
    total2, total0 = a2 + e2, a0 + e0
    out = {
        "total_opt2_s": total2,
        "total_opt0_s": total0,
        # THE INPUTS, kept beside the verdict so the next reader can tell when
        # the conclusion expired rather than inheriting a bare `opt-level = 2`.
        "r_execution_ratio": (e0 / e2) if e2 else None,
        "archive_delta_s": a2 - a0,
        "L_over_T_opt2": (a2 / total2) if total2 else None,
        "L_over_T_opt0": (a0 / total0) if total0 else None,
        # The figure ci.yml's OPT LEVEL note quotes for this lane. Written
        # here so a diff of two samples shows the drift against it directly.
        "r_quoted_by_ci_yml_default_lane": 6.46,
        # The historical orientation of `margin_ratio` (opt-0 over opt-2),
        # kept under its own name now that `margin_ratio` means winner over
        # runner-up: a schema-1 reader diffing samples across the bump wants
        # the series it was already reading, not a silently re-pointed one.
        "pair_opt0_over_opt2_ratio": (total0 / total2) if total2 else None,
    }
    # INSERTION ORDER IS THE TIE-BREAK, and it is deliberate: `sorted` is
    # stable, so an exact dead heat resolves to whichever arm is listed first
    # — opt-0, then opt-2, then opt-1. That reproduces the two-arm rule this
    # replaces (`opt-2 if total2 < total0 else opt-0`) exactly.
    totals = {"opt-0": total0, "opt-2": total2}
    if arm_c:
        a1, e1 = float(arm_c["a1"]), float(arm_c["E1"])
        total1 = a1 + e1
        totals["opt-1"] = total1
        out.update({
            "total_opt1_s": total1,
            # Both ratios a reader of the opt-1 arm wants: what it recovers of
            # the execution win (against opt-0) and what it costs to build.
            "r_execution_ratio_opt1": (e0 / e1) if e1 else None,
            "archive_delta_opt1_s": a1 - a0,
            "L_over_T_opt1": (a1 / total1) if total1 else None,
            # THE TWO NUMBERS THE THIRD ARM EXISTS TO PRODUCE. opt-1 is worth
            # having exactly when it keeps opt-2's execution while refusing
            # opt-2's build cost, so both are recorded as fractions rather
            # than left for a reader to divide out of the table:
            #   ~1.0 execution_kept  = opt-1 runs as fast as opt-2
            #   <1.0 build_penalty_kept = opt-1 pays less than opt-2 to get it
            "execution_kept_vs_opt2": (e2 / e1) if e1 else None,
            "build_penalty_kept_vs_opt2": ((a1 - a0) / (a2 - a0)) if (a2 - a0) else None,
        })
    ranked = sorted(totals.items(), key=lambda kv: kv[1])
    out["totals_s"] = totals
    out["verdict"] = ranked[0][0]
    out["runner_up"] = ranked[1][0]
    # The margin as the reader wants it BOTH ways: seconds say how much slack
    # there is, the ratio says how far the inputs would have to move. Against
    # the RUNNER-UP since schema 2 — with two arms that is the same pair of
    # numbers as before, and with three it is the one that decides anything.
    out["margin_s"] = ranked[1][1] - ranked[0][1]
    out["margin_ratio"] = (ranked[1][1] / ranked[0][1]) if ranked[0][1] else None
    return out


def record(arm_a: dict, arm_b: dict, history: str, sha: str,
           arm_c: dict | None = None) -> tuple[dict, str, str]:
    now = int(time.time())
    sample = {
        "schema": SCHEMA,
        "commit": sha,
        "measured_at_epoch_s": now,
        "measured_at_utc": datetime.fromtimestamp(now, tz=timezone.utc).isoformat(),
        "arm_a": {k: v for k, v in arm_a.items()},
        "arm_b": arm_b,
        "derived": derive(arm_a, arm_b, arm_c),
        "environment": environment(),
        "method": (
            "arm A (opt-2) READ from the step durations of recent code-tier gate runs via the "
            "jobs API — real gate data, never re-run; arms B (opt-0) and C (opt-1) measured "
            "here, each on its own clean target directory, in the GATE test population (NO "
            "--cfg nightly_suite). Verdict is the direct argmin of a+E over the arms present; "
            "REPORTING ONLY, never a gate."
        ),
    }
    # OMITTED, NOT NULLED, when the third arm did not run: a `null` arm_c
    # would read as "opt-1 was measured and came back empty", which is the
    # one thing it must never be mistaken for.
    if arm_c:
        sample["arm_c"] = arm_c
    name = f"{now}-{sha[:7]}.json"
    os.makedirs(history, exist_ok=True)
    dest = os.path.join(history, name)
    with open(dest, "w", encoding="utf-8") as fh:
        json.dump(sample, fh, indent=1, sort_keys=True)
        fh.write("\n")
    return sample, dest, summary(sample)


def summary(sample: dict) -> str:
    d, a, b = sample["derived"], sample["arm_a"], sample["arm_b"]
    c = sample.get("arm_c")
    tree = os.environ.get("CI_TREE_OPT_LEVEL", "2")
    flip = d["verdict"] != f"opt-{tree}"
    rows = [
        f"| opt-2 (arm A, read from {a.get('n', 0)} gate runs) | {a['a2']:.0f} s | {a['E2']:.0f} s "
        f"| {d['total_opt2_s']:.0f} s |",
        f"| opt-0 (arm B, measured here) | {b['a0']:.0f} s | {b['E0']:.0f} s "
        f"| {d['total_opt0_s']:.0f} s |",
    ]
    if c:
        rows.append(f"| **opt-1 (arm C, measured here)** | {c['a1']:.0f} s | {c['E1']:.0f} s "
                    f"| **{d['total_opt1_s']:.0f} s** |")
    out = [
        "### opt-level calibration",
        "",
        f"**Verdict: {d['verdict']} wins** — {d['margin_s']:.0f} s of margin "
        f"({d['margin_ratio']:.2f}x) over {d['runner_up']}. The tree is set to opt-level {tree}.",
        "",
        ("> **THIS IS A FLIP.** The tree's setting and this measurement disagree. That is the "
         "rare actionable event this lane exists for — read the inputs below, then "
         "ci.yml's OPT LEVEL note, before changing anything. Nothing here gates; no row is red."
         if flip else
         "The measurement agrees with the tree's setting. Nothing to do."),
        "",
        "| | archive (a) | execution (E) | total |",
        "|---|---|---|---|",
        *rows,
        "",
        f"* `r` (opt-0/opt-2 execution ratio) = **{d['r_execution_ratio']:.2f}** — "
        f"ci.yml's OPT LEVEL note quotes {d['r_quoted_by_ci_yml_default_lane']} for this lane.",
        f"* `a2 - a0` (what opt-2 costs to build) = **{d['archive_delta_s']:.0f} s**.",
        f"* build share of the total: {d['L_over_T_opt2'] * 100:.0f}% at opt-2, "
        f"{d['L_over_T_opt0'] * 100:.0f}% at opt-0"
        + (f", {d['L_over_T_opt1'] * 100:.0f}% at opt-1." if c else "."),
    ]
    if c:
        out += [
            f"* **the third arm's two numbers**: opt-1 keeps "
            f"**{d['execution_kept_vs_opt2'] * 100:.0f}%** of opt-2's execution speed for "
            f"**{d['build_penalty_kept_vs_opt2'] * 100:.0f}%** of opt-2's build penalty "
            f"(`a1 - a0` = {d['archive_delta_opt1_s']:.0f} s against "
            f"`a2 - a0` = {d['archive_delta_s']:.0f} s).",
        ]
    out += [
        f"* tests executed: {a.get('tests', 'n/a')} (arm A) / {b.get('tests', 'n/a')} (arm B)"
        + (f" / {c.get('tests', 'n/a')} (arm C)" if c else "")
        + " — these must match, or the arms measured different suites.",
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
        # TWO ARMS STILL DERIVE THE TWO-ARM ANSWER. `margin_s` is the pair's
        # gap, the historical ratio is kept under its own name, and no arm_c
        # key is invented — the two committed schema-1 samples must stay
        # readable against this code.
        assert "arm_c" not in sample and "total_opt1_s" not in sample["derived"]
        assert abs(sample["derived"]["margin_s"] - abs(603.0 - 551.0)) < 0.01
        assert abs(sample["derived"]["pair_opt0_over_opt2_ratio"] - 603.0 / 551.0) < 0.01
        assert sample["derived"]["runner_up"] == "opt-0"
        # A DEAD HEAT READS opt-0, exactly as the two-arm rule it replaces did
        # (`opt-2 if total2 < total0 else opt-0`). Measure-zero, and pinned so
        # that a refactor of the ranking cannot quietly re-point it.
        tied = derive(arm_a, {"a0": 0.0, "E0": 551.0})
        assert tied["verdict"] == "opt-0", tied

        # ---------------------------------------------------------- ARM C.
        # THE SHAPE THE THIRD ARM EXISTS TO CATCH: opt-1 within a hair of
        # opt-2's execution for a fraction of its build. Both two-arm answers
        # are wrong here — opt-2 beats opt-0, and opt-1 beats them both — and
        # that is the whole argument for measuring a third point.
        arm_c = {"a1": 260.0, "E1": 124.0, "tests": 2791}
        three, _, text3 = record(arm_a, arm_b, t, "c" * 40, arm_c)
        d3 = three["derived"]
        assert d3["verdict"] == "opt-1" and d3["runner_up"] == "opt-2", d3
        assert abs(d3["total_opt1_s"] - 384.0) < 0.01
        assert abs(d3["margin_s"] - (551.0 - 384.0)) < 0.01
        # opt-1 keeps 119/124 of opt-2's execution speed for (260-127)/(432-127)
        # of its build penalty. These two fractions ARE the finding.
        assert abs(d3["execution_kept_vs_opt2"] - 119.0 / 124.0) < 0.01
        assert abs(d3["build_penalty_kept_vs_opt2"] - 133.0 / 305.0) < 0.01
        assert three["arm_c"] == arm_c
        assert "opt-1 (arm C" in text3 and "THIS IS A FLIP" in text3
        # AND IT MUST NOT WIN BY DEFAULT. A slow, expensive opt-1 loses, and
        # the pair's own answer survives untouched.
        loser, _, _ = record(arm_a, arm_b, t, "d" * 40, {"a1": 400.0, "E1": 400.0})
        assert loser["derived"]["verdict"] == "opt-2", loser["derived"]

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
          "SKIPS rather than zeroes; the verdict is the direct a+E argmin over the arms "
          "present, which for two arms is the two-arm answer it replaced (ties included); the "
          "optional third arm wins only when it earns it and is OMITTED rather than nulled "
          "when it did not run; the cadence fires on a first sample, on the calendar and on "
          ">20% drift in the free half, and holds otherwise; no arm A means no measured arm "
          "is spent; and a flip is reported loudly without failing anything")


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("mode", nargs="?", choices=("read-arm-a", "decide", "record"))
    ap.add_argument("--selftest", action="store_true")
    ap.add_argument("--out")
    ap.add_argument("--arm-a")
    ap.add_argument("--arm-b")
    ap.add_argument("--arm-c", help="the opt-1 arm, if it was measured; the sample "
                                    "omits arm C entirely when this is absent")
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
                # THE SAME DECISION, UNDER THE NAME THE THIRD ARM'S STEP READS.
                # Both measured arms run together or not at all: two arms taken
                # on different nights are two different trees, and comparing
                # them is the mistake this whole lane is built to avoid.
                fh.write(f"run_arm_c={'true' if run else 'false'}\n")
                fh.write(f"reason={why}\n")
        return 0

    with open(args.arm_b, encoding="utf-8") as fh:
        arm_b = json.load(fh)
    arm_c = None
    if args.arm_c and os.path.exists(args.arm_c):
        with open(args.arm_c, encoding="utf-8") as fh:
            arm_c = json.load(fh)
    _, dest, text = record(arm_a, arm_b, args.history, args.sha, arm_c)
    print(f"wrote {dest}")
    print(text)
    if args.summary:
        with open(args.summary, "a", encoding="utf-8") as fh:
            fh.write(text + "\n")
    return 0


if __name__ == "__main__":
    sys.exit(main())
