#!/usr/bin/env python3
"""The slowest tests of a run, summed per test across the legs of ONE job.

WHY THIS EXISTS. Nothing in a run said what the test suite cost, so cost
accumulated where nobody was looking: at the 2026-08-13 audit, 55% of all
workspace test time sat inside modules named after one past review, and the
only reason anyone knows that is that the audit was asked for. This prints
the head of the distribution into the run's own step summary, on every run,
out of output the run already produced.

IT IS A REPORT, NOT A GATE, and that is a decision rather than a stage: a
budget needs a baseline and the baseline is moving fast (the suite went from
10,482 to ~1,243 cpu-s in a day), so a threshold set against today's number
would be wrong next week. Nothing in this file compares a number against a
limit, and the step that runs it cannot fail its job — see the wiring in
`.github/workflows/ci.yml` for how a failure here presents.

THE TWO MEASUREMENT TRAPS THE AUDIT PAID FOR, honoured HERE rather than in
prose somewhere:

  * PER-LEG TIMES ARE NOT COMPARABLE ACROSS LEGS. Fitting a multiplicative
    leg-speed factor across one real run showed one leg running ~1.5x faster
    than its siblings, which manufactures apparent ε-sensitivity out of
    nothing. So a test's number here is the SUM of its executions over the
    legs of one job, the legs that were summed are named in the output, and
    no number is ever placed beside another job's.
  * COST CONCENTRATES SAVAGELY, SO REPORT THE HEAD AND NOT THE MEAN. At
    audit time 20 tests were 55% of all test time while 2,603 tests together
    were 1.7%. A total or an average is not actionable; the top-N list is
    the whole signal, so this prints the top N and the share it accounts
    for, and offers no per-test average anywhere.

WHAT A LEG IS: one `cargo nextest run` invocation, whose output this script
is handed as a file. The `test` job runs one; `test-interval`'s shard 1 runs
three (the archive run plus the two named editor-core rows), and a test that
appears in more than one of them costs the job the sum, which is what the
table shows and what the `legs` column counts.

WHAT IT PARSES, and the shapes are captured from real runs rather than
imagined — every fixture in `selftest` below is verbatim log text, cited to
the run it came from:

    PASS [   0.005s] (2236/2328) viewer::all input_mapping::a_stream_folds
    FAIL [   0.004s] (2005/2328) topo::all interval_body::planted_failure_1
    TRY 1 FAIL [   0.106s] (───) nextest-shapes always_fails
    Summary [  44.390s] 2328 tests run: 2326 passed, 2 failed, 2341 skipped

Downloaded job logs carry an RFC3339 timestamp on every line and the runner's
own file does not; both are read, because the fixtures are downloaded logs
and the live input is a tee.

THE RECAP IS NOT A SECOND EXECUTION. nextest reprints each failure's status
line AFTER the `Summary` line. A parser that counts every matching line
charges a failing test twice — and only on red runs, which is the worst
direction for a report whose other half is "what did this PR add". So rows
count only while the parse is inside a run BODY: `Summary` closes the body
and a `Starting N tests` line opens the next one. The red fixture below
contains exactly that shape and the selftest asserts the count.

cpu-s HERE MEANS the sum of nextest's per-test durations. Each test runs in
its own process, so that is execution time charged to tests, not the job's
wall clock (which the `Summary` line reports separately and which this
script prints beside each leg, unsummed, for exactly the reason in the first
trap above).

Usage:
    slowest-tests.py [--top N] [--job LABEL] --leg 'LABEL=PATH' [--leg ...]
    slowest-tests.py --selftest

Writes GitHub-flavoured Markdown to stdout. Exit status is 0 for every input
it can read: absence is reported IN the report (a leg whose file is missing
is named as missing), because the caller appends this to a step summary and
a nonzero exit there would be a report interfering with a test verdict.
"""

from __future__ import annotations

import argparse
import os
import re
import sys

