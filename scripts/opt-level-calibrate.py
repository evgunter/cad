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

ASYMMETRIC, BECAUSE ONE ARM IS ALREADY FREE — AND IT IS THE TREE'S ARM. Every
code-tier gate run already builds the archive and executes the suite at
whatever `ci.yml` sets, and the jobs API reports each step's start and finish.
So the arm at the TREE's opt level is READ from recent real gate runs — always
current, never synthetic, costing nothing — and the other two are deliberate
measurements, which are the only reason this lane costs minutes at all.

WHICH ARM IS FREE THEREFORE MOVES WHEN THE TREE MOVES, and that is not a
detail. Until 2026-08-25 the free arm was opt-2 because the gate was opt-2,
and the code said `arm_a` and meant `a2`. The gate is opt-1 now. Had the
letters stayed welded to levels, the free read would have gone on filling
`a2`/`E2` with opt-1 durations while a separate measured arm took opt-1 again
— one sample carrying opt-1 twice, once mislabelled, and a verdict computed
off the lie. Schema 3 keys the arms by LEVEL and records `tree_opt_level`, so
the free arm is identified by what it is rather than by where it used to sit.

WHAT A MEASURED ARM MUST NOT BE, and it is the one mistake that would produce
a confident wrong answer: it has to run the GATE'S test population. Building
it with `--cfg nightly_suite` would add the demoted tests, so its `E` would be
computed over a bigger suite than the free arm's and the comparison would
silently be between two different questions. The workflow's measured steps
therefore do NOT set that flag, and this script records the test COUNT of each
arm so a reader can check that they match. (The free arm reports none: the
jobs API gives step durations, not test counts. What the check compares is the
measured arms against each other.)

REPORTING, NEVER GATING (memories/perf-measurement-lane.md, the standing rule
for every timing lane here). Nothing in this file exits non-zero on a
millisecond. A FLIP — the verdict disagreeing with the tree's current setting
— is surfaced loudly in the job summary and is the rare actionable event; a
margin that narrowed is a note for whoever reads next.

    opt-level-calibrate.py read-free-arm --opt-level N --out <json>
                                         [--runs N] [--window N]
    opt-level-calibrate.py decide --free-arm <json> --history <dir>
                                  [--max-age-days N] [--drift F]
    opt-level-calibrate.py record --free-arm <json> [--arm <json> ...]
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

# SCHEMA HISTORY.
#   1  `arm_a` (opt-2, free) + `arm_b` (opt-0, measured). `verdict` is a
#      choice between those two.
#   2  adds optional `arm_c` (opt-1, measured). `verdict` becomes an argmin;
#      `margin_s`/`margin_ratio` move to winner-against-RUNNER-UP, with the
#      old orientation kept as `pair_opt0_over_opt2_ratio`.
#   3  ARMS ARE KEYED BY OPT LEVEL, NOT BY LETTER, and each says where it came
#      from. The letters were always a proxy for the level, and they stopped
#      being one the moment the tree moved off opt-2: the FREE arm is whichever
#      level the gate happens to run, so "arm A" would have had to mean opt-2
#      on Monday and opt-1 on Tuesday. `arms` is now `{"opt-N": {opt_level,
#      source, a, E, ...}}` and `tree_opt_level` records which of them was the
#      free one. `_arms_of()` reads every earlier schema into this shape, so
#      the whole history stays comparable.
SCHEMA = 3

# The three levels this lane considers. Not `range(4)`: opt-3 differs from
# opt-2 only in inlining aggressiveness and would cost a fourth measured arm
# to demonstrate a difference nobody has proposed.
LEVELS = (0, 1, 2)


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
    """`{a, E, shards, labels}` for one workflow run, or None if this run is
    not a DEFAULT-LANE CODE-TIER run — a docs-tier run has no archive job, an
    interval draw has a differently-named one, and a cancelled run has steps
    that never completed. Every one of those is a skip, not an error."""
    a = None
    labels: list[str] = []
    for job in jobs:
        if job.get("name") == ARCHIVE_JOB:
            a = _step_seconds(job, ARCHIVE_STEP)
            labels = list(job.get("labels") or [])
    if a is None:
        return None
    shards = []
    for job in jobs:
        if str(job.get("name", "")).startswith(RUN_JOB_PREFIX):
            s = _step_seconds(job, RUN_STEP)
            if s is None:
                return None  # a partial row would understate the free arm's E
            shards.append(s)
    if not shards:
        return None
    # `a`/`E`, not `a2`/`E2`: this reads the gate's own steps, and the gate's
    # opt level is whatever ci.yml currently sets. Naming the keys after a
    # level here is what would go stale the next time that setting moves.
    return {"a": a, "E": sum(shards), "shards": len(shards), "labels": labels}


