"""The Python gates' Rust lexer: comments and string and char literals
out, the code and its offsets' line structure kept.

`strip(text, literal_newlines)` returns `text` with every `//` comment
removed up to (not including) its newline, every `/* … */` comment
(nested to any depth) replaced by the newlines it spanned, every string
literal (`"…"`, `r#"…"#`, the `b` forms) replaced by `""` — followed by
the newlines it spanned when `literal_newlines` is true — and every char
literal by `' '`. A quote that opens no char literal (a lifetime or a
label) is code.

It jumps from one character that can open a comment or a literal to the
next rather than walking every character: the gates that read the whole
tree through it spent most of their time in the walk.

THE SAME KNOWN SHAPES AS THE WALK IT REPLACED. A raw string is
recognised with at most six `#`s (`r#######"` is code). An unterminated
comment or literal runs to the end of the text, which `rustc` refuses.
"""
import re

_NEXT = re.compile(r'[/"\']|(?<!\w)r#{0,6}"')
_COMMENT = re.compile(r"/\*|\*/")
_STRING_END = re.compile(r'\\.|"', re.S)
_CHAR = re.compile(r"'(?:\\(?:x[0-9a-fA-F]{2}|u\{[0-9a-fA-F]+\}|.)|[^'\\\n])'")


def strip(text, literal_newlines):
    out, i, n = [], 0, len(text)
    while i < n:
        m = _NEXT.search(text, i)
        if not m:
            out.append(text[i:])
            break
        out.append(text[i:m.start()])
        i = m.start()
        c, two = text[i], text[i:i + 2]
        if two == "//":
            j = text.find("\n", i)
            i = n if j < 0 else j
        elif two == "/*":
            depth, j = 1, i + 2
            while depth:
                t = _COMMENT.search(text, j)
                if not t:
                    j = n
                    break
                depth += 1 if t.group() == "/*" else -1
                j = t.end()
            out.append("\n" * text.count("\n", i, j))
            i = j
        elif c == "r":
            close = '"' + "#" * (m.end() - i - 2)
            j = text.find(close, m.end())
            j = n if j < 0 else j + len(close)
            out.append('""' + ("\n" * text.count("\n", i, j) if literal_newlines else ""))
            i = j
        elif c == '"':
            j = i + 1
            while True:
                t = _STRING_END.search(text, j)
                if not t:
                    j = n
                    break
                if t.group() == '"':
                    j = t.start()
                    break
                j = t.end()
            out.append('""' + ("\n" * text.count("\n", i, j) if literal_newlines else ""))
            i = j + 1
        elif c == "'":
            t = _CHAR.match(text, i)
            if t:
                out.append("' '")
                i = t.end()
            else:
                out.append(c)
                i += 1
        else:
            out.append(c)
            i += 1
    return "".join(out)
