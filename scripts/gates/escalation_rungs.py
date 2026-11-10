"""The rung half of escalation-question-marks.sh: a `?` on a band reading
inside the reader of a `climb`, `climb_evidence` or `climb_agreeing`.

Records on stdin, `file:LINE:text`, the code view (comments and literals
blanked). For each call, the rung's reader is every closure in its
argument list, every argument naming a `let` closure bound earlier in
the enclosing `fn`, and, transitively, every such `let` closure a reader
calls by name. A line of a reader holding `)?` and either a `decide*(`
call or the word `band` (the register's own two matchers) prints as
`HIT rung:<file>:<fn>`, keyed by the nearest `fn` before the call. The
last line is `DONE <calls>`: a census without it did not finish.
"""
import re
import sys

CALL = re.compile(r"(?<![A-Za-z0-9_])(climb_agreeing|climb_evidence|climb)\s*\(")
FN = re.compile(r"(?<![A-Za-z0-9_])fn\s+([A-Za-z_][A-Za-z0-9_]*)")
LET = re.compile(r"let\s+(?:mut\s+)?([a-z_][a-z0-9_]*)\s*=\s*(?:move\s+)?\|")
NAMED = re.compile(r"(?<![A-Za-z0-9_.:])([a-z_][a-z0-9_]*)\s*\(")
DECIDE = re.compile(r"decide[a-z_]*\(")


def end_of(text, i, let):
    """Where the expression at `i` ends: the `,` (an argument) or `;` (a
    `let`) at its own depth, or the bracket that closes around it."""
    depth = 0
    stop = ";" if let else ","
    while i < len(text):
        c = text[i]
        if c in "([{":
            depth += 1
        elif c in ")]}":
            if depth == 0:
                return i
            depth -= 1
        elif depth == 0 and c == stop:
            return i
        i += 1
    return i


def reads_band(text, readers):
    """The register's two matchers (a `decide*(` call, the word `band`),
    or the name of a `let` closure that reads the band."""
    return bool(
        DECIDE.search(text)
        or re.search(r"(?<![A-Za-z0-9_])band(?![A-Za-z0-9_])", text)
        or any(re.search(rf"(?<![A-Za-z0-9_.:]){n}(?![A-Za-z0-9_])", text) for n in readers)
    )


def band_readers(lets):
    """The `let` closures whose bodies read the band, transitively."""
    readers = set()
    grew = True
    while grew:
        grew = False
        for name, body in lets.items():
            if name not in readers and reads_band(body, readers):
                readers.add(name)
                grew = True
    return readers


def readings(path, text):
    calls = 0
    for m in CALL.finditer(text):
        if text[max(0, m.start() - 3) : m.start()] == "fn ":
            continue
        fns = list(FN.finditer(text, 0, m.start()))
        if not fns:
            continue
        calls += 1
        fn = fns[-1]
        readers, named, i = [], [], m.end()
        while i < len(text) and text[i] != ")":
            j = end_of(text, i, False)
            arg = text[i:j]
            if re.match(r"\s*(move\s+)?\|", arg):
                readers.append(arg)
            elif re.fullmatch(r"\s*&?\s*[a-z_][a-z0-9_]*\s*", arg):
                named.append(arg.strip(" &\n\t"))
            i = j + 1 if j < len(text) and text[j] == "," else j
        lets, every = {}, {}
        for lm in LET.finditer(text, fn.start(), i):
            s = lm.end() - 1
            every[lm.group(1)] = text[s : end_of(text, s, True)]
            if lm.start() < m.start():
                lets[lm.group(1)] = every[lm.group(1)]
        reads = band_readers(every)
        seen = {n for n in named if n in lets}
        readers += [lets[n] for n in seen]
        body = []
        while readers:
            seg = readers.pop()
            body.append(seg)
            for name in NAMED.findall(seg):
                if name in lets and name not in seen:
                    seen.add(name)
                    readers.append(lets[name])
        for seg in body:
            for line in seg.split("\n"):
                if ")?" in line and reads_band(line, reads):
                    print(f"HIT rung:{path}:{fn.group(1)}")
    return calls


def main():
    calls, cur, buf = 0, None, []
    for rec in sys.stdin:
        m = re.match(r"([^:]+):\d+:(.*)$", rec.rstrip("\n"))
        if not m:
            continue
        if m.group(1) != cur:
            if cur is not None:
                calls += readings(cur, "\n".join(buf))
            cur, buf = m.group(1), []
        buf.append(m.group(2))
    if cur is not None:
        calls += readings(cur, "\n".join(buf))
    print(f"DONE {calls}")


main()
