#!/usr/bin/env python3
"""`PIPESTATUS` names the LAST pipeline the shell ran, so it must be read before
anything else runs — including the assignment on the line above it.

WHY THIS EXISTS. `ci.yml`'s `driver K-telemetry lint` step spent eight commits
unable to fail. It was spelled

    cargo run -- ... 2>&1 | tee out.log || status=$?
    status=${PIPESTATUS[0]:-$status}

and `PIPESTATUS` is rewritten by EVERY command, including `status=$?` — which
succeeds. By the time the second line runs, `PIPESTATUS[0]` is that assignment's
own `0`; the `:-` default never fires because `0` is a perfectly good value; and
`status` is `0` whatever the pipeline said. Both non-zero arms of the `case`
below it were unreachable. The row reported success over a lint that was
refusing every file it was handed unread (run 33828394312).

THE FAILURE IS SILENT AND IT DISARMS A GATE. Nothing goes red — the step passes,
the job passes, the run is green, and the only evidence is a log nobody opens.

SHELLCHECK DOES NOT CATCH IT. Measured, not assumed: shellcheck 0.9.0 on the
five-line reproduction above says nothing at all (it has SC2319/SC2320 for the
sibling `$?` shapes, and no equivalent for `PIPESTATUS`). Nothing in this repo
runs shellcheck either; `work/ciw/shellcheck-is-not-run.md` carries that.

WHAT THIS PROVES. In every shell body under `.github/workflows/` and
`.github/actions/`, and every tracked `*.sh` / `*.bash` file and every tracked
file with a `sh`/`bash` shebang, each command that READS `PIPESTATUS` is
immediately preceded — with no intervening command of any kind — by a command
that IS a pipeline.

It is the property, not the incident. The preceding command being an assignment
is one way to fail it; an `echo`, a `[[ … ]]`, a `let`, a `local`, a function
call, a loop, or nothing at all are the others, and each is a mutant row in
`--selftest`.

A `|` ONLY COUNTS AT PAREN DEPTH ZERO, which is the difference between a
pipeline and a pipe-shaped character. `out=$(cmd | tee log)` is ONE command
whose status is the assignment's, `( a | b )` is a subshell whose status is the
subshell's, `[[ x =~ (a|b) ]]` is a test, and `(( 1 | 2 ))` is arithmetic — a
`PIPESTATUS[0]` read after any of them names something other than the pipeline
the author meant, and all four are mutant rows.

`&` IS NOT A COMMAND BOUNDARY IN A REDIRECTION. `2>&1`, `>&2`, `&>/dev/null`
and `|&` are text; only a background `&` ends a command. This is not a guess:
`scripts/check-ci-mirror-parity.py`'s `CMD_BREAK` comment records meeting the
same defect twice, and the lesson it draws — a check that reds on a CORRECT
change gets routed around, and then detects nothing at all — applies here
exactly. Every spelling above is a mutant row that must stay GREEN.

A COMPOUND COMMAND IS ONE COMMAND, and the tail of a pipeline may be one:
`cmd | while read -r l; do …; done` is a pipeline, so a read after its `done`
is CORRECT. So a closing `fi` / `done` / `esac` / `}` resolves back to the unit
that opened it, and the pure grammar words (`then`, `do`, `else`, `{`) are
transparent. `if cmd | tee f; then` followed by a read is correct too, and
`cmd | tee f` followed by an unrelated `for … done` and a read is not.

WHAT IT DOES NOT PROVE. Not that the pipeline's status is USED once captured.
Not anything about `$?` — that is shellcheck's SC2319/SC2320, and reimplementing
it here would be a second, worse copy of a rule that already exists. Not
anything about a status a pipeline loses to `set -o pipefail` being off. Not
shell it cannot see: a heredoc body, a single-quoted string, a comment (all
three deliberate, all three mutant rows), an `eval`ed string, a command composed
at run time, or a shell body reaching CI by a route this population does not
cover.

WHERE IT IS CONSERVATIVE. A read whose preceding command is a compound this
reader cannot resolve reds rather than passing. That direction is a loud
failure whose message states the fix, not a silent one — but it is the
direction that gets a checker routed around, so every shape met in this tree
has a mutant row asserting it stays green.

Stdlib only, and a line recogniser rather than a YAML parser — the same posture
as `scripts/check-ci-mirror-parity.py`, whose header argues it. A `run:` shape
it cannot read raises `Bail` and fails the run. That is NOT a claim about
arbitrary shell: within a body it recognises, the classes above are read as
agreement, and they are named there rather than left implied.

  check-status-capture.py [--selftest] [--root DIR]
"""

