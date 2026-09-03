#!/usr/bin/env python3
"""Derive the DEMOTED test set — the tests the nightly runs and the gate
does not — as the difference between two `cargo nextest list` outputs.

WHY THIS EXISTS. A test demoted to the nightly carries, at the test:

    #[cfg_attr(not(nightly_suite), ignore = "nightly-only: <reason>")]

so it is `#[ignore]`d in every ordinary build and an ordinary test under
`RUSTFLAGS="--cfg nightly_suite"`. The nightly has to run exactly those and
nothing else — Ev, 2026-08-22: the scheduled job runs *only the demoted
tests, not the whole suite* — and the marker deliberately lives AT the test
rather than in a central roster, so there is no list to read the set off.

There does not need to be one. The set is a property of the two builds, and
this is the same derivation `scripts/interval-only-selection.py` makes for the
`interval` feature: list twice, subtract. A test whose marker is deleted
leaves this set on the next run with nothing to remember, and no annotation
can lie about it.

THE DIFFERENCE IS OVER THE `ignored` FLAG, NOT OVER THE TEST NAMES, and that
is the one thing to get right here. `cargo nextest list` reports EVERY test
including the ignored ones — verified against the pinned 0.9.140:

    gate build     demoted -> {"ignored": true,  "filter-match": {... "ignored"}}
    --cfg build    demoted -> {"ignored": false, "filter-match": {"status": "matches"}}

so subtracting one *name set* from the other gives the EMPTY SET, every time,
for every tree. That empty selection, run with the `--no-tests=pass` this
lane needs for its legitimate empty case, is a nightly that reports green
having executed nothing — the exact silent-zero-coverage failure the lane
exists to prevent. The set below is therefore

    { t : ignored(t) at the gate  AND  not ignored(t) under the cfg }

which is precisely "the tests whose `ignore` is conditional on that cfg".

EV'S CONSTRAINT HOLDS BY CONSTRUCTION, not by a list. A pre-existing plain
`#[ignore]` — a reporting row, an instrument, a test only valid as the sole
test in a process — is `ignored: true` in BOTH listings, so it cancels out of
the difference and can never be selected. That is why the nightly needs no
`--run-ignored` in any spelling, and must never grow one: with the flag, the
whole pre-existing ignored population would run, which is the thing Ev
ruled out.

FAILS CLOSED ON A BROKEN RIG. An empty difference is LEGITIMATE here — unlike
the interval case, a tree with no demoted tests is an ordinary tree — but it
is indistinguishable, from the two listings alone, from the rig failure that
matters: both listings built the same way (RUSTFLAGS not reaching the second
build, a typo in the cfg name, the flag dropped from the workflow). Those
produce two identical listings and therefore the same empty set, and would
zero this lane silently and permanently. So the empty case is PROVED from the
SOURCE, exactly as the interval script proves its own: if not one file under
`crates/` carries the marker, no demoted test can exist and `none()` is the
right answer; if markers ARE in the tree and the difference is still empty,
that is a broken rig and this exits 1.

Usage:
    nightly-only-selection.py <gate.json> <nightly.json>

Writes a nextest filter expression to stdout, and the count to stderr.
"""

import json
import os
import re
import sys

# THE MARKER, as a SOURCE predicate, used only to decide whether an empty
# difference is legitimate. Deliberately LOOSE — a doc comment quoting the
# attribute satisfies it — because the two ways this can be wrong are not
# symmetric, the same asymmetry interval-only-selection.py states for its own
# crate scan: OVER-reporting sends an empty difference to the exit-1 arm,
# which merely asks a human to look; UNDER-reporting emits `none()` for a
# tree that really does have demoted tests, and that is the silent hole. When
# in doubt this must say "there are markers".
#
# Whitespace-tolerant and matched per line because the real markers wrap:
# `#[cfg_attr(` and `not(nightly_suite),` land on different lines.
MARKER_RE = re.compile(r"not\s*\(\s*nightly_suite\s*\)")
MARKER = 'a `cfg_attr(not(nightly_suite), ignore = "nightly-only: …")` marker'


