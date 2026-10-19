#!/usr/bin/env python3
"""The ONE reader of the slow set: `.config/nextest.toml`'s `ci` profile
`default-filter`, `not ( <entry> | ... )`, each entry
`(binary_id(<pkg>[::<bin>]) & <test(=name) or a parenthesised | of them>)`.

`scripts/ci-filter.py` (what a slow-set edit seeds),
`scripts/check-slow-set-names.py` (every name exists) and ci.yml's slow-set
step (what to run) all read it through `entries`, so they cannot disagree on
what it names.

Whitespace is allowed BETWEEN tokens only. nextest keeps a matcher's text
literally, so `test(=a::b )` names no test; a matcher text that is not a plain
binary id or test path is refused here (`SlowSetError`), never trimmed into
one.

    slow_set.py --expr     print the slow set as one nextest filterset
                           (the filter without its `not`), read from
                           .config/nextest.toml
    slow_set.py --per-pr-expr
                           the same, less .config/nightly-only.txt: what
                           ci.yml's seeded slow-set step runs
    slow_set.py --check    every nightly-only line is a slow-set entry, once

`.config/nightly-only.txt` is the part of the slow set no PR runs; its header
says what earns a place there. This is its one reader too.
"""

from __future__ import annotations

import re
import sys
import tomllib

CONFIG = ".config/nextest.toml"
NIGHTLY_ONLY = ".config/nightly-only.txt"
_BINARY_RE = re.compile(r"[A-Za-z0-9_-]+(?:::[A-Za-z0-9_-]+)?")
_NAME_RE = re.compile(r"[A-Za-z0-9_]+(?:::[A-Za-z0-9_]+)*")


class SlowSetError(ValueError):
    pass


def ci_filter(toml_text: str) -> str:
    """The `ci` profile's `default-filter` out of a nextest config's text."""
    try:
        value = tomllib.loads(toml_text)["profile"]["ci"]["default-filter"]
    except (tomllib.TOMLDecodeError, KeyError, TypeError) as exc:
        raise SlowSetError(f"no readable [profile.ci] default-filter ({exc!r})") from exc
    if not isinstance(value, str):
        raise SlowSetError("[profile.ci] default-filter is not a string")
    return value


class _Reader:
    def __init__(self, text: str) -> None:
        self.s, self.i = text, 0

    def ws(self) -> None:
        while self.i < len(self.s) and self.s[self.i].isspace():
            self.i += 1

    def peek(self, tok: str) -> bool:
        self.ws()
        return self.s.startswith(tok, self.i)

    def take(self, tok: str) -> None:
        if not self.peek(tok):
            raise SlowSetError(f"expected {tok!r} at {self.s[self.i:self.i + 60]!r}")
        self.i += len(tok)

    def matcher(self, head: str, shape: re.Pattern[str]) -> str:
        """`head` then the raw text up to `)`, which must be exactly `shape`."""
        self.take(head)
        end = self.s.find(")", self.i)
        if end < 0:
            raise SlowSetError(f"unclosed {head!r}")
        text = self.s[self.i:end]
        if not shape.fullmatch(text):
            raise SlowSetError(f"{head}{text}) is not a plain {shape.pattern!r}")
        self.i = end + 1
        return text


def entries(filter_text: str) -> list[tuple[str, str]]:
    """The `(binary id, exact test name)` pairs, in file order."""
    r = _Reader(filter_text)
    r.take("not")
    r.take("(")
    pairs: list[tuple[str, str]] = []
    while True:
        r.take("(")
        binary = r.matcher("binary_id(", _BINARY_RE)
        r.take("&")
        if r.peek("("):
            r.take("(")
            while True:
                pairs.append((binary, r.matcher("test(=", _NAME_RE)))
                if r.peek(")"):
                    break
                r.take("|")
            r.take(")")
        else:
            pairs.append((binary, r.matcher("test(=", _NAME_RE)))
        r.take(")")
        if r.peek(")"):
            break
        r.take("|")
    r.take(")")
    r.ws()
    if r.i != len(r.s):
        raise SlowSetError(f"trailing text {r.s[r.i:r.i + 60]!r}")
    return pairs


def expr(pairs: list[tuple[str, str]]) -> str:
    """The slow set as one filterset: every pair, grouped by binary."""
    by: dict[str, list[str]] = {}
    for binary, name in pairs:
        by.setdefault(binary, []).append(name)
    terms = (
        f"(binary_id({b}) & ({' | '.join(f'test(={n})' for n in names)}))"
        for b, names in by.items()
    )
    return "(" + " | ".join(terms) + ")"