from __future__ import annotations

import argparse
import os
import re
import subprocess
import sys
import tempfile

YAML_ROOTS = (".github/workflows", ".github/actions")
SHELL_SUFFIXES = (".sh", ".bash")
SHEBANG_RE = re.compile(rb"^#!.*\b(?:ba)?sh\b")

RUN_KEY = re.compile(r"^(?P<lead>\s*(?:-\s+)?)run:(?P<rest>.*)$")
BLOCK_SCALAR = re.compile(r"^\s*\|[-+]?\s*$")

# Reserved words that OPEN a compound command, and the ones that CLOSE one. A
# compound is a single command; its closer resolves back to its opener, which
# is the unit that carries the `|` when the compound is a pipeline's tail.
OPENERS = frozenset({"if", "while", "until", "for", "select", "case", "{"})
CLOSERS = frozenset({"fi", "done", "esac", "}"})
# Pure grammar: not commands, so transparent when looking backwards.
GRAMMAR = frozenset({"then", "do", "else", "{", "}"})


class Bail(Exception):
    """A shape this script cannot read. Fails the run rather than passing it."""


class Unit:
    """One command in the shell's execution order, with where it starts."""

    __slots__ = ("closes", "has_pipe", "line", "opener", "opens", "reads", "text")

    def __init__(self, line: int) -> None:
        self.line = line
        self.text = ""
        self.has_pipe = False
        self.reads = False
        self.opens = 0
        self.closes = 0
        self.opener: int | None = None

    def __repr__(self) -> str:  # pragma: no cover - diagnostics only
        return f"Unit(line={self.line}, text={self.text.strip()!r})"


def _read_at(body: str, i: int, arith: int) -> bool:
    """Is position `i` the start of a read of `PIPESTATUS`?

    `$PIPESTATUS`, `${PIPESTATUS…`, `${#PIPESTATUS…` and `${!PIPESTATUS…`
    everywhere; the bare name additionally inside `(( … ))`, where `st =
    PIPESTATUS[0]` needs no sigil.
    """
    if body.startswith("$", i):
        rest = body[i + 1 :]
        if rest.startswith("PIPESTATUS"):
            return True
        if rest.startswith("{"):
            inner = rest[1:]
            if inner[:1] in ("#", "!"):
                inner = inner[1:]
            return inner.startswith("PIPESTATUS")
        return False
    if arith > 0 and body.startswith("PIPESTATUS", i):
        before = body[i - 1] if i else " "
        return not (before.isalnum() or before == "_")
    return False