def load(path):
    """`{(binary_id, test_name): ignored}` from `cargo nextest list
    --message-format json`. Fails closed on any unexpected SHAPE.

    An EMPTY suite set is not a shape error — it is a real answer for a scope
    with no Rust test binaries — but a missing or non-dict `rust-suites`, or a
    testcase with no `ignored` boolean, means the schema moved under us and
    the whole derivation is guesswork. Refuse rather than guess: this
    script's output decides what runs, and a misread schema would decide it
    is nothing.
    """
    with open(path) as f:
        doc = json.load(f)
    suites = doc.get("rust-suites")
    if not isinstance(suites, dict):
        raise SystemExit(
            "error: {}: no 'rust-suites' object — nextest's list schema "
            "changed; refusing to guess".format(path)
        )
    out = {}
    for key, suite in suites.items():
        binary_id = suite.get("binary-id")
        cases = suite.get("testcases")
        if not isinstance(binary_id, str) or not isinstance(cases, dict):
            raise SystemExit(
                "error: {}: suite {!r} lacks 'binary-id'/'testcases' — "
                "nextest's list schema changed; refusing to guess".format(path, key)
            )
        for name, case in cases.items():
            ignored = case.get("ignored") if isinstance(case, dict) else None
            if not isinstance(ignored, bool):
                raise SystemExit(
                    "error: {}: testcase {!r} in {!r} has no boolean 'ignored' "
                    "field. THAT FIELD IS THE WHOLE DERIVATION here — see this "
                    "script's header — so a schema without it cannot be read as "
                    "an empty demoted set; refusing to guess".format(path, name, binary_id)
                )
            out[(binary_id, name)] = ignored
    return out


def markers_in_tree(root):
    """Files under `crates/` carrying the marker. See `MARKER_RE` for why
    this is a loose text scan and which direction its errors take."""
    hits = []
    crates = os.path.join(root, "crates")
    for dirpath, _dirs, files in os.walk(crates):
        for name in sorted(files):
            if not name.endswith(".rs"):
                continue
            path = os.path.join(dirpath, name)
            with open(path, errors="replace") as f:
                if any(MARKER_RE.search(line) for line in f):
                    hits.append(os.path.relpath(path, root))
    return sorted(hits)


# Characters that need no quoting in nextest's filterset grammar, and that
# cover every binary id / test name a Rust workspace can produce.
SAFE = set("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_:-")


def term(kind, s):
    """An EXACT-match DSL term, emitted UNQUOTED — the same two load-bearing
    details `interval-only-selection.py` documents: the leading `=` makes it
    exact (a bare `test(foo)` is a SUBSTRING match, so `foo` would drag in
    `foo_and_more`), and nextest 0.9.140 does not match the quoted form for
    these values. Anything outside the safe alphabet is a hard error rather
    than a silently malformed filter that would under-select."""
    bad = sorted(set(c for c in s if c not in SAFE))
    if bad or not s:
        raise SystemExit(
            "error: {} {!r} contains character(s) {} that cannot go into an "
            "unquoted nextest filterset term. Teach this script to quote them "
            "(and verify the quoting against the pinned nextest) before this "
            "test can be selected.".format(kind, s, bad)
        )
    return "{}(={})".format(kind, s)


def select(gate, nightly):
    """`(demoted, only_under_cfg, backwards)` — the three ways the two
    listings can differ, separated because only one of them is the answer."""
    demoted = sorted(t for t, ig in gate.items() if ig and nightly.get(t) is False)
    # A test that EXISTS only under the cfg is `#[cfg(nightly_suite)]`, not the
    # sanctioned `cfg_attr` marker. It is not the idiom, but excluding it would
    # leave it running NOWHERE — the failure this whole job exists to close —
    # so it is selected and reported loudly instead.
    only_under_cfg = sorted(t for t in nightly if t not in gate and not nightly[t])
    # ...and the marker written backwards (`cfg_attr(nightly_suite, ignore)`),
    # which loses no coverage — the gate runs it — but is somebody's mistake.
    backwards = sorted(t for t, ig in gate.items() if not ig and nightly.get(t) is True)
    return demoted, only_under_cfg, backwards


