#!/usr/bin/env python3
"""Every name in the slow set names a test that exists.

`.config/nextest.toml`'s `ci` profile excludes the slow set by exact name,
`binary_id(B) & (test(=suite::name) | ...)`. An exact name that matches no test
excludes nothing: a renamed slow test then runs in every per-PR gate, and the
filter still reads as if it were held back. This script reconciles the filter
against a `cargo nextest list --message-format json` listing and fails, naming
each entry, when one matches nothing.

    cargo nextest list --ignore-default-filter <scope> --message-format json \\
      | check-slow-set-names.py [--complete | --members a,b,...]
                                [--elsewhere a,b,...] [--selftest]

WHAT A LISTING CAN DECIDE. A scoped listing (the per-PR `test` job builds the
crates the diff seeds) holds some packages and not others. An entry is checked
when its binary's PACKAGE is in the listing: the binary must be there, and the
name must be one of its tests. An entry whose package the listing does not
hold is left unchecked and counted, never passed silently. `--complete` says
the listing is the whole workspace (the nightly's), so every entry is checked
and a package absent from it is itself a dead entry. `--members` names the
workspace's packages instead (the per-PR `test` job, whose two shards each
list part of the scope): an entry whose package is no member is dead, and one
whose member this listing does not hold is left unchecked. THE GAP: an entry
naming a real member whose package has no test binary in EITHER shard's
listing is unchecked in both, which `--complete` would have failed; no shard
sees the other's listing, so nothing per-PR closes it.

A FEATURE A LISTING DOES NOT BUILD. The kernel's listings are default-feature
builds, and `viewer` gates every module of its toolkit half behind its `app`
feature (`crates/viewer/src/lib.rs`), so a slow test under `app::` is in no
such listing, though the viewer jobs' `--profile ci --features app` run reads
the same filter and needs its entry. `--elsewhere a,b,...` names the packages
this listing cannot decide: their entries are left unchecked and counted, even
under `--complete`, and the job that builds the package with its feature
checks them over its own listing (ci.yml's and nightly.yml's viewer jobs, `-p
viewer --features app`, under `--members`). An entry for a test that exists
only WITHOUT the feature would read as dead there; viewer has none in the slow
set.

The filter is read by `scripts/slow_set.py`, the one parser ci-filter and
ci.yml's slow-set step read it through too; anything it cannot read (a
predicate other than `binary_id(...)` and `test(=...)`, whitespace inside a
matcher's text) fails the run, because this script could not say what it
matches. A name listed twice under one binary fails as well.
"""

from __future__ import annotations

import json
import os
import sys
import tomllib
from collections import Counter
from typing import NoReturn

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from slow_set import CONFIG, SlowSetError, ci_filter
from slow_set import entries as read_entries


def die(msg: str) -> NoReturn:
    sys.exit(f"check-slow-set-names: {msg}")


def entries(filter_text: str) -> list[tuple[str, str]]:
    """The filter's (binary-id, exact test name) pairs, in file order."""
    try:
        return read_entries(filter_text)
    except SlowSetError as exc:
        die(f"the ci filter cannot be read ({exc}); no slow-set entry was checked")


def check(
    pairs: list[tuple[str, str]],
    suites: dict[str, set[str]],
    complete: bool,
    members: set[str] | None = None,
    label: str = "slow-set",
    elsewhere: frozenset[str] = frozenset(),
) -> int:
    """Print the verdict and return the exit status."""
    packages = {b.split("::")[0] for b in suites}
    dead, unchecked, deferred = [], Counter(), Counter()
    for binary, name in pairs:
        package = binary.split("::")[0]
        if members is not None and package not in members:
            dead.append(f"{binary}: {name} (no workspace package `{package}`)")
        elif package in elsewhere:
            deferred[binary] += 1
        elif not complete and package not in packages:
            unchecked[binary] += 1
        elif binary not in suites:
            dead.append(f"{binary}: {name} (no test binary `{binary}` in the listing)")
        elif name not in suites[binary]:
            dead.append(f"{binary}: {name}")
    twice = [f"{b}: {n}" for (b, n), k in Counter(pairs).items() if k > 1]
    checked = len(pairs) - sum(unchecked.values()) - sum(deferred.values())
    print(f"check-slow-set-names: {checked} of {len(pairs)} {label} entries checked")
    for binary, k in sorted(unchecked.items()):
        print(f"  unchecked: {k} under {binary} (its package is not in this listing)")
    for binary, k in sorted(deferred.items()):
        print(
            f"  unchecked: {k} under {binary} (--elsewhere: the job that lists its "
            "package with its features checks it)"
        )
    for line in dead:
        print(f"::error file={CONFIG}::{label} entry matches no test: {line}")
    for line in twice:
        print(f"::error file={CONFIG}::{label} entry listed twice: {line}")
    if dead or twice:
        print(
            f"check-slow-set-names: {len(dead)} dead, {len(twice)} duplicated. "
            "Re-point a renamed test's entry at its new name, or drop it.",
            file=sys.stderr,
        )
        return 1
    return 0