# A downloaded Actions log prefixes every line with the runner's clock; the
# file the runner tees has no prefix. Stripped, not matched around, so every
# pattern below reads one shape.
TIMESTAMP_RE = re.compile(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d+Z ")
ANSI_RE = re.compile(r"\x1b\[[0-9;]*[A-Za-z]")

# One test's terminal status line. The index field is `(i/n)` for a result
# that counts, and `(───)` for a superseded retry attempt (real shape,
# captured from the pinned 0.9.140 with `--retries 1`); BOTH are accepted,
# because a retried test really did cost the job both attempts.
#
# THE INDEX IS RIGHT-ALIGNED, AND THAT IS WHY `\s*` IS IN THERE. nextest pads
# the counter to the width of the total — `(   1/2470)` — so a pattern written
# against a four-digit index matches only the last tenth of a run. That is not
# hypothetical: it is what this regex did on its first hosted run (33342621213),
# where it read 1,473 of 2,470 rows and reported a plausible table built from
# the tail of the suite. Nothing about the output looked wrong, which is the
# argument for the padded rows being in the fixtures below.
#
# `SLOW [>  60.000s]` rows are excluded by the `>` guard. That guard is
# INSURANCE, NOT A LIVE PATH: measured against the pinned 0.9.140, slow
# notifications are not emitted at the default status level, which is what
# every row in this repo runs at. If a row ever raises the status level, a
# `>` line is a test still running and must not be read as a duration.
ROW_RE = re.compile(
    r"^\s*(?P<status>[A-Z][A-Z0-9]*(?: [A-Z0-9]+)*)"
    r" \[\s*(?P<pending>>)?\s*(?P<secs>\d+(?:\.\d+)?)s\]"
    r" \(\s*(?:\d+/\d+|─+)\s*\)"
    r" (?P<binary>\S+) (?P<test>\S+)\s*$"
)
SUMMARY_RE = re.compile(r"^\s*Summary \[\s*(?P<secs>\d+(?:\.\d+)?)s\]\s+(?P<counts>.+?)\s*$")
STARTING_RE = re.compile(r"^\s*Starting \d+ tests? across ")


def parse_leg(text):
    """`(times, executions, summaries)` from one leg's captured output.

    `times` maps `(binary_id, test_name)` to the seconds that leg charged
    it, summed over its executions in that leg (a retry is a second
    execution and costs a second time). `summaries` is every `Summary`
    line's `(wall_seconds, counts)`, in order — one per nextest invocation
    in the file.
    """
    times: dict[tuple[str, str], float] = {}
    executions = 0
    summaries: list[tuple[float, str]] = []
    in_body = True
    for raw in text.splitlines():
        line = TIMESTAMP_RE.sub("", ANSI_RE.sub("", raw))
        if STARTING_RE.match(line):
            in_body = True
            continue
        summary = SUMMARY_RE.match(line)
        if summary:
            summaries.append((float(summary.group("secs")), summary.group("counts")))
            in_body = False
            continue
        if not in_body:
            continue
        row = ROW_RE.match(line)
        if row is None or row.group("pending"):
            continue
        key = (row.group("binary"), row.group("test"))
        times[key] = times.get(key, 0.0) + float(row.group("secs"))
        executions += 1
    return times, executions, summaries


def read_leg(label, path):
    """One leg, read from disk. A missing or unreadable file is a leg that
    is REPORTED as absent, never one that is silently dropped: a job whose
    run step died before nextest started has a real story to tell, and an
    empty table plus no explanation is not it."""
    leg = {"label": label, "path": path, "missing": None,
           "times": {}, "executions": 0, "summaries": []}
    try:
        with open(path, encoding="utf-8", errors="replace") as fh:
            text = fh.read()
    except OSError as exc:
        # The errno text and the file's BASENAME, never the full path: this
        # string is pasted into a public step summary, and the runner's
        # absolute temp path is noise there — the label above already says
        # which leg it was.
        leg["missing"] = "{}: {}".format(os.path.basename(path), exc.strerror or exc)
        return leg
    leg["times"], leg["executions"], leg["summaries"] = parse_leg(text)
    return leg


def fmt_secs(secs):
    """Three decimals below a second, one above. A cheap job is not a free
    one: `0.0 cpu-s in total` beside a table of measured rows reads as though
    the run cost nothing, which is the one thing a cost report may not say.
    Both reports print totals through here so the two cannot drift."""
    return "{:.1f}".format(secs) if secs >= 1.0 else "{:.3f}".format(secs)


def merge(legs):
    """`(binary, test) -> (seconds, legs_that_ran_it)`, summed across the
    legs of ONE job. Never across jobs — see the first trap in the header."""
    total: dict[tuple[str, str], list] = {}
    for leg in legs:
        for key, secs in leg["times"].items():
            row = total.setdefault(key, [0.0, 0])
            row[0] += secs
            row[1] += 1
    return total


def leg_line(leg):
    if leg["missing"] is not None:
        return "- `{}` — **no output captured** ({}). The leg either did not run or died before nextest did.".format(
            leg["label"], leg["missing"])
    if not leg["summaries"]:
        return "- `{}` — {} test executions, no `Summary` line (the leg was cut short).".format(
            leg["label"], leg["executions"])
    parts = "; ".join("{} in {:.1f} s wall".format(counts, wall) for wall, counts in leg["summaries"])
    return "- `{}` — {}".format(leg["label"], parts)


def render(legs, top, job):
    out = []
    out.append("### Slowest {} tests — {}".format(top, job))
    out.append("")
    out.append("Legs summed in this job:")
    for leg in legs:
        out.append(leg_line(leg))
    out.append("")
    total = merge(legs)
    if not total:
        out.append("No test rows were found in this job's captured output, so there is nothing "
                   "to rank. This is a report and gates nothing: the job's verdict is whatever "
                   "its test steps said.")
        out.append("")
        return "\n".join(out)
    grand = sum(secs for secs, _ in total.values())
    ranked = sorted(total.items(), key=lambda kv: (-kv[1][0], kv[0]))[:top]
    head = sum(secs for _, (secs, _) in ranked)
    out.append("cpu-s is the sum of nextest's per-test durations (one process per test), summed "
               "**per test across the legs listed above and no others**: per-leg times are not "
               "comparable across legs or jobs, so nothing here may be read against another "
               "job's summary. Only the head of the distribution is shown, because that is where "
               "the cost is — at the 2026-08-13 audit 20 tests were 55% of all test time while "
               "2,603 tests together were 1.7%.")
    out.append("")
    out.append("| # | cpu-s | % of job | legs | test |")
    out.append("|--:|------:|---------:|-----:|------|")
    for i, ((binary, test), (secs, legs_ran)) in enumerate(ranked, 1):
        out.append("| {} | {:.3f} | {:.1f}% | {} | `{} {}` |".format(
            i, secs, 100.0 * secs / grand, legs_ran, binary, test))
    out.append("")
    out.append("This job's legs executed {} distinct tests for {} cpu-s in total; the {} above "
               "are {:.1f}% of it.".format(len(total), fmt_secs(grand), len(ranked),
                                           100.0 * head / grand))
    out.append("")
    out.append("_Deliberately not a gate (issue 469): a budget needs a baseline and the baseline is "
               "moving, so this reports and does not threshold. Adding teeth later is a decision "
               "someone makes, not a drift._")
    out.append("")
    return "\n".join(out)


# ---------------------------------------------------------------- selftest

# VERBATIM, from `test (interval, eps = 1e-6, 1/2)` of run 33273307209 (job
# 99157246218) — the planted-failure run. Non-contiguous: the run's head, one
# passing row, the two planted failures with the captured output nextest
# interleaves, and the tail. Nothing is edited; `[...]` marks each cut.
RED_FIXTURE = """\
2026-08-29T20:36:58.2474846Z fail-fast mode: --no-fail-fast — every test in this shard runs to completion.
2026-08-29T20:36:59.0815843Z   Extracting 33 binaries (including 1 non-test binary) to /tmp/nextest-archive-1B6qm8
2026-08-29T20:36:59.4267232Z    Extracted 37 files to /tmp/nextest-archive-1B6qm8 in 0.35s
2026-08-29T20:36:59.5567421Z  Nextest run ID c529c2c2-66c7-4ac4-8e49-622c51632128 with nextest profile: default
2026-08-29T20:36:59.5568233Z     Starting 2328 tests across 32 binaries (2341 tests skipped)
2026-08-29T20:37:42.0205880Z         PASS [   0.005s] (2004/2328) topo::all interval_body::interval_cube_upgrades_to_intersections
2026-08-29T20:37:42.0249353Z         FAIL [   0.004s] (2005/2328) topo::all interval_body::qa2_planted_failure_1
2026-08-29T20:37:42.0254028Z   stdout ───
2026-08-29T20:37:42.0255194Z     running 1 test
2026-08-29T20:37:42.0255688Z     test interval_body::qa2_planted_failure_1 ... FAILED
2026-08-29T20:37:42.0258816Z     test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 435 filtered out; finished in 0.00s
2026-08-29T20:37:42.0260808Z     thread 'interval_body::qa2_planted_failure_1' (6074) panicked at crates/topo/tests/interval_body.rs:137:5:
2026-08-29T20:37:42.0263250Z         FAIL [   0.005s] (2006/2328) topo::all interval_body::qa2_planted_failure_3
2026-08-29T20:37:43.9471413Z         PASS [   0.004s] (2328/2328) viewer::all valid_range::the_search_terminates_within_its_advertised_cost
2026-08-29T20:37:43.9473148Z      Summary [  44.390s] 2328 tests run: 2326 passed, 2 failed, 2341 skipped
2026-08-29T20:37:43.9473830Z         FAIL [   0.004s] (2005/2328) topo::all interval_body::qa2_planted_failure_1
2026-08-29T20:37:43.9474589Z         FAIL [   0.005s] (2006/2328) topo::all interval_body::qa2_planted_failure_3
2026-08-29T20:37:43.9723850Z error: test run failed
2026-08-29T20:37:43.9731587Z fail-fast mode was --no-fail-fast: the summary above is this shard's whole failure SURFACE, not its first failure.
"""

# VERBATIM, from `test (interval, eps = 1e-12, 1/2)` of run 33335223850 (job
# 99322535103) — an all-green run, and the SECOND leg of that job (the `band 4
# corpus (interval)` step), which is what makes it the multi-leg case.
GREEN_FIXTURE = """\
2026-08-30T21:16:02.9499449Z   Extracting 33 binaries (including 1 non-test binary) to /tmp/nextest-archive-uERjTx
2026-08-30T21:16:03.4418402Z    Extracted 37 files to /tmp/nextest-archive-uERjTx in 0.50s
2026-08-30T21:16:03.4521211Z  Nextest run ID bafbf7c6-1a30-49f6-b645-75c2de8d1fa6 with nextest profile: default
2026-08-30T21:16:03.4523576Z     Starting 1 test across 1 binary (856 tests and 31 binaries skipped)
2026-08-30T21:16:04.2724382Z         PASS [   0.818s] (1/1) editor-core::all m4_pr8_corpus_interval::every_document_evaluates_green_at_interval
2026-08-30T21:16:04.2726404Z      Summary [   0.819s] 1 test run: 1 passed, 856 skipped
"""

# VERBATIM, from `test (interval, eps = default, 1/2)` of run 33342621213 (job
# 99342426979) — this report's OWN first hosted run, and the run that caught the
# padded index. A four-digit total pads every index below 1000, so the first
# 999 rows of every large run carry leading spaces and the rest do not; both
# are here, from the same leg, three lines apart in the real log.
PADDED_INDEX_FIXTURE = """\
2026-08-30T23:58:45.0370404Z     Starting 2470 tests across 32 binaries (2483 tests skipped)
2026-08-30T23:58:45.0460316Z         PASS [   0.008s] (   1/2470) bvh::all aggregator_headers::no_aggregator_header_restates_the_build_cost_measurement
2026-08-30T23:59:37.6280986Z         PASS [  11.635s] ( 999/2470) geom-brep::all review_r1_rational_probes::probe_sphere_octant
2026-08-30T23:59:37.6940477Z         PASS [   0.065s] (1000/2470) mesh chords::tests::r1_rational_mult_p_minus_one_carrier
2026-08-31T00:00:46.0293449Z      Summary [ 120.992s] 2470 tests run: 2470 passed, 2483 skipped
"""

# VERBATIM, captured locally against the PINNED cargo-nextest 0.9.140 with
# `--retries 1` over a three-test crate. Hosted CI configures no retries, so
# this shape does not appear in any run above — which is exactly why it is
# here: the `(───)` index of a superseded attempt is not a shape a reader can
# guess, and a parser that rejects it silently under-reports a retried test.
RETRY_FIXTURE = """\
────────────
 Nextest run ID deddd61c-3172-4fbd-b515-d242b40cbc29 with nextest profile: default
    Starting 3 tests across 1 binary
        PASS [   0.006s] (1/3) nextest-shapes quick_pass
  TRY 1 FAIL [   0.106s] (───) nextest-shapes always_fails
  TRY 2 FAIL [   0.109s] (2/3) nextest-shapes always_fails
        PASS [   1.507s] (3/3) nextest-shapes slow_pass
────────────
     Summary [   1.507s] 3 tests run: 2 passed, 1 failed, 0 skipped
  TRY 2 FAIL [   0.109s] (2/3) nextest-shapes always_fails
"""


def selftest():
    failures: list[str] = []

    def fail(message):
        failures.append(message)

    red_times, red_execs, red_summaries = parse_leg(RED_FIXTURE)
    planted_1 = ("topo::all", "interval_body::qa2_planted_failure_1")
    if red_times.get(planted_1) != 0.004:
        fail("red fixture: planted failure 1 should be charged 0.004s once, got "
                        "{!r} — the post-`Summary` recap is being counted as a second "
                        "execution".format(red_times.get(planted_1)))
    if red_execs != 4:
        fail("red fixture: 4 rows are inside the run body (2 PASS, 2 FAIL); parsed "
                        "{}".format(red_execs))
    if len(red_times) != 4:
        fail("red fixture: 4 distinct tests, parsed {}".format(len(red_times)))
    if red_summaries != [(44.390, "2328 tests run: 2326 passed, 2 failed, 2341 skipped")]:
        fail("red fixture: the Summary line did not parse: {!r}".format(red_summaries))
    # The captured stdout of a failing test is prose, and prose must not
    # become a row. `test interval_body::… ... FAILED` is in the fixture for
    # this assertion alone.
    for binary, _ in red_times:
        if binary in ("test", "running", "thread"):
            fail("red fixture: a captured-output line parsed as a test row "
                            "({!r})".format(binary))

    green_times, green_execs, green_summaries = parse_leg(GREEN_FIXTURE)
    corpus = ("editor-core::all", "m4_pr8_corpus_interval::every_document_evaluates_green_at_interval")
    if green_times.get(corpus) != 0.818 or green_execs != 1:
        fail("green fixture: expected one row at 0.818s, got {!r} / {} "
                        "executions".format(green_times, green_execs))
    if green_summaries != [(0.819, "1 test run: 1 passed, 856 skipped")]:
        fail("green fixture: green Summary line did not parse: {!r}".format(green_summaries))

    # THE PADDED INDEX, which is what a table built from a tenth of a run looks
    # like from the outside: nothing. All three rows must parse, and the row
    # numbered `(   1/2470)` is the one that did not.
    padded_times, padded_execs, padded_summaries = parse_leg(PADDED_INDEX_FIXTURE)
    if padded_execs != 3 or len(padded_times) != 3:
        fail("padded-index fixture: all three rows must parse — nextest right-aligns "
                        "the index to the width of the total, so a pattern that wants `(1/2470)` "
                        "silently drops the first 999 rows of every large run. Parsed {} of "
                        "3".format(padded_execs))
    if padded_times.get(("geom-brep::all", "review_r1_rational_probes::probe_sphere_octant")) != 11.635:
        fail("padded-index fixture: the 11.635 s row did not parse: {!r}".format(padded_times))
    if padded_summaries != [(120.992, "2470 tests run: 2470 passed, 2483 skipped")]:
        fail("padded-index fixture: Summary did not parse: {!r}".format(padded_summaries))

    retry_times, retry_execs, _ = parse_leg(RETRY_FIXTURE)
    flaky = ("nextest-shapes", "always_fails")
    if abs(retry_times.get(flaky, 0.0) - 0.215) > 1e-9:
        fail("retry fixture: both attempts cost the job, so the test is charged "
                        "0.106 + 0.109 = 0.215s; got {!r}".format(retry_times.get(flaky)))
    if retry_execs != 4:
        fail("retry fixture: 4 rows inside the body (2 PASS, 2 TRY n FAIL); parsed "
                        "{}".format(retry_execs))

    # THE SUMMATION THAT THE FIRST TRAP IS ABOUT: one test present in two of
    # a job's legs is charged the sum, and the `legs` column says two.
    legs = [
        {"label": "leg A", "path": "-", "missing": None, "times": {corpus: 0.818},
         "executions": 1, "summaries": []},
        {"label": "leg B", "path": "-", "missing": None, "times": {corpus: 1.913},
         "executions": 1, "summaries": []},
    ]
    total = merge(legs)
    if total[corpus] != [0.818 + 1.913, 2]:
        fail("cross-leg summation: expected [{}, 2], got {!r}".format(
            0.818 + 1.913, total[corpus]))

    # A leg whose file never appeared is named, and the report still renders.
    absent = read_leg("run archived tests", "/nonexistent/nextest.log")
    if absent["missing"] is None:
        fail("a missing leg file must be reported as missing, not parsed as empty")
    text = render([absent], 20, "test (eps = default, 1/2)")
    if "no output captured" not in text:
        fail("a job with no captured output must say so in the report:\n" + text)
    # The reason names the errno and the BASENAME; the runner's absolute path
    # is not pasted into a public summary.
    if "/nonexistent/" in text or "nextest.log" not in text:
        fail("the missing-leg reason must carry the basename and not the full "
             "runner path:\n" + text)

    # A CHEAP JOB IS NOT A FREE ONE. Both reports print totals through
    # `fmt_secs` for this: a sub-0.05 s job rendered at one decimal is
    # `0.0 cpu-s in total`, which says the run cost nothing.
    if fmt_secs(0.038) != "0.038" or fmt_secs(1.665) != "1.7":
        fail("fmt_secs prints a cheap total as free: {!r} / {!r}".format(
            fmt_secs(0.038), fmt_secs(1.665)))
    cheap = render([{"label": "leg A", "path": "-", "missing": None,
                     "times": {corpus: 0.012}, "executions": 1, "summaries": []}],
                   20, "test (eps = default, 1/2)")
    if "0.012 cpu-s in total" not in cheap:
        fail("a sub-0.05 s job must not report its total as `0.0 cpu-s`:\n" + cheap)

    if failures:
        for line in failures:
            sys.stderr.write("selftest FAILED: {}\n".format(line))
        raise SystemExit(1)
    print("slowest-tests selftest ok: four captured fixtures (green, red including the "
          "post-Summary recap, a padded index, and a retried test), cross-leg summation, "
          "the missing-leg report, and a cheap job whose total is not printed as free")
    return 0


def main(argv):
    parser = argparse.ArgumentParser(add_help=True, description=__doc__.splitlines()[0])
    parser.add_argument("--selftest", action="store_true")
    parser.add_argument("--top", type=int, default=20)
    parser.add_argument("--job", default="this job")
    parser.add_argument("--leg", action="append", default=[],
                        metavar="LABEL=PATH", help="a nextest invocation's captured output")
    args = parser.parse_args(argv[1:])
    if args.selftest:
        return selftest()
    legs = []
    for spec in args.leg:
        label, _, path = spec.partition("=")
        if not path:
            parser.error("--leg wants LABEL=PATH, got {!r}".format(spec))
        legs.append(read_leg(label, path))
    if not legs:
        parser.error("no --leg given: this report is a function of the run's own output")
    sys.stdout.write(render(legs, args.top, args.job))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
