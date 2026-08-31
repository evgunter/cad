#!/usr/bin/env python3
"""What THIS pull request adds to the test suite, and what that costs.

    This PR adds 7 tests costing 43.2 cpu-s per run (3.5% of this job's
    1,243 cpu-s).

WHY THIS EXISTS. Cost accrued to the suite because a PR never said what it
was spending (issue 469): the sentence above, printed on the run itself, is
the report that would have prevented the accumulation the 2026-08-13 audit
found. It is deliberately NOT a gate — a budget needs a baseline and the
baseline is moving fast — so nothing here thresholds anything, and the step
that calls it cannot fail its job.

A SIBLING OF `scripts/interval-only-selection.py`, and the same shape: a set
difference over the `(binary-id, test-name)` pairs of two `cargo nextest
list --message-format json` documents. What it does NOT share is that
script's `load`, and the difference is the whole reason the code is not
imported: the sibling's fail-closed direction is EXIT 1, because it selects
what the interval legs execute and an unrecognised schema there must stop
the run rather than run nothing. This one's fail-closed direction is a
PRINTED SKIP, because it gates nothing and a report that reds a job has
broken something more important than itself. Same question, opposite
disposition; sharing the function would force one of the two answers to be
wrong.

The timing parser IS shared, imported from `scripts/slowest-tests.py` by
path, because "how a nextest status line is read" is one question and two
answers to it would drift.

FAILS CLOSED INTO ABSENCE, WITH THE REASON PRINTED. Every way this can fail
to produce a diff — no base listing, an unreadable one, a schema it does not
recognise, an empty one — prints a block naming what was missing and why.
None of them exits nonzero, and none of them prints a number. A silent skip
would be indistinguishable from "this PR adds nothing", which is the one
reading that must never be manufactured.

WHERE THE BASE LISTING COMES FROM is the caller's problem, not this
script's: it takes a path or it takes a reason. `.github/workflows/ci.yml`
publishes each run's listing as an artifact named for the git TREE it was
built from, and looks the base tree's listing up among prior runs — so the
first run after that wiring lands, and any run whose base tree no previous
run listed, take the stated-skip path. That is the expected steady state on
day one, not a malfunction.

TWO THINGS THE DIFF CANNOT SEE, both stated in the output rather than here:

  * A RENAME IS ONE REMOVAL PLUS ONE ADDITION. The pairs carry no identity
    across a rename, so a renamed test reads as new coverage that costs what
    the old test cost. Honest enough for a report, dishonest if left
    unsaid, so the output says it whenever the numbers are nonzero.
  * THE TWO LISTINGS MAY HAVE BEEN BUILT AT DIFFERENT SCOPES, IN EITHER
    DIRECTION. The change filter scopes a run to the changed crates'
    closure, so the base run may have listed fewer packages than this one —
    or MORE. Every test in a package the base listing does not contain would
    otherwise read as ADDED, and every test in a package THIS listing does
    not contain would read as REMOVED; both are thousands of tests out of
    one differently-scoped run, and "1,500 tests were removed" is the same
    manufactured reading as its mirror. So ADDITIONS, REMOVALS AND THE
    PRICED SET ARE ALL RESTRICTED TO PACKAGES PRESENT IN BOTH LISTINGS, and
    the packages present in only one are NAMED, on the side they are missing
    from, with both readings (a genuinely new or deleted crate, or a
    different change-filter scope) — because these two documents cannot tell
    those apart.

A JOB THAT MEASURED NOTHING SAYS SO INSTEAD OF SAYING ZERO. The cost half of
the sentence comes from this run's own captured output; when that output
holds no test rows at all (the run leg died, its log never appeared), there
is no cost to state. `0.000 cpu-s (0.0% of this job's 0.000 cpu-s)` is a
number nothing measured, in the shape of one that was, so the lead becomes a
stated absence and the added tests are reported as unpriced.

PER-LEG TIMES ARE NOT COMPARABLE, so the cost half of the sentence is the
cost THIS JOB measured, over the added tests THIS JOB ran. A job that ran
one shard says so and names how many added tests it did not run; it never
scales, extrapolates, or borrows the sibling shard's numbers.

Usage:
    pr-added-tests.py --head HEAD.json [--base BASE.json] [--no-base REASON]
                      [--base-source TEXT] [--job LABEL] [--leg 'LABEL=PATH']
    pr-added-tests.py --selftest
"""

