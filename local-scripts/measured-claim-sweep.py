#!/usr/bin/env python3
"""The measured-claim sweep (#651 / #663 / #667 / #681), as actually run.

  measured-claim-sweep.py 'crates/*/src/**/*.rs'
  measured-claim-sweep.py 'Cargo.toml' 'crates/*/Cargo.toml' --marker '#' --ext .toml
  measured-claim-sweep.py 'crates/pncad-py/**/*.py' --marker '#' --ext .py --docstrings
  measured-claim-sweep.py 'docs/guide/*.md' --marker '' --ext .md --paragraphs
  measured-claim-sweep.py '.github/workflows/*.yml' '.github/actions/*/action.yml' \
      --marker '#' --ext .yml
  measured-claim-sweep.py 'scripts/**/*.sh' --marker '#' --ext .sh
  measured-claim-sweep.py 'scripts/**/*.py' --marker '#' --ext .py --docstrings
  measured-claim-sweep.py 'tools/**/*.rs'

Prints every comment BLOCK carrying a provenance word AND a numeral: the
population a reviewer then triages against reviewer-style-lane.md's Q6
(guarded / scheduled register / unguardable-with-a-written-reason).

A BLOCK COUNT IS "WHAT THE PATTERN MATCHED", NEVER "THE CLAIMS ON THIS
SURFACE". Read the blind spots below before quoting one; on every leg run
so far, claims were found by reading that this pattern cannot reach.

NOT A GATE, and deliberately not wired into one. Its output is a
population to read, not a verdict — the over-match rate is high by design
and the triage is the work. It is committed because issue 681's own
argument is that a lane which re-derives the instrument from prose
inherits a NEW blind spot, and two re-derivations have already shown it:
#667's own "478 hits unrestricted" figure reproduced under no variant and
was withdrawn, and the `.md` recipe #681's table describes printed one
block per FILE until #826 read it closely.

WHY `local-scripts/` AND NOT `scripts/` WITH A `--selftest`. `scripts/` is
the hosted half's tree: a check that lives there is expected to be called
by `ci.yml` or by a `scripts/gates/` roster, and both halves would then be
answerable for it. This cannot be either. It has no pass/fail — a claim
needs a human verdict, and a gate that fired on every measured comment
would be noise on its first run. `local-scripts/` is also where CI cannot
reach by construction: every hosted job deletes this tree right after
checkout (the `mirror` job is the single declared exception), so siting it
here makes "never a gate" a property of the location rather than a promise
in a comment. PR-2 legs cite it from a PR body, which is the right shape
for a tool whose output is evidence rather than a verdict.

WHAT THIS COPY ADDS TO THE ONE IN #681's BODY, all of it disclosure-worthy:

  * --paragraphs. `line.find("")` returns 0 for every line, so `--marker
    ''` alone never terminates a block and prints one block per FILE.
    Markdown legs need blank-line blocking, which this flag is (#826).
  * --docstrings. #681 says the Python half "needs its own extraction"
    and does not supply one. Every `ast` string-literal expression
    statement is emitted as a block alongside the `#` blocks. It roughly
    triples the population on `crates/pncad-py` — the docstring half is
    the larger one there, and the ratio is not restated as a figure
    because it moves with the tree and nothing re-takes it.
  * --show, to print block text rather than only locations.

THE VOCABULARY BELOW WAS COPIED VERBATIM FROM ISSUE 681's BODY, READ ON
2026-08-30. Nothing enforces that across the file/issue boundary — there
is no check, and one side can move without the other noticing — so this
sentence is the only record that they were ever the same text. If you
change the pattern here, say so here; if the issue's copy moves, this
paragraph is what tells you the two have diverged.

BLIND SPOTS. Two are #681's own, carried, and neither is fixable by a
better regex: (1) the bare-number arm is TIME UNITS ONLY, and does not
even spell `min` (#826), so a measurement in bytes, percent, counts or a
bare factor is reachable only through the vocabulary arm; (2) a
measurement with no provenance word and no unit is unreachable textually —
only the question finds it. The rest are this instrument's, one per
extraction mode, and are recorded here rather than in a PR body for the
same reason the script itself is:

  * ANY `--ext`: a file with no extension is skipped in silence.
    `local-scripts/hooks/pre-push` is the live instance — bash, carrying a
    measured claim, invisible to every suffix-selected sweep of this tree.
  * `--marker '#'` on `.toml`: everything after the FIRST `#` on a line is
    read as a comment, so a `#` inside a string opens a false block, and a
    claim inside a `description = "..."` VALUE is never seen at all.
    Non-comment lines break blocks, so a claim split across a `key =
    value` line arrives as two.
  * `--marker '#'` on `.sh`: no heredoc or string awareness whatever. A
    usage heredoc is invisible unless its lines happen to start with `#`,
    and a drifted default inside one is exactly the shape this sweep is
    for — `render-hosted.sh` carried one (`--budget-min (default: 150)`
    against a variable set to 200) that this instrument could not see.
  * `--docstrings` on `.py`: only string-literal EXPRESSION statements. A
    claim in an assertion message, an `argparse` help string, or any
    assigned string constant is not read — and a test states its numbers
    in an assertion message as often as in a comment.
  * `--paragraphs` on `.md`: blank-line blocking splits a claim written
    across a paragraph break, so a number in one paragraph and its
    provenance word in the next match neither. Tables are one line per
    row, which is where this bites: a row's count and the sentence that
    earned it are usually different blocks.
  * `--marker '#'` on `.yml`, ADDED FOR THE WORKFLOWS LEG: two holes, and
    the second is the one that bites. (a) The same string-blindness as
    `.toml` — a `#` inside a quoted scalar or inside a `${{ }}` expression
    opens a false block. (b) A WORKFLOW IS TWO LANGUAGES AND THIS READS
    THEM AS ONE: `#` lines inside a `run:` block are shell comments, not
    YAML ones, and they come out of this sweep indistinguishable from a
    comment on a job key. That is not only noise — it is also how the
    workflows leg reached shell comments it would otherwise have missed —
    but nothing in the output says which layer a block belongs to, so a
    reader must open the file. What no marker reaches at all: a claim
    written in a step `name:`, in a job's `if:`, or inside a
    `$GITHUB_STEP_SUMMARY` heredoc, and those heredocs are where this
    repo's workflows put their reports.
  * THE `.github/` GLOB, and it is a leg's hole rather than a mode's:
    `.github/workflows/*.yml` is the surface issue 681 names, and it
    MISSES `.github/actions/*/action.yml`. The shape to expect there,
    stated rather than illustrated with today's instances: a composite
    action's header justifies the action's existence by quoting the
    WORKFLOW's shape back at it — how many jobs call it, at what profile,
    against what cost — so its claims are extractions whose premises live
    in a file it does not sit beside, and a ruling that moves the
    workflow leaves them behind with nothing pointing at them. The two
    instances this leg found are in PR #1331's body if a reader wants
    them; naming them here would be a third copy of a figure that is now
    corrected in exactly one place. Sweep both globs; the invocation
    above does.
  * `//` on `tools/`: transfers from the `crates/*/src` leg unchanged, and
    inherits its hole — a claim in an `assert!` or `panic!` message, or in
    a `#[doc = "..."]` attribute, is not a `//` comment and is not read.
    In a tools crate that matters more than in the kernel, because the
    lints state their thresholds in the failure text a reader actually
    sees.
  * EVERY MODE, on this repo: `measure`, `measured` and `measurement` are
    DOMAIN VOCABULARY here (measurement nodes, mass properties, the
    guide's author -> validate -> measure -> tessellate ladder), so on
    prose and on the document layer most matches are the verb rather than
    provenance. Not suppressible without losing real rows; budget triage
    time for it instead.

SURFACES THIS INSTRUMENT IS READY FOR AND THAT NOBODY HAS SWEPT — the
blind-spot list's other half, because a surface deferred without a
recorded reason is indistinguishable from a surface forgotten:

  * `crates/*/tests` — DEFERRED, AND NOT THIS SWEEP'S TO TAKE. It is
    Track W's ground: `docs/SMELL-SCAN-2026-08.md` §Track W fences
    `crates/*/tests/` in every crate plus `crates/test-utils/`, and the
    track owns test-side MECHANISMS, which is exactly what a pinned
    literal nothing re-takes is. Its live rows already reach into this
    class — D383's undisclosed duplicate slack, seven copies across three
    files with no derivation, is a measured-claim row in all but name.
    The class home for anything found there stays issue 651.
    The instrument needs no new variant for it:
    `measured-claim-sweep.py 'crates/*/tests/**/*.rs'` runs the `//`
    mode unchanged, with the `assert!`-message hole named above, which on
    a test surface is the largest hole this instrument has. A W-lane
    picking this up finds the tool ready and owes no re-derivation.
  * `docs/` PROSE outside the dated registers — DEFERRED, and the reason
    is the one the plan gave rather than an absence of a variant:
    HIGHEST OVER-MATCH, LOWEST YIELD. Every line is comment text, so the
    numeral filter and `--paragraphs` do the whole job with nothing to
    restrict them, and the domain-verb over-match above lands hardest
    exactly here. The variant is `--marker '' --ext .md --paragraphs`,
    already present and already exercised; what is missing is a reader
    with the budget to triage the population it returns.
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
