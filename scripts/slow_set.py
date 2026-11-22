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
tokens. A PR that spells it on an added line is `is_new`. A PR that edits
the file it lives in (its body, its doc, a table or const beside it, a golden
that file names in a string literal) is `is_edited`. Either one is also what
check-fast-set-times.py holds a fast-set test to its 0.25 s bar by. A file
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


# `-E` is one argv string, and Linux refuses one over 128 KiB (MAX_ARG_STRLEN)
# with E2BIG before cargo starts. Every exact name of the slow set spells about
# 200 KB, so a wide selection is said coarsely, and past this size widened.
EXPR_LIMIT = 64 * 1024

# (binary, path, kind): `all` is every slow entry of the binary, `under` every
# one whose name starts `path::`, `exact` the one named `path`.
Term = tuple[str, str, str]


def _cover(binary: str, names: list[str], chosen: set[str], prefix: str) -> list[Term]:
    """The fewest terms that hold exactly the chosen `names` (all under
    `prefix`, `""` for the binary's root): a module whose slow entries are
    all chosen is one term."""
    if len(names) > 1 and all(n in chosen for n in names):
        return [(binary, prefix, "under" if prefix else "all")]
    depth = prefix.count("::") + 1 if prefix else 0
    out: list[Term] = []
    groups: dict[str, list[str]] = {}
    for n in names:
        parts = n.split("::")
        if len(parts) == depth + 1:
            out += [(binary, n, "exact")] if n in chosen else []
        else:
            groups.setdefault("::".join(parts[: depth + 1]), []).append(n)
    for head, under in groups.items():
        out += _cover(binary, under, chosen, head)
    return out


def terms(pairs: list[tuple[str, str]], chosen: list[tuple[str, str]]) -> list[Term]:
    """`chosen` as `Term`s over the slow set `pairs`, widened a binary at a
    time (the one spelling the most) to `all` until the text fits
    EXPR_LIMIT. Widening adds slow entries and never drops one."""
    by: dict[str, list[str]] = {}
    for b, n in pairs:
        by.setdefault(b, []).append(n)
    picked = {(b, n) for b, n in chosen}
    out = {
        b: _cover(b, names, {n for n in names if (b, n) in picked}, "")
        for b, names in by.items()
        if any((b, n) in picked for n in names)
    }
    size = {b: len(_render(ts)) + 3 for b, ts in out.items()}
    while sum(size.values()) > EXPR_LIMIT:
        widest = max(size, key=size.__getitem__)
        if out[widest] == [(widest, "", "all")]:
            raise SlowSetError(f"{len(out)} binaries spell over {EXPR_LIMIT} bytes even whole")
        out[widest] = [(widest, "", "all")]
        size[widest] = len(_render(out[widest])) + 3
    return [t for ts in out.values() for t in ts]


def _render(ts: list[Term]) -> str:
    by: dict[str, list[str]] = {}
    for b, path, kind in ts:
        by.setdefault(b, []).append(
            "all()" if kind == "all" else f"test(/^{path}::/)" if kind == "under" else f"test(={path})"
        )
    return " | ".join(f"(binary_id({b}) & ({' | '.join(ps)}))" for b, ps in by.items())


def covers(ts: list[Term], binary: str, name: str) -> bool:
    """Whether the rendered `ts` match the slow entry `(binary, name)`."""
    return any(
        b == binary
        and (kind == "all" or (kind == "under" and name.startswith(path + "::")) or name == path)
        for b, path, kind in ts
    )


def new_expr(pairs: list[tuple[str, str]], tokens: set[str]) -> str:
    """`selected` as one filterset, `none()` when it is empty. Its terms
    are taken inside `not default()`, the `ci` profile's slow set, so a
    coarse term reaches no fast-set test."""
    chosen = selected(pairs, tokens)
    return f"(not default() & ({_render(terms(pairs, chosen))}))" if chosen else "none()"


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
    assert selected(want, {"a:y", "b:x"}) == [("a::all", "m::y")], "a leaf, in its own package"
    assert selected(want, {"b:mod:n"}) == [("b", "n::z")], "a module on the path"
    # An edit inside a test's body, its doc or a table beside it spells no
    # name: the file it lands in selects the test.
    assert selected(want, {"a:fn:y"}) == [("a::all", "m::y")], "a file defining the leaf"
    assert selected(want, {"b:fn:y", "a:fn:z"}) == [], "a leaf in another package"
    assert selected(want, {"a:file:m"}) == want[:2], (
        "a file standing for a module on the path (a macro-made leaf)"
    )
    assert new_expr(want, {"a:fn:y"}) == "(not default() & ((binary_id(a::all) & (test(=m::y)))))"
    assert new_expr(want, {"a:file:m"}) == "(not default() & ((binary_id(a::all) & (all()))))", (
        "every slow entry of a binary is one term"
    )
    _selftest_compact()
    top = [("a::all", "every_file"), ("b", "lone")]
    assert selected(top, {"a:file:all", "b:file:lib"}) == top, "a binary's root holds its top tests"
    assert selected(top, {"a:file:lib", "b:file:all"}) == [], "another binary's root"
    assert not is_new("a::all", "m::y", {"a:fn:y", "a:file:m"}), (
        "an edited file does not make its tests new to the fast-set bar"
    )
    print("slow_set: selftest ok")


def _selftest_compact() -> None:
    """The expression holds exactly the selection while it fits, a superset
    of it past EXPR_LIMIT, and always fits: 4800 entries whose exact names
    alone spell about 600 KB."""
    pad = "_and_its_long_descriptive_name_" * 2
    pairs = [
        (f"pkg{b}::all", f"suite{m}::case{t}{pad}")
        for b in range(30)
        for m in range(40)
        for t in range(4)
    ] + [("pkg0::all", "top_level"), ("pkg0::all", "suite0::deeper::case")]
    assert len(expr(pairs)) > 4 * EXPR_LIMIT, len(expr(pairs))

    def check(chosen: list[tuple[str, str]], exact: bool, why: str) -> None:
        ts = terms(pairs, chosen)
        assert len(_render(ts)) <= EXPR_LIMIT, f"{why}: {len(_render(ts))} bytes"
        by: dict[str, list[Term]] = {}
        for t in ts:
            by.setdefault(t[0], []).append(t)
        got = [p for p in pairs if covers(by.get(p[0], []), *p)]
        assert set(chosen) <= set(got), f"{why}: a chosen entry is dropped"
        assert not exact or got == chosen, f"{why}: {len(got)} matched for {len(chosen)} chosen"

    check(pairs, True, "every entry")
    whole_suites = [p for p in pairs if p[1].startswith(("suite1::", "suite2::")) or p[1] == "top_level"]
    check(whole_suites, True, "whole modules and a top-level test")
    check([("pkg3::all", f"suite5::case1{pad}"), ("pkg0::all", "suite0::deeper::case")], True, "two lone tests")
    check(pairs[::2], False, "every other entry, widened")
    try:
        with open(CONFIG, encoding="utf-8") as fh:
            real = entries(ci_filter(fh.read()))
    except OSError:
        print(f"slow_set: selftest: no {CONFIG} here, the tree's own slow set is not sized")
        return
    ts = terms(real, real)
    assert len(_render(ts)) <= EXPR_LIMIT and all(covers(ts, *p) for p in real), (
        f"the whole of {CONFIG}'s slow set selected: {len(_render(ts))} bytes"
    )


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
