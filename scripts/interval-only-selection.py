#!/usr/bin/env python3
"""Derive the `interval` feature's OWN tests — the set the interval run
legs execute — as the difference between two `cargo nextest list`
outputs.

WHY THIS EXISTS. The `interval` cargo feature is ADDITIVE: every
`#[cfg(feature = "interval")]` item in `crates/*/src` is an `impl ... for
Interval`, a `mod`/`use`/`type` for `Interval`, or a test-only item, and
nothing under `crates/*/src` is compiled `cfg(not(feature =
"interval"))` (both properties are GATED by
scripts/check-interval-cfg-additive.py, which fails loudly the moment
either stops holding). Under `crates/*/tests` that gate deliberately
ALLOWS negation and requires whole-item gating instead — the loud-skip
marker rows this file's own reverse-direction note below relies on are
exactly that — and whole-item gating is what carries the argument here.
Rust has no specialization, so adding impls cannot change how the f64
paths monomorphize. A test that is present in BOTH builds therefore
executes the same machine code in both, and running it in the interval
legs re-proves, at the same two epsilon values, exactly what the default
legs already proved.

Measured on CI run 31665082966: of the 2995 tests in the interval legs,
214 are cfg-gated (168.7 cpu-s of coverage the default legs cannot
have); the other 2781 cost 4406.7 cpu-s — 42.0% of the whole run's
10482 cpu-s of test execution — re-running identical code. This script
is what lets the interval legs run the 214 and skip the 2781.

DERIVED, NEVER DECLARED. The set is computed from what the two archives
actually contain, so no test can opt itself out and no annotation can
lie. A test that stops being cfg-gated returns to the default legs
automatically; a newly gated test is picked up with no edit here. That
is the whole reason this is a set difference and not an allowlist.

FAILS CLOSED. An empty difference, a malformed list, or a list whose
shape this script does not recognise is an ERROR (exit 1), never an
empty filter that would silently run nothing.

ONE legitimate empty case, decided from OUTSIDE the two lists. The
guard above was written when this ran over the whole workspace, where
"the interval build adds nothing" really is impossible. Under a SCOPED
run it is ordinary: a change closure can scope both builds to crates
that carry no interval-gated tests at all, and then an empty difference
means "nothing to run here", not "coverage was lost". `-p pncad-py` is
the first such closure to exist — that crate contains not one
`cfg(feature = "interval")` item, in src or tests.

The two lists CANNOT tell those apart: a scope with no gated tests and
a scope whose lists were built with the same features both produce two
identical lists. So the legitimacy signal is not read from them. It is
read from the SOURCE: for every crate in the scope (taken from the
lists' own suites), does that crate contain any cfg naming the interval
feature? If not one of them does, no interval-gated test can exist in
this scope, the empty difference is proved rather than assumed, and
`none()` goes to stdout with exit 0.

That implication only runs one way, which is what keeps it safe. No
gate in the source ⇒ no gated test. A crate that HAS gates and still
contributes no difference takes the exit-1 arm exactly as before — so
deleted interval lanes (whose src-side impls remain) and lists built
with the same features both still fail closed, and every full-workspace
run still passes through crates full of gates.

Usage:
    interval-only-selection.py <default.json> <interval.json>

Writes a nextest filter expression to stdout.
"""

import importlib.util
import json
import os
import sys

# THE PREDICATE IS IMPORTED, NOT RESTATED. Whether a line carries a cfg
# naming the interval feature is one question with one home:
# check-interval-cfg-additive.py's `carries_interval_cfg`. This script
# used to answer it with a bare substring test over a whole file, which
# is a question about TEXT and not about what compiles — a doc comment
# QUOTING the attribute satisfied it, so `crates/step-import`, whose
# only occurrence is the sentence in `tests/all.rs` explaining that
# inner attributes survive `#[path]` aggregation, counted as gated. That
# is the same defect #753 fixed in scripts/gates/probe-suite-census.sh,
# and the reason the fix is an import rather than a second anchored
# regex here: two gates answering one question two ways is how the first
# one drifted.
#
# The direction that matters is stated at `CARRIES_CFG` there: this scan
# must stay a SUPERSET of what the tripwire calls a gate, because
# under-reporting here emits `none()` for a scope that really does have
# interval-gated tests.
FEATURE_CFG_DESCRIPTION = 'cfg attribute naming `feature = "interval"`'