def main(argv):
    # CHEAP MODE, and the reason it exists is a BILLING one rather than a
    # tidiness one. The two listings this script normally consumes each cost
    # a full workspace build (~11 billed minutes together), and they are
    # pure waste on a tree that carries no marker at all — which is the
    # tree's state whenever no demotion is currently in force. So the job
    # asks this question FIRST, with a text scan that compiles nothing, and
    # skips both builds when the answer is no.
    #
    # It reuses `markers_in_tree` rather than grepping in YAML on purpose:
    # `MARKER_RE` and the direction of its errors are stated once, here, and
    # a second copy in a workflow would be the one that goes stale. Prints
    # `yes`/`no` and exits 0 either way — ABSENCE IS NOT AN ERROR in this
    # mode, which is exactly the distinction the full path draws differently
    # (there, markers present with an empty difference IS fatal).
    if len(argv) == 2 and argv[1] == "--markers-present":
        root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
        hits = markers_in_tree(root)
        print("yes" if hits else "no")
        if hits:
            sys.stderr.write(
                "{} file(s) carry {}:\n".format(len(hits), MARKER)
                + "".join("  {}\n".format(h) for h in hits)
            )
        return 0
    if len(argv) != 3:
        raise SystemExit(__doc__)
    gate = load(argv[1])
    nightly = load(argv[2])
    root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

    demoted, only_under_cfg, backwards = select(gate, nightly)

    for t in only_under_cfg:
        sys.stderr.write(
            "WARNING: {} {} exists ONLY under --cfg nightly_suite, so it is a "
            "`#[cfg(nightly_suite)]` item rather than the sanctioned "
            "`cfg_attr(not(nightly_suite), ignore = ...)` marker. Selected anyway "
            "— excluding it would leave it running nowhere — but write it as the "
            "marker: only the marker keeps the test visible to the gate's own "
            "listing, which is what makes this derivation possible at all.\n".format(*t)
        )
    for t in backwards:
        sys.stderr.write(
            "WARNING: {} {} runs at the GATE and is ignored under --cfg "
            "nightly_suite — the marker is inverted (`cfg_attr(nightly_suite, "
            "ignore)`). No coverage is lost, the gate runs it; it is not "
            "selected here.\n".format(*t)
        )

    selected = demoted + only_under_cfg
    if not selected:
        # The empty case, PROVED from the source rather than inferred from the
        # empty difference — see the header. The implication runs one way, and
        # that is what keeps it safe: no marker in the tree ⇒ no demoted test.
        hits = markers_in_tree(root)
        if not hits:
            sys.stderr.write(
                "NOTE: no demoted tests — not one file under crates/ carries {}, "
                "so this tree HAS none and the nightly has nothing of this kind "
                "to run. Emitting `none()`.\n".format(MARKER)
            )
            print("none()")
            return 0
        raise SystemExit(
            "error: the --cfg nightly_suite build un-ignores NO tests, and {} "
            "file(s) under crates/ carry {} ({}). So this is not an empty tree: "
            "either RUSTFLAGS did not reach the second build, or the cfg name is "
            "misspelt at the marker, or the two listings were built the same way. "
            "Refusing to emit an empty filter — that would report green having "
            "run nothing, every night, until someone noticed.".format(
                len(hits), MARKER, ", ".join(hits)
            )
        )

    still_ignored = sum(1 for t, ig in gate.items() if ig and nightly.get(t) is True)
    sys.stderr.write(
        "demoted (nightly-only) tests: {} selected of {} in the gate listing; "
        "{} test(s) carry a plain #[ignore] and are ignored under BOTH builds, so "
        "they cancel out of this difference and the nightly does not run them "
        "either (Ev's constraint, by construction rather than by a list)\n".format(
            len(selected), len(gate), still_ignored
        )
    )
    for b, n in selected:
        sys.stderr.write("    {} {}\n".format(b, n))
    print(
        " | ".join(
            "({} & {})".format(term("binary_id", b), term("test", n))
            for b, n in selected
        )
    )
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