from __future__ import annotations

import argparse
import importlib.util
import json
import os
import sys
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
FIXTURES = os.path.join(HERE, "fixtures")


def _load_sibling():
    """`scripts/slowest-tests.py`, imported by path — its filename has a
    hyphen in it, so `import` cannot name it. The same device, for the same
    reason, as `interval-only-selection.py`'s import of its tripwire."""
    path = os.path.join(HERE, "slowest-tests.py")
    spec = importlib.util.spec_from_file_location("slowest_tests", path)
    if spec is None or spec.loader is None:
        raise SystemExit(
            "error: cannot load {} — this script has no timing parser of its own "
            "and must not invent a second one".format(path)
        )
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


slowest = _load_sibling()


class ListProblem(Exception):
    """A listing this script will not guess at. Carries the sentence the
    skip block prints — every raise site writes one a reader can act on."""


def load(path):
    """`(pairs, packages, owner)` from `cargo nextest list --message-format
    json` — the `(binary-id, test-name)` set, the package names, and the map
    from pair to owning package that the both-listings restriction needs.

    ONE PASS, and the map comes out of it rather than out of a second read of
    the same file: a re-parse with bare subscripts would be a shape assumption
    made after `load` had just refused to make one.

    Shape errors raise `ListProblem` rather than exiting: see the header for
    why this script's fail-closed direction is a printed skip. Emptiness is
    a shape error HERE, unlike in the sibling: an empty listing cannot be
    diffed into a meaningful statement, and "this PR adds every test in the
    suite" is exactly the sentence that must not be manufactured.
    """
    try:
        with open(path, encoding="utf-8") as fh:
            doc = json.load(fh)
    except OSError as exc:
        raise ListProblem("{} could not be read ({})".format(path, exc)) from exc
    except ValueError as exc:
        raise ListProblem("{} is not JSON ({})".format(path, exc)) from exc
    suites = doc.get("rust-suites") if isinstance(doc, dict) else None
    if not isinstance(suites, dict):
        raise ListProblem(
            "{} has no 'rust-suites' object — nextest's list schema changed, and this "
            "report refuses to guess at it".format(path)
        )
    if not suites:
        raise ListProblem("{} lists no test suites at all".format(path))
    pairs = set()
    packages = set()
    owner = {}
    for key, suite in suites.items():
        binary_id = suite.get("binary-id") if isinstance(suite, dict) else None
        cases = suite.get("testcases") if isinstance(suite, dict) else None
        package = suite.get("package-name") if isinstance(suite, dict) else None
        if not isinstance(binary_id, str) or not isinstance(cases, dict) or not isinstance(package, str):
            raise ListProblem(
                "{}: suite {!r} lacks 'binary-id'/'testcases'/'package-name' — nextest's "
                "list schema changed, and this report refuses to guess at it".format(path, key)
            )
        packages.add(package)
        for name in cases:
            pairs.add((binary_id, name))
            owner[(binary_id, name)] = package
    return pairs, packages, owner


def plural(n, noun):
    return "{} {}{}".format(n, noun, "" if n == 1 else "s")


# A cheap test is not a free one, and both reports answer that the same way:
# the formatter lives in the sibling, imported rather than re-spelled here.
fmt_secs = slowest.fmt_secs


def package_list(names):
    return ", ".join("`{}`".format(p) for p in names)


def skip_block(job, reason):
    return "\n".join([
        "### What this PR adds to the test suite — {}".format(job),
        "",
        "**Skipped, and here is why:** {}".format(reason),
        "",
        "Nothing is inferred from that absence. This report is a set difference between "
        "two real `cargo nextest list` documents or it is nothing at all — a missing base "
        "listing must not be read as \"this PR adds no tests\". It is also not a failure: "
        "this block gates nothing and cannot red a job.",
        "",
    ])