def suites_of(listing: dict) -> dict[str, set[str]]:
    try:
        return {bid: set(s["testcases"]) for bid, s in listing["rust-suites"].items()}
    except (KeyError, TypeError) as exc:
        die(
            f"the listing is not `cargo nextest list --message-format json` output ({exc!r})"
        )


def selftest() -> int:
    text = (
        "not (\n  (binary_id(a::all) & (test(=m::x) | test(=m::y))) |\n"
        "  (binary_id(b) & test(=n::z))\n)"
    )
    pairs = entries(text)
    assert pairs == [("a::all", "m::x"), ("a::all", "m::y"), ("b", "n::z")], pairs
    live = {"a::all": {"m::x", "m::y"}, "b": {"n::z"}}
    assert check(pairs, live, complete=True) == 0, "every name live"
    assert check(pairs, {"a::all": {"m::x"}, "b": {"n::z"}}, complete=True) == 1, (
        "renamed test"
    )
    assert check(pairs, {"a::all": {"m::x", "m::y"}}, complete=True) == 1, (
        "binary gone, complete"
    )
    assert check(pairs, {"a::all": {"m::x", "m::y"}}, complete=False) == 0, (
        "package out of scope"
    )
    assert (
        check(pairs, {"a::all": {"m::x", "m::y"}, "b::all": set()}, complete=False) == 1
    ), "package in scope, binary gone"
    assert check([*pairs, ("b", "n::z")], live, complete=True) == 1, "duplicate"
    half = {"a::all": {"m::x", "m::y"}}
    assert check(pairs, half, complete=False, members={"a", "b"}) == 0, (
        "member another shard lists"
    )
    assert check(pairs, half, complete=False, members={"a"}) == 1, "no such package"
    # THE GAP: `b` is a member, but no listing holds a `b` binary. Each shard
    # passes it unchecked; the whole-workspace `--complete` listing fails it.
    assert check(pairs, half, complete=False, members={"a", "b"}) == 0, (
        "member with no binary in any listing passes under --members"
    )
    assert check(pairs, half, complete=True) == 1, (
        "the same listing fails it under --complete"
    )
    # A package whose tests sit behind a feature this listing did not build
    # (viewer's `app`): its entries are dead here and live in the listing
    # that builds it, so this listing leaves them to that one, and the
    # featured listing (which holds them) checks them under `--members`.
    plain = {"a::all": {"m::x", "m::y"}, "b": set()}
    assert check(pairs, plain, complete=True) == 1, "a featured test reads dead unfeatured"
    assert check(pairs, plain, complete=True, elsewhere=frozenset({"b"})) == 0, (
        "--elsewhere leaves the featured package to its own listing"
    )
    assert check(pairs, half, complete=True, elsewhere=frozenset({"b"})) == 0, (
        "--elsewhere holds under --complete with the package absent"
    )
    assert check(pairs, {"b": {"n::z"}}, complete=False, members={"a", "b"}) == 0, (
        "the featured listing checks the package it builds"
    )
    assert check(pairs, {"b": {"n::q"}}, complete=False, members={"a", "b"}) == 1, (
        "the featured listing still fails a dead entry of its package"
    )
    assert check(pairs, plain, complete=False, members={"a"}, elsewhere=frozenset({"b"})) == 1, (
        "--elsewhere does not excuse a package that is no member"
    )
    # The same through `run`, the wiring CI calls: a flag dropped between
    # the argument and `check` (the slow set's or an override's) reds here.
    raw = (
        '[profile.ci]\ndefault-filter = """' + text + '"""\n'
        '[[profile.ci.overrides]]\nfilter = "(binary_id(b) & test(=n::z))"\n'
    )

    def listed(s: dict[str, set[str]]) -> dict:
        return {"rust-suites": {b: {"testcases": {n: {} for n in ns}} for b, ns in s.items()}}

    assert run(["--complete"], raw, listed(plain)) == 1, "run: featured test dead under --complete"
    assert run(["--complete", "--elsewhere", "b"], raw, listed(plain)) == 0, (
        "run: --elsewhere reaches the slow set and the override under --complete"
    )
    assert run(["--elsewhere", "b", "--complete"], raw, listed(half)) == 0, (
        "run: --elsewhere before --complete keeps --complete"
    )
    assert run(["--complete"], raw, listed(half)) == 1, "run: --complete is read"
    assert run(["--members", "a,b", "--elsewhere", "b"], raw, listed({"b": {"n::q"}})) == 0, (
        "run: --elsewhere reaches --members' check"
    )
    assert run(["--members", "a,b"], raw, listed({"b": {"n::q"}})) == 1, "run: --members is read"
    for argv in (["--elsewhere", "--complete"], ["--members", "--complete"], ["--elsewhere"]):
        try:
            run(argv, raw, listed(live))
        except SystemExit:
            continue
        raise AssertionError(f"run took a flag or nothing as a package list: {argv}")
    try:
        entries(text.replace("test(=m::x)", "test(=m::x )"))
    except SystemExit:
        pass
    else:
        raise AssertionError("a space inside a name was read as a name")
    print("check-slow-set-names: selftest ok")
    return 0