def _load_tripwire():
    """The sibling tripwire, imported by path — its filename has hyphens
    in it, so `import` cannot name it."""
    path = os.path.join(
        os.path.dirname(os.path.abspath(__file__)), "check-interval-cfg-additive.py"
    )
    spec = importlib.util.spec_from_file_location("check_interval_cfg_additive", path)
    if spec is None or spec.loader is None:
        raise SystemExit(
            "error: cannot load {} — this script's crate scan has no predicate "
            "of its own and must not invent one".format(path)
        )
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


carries_interval_cfg = _load_tripwire().carries_interval_cfg


def load(path):
    """`((binary_id, test_name) pairs, package names)` from `cargo
    nextest list --message-format json`. Fails closed on any unexpected
    SHAPE.

    Emptiness is not a shape error here. A missing or non-dict
    `rust-suites` still means the schema moved and is fatal, but an
    EMPTY suite set is a real answer nextest gives for a scope with no
    Rust test binaries at all, and whether that is legitimate is a
    question about the SOURCE, not about this list. Deciding it here
    was what made the scoped-no-op closure unrepresentable.

    The package names come back because they ARE the scope — the set of
    crates both builds were pointed at, taken from the run rather than
    re-derived from a CLI string this script never sees.
    """
    with open(path) as f:
        doc = json.load(f)
    suites = doc.get("rust-suites")
    if not isinstance(suites, dict):
        raise SystemExit(
            "error: {}: no 'rust-suites' object — nextest's list schema "
            "changed; refusing to guess".format(path)
        )
    out = set()
    packages = set()
    for key, suite in suites.items():
        binary_id = suite.get("binary-id")
        cases = suite.get("testcases")
        package = suite.get("package-name")
        if (
            not isinstance(binary_id, str)
            or not isinstance(cases, dict)
            or not isinstance(package, str)
        ):
            raise SystemExit(
                "error: {}: suite {!r} lacks 'binary-id'/'testcases'/"
                "'package-name' — nextest's list schema changed; refusing "
                "to guess".format(path, key)
            )
        packages.add(package)
        for name in cases:
            out.add((binary_id, name))
    return out, packages


def crates_with_interval_gates(root, packages):
    """Which of `packages` contain any cfg naming the interval feature.

    A syntactic scan of the crate's whole source, both halves, through
    the tripwire's own predicate — deliberately COARSER than "has a
    gated test", because the two ways it can be wrong are not symmetric.
    Over-reporting (a crate whose only gates are src-side impls) sends an
    empty difference to the exit-1 arm, which is the safe direction.
    Under-reporting is the dangerous one, which is why the predicate is
    the loosest of the tripwire's three and why it is imported from
    there: a gate form this cannot see is one the tripwire cannot police
    either, so the blind spot is stated once instead of diverging.

    A package with no crate directory is reported as gated, for the same
    reason — absence cannot be proved from a directory that is not
    there.
    """
    gated = set()
    for package in sorted(packages):
        crate = os.path.join(root, "crates", package)
        if not os.path.isdir(crate):
            gated.add(package)
            continue
        for half in ("src", "tests"):
            for dirpath, _dirs, files in os.walk(os.path.join(crate, half)):
                for name in files:
                    if not name.endswith(".rs"):
                        continue
                    with open(os.path.join(dirpath, name)) as f:
                        if any(carries_interval_cfg(line) for line in f):
                            gated.add(package)
    return gated


# Characters that need no quoting in nextest's filterset grammar, and
# that cover every binary id / test name a Rust workspace can produce
# (identifiers, `::` paths, and the `-` in crate names).
SAFE = set("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_:-")