def render(job, base_path, base_source, head_path, legs, top=20):
    try:
        base_pairs, base_packages, base_owner = load(base_path)
        head_pairs, head_packages, head_owner = load(head_path)
    except ListProblem as problem:
        return skip_block(job, str(problem))

    # BOTH DIRECTIONS ARE SCOPED THE SAME WAY, which is what makes the header's
    # "restricted to packages present in BOTH listings" true of all three sets.
    # A base run listing one package this run did not would otherwise put every
    # test of that package into `removed` and print "1,500 tests were removed"
    # as flat fact — the mirror image of the addition-side reading this
    # restriction was written for, and just as wrong.
    shared = base_packages & head_packages
    added = sorted(p for p in head_pairs - base_pairs if head_owner.get(p) in shared)
    excluded = sorted(p for p in head_pairs - base_pairs if head_owner.get(p) not in shared)
    removed = sorted(p for p in base_pairs - head_pairs if base_owner.get(p) in shared)
    excluded_removals = sorted(p for p in base_pairs - head_pairs
                               if base_owner.get(p) not in shared)
    head_only_packages = sorted(head_packages - base_packages)
    base_only_packages = sorted(base_packages - head_packages)

    times = slowest.merge(legs)
    job_total = sum(secs for secs, _ in times.values())
    ran = [(times[p][0], p) for p in added if p in times]
    cost = sum(secs for secs, _ in ran)
    share = (100.0 * cost / job_total) if job_total else 0.0
    not_run = len(added) - len(ran)

    out = ["### What this PR adds to the test suite — {}".format(job), ""]
    if not added and not removed and not excluded and not excluded_removals:
        out.append("**This PR adds no tests and removes none** — the two listings' "
                   "`(binary-id, test-name)` sets are identical ({} tests).".format(len(head_pairs)))
    elif not times:
        # NO ROWS WERE MEASURED, so there is no cost, and a zero in the shape of
        # a measurement is the one thing this report may not print. Same answer
        # as the sibling's "no test rows were found … nothing to rank", and the
        # unpriced count leads it rather than sitting under a `0.000`.
        out.append("**This PR adds {}, and this job measured nothing to price {} against.** No test "
                   "rows were found in this job's captured output — the run leg produced no output, "
                   "or died before nextest did — so all {} of them are unpriced here and no number "
                   "is stated. A zero beside them would be a figure nothing measured, not a cheap "
                   "test.".format(plural(len(added), "test"),
                                      "it" if len(added) == 1 else "them", len(added)))
    else:
        out.append("**This PR adds {} test{} costing {} cpu-s per run ({:.1f}% of this job's "
                   "{} cpu-s).**".format(len(added), "" if len(added) == 1 else "s",
                                         fmt_secs(cost), share, fmt_secs(job_total)))
    out.append("")
    out.append("- Base listing: {} — {} tests over {}. This run's listing: {} tests "
               "over {}.".format(base_source, len(base_pairs), plural(len(base_packages), "package"),
                                 len(head_pairs), plural(len(head_packages), "package")))
    if added and times and not_run:
        out.append("- {} of the {} added tests did not run in this job's legs (this job is one "
                   "shard of one lane). Their cost is NOT summed here and NOT estimated: "
                   "per-leg times are not comparable, so the job that ran them is the only "
                   "place they can honestly be priced.".format(not_run, len(added)))
    if added or removed:
        out.append("- {} test{} removed. **A rename shows as one removal plus one addition**, so a "
                   "nonzero pair here can be a rename rather than new coverage — this diff is over "
                   "names and cannot tell them apart.".format(
                       len(removed), " was" if len(removed) == 1 else "s were"))
    if head_only_packages:
        out.append("- {} package{} in this run's listing and not in the base listing ({}), so their "
                   "{} test{} excluded from the count above. Either they are new crates or the two "
                   "listings were built at different change-filter scopes; these two documents "
                   "cannot tell those apart.".format(
                       len(head_only_packages), " is" if len(head_only_packages) == 1 else "s are",
                       package_list(head_only_packages),
                       len(excluded), " is" if len(excluded) == 1 else "s are"))
    if base_only_packages:
        out.append("- {} package{} in the BASE listing and not in this run's ({}), so their "
                   "{} test{} excluded from the removal count above. Either the crates were "
                   "deleted or the base run was scoped wider than this one; these two documents "
                   "cannot tell those apart, and neither reading may be printed as a removal "
                   "count.".format(
                       len(base_only_packages), " is" if len(base_only_packages) == 1 else "s are",
                       package_list(base_only_packages),
                       len(excluded_removals), " is" if len(excluded_removals) == 1 else "s are"))
    out.append("")
    if ran:
        out.append("| cpu-s | added test (measured in this job) |")
        out.append("|------:|------|")
        for secs, (binary, test) in sorted(ran, reverse=True)[:top]:
            out.append("| {:.3f} | `{} {}` |".format(secs, binary, test))
        if len(ran) > top:
            out.append("")
            out.append("({} further added tests ran in this job and are not listed; cost "
                       "concentrates, so this is the head of the addition.)".format(len(ran) - top))
        out.append("")
    out.append("_Deliberately not a gate (issue 469): a budget needs a baseline and the baseline is "
               "moving, so this reports and does not threshold._")
    out.append("")
    return "\n".join(out)


