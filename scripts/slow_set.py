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
                           or edits (`selected`): what ci.yml's slow-set
                           step runs; the selection's size goes to stderr

A slow test runs per PR on a PR that adds or edits it, and in the nightly
every night. Both readings come from `check-fast-set-times.py --new-since`
tokens. A PR that spells it on an added line is `is_new`, the rule
check-fast-set-times.py holds new fast-set tests to its stricter bar by. A PR
that edits the file it lives in (its body, its doc, a table or const beside
it, a golden that file names in a string literal) is `is_edited`. A file
"holds" a test when it defines a `fn` of the test's leaf name, or stands for a
module on the test's path, so a leaf a macro makes is still found.

What neither sees: an edit in ANOTHER file the test reads (a `tests/common`
helper or fixture list, the `src/` it exercises), and a data file the test
reaches by a name it builds (`format!`, a directory listing) rather than spells.
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


def is_edited(binary_id: str, name: str, tokens: set[str]) -> bool:
    """Whether this test lives in a file the diff edits, by the diff's
    `pkg:fn:name` / `pkg:file:module` tokens: a file defining its leaf, or
    standing for a module on its path (the binary's root for a test at the
    top)."""
    pkg, _, binary = binary_id.partition("::")
    *mods, leaf = name.split("::")
    return f"{pkg}:fn:{leaf}" in tokens or any(
        f"{pkg}:file:{m}" in tokens for m in mods or [binary or "lib"]
    )


def selected(pairs: list[tuple[str, str]], tokens: set[str]) -> list[tuple[str, str]]:
    """The entries the diff spells or edits: what ci.yml's slow-set step runs."""
    return [(b, n) for b, n in pairs if is_new(b, n, tokens) or is_edited(b, n, tokens)]


def new_expr(pairs: list[tuple[str, str]], tokens: set[str]) -> str:
    """`selected` as one filterset; `none()` when it is empty."""
    chosen = selected(pairs, tokens)
    return expr(chosen) if chosen else "none()"


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
    # An edit inside a test's body, its doc or a table beside it spells no
    # name: the file it lands in selects the test.
    assert new_expr(want, {"a:fn:y"}) == expr([("a::all", "m::y")]), "a file defining the leaf"
    assert new_expr(want, {"b:fn:y", "a:fn:z"}) == "none()", "a leaf in another package"
    assert new_expr(want, {"a:file:m"}) == expr([("a::all", "m::x"), ("a::all", "m::y")]), (
        "a file standing for a module on the path (a macro-made leaf)"
    )
    top = [("a::all", "every_file"), ("b", "lone")]
    assert selected(top, {"a:file:all", "b:file:lib"}) == top, "a binary's root holds its top tests"
    assert selected(top, {"a:file:lib", "b:file:all"}) == [], "another binary's root"
    assert not is_new("a::all", "m::y", {"a:fn:y", "a:file:m"}), (
        "an edited file does not make its tests new to the fast-set bar"
    )
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
        tokens = set(fh.read().split())
    spelled = sum(is_new(b, n, tokens) for b, n in pairs)
    chosen = len(selected(pairs, tokens))
    print(
        f"slow_set: {chosen} of {len(pairs)} slow-set tests selected "
        f"({spelled} spelled on an added line, {chosen - spelled} more in a file the diff edits)",
        file=sys.stderr,
    )
    print(new_expr(pairs, tokens))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