def nightly_only(text: str) -> list[tuple[str, str]]:
    """The `(binary id, exact test name)` lines of a nightly-only list, in
    file order. `#` starts a comment line; blank lines are skipped."""
    pairs: list[tuple[str, str]] = []
    for n, line in enumerate(text.splitlines(), 1):
        if not line.strip() or line.startswith("#"):
            continue
        parts = line.split(" ")
        if len(parts) != 2 or not _BINARY_RE.fullmatch(parts[0]) or not _NAME_RE.fullmatch(parts[1]):
            raise SlowSetError(f"line {n}: not `<binary id> <test name>`: {line!r}")
        pairs.append((parts[0], parts[1]))
    return pairs


def check(slow: list[tuple[str, str]], nightly: list[tuple[str, str]]) -> list[str]:
    """What is wrong with a nightly-only list against the slow set: an entry
    the slow set does not hold (the default-eps step would still run it), or
    one listed twice."""
    held = set(slow)
    errors = [f"{b} {n}: not in the slow set" for b, n in nightly if (b, n) not in held]
    seen: set[tuple[str, str]] = set()
    for pair in nightly:
        if pair in seen:
            errors.append(f"{pair[0]} {pair[1]}: listed twice")
        seen.add(pair)
    return errors


def selftest() -> None:
    text = "not (\n  (binary_id(a::all) & (test(=m::x) | test(=m::y))) |\n  (binary_id(b) & test(=n::z))\n)\n"
    want = [("a::all", "m::x"), ("a::all", "m::y"), ("b", "n::z")]
    assert entries(text) == want, entries(text)
    assert entries(expr(want).join(["not ", ""])) == want, "expr round-trips"
    for bad, why in (
        (text.replace("test(=m::x)", "test(=m::x )"), "space inside a name"),
        (text.replace("binary_id(a::all)", "binary_id(a:: all)"), "space inside a binary id"),
        (text.replace("test(=m::x)", "test(/m::x/)"), "a non-exact matcher"),
        (text.replace(") |\n  (binary_id(b)", ") &\n  (binary_id(b)"), "a non-| join"),
        (text.replace("not (", "("), "no `not`"),
        (text + "| all()", "trailing text"),
    ):
        try:
            entries(bad)
        except SlowSetError:
            continue
        raise SystemExit(f"slow_set selftest FAILED: {why} was read")
    listed = nightly_only("# header\n\na::all m::y\nb n::z\n")
    assert listed == [("a::all", "m::y"), ("b", "n::z")], listed
    assert check(want, listed) == [], check(want, listed)
    assert check(want, [("a::all", "m::q")]) == ["a::all m::q: not in the slow set"]
    assert check(want, listed + listed[:1]) == ["a::all m::y: listed twice"]
    for bad in ("a::all  m::y", "a::all m::y extra", "a::all"):
        try:
            nightly_only(bad)
        except SlowSetError:
            continue
        raise SystemExit(f"slow_set selftest FAILED: nightly-only line {bad!r} was read")
    print("slow_set: selftest ok")


def main(argv: list[str]) -> int:
    if argv == ["--selftest"]:
        selftest()
        return 0
    if argv not in (["--expr"], ["--per-pr-expr"], ["--check"]):
        sys.exit("usage: slow_set.py --expr | --per-pr-expr | --check | --selftest")
    try:
        with open(CONFIG, encoding="utf-8") as fh:
            slow = entries(ci_filter(fh.read()))
    except (OSError, SlowSetError) as exc:
        sys.exit(f"slow_set: {CONFIG}: {exc}")
    if argv == ["--expr"]:
        print(expr(slow))
        return 0
    try:
        with open(NIGHTLY_ONLY, encoding="utf-8") as fh:
            nightly = nightly_only(fh.read())
    except (OSError, SlowSetError) as exc:
        sys.exit(f"slow_set: {NIGHTLY_ONLY}: {exc}")
    if argv == ["--check"]:
        errors = check(slow, nightly)
        for e in errors:
            print(f"::error file={NIGHTLY_ONLY}::{e}")
        if errors:
            return 1
        print(f"slow_set: {len(nightly)} of the slow set's {len(slow)} entries are nightly-only")
        return 0
    skip = set(nightly)
    per_pr = [pair for pair in slow if pair not in skip]
    # An empty filterset is not one nextest reads; `none()` runs nothing.
    print(expr(per_pr) if per_pr else "none()")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