def scan_units(body: str, first_line: int = 1) -> list[Unit]:
    """Split a shell body into the commands it runs, in order.

    Boundaries are the unquoted list operators `;` `;;` `&&` `||`, a background
    `&`, and the newline. A `|` is NOT a boundary — at paren depth zero it is
    what makes a unit a pipeline, and deeper it is text.
    """
    units: list[Unit] = []
    line = first_line
    cur = Unit(line)
    quote: str | None = None
    heredocs: list[tuple[str, bool]] = []
    # Nesting: (kind, quote to restore on close). Kinds are "p" = ( or $(,
    # "a" = (( or $((, "b" = [[. The saved quote is what makes `"$(cmd "$x")"`
    # readable: inside `$( … )` the shell starts quoting afresh, so the inner
    # `"` opens a string rather than closing the outer one.
    nest: list[tuple[str, str | None]] = []
    i = 0
    n = len(body)

    def close() -> None:
        nonlocal cur, line
        if cur.text.strip():
            units.append(cur)
        cur = Unit(line)

    def arith() -> int:
        return sum(1 for kind, _ in nest if kind == "a")

    while i < n:
        ch = body[i]

        if quote == "'":
            cur.text += ch
            if ch == "\n":
                line += 1
            elif ch == "'":
                quote = None
            i += 1
            continue

        if quote == '"':
            if ch == "\\" and i + 1 < n:
                cur.text += body[i : i + 2]
                if body[i + 1] == "\n":
                    line += 1
                i += 2
                continue
            if body.startswith("$(", i):
                # A substitution inside a string quotes independently.
                nest.append(("p", quote))
                quote = None
                cur.text += "$("
                i += 2
                continue
            if _read_at(body, i, 0):
                cur.reads = True
            cur.text += ch
            if ch == "\n":
                line += 1
            elif ch == '"':
                quote = None
            i += 1
            continue

        # ---- unquoted ----
        if ch == "\\" and i + 1 < n:
            if body[i + 1] == "\n":
                line += 1  # line continuation: the newline is not a boundary
                i += 2
                continue
            cur.text += body[i : i + 2]
            i += 2
            continue

        if ch == "#" and (not cur.text or cur.text[-1].isspace()):
            j = body.find("\n", i)
            i = j if j != -1 else n
            continue

        if ch == "\n":
            line += 1
            close()
            i += 1
            if heredocs:
                i, line = _skip_heredocs(body, i, line, heredocs)
                heredocs = []
                cur = Unit(line)
            continue

        if ch in "'\"":
            quote = ch
            cur.text += ch
            i += 1
            continue

        if body.startswith("$((", i) or body.startswith("((", i):
            width = 3 if ch == "$" else 2
            nest.append(("a", None))
            cur.text += body[i : i + width]
            i += width
            continue

        if body.startswith("))", i) and nest and nest[-1][0] == "a":
            nest.pop()
            cur.text += "))"
            i += 2
            continue

        if body.startswith("[[", i):
            nest.append(("b", None))
            cur.text += "[["
            i += 2
            continue

        if body.startswith("]]", i) and nest and nest[-1][0] == "b":
            nest.pop()
            cur.text += "]]"
            i += 2
            continue

        if ch == "(":
            nest.append(("p", None))
            cur.text += ch
            i += 1
            continue

        if ch == ")":
            if nest:
                # A `case` pattern's unmatched `)` simply finds nothing to pop.
                quote = nest.pop()[1]
            cur.text += ch
            i += 1
            continue

        if body.startswith("<<<", i):
            cur.text += "<<<"
            i += 3
            continue

        # `<<` is a heredoc only outside arithmetic, where it is a left shift.
        if body.startswith("<<", i) and arith() == 0:
            i, delim, strip = _read_heredoc_delim(body, i)
            heredocs.append((delim, strip))
            cur.text += "<<"
            continue

        if body.startswith("&&", i) or body.startswith("||", i):
            close()
            i += 2
            continue

        if body.startswith(";;", i):
            close()
            i += 2
            continue

        if ch == ";":
            close()
            i += 1
            continue

        if ch == "&":
            # NOT a boundary in a redirection: `2>&1`, `>&2`, `&>file`, `|&`.
            prev = cur.text.rstrip()[-1:] if cur.text.strip() else ""
            nxt = body[i + 1] if i + 1 < n else ""
            if prev in (">", "<", "|") or nxt in (">", "<"):
                cur.text += ch
                i += 1
                continue
            close()
            i += 1
            continue

        if ch == "|":
            if not nest:
                cur.has_pipe = True
            cur.text += ch
            i += 1
            continue

        if _read_at(body, i, arith()):
            cur.reads = True

        cur.text += ch
        i += 1

    if quote is not None:
        raise Bail("unterminated quote in a shell body; cannot read its commands")
    close()
    _mark_compounds(units)
    return units


def _mark_compounds(units: list[Unit]) -> None:
    """Pair each compound's closer with the unit that opened it."""
    stack: list[int] = []
    for idx, unit in enumerate(units):
        for word in unit.text.split():
            if word in CLOSERS and word != "{":
                unit.closes += 1
                if stack:
                    unit.opener = stack.pop()
            elif word in OPENERS:
                unit.opens += 1
                stack.append(idx)


def _read_heredoc_delim(body: str, i: int) -> tuple[int, str, bool]:
    """Read the delimiter word after `<<` / `<<-`, returning the new index."""
    i += 2
    strip = False
    if i < len(body) and body[i] == "-":
        strip = True
        i += 1
    while i < len(body) and body[i] in " \t":
        i += 1
    quote = None
    if i < len(body) and body[i] in "'\"":
        quote = body[i]
        i += 1
    word = ""
    while i < len(body):
        c = body[i]
        if quote is not None:
            if c == quote:
                i += 1
                break
        elif c in " \t\n;&|<>()":
            break
        word += c
        i += 1
    if not word:
        raise Bail("a `<<` heredoc with no delimiter word; cannot find its body")
    return i, word, strip


def _skip_heredocs(
    body: str, i: int, line: int, heredocs: list[tuple[str, bool]]
) -> tuple[int, int]:
    """Consume every pending heredoc body. Its text is data, never code.

    The terminator must stand ALONE on its line: bash accepts no leading
    whitespace at all, and `<<-` strips leading TABS only. Matching a stripped
    line would end the skip early and read the rest of the body as code.
    """
    for delim, strip in heredocs:
        while True:
            j = body.find("\n", i)
            raw = body[i:j] if j != -1 else body[i:]
            candidate = raw.lstrip("\t") if strip else raw
            i = (j + 1) if j != -1 else len(body)
            line += 1
            if candidate == delim:
                break
            if j == -1:
                raise Bail(f"heredoc `{delim}` is never terminated")
    return i, line