def read_free_arm(opt_level: int, runs: int, window: int) -> dict:
    """The arm that costs nothing, at WHATEVER LEVEL THE GATE RUNS.

    This used to be `read_arm_a` and used to mean opt-2, because opt-2 was
    what ci.yml set. It is free for one reason only — every code-tier gate run
    already builds that archive and executes that suite — so the level it
    reports is the TREE's, not a level this lane chooses. Move the tree and
    the free arm moves with it; that is why `opt_level` is a parameter here
    and `CI_TREE_OPT_LEVEL` is its single source of truth."""
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
    return summarise_free_arm(opt_level, samples)


def summarise_free_arm(opt_level: int, samples: list[dict]) -> dict:
    if not samples:
        return {"opt_level": opt_level, "source": "gate-runs", "n": 0, "samples": []}
    return {
        "opt_level": opt_level,
        "source": "gate-runs",
        "n": len(samples),
        # MEDIAN, not mean: a hosted 2-vCPU runner has a fat tail (the same
        # sentence docs/perf-data/rebuild-latency/README.md carries), and one
        # contended run would drag a mean far enough to move the verdict.
        "a": statistics.median(s["a"] for s in samples),
        "E": statistics.median(s["E"] for s in samples),
        "samples": samples,
    }


# ------------------------------------------------------------ the cadence half


def _arms_of(sample: dict) -> dict[str, dict]:
    """Every schema's arms, in schema 3's shape: `{"opt-N": {opt_level, source,
    a, E, ...}}`.

    THE WHOLE HISTORY HAS TO STAY COMPARABLE. This lane's argument is that a
    verdict expires and that you can only tell when by diffing samples, so a
    schema bump that stranded the earlier ones would break the thing the lane
    is for. Schemas 1 and 2 named their arms by letter with the level welded
    into the key (`arm_a.a2`, `arm_b.E0`, `arm_c.a1`); this reads that back."""
    if sample.get("schema", 1) >= 3:
        return sample.get("arms", {})
    out = {}
    for key, level, src in (("arm_a", 2, "gate-runs"), ("arm_b", 0, "measured"),
                            ("arm_c", 1, "measured")):
        arm = sample.get(key)
        if not arm:
            continue
        out[f"opt-{level}"] = dict(arm, opt_level=level, source=src,
                                   a=float(arm[f"a{level}"]), E=float(arm[f"E{level}"]))
    return out


def free_arm_of(sample: dict) -> dict | None:
    """The arm that cost nothing, whichever level it was taken at."""
    for arm in _arms_of(sample).values():
        if arm.get("source") == "gate-runs":
            return arm
    return None


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


