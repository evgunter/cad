"""The rung half of escalation-question-marks.sh: a `?` on a band reading
inside the reader of one of the ladder's entry points.

Arguments: the entry points' names, read by the gate off
`geom_core::ladder`'s `pub fn climb*` (one home: a combinator added there
joins this count by construction). Records on stdin, `file:LINE:text`,
the code view (comments and literals blanked). Calls inside the ladder's
own home are its combinators delegating to one another, not readers.

For each call, a rung's reader is:
- every closure in its arguments, its parameters and return type skipped
  whole (a comma in a generic type there does not end the argument);
- every argument naming a `let` closure bound earlier in the `fn`;
- every such closure a reader calls by name, transitively;
- a reader that names a parameter of the enclosing `fn`: each closure (or
  `let` closure) its direct callers pass for that parameter, read in the
  caller's `fn` and keyed by it. One level only.

A line of a reader holding `)?` and a `decide*(` call, the word `band`,
or the name of a `let` closure that reads the band (transitively) prints
as `HIT rung:<file>:<fn>`. The last line is `DONE <calls>`: a census
without it did not finish.
"""
import bisect
import re
import sys

FN = re.compile(r"(?<![A-Za-z0-9_])fn\s+([A-Za-z_][A-Za-z0-9_]*)")
LET = re.compile(r"let\s+(?:mut\s+)?([a-z_][a-z0-9_]*)\s*=\s*(?:move\s+)?\|")
NAMED = re.compile(r"(?<![A-Za-z0-9_.:])([a-z_][a-z0-9_]*)\s*\(")
DECIDE = re.compile(r"decide[a-z_]*\(")
CLOSURE = re.compile(r"\s*(?:move\s+)?\|")
HOME = "crates/geom-core/src/ladder.rs"


def end_of(text, i, stop, angles=False):
    """Where the expression at `i` ends: `stop` at its own depth, or the
    bracket closing around it. A closure's parameters and return type are
    skipped whole, so a comma in a generic type there does not end it;
    `angles` counts `<>` as brackets too (a signature, where no `<` is a
    comparison)."""
    depth = 0
    m = CLOSURE.match(text, i)
    if m and stop == ",":
        j = text.index("|", m.start())
        k = j + 1
        while k < len(text) and text[k] != "|":
            k += 1
        k += 1
        if re.match(r"\s*->", text[k:]):
            # The body is the block after the return type.
            k = text.index("{", k)
        i = k
    open_, close = ("([{<", ")]}>") if angles else ("([{", ")]}")
    while i < len(text):
        c = text[i]
        if c in open_:
            depth += 1
        elif c in close:
            if angles and c == ">" and text[i - 1] == "-":
                pass
            elif depth == 0:
                return i
            else:
                depth -= 1
        elif depth == 0 and c == stop:
            return i
        i += 1
    return i


def args(text, i):
    """The argument texts of a call whose `(` ends at `i`, and where the
    call closes."""
    out = []
    while i < len(text) and text[i] != ")":
        j = end_of(text, i, ",")
        out.append(text[i:j])
        if j >= len(text) or text[j] != ",":
            return out, j
        i = j + 1
    return out, i


BAND = re.compile(r"(?<![A-Za-z0-9_])band(?![A-Za-z0-9_])")
WORD = re.compile(r"(?<![A-Za-z0-9_.:])([a-z_][a-z0-9_]*)(?![A-Za-z0-9_])")


def reads_band(text, readers):
    return bool(
        DECIDE.search(text)
        or BAND.search(text)
        or (readers and not readers.isdisjoint(WORD.findall(text)))
    )


def band_readers(lets):
    words = {n: set(WORD.findall(b)) for n, b in lets.items()}
    readers = {n for n, b in lets.items() if DECIDE.search(b) or BAND.search(b)}
    grew = True
    while grew:
        grew = False
        for name, ws in words.items():
            if name not in readers and not readers.isdisjoint(ws):
                readers.add(name)
                grew = True
    return readers