def _preceding(units: list[Unit], idx: int) -> Unit | None:
    """The command that actually ran before `units[idx]`.

    Grammar words are transparent; a closer resolves back to its opener, since
    the compound it ends is ONE command and may be a pipeline's tail.
    """
    j = idx - 1
    seen = 0
    while j >= 0:
        seen += 1
        if seen > len(units):  # pragma: no cover - a cycle cannot arise
            raise Bail("compound nesting does not resolve")
        unit = units[j]
        if unit.closes and unit.opener is not None:
            j = unit.opener
            return units[j]
        if unit.text.split() and all(t in GRAMMAR for t in unit.text.split()):
            j -= 1
            continue
        return unit
    return None


def violations(body: str, first_line: int = 1) -> list[tuple[int, str]]:
    """Every `PIPESTATUS` read whose preceding command is not a pipeline."""
    units = scan_units(body, first_line)
    out: list[tuple[int, str]] = []
    for idx, unit in enumerate(units):
        if not unit.reads:
            continue
        prev = _preceding(units, idx)
        if prev is None:
            out.append((unit.line, "no command runs before this read"))
            continue
        if prev.has_pipe:
            continue
        out.append(
            (
                unit.line,
                "the command before it is not a pipeline, so PIPESTATUS has "
                f"already been rewritten by it: `{prev.text.strip()}` "
                f"(line {prev.line})",
            )
        )
    return out


def count_reads(body: str, first_line: int = 1) -> int:
    return sum(1 for u in scan_units(body, first_line) if u.reads)


def yaml_bodies(path: str, text: str) -> list[tuple[int, str]]:
    """Every `run:` shell body in a workflow or composite action, as
    (first line, body)."""
    lines = text.split("\n")
    bodies: list[tuple[int, str]] = []
    i = 0
    while i < len(lines):
        m = RUN_KEY.match(lines[i])
        if not m:
            i += 1
            continue
        key_col = len(m.group("lead"))
        rest = m.group("rest")
        if not BLOCK_SCALAR.match(rest):
            stripped = rest.strip()
            if not stripped:
                raise Bail(f"{path}:{i + 1}: a `run:` with no scalar and no block")
            if stripped.startswith(">"):
                raise Bail(f"{path}:{i + 1}: a folded `run: >` scalar is not read")
            bodies.append((i + 1, stripped))
            i += 1
            continue
        body: list[str] = []
        j = i + 1
        while j < len(lines):
            ln = lines[j]
            if ln.strip() and (len(ln) - len(ln.lstrip(" "))) <= key_col:
                break
            body.append(ln)
            j += 1
        while body and not body[-1].strip():
            body.pop()
        if not body:
            raise Bail(f"{path}:{i + 1}: a `run: |` block with an empty body")
        base = min(len(b) - len(b.lstrip(" ")) for b in body if b.strip())
        bodies.append((i + 2, "\n".join(b[base:] for b in body)))
        i = j
    return bodies


def _tracked(root: str) -> list[str]:
    try:
        out = subprocess.run(
            ["git", "ls-files", "-z"],
            cwd=root,
            check=True,
            capture_output=True,
            text=True,
        ).stdout
    except (OSError, subprocess.CalledProcessError) as exc:
        raise Bail(f"cannot list tracked files ({exc})") from exc
    return [p for p in out.split("\0") if p]


def shell_files(root: str) -> list[str]:
    """Tracked `*.sh` / `*.bash`, plus any tracked file with a sh/bash shebang —
    `local-scripts/hooks/pre-push` carries one and no suffix."""
    found = []
    for rel in _tracked(root):
        full = os.path.join(root, rel)
        if rel.endswith(SHELL_SUFFIXES):
            found.append(rel)
            continue
        try:
            with open(full, "rb") as fh:
                head = fh.readline(256)
        except OSError:
            continue
        if SHEBANG_RE.match(head):
            found.append(rel)
    return sorted(found)


def yaml_files(root: str) -> list[str]:
    found: list[str] = []
    for d in YAML_ROOTS:
        full = os.path.join(root, d)
        if not os.path.isdir(full):
            continue
        for dirpath, _dirs, names in os.walk(full):
            for name in sorted(names):
                if name.endswith((".yml", ".yaml")):
                    found.append(
                        os.path.relpath(os.path.join(dirpath, name), root)
                    )
    if not found:
        raise Bail(
            f"no YAML under {' or '.join(YAML_ROOTS)}; this check has nothing to read"
        )
    return sorted(found)