def decide(free_arm: dict, history: str, max_age_days: float, drift: float,
           now: float | None = None) -> tuple[bool, str]:
    """Do the MEASURED arms have to run tonight? Returns `(run, reason)`.

    FAILS TOWARDS *NOT* SPENDING THE MINUTES, with one exception. No arm A
    means no comparison is possible, so arm B would produce half a sample and
    a number nobody can read — skip. An empty history means the lane has never
    calibrated — run. Between those, the two triggers are the calendar and
    drift in the free half."""
    if not free_arm.get("n"):
        return False, ("no free-arm samples: no recent code-tier run carried both "
                       f"`{ARCHIVE_JOB} / {ARCHIVE_STEP}` and `{RUN_JOB_PREFIX}… / {RUN_STEP}`. "
                       "A measured arm with nothing to compare against measures "
                       "nothing, so none is run")
    prev = newest_sample(history)
    if prev is None:
        return True, "no previous calibration in the history: this is the first sample"
    now = time.time() if now is None else now
    age_days = (now - float(prev.get("measured_at_epoch_s", 0))) / 86400.0
    if age_days >= max_age_days:
        return True, f"the last calibration is {age_days:.1f} days old (cadence: {max_age_days:g} days)"
    prev_free = free_arm_of(prev) or {}
    # A TREE THAT MOVED IS A RECALIBRATION, unconditionally: the previous
    # sample's free arm was taken at a different opt level, so its E is not a
    # baseline this one can drift against. Without this the lane would compare
    # opt-2 execution with opt-1 execution, call the difference drift, and
    # either fire or hold for entirely the wrong reason.
    if prev_free.get("opt_level") != free_arm.get("opt_level"):
        return True, (f"the tree's opt level moved (opt-{prev_free.get('opt_level')} -> "
                      f"opt-{free_arm.get('opt_level')}) since the last calibration: the free "
                      "arm is a different measurement now and the history has no baseline for it")
    then = float(prev_free.get("E") or 0.0)
    if then > 0:
        moved = abs(free_arm["E"] - then) / then
        if moved > drift:
            return True, (f"the free arm's E has moved {moved * 100:.0f}% since the last "
                          f"calibration ({then:.0f} s -> {free_arm['E']:.0f} s), past the "
                          f"{drift * 100:.0f}% drift trigger")
    return False, (f"the last calibration is {age_days:.1f} days old and the free arm's E has "
                   f"held ({then:.0f} s -> {free_arm['E']:.0f} s); nothing to re-measure")


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


def derive(arms: dict[str, dict]) -> dict:
    """The verdict, as an ARGMIN of `a + E` over the arms this sample carries.

    WHY IT IS NOT A PAIR (2026-08-25). The two-arm version could only answer
    "opt-2 or opt-0", and nobody had ever checked that this was the right
    question: `a + E` is minimised over a knob with more than two settings,
    and the two arms sat at opposite extremes of BOTH terms. opt-1 is the
    interior point — rustc's optimiser on, its most expensive passes off — and
    until 2026-08-25 it had never appeared in any measurement, note or
    decision in this repository.

    WHY IT TAKES A LEVEL MAP RATHER THAN THREE NAMED ARMS (schema 3). Which
    arm is free is a fact about ci.yml, not about this lane: the gate's own
    runs are free to read, so the tree's level is the free one and the others
    are bought. Naming the parameters `arm_a`/`arm_b`/`arm_c` welded a level
    to each letter and would have made the free arm's identity a lie the day
    the tree moved — which is exactly the day this signature changed.

    Missing arms are simply absent from the ranking; the derived figures that
    need a level this sample lacks are omitted rather than faked."""
    def total(level: int) -> float | None:
        arm = arms.get(f"opt-{level}")
        return None if arm is None else float(arm["a"]) + float(arm["E"])

    def part(level: int, key: str) -> float | None:
        arm = arms.get(f"opt-{level}")
        return None if arm is None else float(arm[key])

    t0, t1, t2 = total(0), total(1), total(2)
    a0, a1, a2 = part(0, "a"), part(1, "a"), part(2, "a")
    e0, e1, e2 = part(0, "E"), part(1, "E"), part(2, "E")

    out: dict = {
        "total_opt0_s": t0,
        "total_opt1_s": t1,
        "total_opt2_s": t2,
        # The figure ci.yml's OPT LEVEL note quotes for this lane. Written
        # here so a diff of two samples shows the drift against it directly.
        "r_quoted_by_ci_yml_default_lane": 6.46,
    }
    # THE INPUTS, kept beside the verdict so the next reader can tell when the
    # conclusion expired rather than inheriting a bare opt level. Each is
    # written only when the sample actually carries the arms it is about.
    if e0 is not None and e2:
        out["r_execution_ratio"] = e0 / e2
    if e0 is not None and e1:
        out["r_execution_ratio_opt1"] = e0 / e1
    if a0 is not None and a2 is not None:
        out["archive_delta_s"] = a2 - a0
    if a0 is not None and a1 is not None:
        out["archive_delta_opt1_s"] = a1 - a0
    if t0 and t2:
        out["pair_opt0_over_opt2_ratio"] = t0 / t2
    for level, a, t in ((0, a0, t0), (1, a1, t1), (2, a2, t2)):
        if a is not None and t:
            out[f"L_over_T_opt{level}"] = a / t
    # THE TWO NUMBERS THE opt-1 ARM EXISTS TO PRODUCE. opt-1 is worth having
    # exactly when it keeps opt-2's execution while refusing opt-2's build
    # cost, so both are recorded as fractions rather than left for a reader to
    # divide out of the table:
    #   ~1.0 execution_kept     = opt-1 runs as fast as opt-2
    #   <1.0 build_penalty_kept = opt-1 pays less than opt-2 to get there
    if e1 and e2 is not None:
        out["execution_kept_vs_opt2"] = e2 / e1
    if None not in (a0, a1, a2) and (a2 - a0):
        out["build_penalty_kept_vs_opt2"] = (a1 - a0) / (a2 - a0)

    # INSERTION ORDER IS THE TIE-BREAK, and it is deliberate: `sorted` is
    # stable, so an exact dead heat resolves to whichever level is listed
    # first — opt-0, then opt-2, then opt-1. That reproduces the original
    # two-arm rule (`opt-2 if total2 < total0 else opt-0`) exactly.
    totals = {f"opt-{lv}": t for lv, t in ((0, t0), (2, t2), (1, t1)) if t is not None}
    ranked = sorted(totals.items(), key=lambda kv: kv[1])
    out["totals_s"] = totals
    out["verdict"] = ranked[0][0]
    # The margin as the reader wants it BOTH ways: seconds say how much slack
    # there is, the ratio says how far the inputs would have to move. Against
    # the RUNNER-UP since schema 2 — with two arms that is the same pair of
    # numbers the lane always reported, and with three it is the one that
    # decides anything.
    if len(ranked) > 1:
        out["runner_up"] = ranked[1][0]
        out["margin_s"] = ranked[1][1] - ranked[0][1]
        out["margin_ratio"] = (ranked[1][1] / ranked[0][1]) if ranked[0][1] else None
    return out


