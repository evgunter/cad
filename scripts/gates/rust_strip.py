"""The Python gates' Rust lexer: where the comments and the string and
char literals of a Rust text are, and the two views built from that.

`spans(text)` yields `(kind, start, end)` for each comment (`line`,
`block`), string literal (`string`) and char literal (`char`), in order.
A `/* … */` comment nests to any depth; a string is `"…"` with `\\`
escapes or a raw `r#…"…"#…` without them, either one with its `b` or
`c` prefix; a quote that opens no char literal (a lifetime, a label) is
code. A prefix counts only where nothing identifier-shaped precedes it.
An unterminated comment or literal runs to the end of the text, which
`rustc` refuses.

`strip(text, literal_newlines)` drops each comment (a block comment
leaves the newlines it spanned), writes each string as `""` — followed
by the newlines it spanned when `literal_newlines` — and each char
literal as `' '`. `blank(text)` writes every character of every span
but a newline as a space, so an offset still names its line and column.

The scan jumps from one place a span can open to the next rather than
walking every character.

`python3 rust_strip.py --selftest` checks both views against a table;
`scripts/run-gates.sh --selftest` runs it.
"""
import re
import sys

_NEXT = re.compile(
    r"""//|/\*"""
    r"""|(?<!\w)[bc]?r(#*)\""""
    r"""|(?<!\w)[bc]"|\""""
    r"""|'(?:\\(?:x[0-9a-fA-F]{2}|u\{[0-9a-fA-F]+\}|.)|[^'\\\n])'"""
)
_COMMENT = re.compile(r"/\*|\*/")
_STRING_END = re.compile(r'\\.|"', re.S)
_NOT_NEWLINE = re.compile(r"[^\n]")


def spans(text):
    i, n = 0, len(text)
    while True:
        m = _NEXT.search(text, i)
        if not m:
            return
        k, tok = m.start(), m.group()
        if tok == "//":
            j = text.find("\n", k)
            j, kind = (n if j < 0 else j), "line"
        elif tok == "/*":
            depth, j = 1, k + 2
            while depth:
                t = _COMMENT.search(text, j)
                if not t:
                    j = n
                    break
                depth += 1 if t.group() == "/*" else -1
                j = t.end()
            kind = "block"
        elif tok.startswith("'"):
            j, kind = m.end(), "char"
        elif m.group(1) is not None:
            close = '"' + m.group(1)
            j = text.find(close, m.end())
            j, kind = (n if j < 0 else j + len(close)), "string"
        else:
            t = _STRING_END.search(text, m.end())
            while t and t.group() != '"':
                t = _STRING_END.search(text, t.end())
            j, kind = (t.end() if t else n), "string"
        yield kind, k, j
        i = j


def strip(text, literal_newlines):
    out, i = [], 0
    for kind, k, j in spans(text):
        out.append(text[i:k])
        if kind == "block":
            out.append("\n" * text.count("\n", k, j))
        elif kind == "string":
            out.append('""' + ("\n" * text.count("\n", k, j) if literal_newlines else ""))
        elif kind == "char":
            out.append("' '")
        i = j
    out.append(text[i:])
    return "".join(out)


def blank(text):
    out, i = [], 0
    for _, k, j in spans(text):
        out.append(text[i:k])
        out.append(_NOT_NEWLINE.sub(" ", text[k:j]))
        i = j
    out.append(text[i:])
    return "".join(out)


# (text, strip(text, False), strip(text, True) where it differs). Each
# blank(text) is checked to keep every offset and newline and to leave
# nothing strip would take out.
_ROWS = [
    ('let p = br"\\"; x.atan2(y); let q = "";', 'let p = ""; x.atan2(y); let q = "";', None),
    ('let p = cr#"a"b"#; f();', 'let p = ""; f();', None),
    ('let p = b"\\""; g();', 'let p = ""; g();', None),
    ('let p = c"x"; g();', 'let p = ""; g();', None),
    ('r#######"a"#######; h()', '""; h()', None),
    ('xbr"q" k', 'xbr"" k', None),
    ('_r"x" k', '_r"" k', None),
    ('a "b\nc" d', 'a "" d', 'a ""\n d'),
    ('/* a /* b */ c */ d', ' d', None),
    ('/*/ x */ y', ' y', None),
    ('/**/x', 'x', None),
    ('/* a\n b */ c', '\n c', None),
    ('x // c { \ny', 'x \ny', None),
    ("'\\'' x", "' ' x", None),
    ("'\\x41' '\\u{1F}' 'ab' 'a'", "' ' ' ' 'ab' 'a'", None),
    ("fn f<'a>(s: &'a str) {}", "fn f<'a>(s: &'a str) {}", None),
    ('"a\\', '""', None),
    ('"abc', '""', None),
    ('a/b', 'a/b', None),
    ('/* open', '', None),
]


def _selftest():
    bad = []
    for text, want, want_nl in _ROWS:
        for nl, w in ((False, want), (True, want if want_nl is None else want_nl)):
            got = strip(text, nl)
            if got != w:
                bad.append(f"strip({text!r}, {nl}) = {got!r}, wanted {w!r}")
        b = blank(text)
        if len(b) != len(text) or [i for i, c in enumerate(b) if c == "\n"] != [i for i, c in enumerate(text) if c == "\n"]:
            bad.append(f"blank({text!r}) = {b!r} moved a character or a newline")
        elif list(spans(b)):
            bad.append(f"blank({text!r}) = {b!r} left a span behind")
    for line in bad:
        print(line, file=sys.stderr)
    if bad:
        sys.exit(1)
    print(f"rust_strip selftest OK: {len(_ROWS)} rows, both views")


if __name__ == "__main__":
    if sys.argv[1:] != ["--selftest"]:
        sys.exit("usage: rust_strip.py --selftest")
    _selftest()