def check_tree(root: str) -> tuple[list[str], int, int]:
    """Returns (failures, files read, PIPESTATUS reads checked)."""
    failures: list[str] = []
    reads = 0
    files = 0

    for rel in yaml_files(root):
        files += 1
        with open(os.path.join(root, rel), encoding="utf-8") as fh:
            text = fh.read()
        for first, body in yaml_bodies(rel, text):
            reads += count_reads(body, first)
            for line, why in violations(body, first):
                failures.append(f"{rel}:{line}: {why}")

    for rel in shell_files(root):
        files += 1
        with open(os.path.join(root, rel), encoding="utf-8") as fh:
            body = fh.read()
        reads += count_reads(body)
        for line, why in violations(body):
            failures.append(f"{rel}:{line}: {why}")

    return failures, files, reads


# --------------------------------------------------------------------------
# The mutant table. Every row names the failure it must catch; each is run
# through THREE carriers — the body directly, a tracked `*.sh` file, and a
# workflow `run: |` block — because a row that only ever reaches the parser
# through one of them leaves the others deletable with nothing red.
# --------------------------------------------------------------------------

GREEN, RED, BAIL = "green", "red", "bail"

MUTANTS: tuple[tuple[str, str, str, tuple[int, ...]], ...] = (
    (
        "the correct spelling: read on the line after the pipeline",
        "cmd 2>&1 | tee out.log\nstatus=${PIPESTATUS[0]}\n",
        GREEN,
        (),
    ),
    (
        "the historical defect: `|| status=$?` clobbers it first",
        "cmd 2>&1 | tee out.log || status=$?\nstatus=${PIPESTATUS[0]:-$status}\n",
        RED,
        (2,),
    ),
    (
        "an intervening `echo`",
        'cmd | tee out.log\necho "done"\nstatus=${PIPESTATUS[0]}\n',
        RED,
        (3,),
    ),
    (
        "an intervening `[[ … ]]` test",
        "cmd | tee out.log\n[[ -s out.log ]]\nstatus=${PIPESTATUS[0]}\n",
        RED,
        (3,),
    ),
    (
        "a `[[ … ]]` whose regex CONTAINS a `|` — the pipe-shaped character",
        "[[ $x =~ (a|b) ]]\nstatus=${PIPESTATUS[0]}\n",
        RED,
        (2,),
    ),
    (
        "a command substitution containing a pipeline: `out=$(cmd | tee log)`",
        'out=$(cmd | tee log)\nstatus=${PIPESTATUS[0]}\n',
        RED,
        (2,),
    ),
    (
        "a subshell containing a pipeline: `( a | b )`",
        "( false | true )\nstatus=${PIPESTATUS[0]}\n",
        RED,
        (2,),
    ),
    (
        "arithmetic containing a bitwise or: `(( 1 | 2 ))`",
        "(( 1 | 2 ))\nstatus=${PIPESTATUS[0]}\n",
        RED,
        (2,),
    ),
    (
        "an intervening `let` arithmetic",
        "cmd | tee out.log\nlet n=n+1\nstatus=${PIPESTATUS[0]}\n",
        RED,
        (3,),
    ),
    (
        "an intervening `local` declaration",
        "cmd | tee out.log\nlocal rc=0\nstatus=${PIPESTATUS[0]}\n",
        RED,
        (3,),
    ),
    (
        "an intervening function call",
        "cmd | tee out.log\nannounce_group\nstatus=${PIPESTATUS[0]}\n",
        RED,
        (3,),
    ),
    (
        "an intervening loop, which is one command and not a pipeline",
        "cmd | tee out.log\nfor x in a b; do echo \"$x\"; done\n"
        "status=${PIPESTATUS[0]}\n",
        RED,
        (3,),
    ),
    (
        "the read sunk into an `if` body, so the CONDITION clobbered it",
        "cmd | tee out.log\nif true; then\n  status=${PIPESTATUS[0]}\nfi\n",
        RED,
        (3,),
    ),
    (
        "index 1 rather than 0",
        'cmd | tee out.log\necho "x"\nstatus=${PIPESTATUS[1]}\n',
        RED,
        (3,),
    ),
    (
        "the whole array",
        'cmd | tee out.log\necho "x"\nrc=("${PIPESTATUS[@]}")\n',
        RED,
        (3,),
    ),
    (
        "the array's LENGTH, `${#PIPESTATUS[@]}`",
        'cmd | tee out.log\necho "x"\nn=${#PIPESTATUS[@]}\n',
        RED,
        (3,),
    ),
    (
        "the array's INDICES, `${!PIPESTATUS[@]}`",
        'cmd | tee out.log\necho "x"\nfor i in ${!PIPESTATUS[@]}; do :; done\n',
        RED,
        (3,),
    ),
    (
        "the BARE name inside arithmetic, `(( st = PIPESTATUS[0] ))`",
        'cmd | tee out.log\necho "x"\n(( st = PIPESTATUS[0] ))\n',
        RED,
        (3,),
    ),
    (
        "the unbraced spelling",
        'cmd | tee out.log\necho "x"\nstatus=$PIPESTATUS\n',
        RED,
        (3,),
    ),
    (
        "a read inside double quotes still counts",
        'cmd | tee out.log\necho "x"\nif [ "${PIPESTATUS[0]}" -ne 0 ]; then :; fi\n',
        RED,
        (3,),
    ),
    (
        "the preceding command is not a pipeline at all",
        "cmd >out.log\nstatus=${PIPESTATUS[0]}\n",
        RED,
        (2,),
    ),
    (
        "nothing runs before the read",
        "status=${PIPESTATUS[0]}\n",
        RED,
        (1,),
    ),
    # ---- shapes that are CORRECT and must stay green: a checker that reds on
    # ---- a correct change gets routed around, and then detects nothing.
    (
        "same line, after `;` — still the immediately preceding command",
        "cmd | tee out.log; status=${PIPESTATUS[0]}\n",
        GREEN,
        (),
    ),
    (
        "the pipeline spread over line continuations (render.yml's shape)",
        'FOO=bar \\\n  xvfb-run -a \\\n  demos/render.sh 2>&1 | tee "$out"\n'
        "status=${PIPESTATUS[0]}\n",
        GREEN,
        (),
    ),
    (
        "a redirection AFTER the pipe: `| tee f >&2` — `&` is not a boundary",
        'cmd | tee "$log" >&2\nstatus=${PIPESTATUS[0]}\n',
        GREEN,
        (),
    ),
    (
        "`2>&1` on the LAST stage",
        "false | true 2>&1\nstatus=${PIPESTATUS[0]}\n",
        GREEN,
        (),
    ),
    (
        "`&>/dev/null` on the last stage",
        "false | true &>/dev/null\nstatus=${PIPESTATUS[0]}\n",
        GREEN,
        (),
    ),
    (
        "the `|&` pipe operator",
        "false |& true\nstatus=${PIPESTATUS[0]}\n",
        GREEN,
        (),
    ),
    (
        "a brace group as the pipeline's tail: `{ a | b; }`",
        "{ false | true; }\nstatus=${PIPESTATUS[0]}\n",
        GREEN,
        (),
    ),
    (
        "a loop as the pipeline's TAIL (`| while read; do …; done`)",
        'cmd | while read -r l; do echo "$l"; done\nstatus=${PIPESTATUS[0]}\n',
        GREEN,
        (),
    ),
    (
        "a pipeline as an `if` CONDITION, read in the body",
        "if cmd | tee out.log; then\n  status=${PIPESTATUS[0]}\nfi\n",
        GREEN,
        (),
    ),
    (
        "a background `&` IS a boundary",
        "cmd | tee out.log\nspawn & status=${PIPESTATUS[0]}\n",
        RED,
        (2,),
    ),
    (
        "`&&` is a boundary",
        "cmd | tee out.log && echo ok\nstatus=${PIPESTATUS[0]}\n",
        RED,
        (2,),
    ),
    (
        "a `#` inside double quotes is not a comment",
        'echo "a # b" | tee out.log\nstatus=${PIPESTATUS[0]}\n',
        GREEN,
        (),
    ),
    (
        "the defect quoted in a comment is prose, not code (ci.yml's tombstone)",
        "cmd | tee out.log\n"
        "# ... | tee ... || status=$?\n"
        "# status=${PIPESTATUS[0]:-$status}\n"
        "status=${PIPESTATUS[0]}\n",
        GREEN,
        (),
    ),
    (
        "the defect inside single quotes is a string, not code",
        "cmd | tee out.log\necho 'status=${PIPESTATUS[0]:-$status}'\ntrue\n",
        GREEN,
        (),
    ),
    (
        "a single-quoted string spanning lines still advances the line count",
        "cmd | tee out.log\necho 'a\nb'\nstatus=${PIPESTATUS[0]}\n",
        RED,
        (4,),
    ),
    (
        "the defect inside a heredoc body is data, not code",
        "cat <<'EOF'\nfoo | bar\nstatus=${PIPESTATUS[0]:-$status}\nEOF\ntrue\n",
        GREEN,
        (),
    ),
    (
        "`<<-` strips TABS, so its terminator is found",
        "cmd | tee out.log\ncat <<-EOF\n\tbody\n\tEOF\nstatus=${PIPESTATUS[0]}\n",
        RED,
        (5,),
    ),
    (
        "an un-dashed heredoc's terminator must stand alone, unindented",
        "cat <<EOF\n  EOF\nreal_command\nEOF\ntrue\n",
        GREEN,
        (),
    ),
    (
        "a left shift is arithmetic, not a heredoc",
        "cmd | tee out.log\nn=$(( 1 << 2 ))\nstatus=${PIPESTATUS[0]}\n",
        RED,
        (3,),
    ),
    (
        "a `case` arm terminator is a command boundary too",
        "cmd | tee out.log\ncase $x in\n  a) true ;;\nesac\nstatus=${PIPESTATUS[0]}\n",
        RED,
        (5,),
    ),
    (
        "an unterminated quote is refused, not read as agreement",
        'echo "unterminated\n',
        BAIL,
        (),
    ),
)


