#!/usr/bin/env python3
"""The measured-claim sweep (#651 / #663 / #667 / #681), as actually run.

  measured-claim-sweep.py 'crates/*/src/**/*.rs'
  measured-claim-sweep.py 'Cargo.toml' 'crates/*/Cargo.toml' --marker '#' --ext .toml
  measured-claim-sweep.py 'crates/pncad-py/**/*.py' --marker '#' --ext .py --docstrings
  measured-claim-sweep.py 'docs/guide/*.md' --marker '' --ext .md --paragraphs

Prints every comment BLOCK carrying a provenance word AND a numeral: the
population a reviewer then triages against reviewer-style-lane.md's Q6
(guarded / scheduled register / unguardable-with-a-written-reason).

NOT A GATE, and deliberately not wired into one. Its output is a
population to read, not a verdict — the over-match rate is high by design
and the triage is the work. It is committed because issue 681's own
argument is that a lane which re-derives the instrument from prose
inherits a NEW blind spot, and two re-derivations have already shown it:
#667's own "478 hits unrestricted" figure reproduced under no variant and
was withdrawn, and the `.md` recipe #681's table describes printed one
block per FILE until #826 read it closely.

WHAT THIS COPY ADDS TO THE ONE IN #681's BODY, all of it disclosure-worthy:

  * --paragraphs. `line.find("")` returns 0 for every line, so `--marker
    ''` alone never terminates a block and prints one block per FILE.
    Markdown legs need blank-line blocking, which this flag is (#826).
  * --docstrings. #681 says the Python half "needs its own extraction"
    and does not supply one. Every `ast` string-literal expression
    statement is emitted as a block alongside the `#` blocks. On
    crates/pncad-py it takes 9 blocks to 28, so the docstring half is
    two thirds of that surface's population.
  * --show, to print block text rather than only locations.

THE VOCABULARY IS #681's, UNMODIFIED, INCLUDING ITS HOLES. Two are known
and neither is fixable by a better regex: (1) the bare-number arm is TIME
UNITS ONLY, and does not even spell `min` (#826), so a measurement in
bytes, percent, counts or a bare factor is reachable only through the
vocabulary arm; (2) a measurement with no provenance word and no unit is
unreachable textually — only the question finds it. A third is specific
to this repo and bites the Rust and Python legs hardest: `measure`,
`measured` and `measurement` are DOMAIN VOCABULARY here (measurement
nodes, mass properties, the guide's ladder), so on prose and on the
document-layer tests most matches are over-matches on the verb.
"""
import argparse
import ast
import pathlib
import re
import sys

PROV = re.compile(r"""(?ix)
  \b(re-)?measur(e|ed|es|ing|ement|ements)\b
| \bbenchmark\w* | \bprofiled\b | \bprofiling\b
| \btimed\s+at\b | \bwall-?\s?clock\b | \bwall\s+time\b
| \bempiric\w* | \bcalibrat\w* | \bhand-tuned\b | \bin\ practice\b
| \bobserved\s+(at|max|maximum|min|minimum|worst|to\ be|residual|deviation|value|range)\b
| \bwe\ observed\b | \bobserved\ on\b | \bexperimentally\b | \bin\ the\ wild\b
| \bon\ (this|the)\ box\b | \bon\ the\ (M\d|CI)\b | \bspeedup\b | \bslowdown\b
| \b\d+(\.\d+)?\s*[x×]\s*(faster|slower|more|less|cheaper|bigger|smaller)\b
| \bPERF-PLAN\b | \bK-REPORT\b | \bTESS-BUDGET\b | \bGENERICS-BUILD-COST\b
| \bLOCAL-BUILD-PERF\b | \bPERF-SCAN\b | \bperf-data\b | \bTESS-SPAN\b
| \brebuild\ latency\b
| \b\d+(\.\d+)?\s*(ms|µs|us|ns|sec|secs|seconds?|minutes?)\b
""")


def blocks(path, marker, paragraphs):
    """Contiguous runs of comment lines -> (first_lineno, comment text)."""
    out, cur, start = [], [], None
    for n, line in enumerate(path.read_text(errors="replace").splitlines(), 1):
        i = -1 if (paragraphs and not line.strip()) else line.find(marker)
        if i < 0:
            if cur:
                out.append((start, "\n".join(cur)))
                cur, start = [], None
        else:
            if not cur:
                start = n
            cur.append(line[i + len(marker):])
    if cur:
        out.append((start, "\n".join(cur)))
    return out


def docstrings(path):
    """Every string-literal expression statement -> (lineno, text)."""
    try:
        tree = ast.parse(path.read_text(errors="replace"))
    except SyntaxError as exc:
        print(f"{path}: unparsed ({exc})", file=sys.stderr)
        return []
    return [
        (node.lineno, node.value.value)
        for node in ast.walk(tree)
        if isinstance(node, ast.Expr)
        and isinstance(node.value, ast.Constant)
        and isinstance(node.value.value, str)
    ]


ap = argparse.ArgumentParser()
ap.add_argument("roots", nargs="+")
ap.add_argument("--marker", default="//")
ap.add_argument("--ext", default=".rs")
ap.add_argument("--paragraphs", action="store_true")
ap.add_argument("--docstrings", action="store_true")
ap.add_argument("--show", action="store_true")
a = ap.parse_args()

hits = 0
for root in a.roots:
    for p in sorted(pathlib.Path(".").glob(root)):
        if not p.is_file() or not p.name.endswith(a.ext):
            continue
        found = blocks(p, a.marker, a.paragraphs)
        if a.docstrings:
            found = sorted(found + docstrings(p))
        for start, text in found:
            if PROV.search(text) and re.search(r"\d", text):
                hits += 1
                print(f"{p}:{start}")
                if a.show:
                    print("\n".join("    " + ln for ln in text.splitlines()))
print(f"--- {hits} blocks", file=sys.stderr)
