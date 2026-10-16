#!/usr/bin/env python3
"""Every name in the slow set names a test that exists.

`.config/nextest.toml`'s `ci` profile excludes the slow set by exact name,
`binary_id(B) & (test(=suite::name) | ...)`. An exact name that matches no test
excludes nothing: a renamed slow test then runs in every per-PR gate, and the
filter still reads as if it were held back. This script reconciles the filter
against a `cargo nextest list --message-format json` listing and fails, naming
each entry, when one matches nothing.

    cargo nextest list --ignore-default-filter <scope> --message-format json \\
      | check-slow-set-names.py [--complete] [--selftest]

WHAT A LISTING CAN DECIDE. A scoped listing (the per-PR `test` job builds the
crates the diff seeds) holds some packages and not others. An entry is checked
when its binary's PACKAGE is in the listing: the binary must be there, and the
name must be one of its tests. An entry whose package the listing does not
hold is left unchecked and counted, never passed silently. `--complete` says
the listing is the whole workspace (the nightly's), so every entry is checked
and a package absent from it is itself a dead entry.

The listing is the default-feature build both gating `--profile ci` runs over
the kernel use; a name that exists only under a feature would read as dead
here, and belongs in no default-feature exclusion list anyway.

The filter's grammar is checked too: a predicate other than `binary_id(...)`
and `test(=...)` fails the run, because this script could not say what it
matches. A name listed twice under one binary fails as well.
"""

from __future__ import annotations

import json
import re
import sys
import tomllib
from collections import Counter
from typing import NoReturn

CONFIG = ".config/nextest.toml"
PREDICATE_RE = re.compile(r"([a-z_]+)\(")
NAME_RE = re.compile(r"test\(=([^)]*)\)")


def die(msg: str) -> NoReturn:
    sys.exit(f"check-slow-set-names: {msg}")


def entries(filter_text: str) -> list[tuple[str, str]]:
    """The filter's (binary-id, exact test name) pairs, in file order."""
    kinds = set(PREDICATE_RE.findall(filter_text)) - {"binary_id", "test"}
    if kinds:
        die(
            f"the ci filter uses {sorted(kinds)}, which this check cannot read; "
            "no slow-set entry was checked"
        )
    if re.search(r"test\([^=]", filter_text):
        die(
            "the ci filter holds a `test(...)` that is not an exact `test(=...)`; "
            "no slow-set entry was checked"
        )
    pairs = []
    for chunk in filter_text.split("binary_id(")[1:]:
        binary = chunk[: chunk.index(")")]
        pairs += [(binary, name) for name in NAME_RE.findall(chunk)]
    if not pairs:
        die(
            "the ci filter holds no `binary_id(...) & test(=...)` entry; nothing was checked"
        )
    return pairs


def check(
    pairs: list[tuple[str, str]], suites: dict[str, set[str]], complete: bool
) -> int:
    """Print the verdict and return the exit status."""
    packages = {b.split("::")[0] for b in suites}
    dead, unchecked = [], Counter()
    for binary, name in pairs:
        if not complete and binary.split("::")[0] not in packages:
            unchecked[binary] += 1
        elif binary not in suites:
            dead.append(f"{binary}: {name} (no test binary `{binary}` in the listing)")
        elif name not in suites[binary]:
            dead.append(f"{binary}: {name}")
    twice = [f"{b}: {n}" for (b, n), k in Counter(pairs).items() if k > 1]
    checked = len(pairs) - sum(unchecked.values())
    print(f"check-slow-set-names: {checked} of {len(pairs)} slow-set entries checked")
    for binary, k in sorted(unchecked.items()):
        print(f"  unchecked: {k} under {binary} (its package is not in this listing)")
    for line in dead:
        print(f"::error file={CONFIG}::slow-set entry matches no test: {line}")
    for line in twice:
        print(f"::error file={CONFIG}::slow-set entry listed twice: {line}")
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
    print("check-slow-set-names: selftest ok")
    return 0


def main(argv: list[str]) -> int:
    if "--selftest" in argv:
        return selftest()
    if set(argv) - {"--complete"}:
        die(
            f"unknown arguments {argv}; usage: check-slow-set-names.py [--complete] [--selftest]"
        )
    complete = "--complete" in argv
    with open(CONFIG, "rb") as fh:
        try:
            text = tomllib.load(fh)["profile"]["ci"]["default-filter"]
        except KeyError:
            die(f"{CONFIG} has no [profile.ci] default-filter")
    try:
        listing = json.load(sys.stdin)
    except json.JSONDecodeError as exc:
        die(f"stdin is not a JSON listing ({exc})")
    return check(entries(text), suites_of(listing), complete)


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