def _carriers(tmp: str, body: str) -> list[tuple[str, str]]:
    """The same body, reached by each route the population covers."""
    wf = os.path.join(tmp, ".github", "workflows")
    os.makedirs(wf, exist_ok=True)
    indented = "\n".join("          " + ln for ln in body.split("\n"))
    with open(os.path.join(wf, "ci.yml"), "w", encoding="utf-8") as fh:
        fh.write(
            "jobs:\n  a:\n    steps:\n      - name: s\n        run: |\n"
            f"{indented}\n        env:\n          FOO: bar\n"
        )
    with open(os.path.join(tmp, "t.sh"), "w", encoding="utf-8") as fh:
        fh.write("#!/usr/bin/env bash\n" + body)
    return [("workflow `run: |`", "ci.yml"), ("tracked *.sh", "t.sh")]


def _tree_outcome(tmp: str) -> tuple[str, list[str]]:
    try:
        failures, _files, _reads = check_tree(tmp)
    except Bail:
        return BAIL, []
    return (RED if failures else GREEN), failures


def selftest() -> int:
    bad = 0
    with tempfile.TemporaryDirectory() as tmp:
        subprocess.run(["git", "init", "-q"], cwd=tmp, check=True)
        for name, body, want, lines in MUTANTS:
            results = []
            try:
                got = violations(body)
                results.append(("direct", RED if got else GREEN, got))
            except Bail:
                results.append(("direct", BAIL, []))
                got = []
            if results[0][1] == want and want != BAIL:
                got_lines = tuple(ln for ln, _ in got)
                if lines and got_lines != lines:
                    bad += 1
                    print(f"SELFTEST FAIL: {name}\n  lines {got_lines} != {lines}")
            _carriers(tmp, body)
            subprocess.run(["git", "add", "-f", "t.sh", ".github"], cwd=tmp, check=True)
            outcome, failures = _tree_outcome(tmp)
            results.append(("carriers", outcome, failures))

            if any(r[1] != want for r in results):
                bad += 1
                print(f"SELFTEST FAIL: {name}  (wanted {want})")
                for where, outcome, detail in results:
                    print(f"    {where}: {outcome}  {detail}")
            else:
                print(f"  ok  ({want:5})  {name}")

        # The inline `run: cmd` scalar is its own route into the parser.
        wf = os.path.join(tmp, ".github", "workflows")
        os.remove(os.path.join(tmp, "t.sh"))
        subprocess.run(["git", "rm", "-q", "--cached", "t.sh"], cwd=tmp, check=True)
        with open(os.path.join(wf, "ci.yml"), "w", encoding="utf-8") as fh:
            fh.write(
                "jobs:\n  a:\n    steps:\n"
                "      - run: cmd | tee f || s=$?; s=${PIPESTATUS[0]:-$s}\n"
                "      - run: cmd | tee g\n"
            )
        outcome, failures = _tree_outcome(tmp)
        if outcome != RED or len(failures) != 1:
            bad += 1
            print(f"SELFTEST FAIL: inline `run:` scalar, got {outcome} {failures}")
        else:
            print("  ok  (red  )  the defect in an inline `run: cmd` scalar")

        # A composite action's `run:` is the same CI, by a path the workflow
        # glob does not reach.
        act = os.path.join(tmp, ".github", "actions", "helper")
        os.makedirs(act, exist_ok=True)
        with open(os.path.join(wf, "ci.yml"), "w", encoding="utf-8") as fh:
            fh.write("jobs:\n  a:\n    steps:\n      - run: true\n")
        with open(os.path.join(act, "action.yml"), "w", encoding="utf-8") as fh:
            fh.write(
                "runs:\n  using: composite\n  steps:\n    - run: |\n"
                "        cmd | tee f || s=$?\n"
                "        s=${PIPESTATUS[0]:-$s}\n"
            )
        subprocess.run(["git", "add", "-f", ".github"], cwd=tmp, check=True)
        outcome, failures = _tree_outcome(tmp)
        if outcome != RED or "actions/helper/action.yml" not in failures[0]:
            bad += 1
            print(f"SELFTEST FAIL: composite action, got {outcome} {failures}")
        else:
            print("  ok  (red  )  the defect in a composite action's `run:`")

        # A shebang file with no suffix is in the population.
        hook = os.path.join(tmp, "hooks")
        os.makedirs(hook, exist_ok=True)
        with open(os.path.join(act, "action.yml"), "w", encoding="utf-8") as fh:
            fh.write("runs:\n  using: composite\n  steps:\n    - run: true\n")
        with open(os.path.join(hook, "pre-push"), "w", encoding="utf-8") as fh:
            fh.write("#!/bin/bash\ncmd | tee f || s=$?\ns=${PIPESTATUS[0]:-$s}\n")
        subprocess.run(
            ["git", "add", "-f", "hooks", ".github"], cwd=tmp, check=True
        )
        outcome, failures = _tree_outcome(tmp)
        if outcome != RED or "hooks/pre-push" not in failures[0]:
            bad += 1
            print(f"SELFTEST FAIL: shebang file, got {outcome} {failures}")
        else:
            print("  ok  (red  )  the defect in a suffixless file with a shebang")

        # The census counts reads, so a reads counter stuck at 0 is caught.
        with open(os.path.join(hook, "pre-push"), "w", encoding="utf-8") as fh:
            fh.write("#!/bin/bash\ncmd | tee f\ns=${PIPESTATUS[0]}\nb | c\n"
                     "n=${#PIPESTATUS[@]}\n")
        subprocess.run(["git", "add", "-f", "hooks"], cwd=tmp, check=True)
        failures, _files, reads = check_tree(tmp)
        if failures or reads != 2:
            bad += 1
            print(f"SELFTEST FAIL: census, {reads} read(s), failures {failures}")
        else:
            print("  ok  (green)  the census counts every read it checked")

        # An unreadable `run:` shape fails rather than passing.
        with open(os.path.join(wf, "ci.yml"), "a", encoding="utf-8") as fh:
            fh.write("      - run: >\n          folded\n")
        outcome, _ = _tree_outcome(tmp)
        if outcome != BAIL:
            bad += 1
            print("SELFTEST FAIL: a folded `run: >` scalar was read as agreement")
        else:
            print("  ok  (bail )  an unreadable `run:` shape fails rather than passes")

    if bad:
        print(f"\n{bad} selftest row(s) failed.")
        return 1
    print(f"\nselftest: {len(MUTANTS)} mutants x 3 carriers, plus 6 population "
          "and refusal rows — all as specified.")
    return 0


def main(argv: list[str]) -> int:
    ap = argparse.ArgumentParser(description="PIPESTATUS capture check")
    ap.add_argument("--selftest", action="store_true")
    ap.add_argument("--root", default=".")
    args = ap.parse_args(argv)

    if args.selftest:
        return selftest()

    failures, files, reads = check_tree(args.root)
    if failures:
        print(
            "A `PIPESTATUS` read is taken after something else has already "
            "rewritten it. The status it names is that other command's, so "
            "every non-zero arm below it is unreachable and the row cannot "
            "fail.\n"
        )
        for f in failures:
            print(f"  {f}")
        print(
            "\nThe fix is to read `PIPESTATUS` on the line immediately after "
            "the pipeline, with nothing in between:\n"
            "    some_command 2>&1 | tee out.log\n"
            "    status=${PIPESTATUS[0]}"
        )
        return 1
    print(
        f"status capture: {reads} PIPESTATUS read(s) across {files} shell "
        "file(s), workflow(s) and composite action(s); each is taken on the "
        "command immediately after its pipeline."
    )
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main(sys.argv[1:]))
    except Bail as exc:
        print(f"check-status-capture: {exc}", file=sys.stderr)
        sys.exit(2)