# ---------------------------------------------------------------- selftest

# THE FIXTURES ARE REAL OUTPUT, not hand-written JSON that agrees with this
# parser by construction. `scripts/fixtures/nextest-list-{base,head}.json`
# and `nextest-run-head.txt` were produced by the PINNED cargo-nextest
# 0.9.140 over a three-test throwaway crate and then the same crate with
# `quick_pass` renamed to `quick_pass_renamed` and `extra_test` added — one
# rename and one addition, which is the pair the rename note is about. The
# only edit is the crate's absolute path, rewritten to `/tmp/probe`; no
# field this script reads was touched.
BASE_LIST = os.path.join(FIXTURES, "nextest-list-base.json")
HEAD_LIST = os.path.join(FIXTURES, "nextest-list-head.json")
HEAD_RUN = os.path.join(FIXTURES, "nextest-run-head.txt")


def selftest():
    failures = []

    def fail(msg):
        failures.append(msg)

    base_pairs, base_packages, _ = load(BASE_LIST)
    head_pairs, head_packages, _ = load(HEAD_LIST)
    if base_packages != {"nextest-shapes"} or head_packages != {"nextest-shapes"}:
        fail("fixture packages moved: {!r} / {!r}".format(base_packages, head_packages))
    if sorted(n for _, n in head_pairs - base_pairs) != ["extra_test", "quick_pass_renamed"]:
        fail("the addition set is wrong: {!r}".format(head_pairs - base_pairs))
    if sorted(n for _, n in base_pairs - head_pairs) != ["quick_pass"]:
        fail("the removal set is wrong: {!r}".format(base_pairs - head_pairs))

    legs = [slowest.read_leg("run archived tests", HEAD_RUN)]
    report = render("test (eps = default, 1/2)", BASE_LIST, "the fixture", HEAD_LIST, legs)
    # 0.013 (extra_test) + 0.025 (quick_pass_renamed) of a 1.665 cpu-s leg.
    if "adds 2 tests costing 0.038 cpu-s per run (2.3% of this job's 1.7 cpu-s)" not in report:
        fail("the measured cost of the two added tests is not in the report:\n" + report)
    for want in ("rename shows as one removal plus one addition", "1 test was removed",
                 "`nextest-shapes extra_test`", "not a gate"):
        if want not in report:
            fail("the report is missing {!r}:\n{}".format(want, report))

    # A base listing that is not there is a STATED skip, never a number and
    # never a red — the day-one steady state of the ci.yml wiring.
    missing = render("test (eps = default, 1/2)", os.path.join(FIXTURES, "no-such-list.json"),
                     "nothing", HEAD_LIST, legs)
    if "Skipped" not in missing or "could not be read" not in missing:
        fail("a missing base listing must print a stated skip:\n" + missing)
    if "**This PR adds" in missing:
        fail("a skip must not print a count:\n" + missing)

    # A schema this script does not recognise is the same stated skip. The
    # fixture is the real head listing with its one recognised key renamed,
    # which is what a nextest schema change would look like from here.
    broken = os.path.join(FIXTURES, "nextest-list-head.json")
    with open(broken, encoding="utf-8") as fh:
        doc = json.load(fh)
    doc["rust-suites-renamed-upstream"] = doc.pop("rust-suites")
    with tempfile.TemporaryDirectory() as tmpdir:
        tmp = os.path.join(tmpdir, "moved-schema.json")
        with open(tmp, "w", encoding="utf-8") as fh:
            json.dump(doc, fh)
        schema = render("test (eps = default, 1/2)", tmp, "the fixture", HEAD_LIST, legs)
    if "no 'rust-suites' object" not in schema:
        fail("a moved schema must print the stated skip that names it:\n" + schema)

    # A BASE LISTING SCOPED WIDER THAN THIS RUN'S. Derived from the real base
    # fixture the same way the moved-schema case above is derived — its one
    # real suite is repeated under a second package name, which is what one
    # extra package in a differently-scoped base run looks like from here. The
    # unscoped removal set was `base - head`, so those three tests would have
    # been announced as removals: "4 tests were removed", flat, from a PR that
    # removed one. The claim being asserted is that the answer stays scoped
    # AND caveated — the extra package named, its tests counted out of the
    # removal line, and the removal line still reporting the one real removal.
    with open(BASE_LIST, encoding="utf-8") as fh:
        wider = json.load(fh)
    only = dict(wider["rust-suites"]["nextest-shapes"])
    only["package-name"] = "nextest-shapes-extra"
    only["binary-id"] = "nextest-shapes-extra"
    wider["rust-suites"]["nextest-shapes-extra"] = only
    with tempfile.TemporaryDirectory() as tmpdir:
        tmp = os.path.join(tmpdir, "wider-base.json")
        with open(tmp, "w", encoding="utf-8") as fh:
            json.dump(wider, fh)
        wide = render("test (eps = default, 1/2)", tmp, "the fixture", HEAD_LIST, legs)
    if "1 test was removed" not in wide:
        fail("a wider-scope base must still report only the removals inside the SHARED "
             "packages — its extra package's tests are not removals:\n" + wide)
    for want in ("1 package is in the BASE listing and not in this run's (`nextest-shapes-extra`)",
                 "3 tests are excluded from the removal count",
                 "the base run was scoped wider than this one"):
        if want not in wide:
            fail("a wider-scope base must name the base-only package and caveat it; "
                 "missing {!r}:\n{}".format(want, wide))
    if "4 tests were removed" in wide:
        fail("the removal side is unscoped: a wider base is being read as a mass "
             "deletion:\n" + wide)

    # A JOB THAT MEASURED NOTHING says so, rather than pricing the addition at
    # zero. The leg file does not exist, so `merge` returns no rows at all —
    # the exact input that used to print "costing 0.000 cpu-s per run (0.0% of
    # this job's 0.000 cpu-s)", a manufactured number in the shape of a
    # measured one.
    dead = [slowest.read_leg("run archived tests", os.path.join(FIXTURES, "no-such-run.txt"))]
    unmeasured = render("test (eps = default, 1/2)", BASE_LIST, "the fixture", HEAD_LIST, dead)
    if "0.000 cpu-s" in unmeasured or "0.0%" in unmeasured:
        fail("a job that measured nothing must not print a zero cost:\n" + unmeasured)
    for want in ("measured nothing to price them against",
                 "No test rows were found in this job's captured output",
                 "all 2 of them are unpriced here"):
        if want not in unmeasured:
            fail("the measured-nothing lead is missing {!r}:\n{}".format(want, unmeasured))
    if "did not run in this job's legs" in unmeasured:
        fail("the unpriced count belongs in the lead, not in a bullet under a "
             "headline that is not there:\n" + unmeasured)
    if "1 test was removed" not in unmeasured:
        fail("a job that measured nothing still knows the DIFF — only the cost is "
             "absent:\n" + unmeasured)

    if failures:
        for line in failures:
            sys.stderr.write("selftest FAILED: {}\n".format(line))
        raise SystemExit(1)
    print("pr-added-tests selftest ok: a real rename+addition diff priced from a real run; a "
          "missing base listing and a moved schema both printing stated skips; a wider-scope "
          "base scoped and caveated on the REMOVAL side; and a job that measured nothing "
          "saying so instead of pricing the addition at zero")
    return 0


def main(argv):
    parser = argparse.ArgumentParser(add_help=True, description=__doc__.splitlines()[0])
    parser.add_argument("--selftest", action="store_true")
    parser.add_argument("--head")
    parser.add_argument("--base")
    parser.add_argument("--no-base", metavar="REASON",
                        help="print the stated skip with this reason (no base listing was found)")
    parser.add_argument("--base-source", default="an earlier run's published listing")
    parser.add_argument("--job", default="this job")
    parser.add_argument("--leg", action="append", default=[], metavar="LABEL=PATH")
    args = parser.parse_args(argv[1:])
    if args.selftest:
        return selftest()
    if not args.head:
        parser.error("--head is required")
    if not args.base:
        sys.stdout.write(skip_block(args.job, args.no_base or "no base listing was supplied"))
        return 0
    legs = []
    for spec in args.leg:
        label, _, path = spec.partition("=")
        if not path:
            parser.error("--leg wants LABEL=PATH, got {!r}".format(spec))
        legs.append(slowest.read_leg(label, path))
    sys.stdout.write(render(args.job, args.base, args.base_source, args.head, legs))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