class Tree:
    def __init__(self, files, entries):
        self.files = files
        self.fns = {p: list(FN.finditer(t)) for p, t in files.items()}
        self.starts = {p: [m.start() for m in ms] for p, ms in self.fns.items()}
        self.paths = {id(t): p for p, t in files.items()}
        self.memo = {}
        self.call = re.compile(r"(?<![A-Za-z0-9_])(" + "|".join(sorted(entries, key=len, reverse=True)) + r")\s*\(")

    def enclosing(self, text, pos):
        p = self.paths[id(text)]
        i = bisect.bisect_left(self.starts[p], pos)
        return self.fns[p][i - 1] if i else None

    def params(self, text, fn):
        i = text.index("(", fn.end())
        j = end_of(text, i + 1, ")", angles=True)
        names = []
        for p in re.split(r",(?![^<>()]*[>)])", text[i + 1 : j]):
            p = p.strip()
            if not p:
                continue
            m = re.match(r"(?:mut\s+)?&?\s*(?:mut\s+)?([a-z_][a-z0-9_]*)\s*:?", p)
            names.append(m.group(1) if m else None)
        return names

    def lets(self, text, fn, until):
        out = {}
        for lm in LET.finditer(text, fn.start(), until):
            s = lm.end() - 1
            out[lm.group(1)] = (lm.start(), text[s : end_of(text, s, ";")])
        return out

    def readers(self, path, text, fn, segs, depth, start, limit):
        """(key, line) for each counted line of the readers `segs` read in
        `fn` of `path`, following let closures, and fn parameters to the
        closures callers pass."""
        lets = self.lets(text, fn, limit)
        every = {n: b for n, (_, b) in lets.items()}
        reads = band_readers(every)
        params = self.params(text, fn)
        seen, todo, body, hits = set(), list(segs), [], []
        while todo:
            seg = todo.pop()
            body.append(seg)
            for name in NAMED.findall(seg) + [s.strip(" &\n\t") for s in segs if re.fullmatch(r"\s*&?\s*[a-z_][a-z0-9_]*\s*", s)]:
                if name in seen:
                    continue
                if name in lets and lets[name][0] < start:
                    seen.add(name)
                    todo.append(lets[name][1])
                elif name in params and depth < 1:
                    seen.add(name)
                    hits += self.callers(fn.group(1), params.index(name), params[0] == "self", depth + 1)
        key = f"rung:{path}:{fn.group(1)}"
        for seg in body:
            for line in seg.split("\n"):
                if ")?" in line and reads_band(line, reads):
                    hits.append((key, line.strip()))
        return hits

    def callers(self, name, index, method, depth):
        if (name, index, method) not in self.memo:
            self.memo[(name, index, method)] = self._callers(name, index, method, depth)
        return self.memo[(name, index, method)]

    def _callers(self, name, index, method, depth):
        hits = []
        pat = re.compile(rf"(?<![A-Za-z0-9_])(\.)?{name}\s*(?:::<[^()]*>)?\(")
        for path, text in self.files.items():
            for m in pat.finditer(text):
                if text[max(0, m.start() - 3) : m.start()] == "fn ":
                    continue
                fn = self.enclosing(text, m.start())
                if fn is None:
                    continue
                got, _ = args(text, m.end())
                k = index - 1 if (method and m.group(1)) else index
                if not (0 <= k < len(got)):
                    continue
                seg = got[k]
                if CLOSURE.match(seg) or re.fullmatch(r"\s*&?\s*[a-z_][a-z0-9_]*\s*", seg):
                    _, end = args(text, m.end())
                    hits += self.readers(path, text, fn, [seg], depth, m.start(), end)
        return hits

    def climbs(self):
        n = 0
        for path, text in self.files.items():
            if path == HOME:
                continue
            for m in self.call.finditer(text):
                if text[max(0, m.start() - 3) : m.start()] == "fn ":
                    continue
                fn = self.enclosing(text, m.start())
                if fn is None:
                    continue
                n += 1
                got, end = args(text, m.end())
                segs = [a for a in got if CLOSURE.match(a) or re.fullmatch(r"\s*&?\s*[a-z_][a-z0-9_]*\s*", a)]
                for key, line in self.readers(path, text, fn, segs, 0, m.start(), end):
                    yield key, line
        self.n = n


def main():
    entries = [a for a in sys.argv[1:] if a != "--lines"]
    if not entries:
        sys.exit("escalation_rungs.py: no ladder entry points given")
    files = {}
    for rec in sys.stdin:
        m = re.match(r"([^:]+):\d+:(.*)$", rec.rstrip("\n"))
        if m:
            files.setdefault(m.group(1), []).append(m.group(2))
    tree = Tree({p: "\n".join(ls) for p, ls in files.items()}, entries)
    for key, line in tree.climbs():
        print(f"HIT {key}\t{line}" if "--lines" in sys.argv else f"HIT {key}")
    print(f"DONE {tree.n}")


main()