def term(kind, s):
    """An EXACT-match DSL term, emitted UNQUOTED.

    Two things are load-bearing. The leading `=` makes it exact: a bare
    `test(foo)` is a SUBSTRING match in nextest's grammar, so a test
    named `foo` would also drag in `foo_and_more`. And the value is
    unquoted because nextest 0.9.140's quoted form does not match here
    (verified against the pinned binary: `binary_id(=geom-core::all)`
    matches, `binary_id(="geom-core::all")` reports "no binary IDs
    matched"). Unquoted is safe for Rust test paths — but only for
    those, so anything outside the safe alphabet is a hard error rather
    than a silently malformed filter that would under-select."""
    bad = sorted(set(c for c in s if c not in SAFE))
    if bad or not s:
        raise SystemExit(
            "error: {} {!r} contains character(s) {} that cannot go into an "
            "unquoted nextest filterset term. Teach this script to quote "
            "them (and verify the quoting against the pinned nextest) "
            "before this test can be selected.".format(kind, s, bad)
        )
    return "{}(={})".format(kind, s)


def main(argv):
    if len(argv) != 3:
        raise SystemExit(__doc__)
    default_tests, default_packages = load(argv[1])
    interval_tests, interval_packages = load(argv[2])
    root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    scope = default_packages | interval_packages

    only_interval = sorted(interval_tests - default_tests)
    # The reverse direction — tests the DEFAULT build has and the interval
    # build does not — is reported, not fatal. It is legitimate and in use:
    # the "loud skip" marker rows (interval_lane_skipped_no_certified_
    # coverage_here in sweep and topo) are `cfg(not(feature = "interval"))`
    # on purpose, so that a lane which contributes no certified interval
    # coverage stays VISIBLE in the battery log instead of silently
    # vanishing. Those rows still run — in the default legs, which is the
    # only place they exist — so nothing goes unrun and this reduction
    # takes nothing away from them.
    #
    # It is printed on every run anyway, because the set going quietly
    # from 4 to 40 is exactly the sort of drift worth seeing. What would
    # be genuinely dangerous — a test present in BOTH builds that behaves
    # differently under the feature, and so is skipped here while its
    # interval half runs nowhere — is not visible in this diff at all;
    # that is gated structurally by
    # scripts/check-interval-cfg-additive.py's rule that every interval
    # cfg in crates/*/tests must gate a whole item, never an inner block.
    only_default = sorted(default_tests - interval_tests)
    if only_default:
        sys.stderr.write(
            "default-only tests (not run by these legs; they run in the "
            "default legs, where they are the only place they exist): {}\n".format(
                len(only_default)
            )
        )
        for b, n in only_default:
            sys.stderr.write("    {} {}\n".format(b, n))

    if not only_interval:
        # The scoped no-op, PROVED from the source rather than inferred
        # from the empty difference: not one crate in this scope carries
        # a cfg naming the feature, so no interval-gated test can exist
        # here and there is nothing for these legs to run. See the
        # header for why the one-way implication is what keeps the
        # catastrophe on the other branch.
        gated = crates_with_interval_gates(root, scope)
        if not gated:
            sys.stderr.write(
                "NOTE: scoped no-op — the {} crate(s) in this scope ({}) "
                "carry no {} at all, so this scope HAS no "
                "interval-gated tests and these legs have nothing to run. "
                "Emitting `none()`. If you expected tests here, the SCOPE "
                "is wrong, not this selection.\n".format(
                    len(scope),
                    ", ".join(sorted(scope)) or "none",
                    FEATURE_CFG_DESCRIPTION,
                )
            )
            print("none()")
            return 0
        raise SystemExit(
            "error: the interval build adds NO tests of its own, and this "
            "scope DOES carry interval cfgs ({}) — so this is not the "
            "scoped no-op. Either the interval test lanes were deleted (in "
            "which case delete the interval run legs too, deliberately) or "
            "the two lists were built with the same features. Refusing to "
            "emit an empty filter.".format(", ".join(sorted(gated)))
        )

    sys.stderr.write(
        "interval-only tests: {} of {} ({} shared with the default legs, "
        "not re-run here)\n".format(
            len(only_interval), len(interval_tests), len(default_tests & interval_tests)
        )
    )
    print(
        " | ".join(
            "({} & {})".format(term("binary_id", b), term("test", n))
            for b, n in only_interval
        )
    )
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