def _list_arg(argv: list[str], flag: str) -> tuple[set[str] | None, list[str]]:
    """`flag`'s comma-separated value and `argv` without the pair. A value
    that is itself a flag is refused: `--elsewhere --complete` would
    otherwise read `--complete` as a package and drop it as a mode."""
    if flag not in argv:
        return None, argv
    i = argv.index(flag)
    if i + 1 >= len(argv) or not argv[i + 1] or argv[i + 1].startswith("--"):
        die(f"{flag} takes a comma-separated package list")
    return set(argv[i + 1].split(",")), argv[:i] + argv[i + 2 :]


def run(argv: list[str], raw: str, listing: dict) -> int:
    """The check over the nextest config text `raw` and a parsed listing,
    with `argv`'s flags: `main` minus its two reads, so the selftest drives
    the same wiring CI does."""
    members, argv = _list_arg(argv, "--members")
    named, argv = _list_arg(argv, "--elsewhere")
    elsewhere = frozenset(named or ())
    if "--complete" in argv and members is not None:
        die(
            "--complete and --members are mutually exclusive: a complete listing "
            "needs no member list"
        )
    if set(argv) - {"--complete"}:
        die(
            f"unknown arguments {argv}; usage: check-slow-set-names.py "
            "[--complete | --members a,b,...] [--elsewhere a,b,...] [--selftest]"
        )
    complete = "--complete" in argv
    try:
        text = ci_filter(raw)
        overrides = tomllib.loads(raw)["profile"]["ci"].get("overrides", [])
    except (SlowSetError, tomllib.TOMLDecodeError) as exc:
        die(f"{CONFIG}: {exc}")
    suites = suites_of(listing)
    rc = check(entries(text), suites, complete, members, elsewhere=elsewhere)
    # A `ci` override names its tests the same way, so a renamed one is as
    # dead as a renamed slow-set entry: each override is checked on its own.
    # An override's filter must be the slow set's exact-name shape, `(binary_id(B) & (test(=..) | ..))`; any other spelling fails here.
    for i, override in enumerate(overrides):
        rc |= check(
            entries(f"not ({override['filter']})"), suites, complete, members,
            f"ci override {i}", elsewhere,
        )
    return rc


def main(argv: list[str]) -> int:
    if "--selftest" in argv:
        return selftest()
    try:
        with open(CONFIG, encoding="utf-8") as fh:
            raw = fh.read()
    except OSError as exc:
        die(f"{CONFIG}: {exc}")
    try:
        listing = json.load(sys.stdin)
    except json.JSONDecodeError as exc:
        die(f"stdin is not a JSON listing ({exc})")
    return run(argv, raw, listing)


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