def record(arms: dict[str, dict], history: str, sha: str) -> tuple[dict, str, str]:
    # A LONE ARM COMPARES NOTHING. `main` already declines to get here, but the
    # invariant belongs next to the code that depends on it: `derive` leaves
    # `margin_s`/`runner_up` unset with fewer than two arms and `summary`
    # reads both, so a one-arm sample would be a KeyError at the point of
    # writing rather than a refusal at the point of asking.
    if len(arms) < 2:
        raise ValueError(f"record needs at least two arms to compare; got {sorted(arms)}")
    now = int(time.time())
    free = next((a for a in arms.values() if a.get("source") == "gate-runs"), None)
    sample = {
        "schema": SCHEMA,
        "commit": sha,
        "measured_at_epoch_s": now,
        "measured_at_utc": datetime.fromtimestamp(now, tz=timezone.utc).isoformat(),
        # WHICH LEVEL WAS FREE, recorded rather than inferred. It is the one
        # fact about a sample that a reader cannot reconstruct from the
        # numbers, and the one that decides which arm is real gate data.
        "tree_opt_level": free.get("opt_level") if free else None,
        "arms": arms,
        "derived": derive(arms),
        "environment": environment(),
        "method": (
            "the arm at the TREE's own opt level is READ from the step durations of recent "
            "code-tier gate runs via the jobs API — real gate data, never re-run; the other "
            "levels are MEASURED here, each on its own clean target directory, in the GATE "
            "test population (NO --cfg nightly_suite). Verdict is the direct argmin of a+E "
            "over the arms present; REPORTING ONLY, never a gate."
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
    d = sample["derived"]
    arms = _arms_of(sample)
    tree = os.environ.get("CI_TREE_OPT_LEVEL", str(sample.get("tree_opt_level", "")))
    flip = d["verdict"] != f"opt-{tree}"
    rows = []
    for level in LEVELS:
        arm = arms.get(f"opt-{level}")
        if arm is None:
            continue
        if arm.get("source") == "gate-runs":
            n = arm.get("n", 0)
            where = f"read from {n} gate run{'' if n == 1 else 's'}"
        else:
            where = "measured here"
        win = d["verdict"] == f"opt-{level}"
        label = f"**opt-{level}** ({where})" if win else f"opt-{level} ({where})"
        total = f"**{d[f'total_opt{level}_s']:.0f} s**" if win else f"{d[f'total_opt{level}_s']:.0f} s"
        rows.append(f"| {label} | {arm['a']:.0f} s | {arm['E']:.0f} s | {total} |")
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
    ]
    if "r_execution_ratio" in d:
        out.append(f"* `r` (opt-0/opt-2 execution ratio) = **{d['r_execution_ratio']:.2f}** — "
                   f"ci.yml's OPT LEVEL note quotes {d['r_quoted_by_ci_yml_default_lane']} "
                   "for this lane.")
    if "archive_delta_s" in d:
        out.append(f"* `a2 - a0` (what opt-2 costs to build) = "
                   f"**{d['archive_delta_s']:.0f} s**.")
    shares = ", ".join(f"{d[f'L_over_T_opt{lv}'] * 100:.0f}% at opt-{lv}"
                       for lv in LEVELS if f"L_over_T_opt{lv}" in d)
    if shares:
        out.append(f"* build share of the total: {shares}.")
    if "execution_kept_vs_opt2" in d and "build_penalty_kept_vs_opt2" in d:
        out.append(
            f"* **the opt-1 arm's two numbers**: opt-1 keeps "
            f"**{d['execution_kept_vs_opt2'] * 100:.0f}%** of opt-2's execution speed for "
            f"**{d['build_penalty_kept_vs_opt2'] * 100:.0f}%** of opt-2's build penalty "
            f"(`a1 - a0` = {d['archive_delta_opt1_s']:.0f} s against "
            f"`a2 - a0` = {d['archive_delta_s']:.0f} s).")
    counts = " / ".join(f"{arms[f'opt-{lv}'].get('tests', 'n/a')} (opt-{lv})"
                        for lv in LEVELS if f"opt-{lv}" in arms)
    out += [
        f"* tests executed: {counts} — these must match, or the arms measured different "
        "suites. The free arm reports `n/a`: the jobs API gives step durations, not test "
        "counts.",
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
    assert abs(got["a"] - 432.0) < 0.5, got
    assert abs(got["E"] - 119.0) < 0.5, got

    # A DOCS-TIER RUN, and the shape that matters: no archive job at all. It
    # must be a skip, not a zero — a zero would enter the median and drag the
    # figure the verdict rests on towards nothing.
    assert sample_run([j for j in _FAKE_JOBS["jobs"] if j["name"] != ARCHIVE_JOB]) is None
    # A CANCELLED SHARD. Half a row understates the free arm's E, which biases
    # the verdict towards the tree's own level — the direction nobody would
    # question. So it is refused.
    partial = json.loads(json.dumps(_FAKE_JOBS["jobs"]))
    partial[1]["steps"][0]["conclusion"] = "cancelled"
    assert sample_run(partial) is None
    # A RENAMED STEP goes quiet, never wrong.
    renamed = json.loads(json.dumps(_FAKE_JOBS["jobs"]))
    renamed[0]["steps"][1]["name"] = "build test binaries + archive (v2)"
    assert sample_run(renamed) is None

    free2 = summarise_free_arm(2, [dict(got, run_id=i) for i in range(3)])
    assert free2["n"] == 3 and abs(free2["E"] - 119.0) < 0.5
    assert free2["opt_level"] == 2 and free2["source"] == "gate-runs"

    with tempfile.TemporaryDirectory() as t:
        run, why = decide(free2, t, MAX_AGE_DAYS, DRIFT)
        assert run and "first sample" in why, why
        assert decide({"n": 0, "samples": []}, t, MAX_AGE_DAYS, DRIFT)[0] is False

        # THE ORIGINAL TWO-ARM ARITHMETIC, and it must still come out the same
        # way: opt-0 is 4x slower to execute and 3x cheaper to build, which on
        # these numbers makes opt-2 win.
        m0 = {"opt_level": 0, "source": "measured", "a": 127.0, "E": 476.0, "tests": 2791}
        arms = {"opt-2": free2, "opt-0": m0}
        sample, dest, text = record(arms, t, "0123456789abcdef")
        d = sample["derived"]
        assert d["verdict"] == "opt-2" and d["runner_up"] == "opt-0", d
        assert abs(d["r_execution_ratio"] - 4.0) < 0.01
        assert abs(d["margin_s"] - abs(603.0 - 551.0)) < 0.01
        assert abs(d["pair_opt0_over_opt2_ratio"] - 603.0 / 551.0) < 0.01
        assert sample["tree_opt_level"] == 2
        assert os.path.basename(dest).endswith("-0123456.json")
        assert "opt-2 wins" in text and "| opt-0 (measured here) |" in text
        # NO opt-1 KEYS INVENTED when no opt-1 arm was taken.
        assert "total_opt1_s" in d and d["total_opt1_s"] is None
        assert "execution_kept_vs_opt2" not in d and "opt-1" not in d["totals_s"]
        # AND A LONE ARM IS REFUSED AT THE POINT OF ASKING.
        try:
            record({"opt-2": free2}, t, "0" * 40)
        except ValueError as exc:
            assert "at least two arms" in str(exc), exc
        else:
            raise AssertionError("a one-arm sample must be refused, not written")
        # A DEAD HEAT READS opt-0, exactly as the two-arm rule it replaces did.
        tied = derive({"opt-2": free2, "opt-0": dict(m0, a=0.0, E=551.0)})
        assert tied["verdict"] == "opt-0", tied

        # ------------------------------------------------- THE THIRD LEVEL.
        # The shape the opt-1 arm exists to catch: opt-1 within a hair of
        # opt-2's execution for a fraction of its build. Both two-arm answers
        # are wrong here — opt-2 beats opt-0, and opt-1 beats them both.
        m1 = {"opt_level": 1, "source": "measured", "a": 260.0, "E": 124.0, "tests": 2791}
        three, _, text3 = record({**arms, "opt-1": m1}, t, "c" * 40)
        d3 = three["derived"]
        assert d3["verdict"] == "opt-1" and d3["runner_up"] == "opt-2", d3
        assert abs(d3["total_opt1_s"] - 384.0) < 0.01
        assert abs(d3["margin_s"] - (551.0 - 384.0)) < 0.01
        assert abs(d3["execution_kept_vs_opt2"] - 119.0 / 124.0) < 0.01
        assert abs(d3["build_penalty_kept_vs_opt2"] - 133.0 / 305.0) < 0.01
        assert "**opt-1** (measured here)" in text3 and "THIS IS A FLIP" in text3
        # AND IT MUST NOT WIN BY DEFAULT.
        slow = record({**arms, "opt-1": dict(m1, a=400.0, E=400.0)}, t, "d" * 40)[0]
        assert slow["derived"]["verdict"] == "opt-2", slow["derived"]

        # ------------------------------ THE TREE MOVING OFF opt-2 (schema 3).
        # The free arm is now opt-1 and the two MEASURED arms are 0 and 2. The
        # verdict arithmetic is untouched; what must not happen is the free
        # arm being read as opt-2 because it used to be.
        free1 = summarise_free_arm(1, [{"a": 260.0, "E": 124.0, "shards": 2}])
        # THE CADENCE MUST NOTICE FIRST, while the newest sample in the
        # history is still an opt-2-free one: drifting an opt-1 E against an
        # opt-2 baseline would compare two different measurements, so a moved
        # tree recalibrates unconditionally.
        run, why = decide(free1, t, MAX_AGE_DAYS, DRIFT)
        assert run and "tree's opt level moved" in why, why
        moved, _, textm = record({"opt-1": free1, "opt-0": m0,
                                  "opt-2": {"opt_level": 2, "source": "measured",
                                            "a": 432.0, "E": 119.0, "tests": 2791}},
                                 t, "e" * 40)
        assert moved["tree_opt_level"] == 1, moved["tree_opt_level"]
        assert moved["derived"]["verdict"] == "opt-1", moved["derived"]
        assert "**opt-1** (read from 1 gate run)" in textm, textm
        assert "opt-2 (measured here)" in textm, textm
        # ...and once the history's newest sample IS opt-1-free, it holds.
        run, why = decide(free1, t, MAX_AGE_DAYS, DRIFT)
        assert not run and "held" in why, why

        # ------------------------------------------- READING THE OLD SCHEMAS.
        # Schemas 1 and 2 welded the level into each arm's key names. The
        # history has to stay comparable or the lane loses the thing it is for.
        legacy = {"schema": 1, "arm_a": {"n": 5, "a2": 683.0, "E2": 94.0},
                  "arm_b": {"a0": 211.0, "E0": 791.0, "tests": "unknown"}}
        la = _arms_of(legacy)
        assert set(la) == {"opt-2", "opt-0"}, la
        assert la["opt-2"]["a"] == 683.0 and la["opt-0"]["E"] == 791.0
        assert free_arm_of(legacy)["opt_level"] == 2
        assert derive(la)["verdict"] == "opt-2"
        legacy2 = {"schema": 2, **legacy, "arm_c": {"a1": 300.0, "E1": 100.0}}
        assert set(_arms_of(legacy2)) == {"opt-2", "opt-0", "opt-1"}
        assert derive(_arms_of(legacy2))["verdict"] == "opt-1"

        # THE CADENCE, on a history whose newest sample is schema 3.
        run, why = decide(free2, t, MAX_AGE_DAYS, DRIFT, now=time.time() + 8 * 86400)
        assert run and "days old" in why, why

    env = environment()
    for key in ("runner", "nproc", "rustflags", "cargo_profile", "debug_assertions",
                "tolerance_eps", "nightly_suite_cfg"):
        assert key in env, key

    print("opt-level-calibrate selftest OK: a run's free-arm sample is read from the archive "
          "step and the summed shards; a docs-tier run, a cancelled shard and a renamed step "
          "are SKIPS rather than zeroes; the verdict is the direct a+E argmin over the arms "
          "present, which for two arms is the two-arm answer it replaced (ties included); the "
          "optional opt-1 arm wins only when it earns it and invents no keys when it is "
          "absent; the FREE arm follows the tree's opt level rather than a fixed letter, and a "
          "tree that moved recalibrates unconditionally instead of drifting one level's E "
          "against another's; schema-1 and schema-2 samples still read and still derive; the "
          "cadence fires on a first sample, on the calendar and on >20% drift, and holds "
          "otherwise; no free arm means no measured arm is spent; and a flip is reported "
          "loudly without failing anything")


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("mode", nargs="?", choices=("read-free-arm", "decide", "record"))
    ap.add_argument("--selftest", action="store_true")
    ap.add_argument("--out")
    ap.add_argument("--free-arm", help="the arm read from gate runs, at the TREE's opt level")
    ap.add_argument("--arm", action="append", default=[],
                    help="an arm's JSON, repeatable. Each file is self-describing "
                         "(`opt_level` + `source`); a path that does not exist is skipped, "
                         "which is how a measured arm that failed to build costs its own row "
                         "and nothing else")
    ap.add_argument("--opt-level", type=int,
                    default=int(os.environ.get("CI_TREE_OPT_LEVEL", "2")),
                    help="the level the GATE runs, i.e. the level the free arm reports")
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

    if args.mode == "read-free-arm":
        free = read_free_arm(args.opt_level, args.runs, args.window)
        with open(args.out, "w", encoding="utf-8") as fh:
            json.dump(free, fh, indent=1, sort_keys=True)
        print(f"free arm (opt-{args.opt_level}): {free['n']} sample(s)"
              + (f", a = {free['a']:.0f} s, E = {free['E']:.0f} s" if free["n"] else ""))
        return 0

    with open(args.free_arm, encoding="utf-8") as fh:
        free = json.load(fh)

    if args.mode == "decide":
        run, why = decide(free, args.history, args.max_age_days, args.drift)
        print(f"measured arms: {'RUN' if run else 'skip'} — {why}")
        out = os.environ.get("GITHUB_OUTPUT")
        if out:
            with open(out, "a", encoding="utf-8") as fh:
                fh.write(f"run_measured={'true' if run else 'false'}\n")
                fh.write(f"reason={why}\n")
        return 0

    # THE FREE ARM IS ALWAYS PRESENT; the measured ones are each optional, and
    # a missing file is a skipped arm rather than an error. Losing one costs
    # its row, which is why the workflow can let a failed build return
    # `measured=false` instead of failing the job.
    arms = {f"opt-{free['opt_level']}": free}
    for path in args.arm:
        if not os.path.exists(path):
            print(f"no arm at {path}: skipped")
            continue
        with open(path, encoding="utf-8") as fh:
            arm = json.load(fh)
        arms[f"opt-{arm['opt_level']}"] = arm
    if len(arms) < 2:
        print("only the free arm survived; a lone arm compares nothing, so no sample is written")
        return 0
    _, dest, text = record(arms, args.history, args.sha)
    print(f"wrote {dest}")
    print(text)
    if args.summary:
        with open(args.summary, "a", encoding="utf-8") as fh:
            fh.write(text + "\n")
    return 0


if __name__ == "__main__":
    sys.exit(main())
