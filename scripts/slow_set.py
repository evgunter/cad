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
    slow_set.py --new-expr <new-tests.txt>
                           the same, narrowed to the entries the diff adds
                           or edits: what ci.yml's slow-set step runs

A slow test runs per PR only on a PR whose added lines spell it, which is the
PR that adds it or edits it; every other night it runs in the nightly. What
counts as spelling it is `is_new`, the rule check-fast-set-times.py holds new
fast-set tests to its stricter bar by, read from the same
`check-fast-set-times.py --new-since` tokens.
"""

from __future__ import annotations

import re
import sys
import tomllib

CONFIG = ".config/nextest.toml"
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


def is_new(binary_id: str, name: str, tokens: set[str]) -> bool:
    """Whether the diff's `pkg:ident` / `pkg:mod:name` tokens spell this
    test: its leaf name, or a module on its path the diff declares."""
    pkg = binary_id.split("::")[0]
    *mods, leaf = name.split("::")
    return f"{pkg}:{leaf}" in tokens or any(f"{pkg}:mod:{m}" in tokens for m in mods)


def new_expr(pairs: list[tuple[str, str]], tokens: set[str]) -> str:
    """The entries `tokens` spell, as one filterset; `none()` when none is."""
    spelled = [(b, n) for b, n in pairs if is_new(b, n, tokens)]
    return expr(spelled) if spelled else "none()"


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
    assert new_expr(want, set()) == "none()"
    assert new_expr(want, {"a:y", "b:x"}) == expr([("a::all", "m::y")]), "a leaf, in its own package"
    assert new_expr(want, {"b:mod:n"}) == expr([("b", "n::z")]), "a module on the path"
    print("slow_set: selftest ok")


def main(argv: list[str]) -> int:
    if argv == ["--selftest"]:
        selftest()
        return 0
    if argv != ["--expr"] and not (len(argv) == 2 and argv[0] == "--new-expr"):
        sys.exit("usage: slow_set.py --expr | --new-expr <new-tests.txt> | --selftest")
    try:
        with open(CONFIG, encoding="utf-8") as fh:
            pairs = entries(ci_filter(fh.read()))
    except (OSError, SlowSetError) as exc:
        sys.exit(f"slow_set: {CONFIG}: {exc}")
    if argv == ["--expr"]:
        print(expr(pairs))
        return 0
    with open(argv[1], encoding="utf-8") as fh:
        print(new_expr(pairs, set(fh.read().split())))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
