#!/usr/bin/env python3
"""Derive the `interval` feature's OWN tests — the set the interval run
legs execute — as the difference between two `cargo nextest list`
outputs.

WHY THIS EXISTS. The `interval` cargo feature is ADDITIVE: every
`#[cfg(feature = "interval")]` item in `crates/*/src` is an `impl ... for
Interval`, a `mod`/`use`/`type` for `Interval`, or a test-only item, and
there is no `cfg(not(feature = "interval"))` anywhere in the tree (both
properties are GATED by scripts/check-interval-cfg-additive.py, which
fails loudly the moment either stops holding). Rust has no
specialization, so adding impls cannot change how the f64 paths
monomorphize. A test that is present in BOTH builds therefore executes
the same machine code in both, and running it in the interval legs
re-proves, at the same two epsilon values, exactly what the default legs
already proved.

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

Usage:
    interval-only-selection.py <default.json> <interval.json>

Writes a nextest filter expression to stdout.
"""

import json
import sys


def load(path):
    """(binary_id, test_name) pairs from `cargo nextest list
    --message-format json`. Fails closed on any unexpected shape."""
    with open(path) as f:
        doc = json.load(f)
    suites = doc.get("rust-suites")
    if not isinstance(suites, dict) or not suites:
        raise SystemExit(
            "error: {}: no 'rust-suites' object — nextest's list schema "
            "changed, or the list is empty; refusing to guess".format(path)
        )
    out = set()
    for key, suite in suites.items():
        binary_id = suite.get("binary-id")
        cases = suite.get("testcases")
        if not isinstance(binary_id, str) or not isinstance(cases, dict):
            raise SystemExit(
                "error: {}: suite {!r} lacks 'binary-id'/'testcases' — "
                "nextest's list schema changed; refusing to guess".format(path, key)
            )
        for name in cases:
            out.add((binary_id, name))
    if not out:
        raise SystemExit("error: {}: listed zero tests".format(path))
    return out


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
    default_tests = load(argv[1])
    interval_tests = load(argv[2])

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
        raise SystemExit(
            "error: the interval build adds NO tests of its own. Either the "
            "interval test lanes were deleted (in which case delete the "
            "interval run legs too, deliberately) or the two lists were "
            "built with the same features. Refusing to emit an empty filter."
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
