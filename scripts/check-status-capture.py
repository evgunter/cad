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
That is the class this repo keeps paying for: an artifact that reports instead of
gating.

SHELLCHECK DOES NOT CATCH IT. Measured, not assumed: shellcheck 0.9.0 on the
five-line reproduction above says nothing at all (it has SC2319/SC2320 for the
sibling `$?` shapes, and no equivalent for `PIPESTATUS`). Nothing in this repo
runs shellcheck either — the `# shellcheck` annotations in `local-scripts/` are
addressed to a tool no workflow invokes.

WHAT THIS PROVES. In every shell body under `.github/workflows/` and every
tracked `*.sh` / `*.bash` file, each command that READS `PIPESTATUS` is
immediately preceded — with no intervening command of any kind — by a command
that IS a pipeline.

It is the property, not the incident. The preceding command being an assignment
is one way to fail it; an `echo`, a `[[ … ]]`, a `let`, a `local`, a function
call, a `then`, or nothing at all are the others, and each is a mutant row in
`--selftest`. The index is not read either, so `${PIPESTATUS[1]}` and
`${PIPESTATUS[@]}` are covered the same way.

WHAT IT DOES NOT PROVE. Not that the pipeline's status is USED once captured —
a `status` nobody tests is a different defect. Not anything about `$?` (that is
shellcheck's SC2319/SC2320, and this repo runs no shellcheck; see
`work/ciw/shellcheck-is-not-run.md`). Not anything about a status a pipeline
loses to `set -o pipefail` being off. Not shell that reaches CI by a route this
does not read: a heredoc body, a string this script cannot see through, a shell
file that is neither `*.sh` nor `*.bash` nor a workflow `run:`, or a command
composed at run time.

Stdlib only, and a line recogniser rather than a YAML parser — the same posture
as `scripts/check-ci-mirror-parity.py`, whose header argues it. Anything it
cannot recognise raises `Bail` and fails, rather than being read as agreement.

  check-status-capture.py [--selftest] [--root DIR]
"""

from __future__ import annotations

import argparse
import os
import re
import subprocess
import sys
import tempfile

WORKFLOW_DIRS = (".github/workflows",)
SHELL_SUFFIXES = (".sh", ".bash")

# `run:` on a step, with or without the sequence dash. The capture is the
# scalar after the key; a block scalar is `|`, `|-` or `|+`.
RUN_KEY = re.compile(r"^(?P<lead>\s*(?:-\s+)?)run:(?P<rest>.*)$")
BLOCK_SCALAR = re.compile(r"^\s*\|[-+]?\s*$")


class Bail(Exception):
    """A shape this script cannot read. Fails the run rather than passing it."""


class Unit:
    """One command in the shell's execution order, with where it starts."""

    def __init__(self, line: int) -> None:
        self.line = line
        self.text = ""
        self.has_pipe = False
        self.reads_pipestatus = False

    def __repr__(self) -> str:  # pragma: no cover - diagnostics only
        return f"Unit(line={self.line}, text={self.text.strip()!r})"


def scan_units(body: str, first_line: int = 1) -> list[Unit]:
    """Split a shell body into the commands it runs, in order.

    Boundaries are the unquoted list operators `;` `;;` `&&` `||` `&` and the
    newline. A `|` is NOT a boundary — it is what makes a unit a pipeline.
    Comments, single- and double-quoted strings, escapes and heredoc bodies are
    read for quoting only; a `PIPESTATUS` inside a comment or a single-quoted
    string is not a read, and a heredoc body is not code.
    """
    units: list[Unit] = []
    line = first_line
    cur = Unit(line)
    quote: str | None = None
    pending_heredocs: list[tuple[str, bool]] = []
    i = 0
    n = len(body)

    def close() -> None:
        nonlocal cur, line
        if cur.text.strip():
            units.append(cur)
        cur = Unit(line)

    while i < n:
        ch = body[i]

        if quote == "'":
            cur.text += ch
            if ch == "'":
                quote = None
            elif ch == "\n":
                line += 1
            i += 1
            continue

        if quote == '"':
            if ch == "\\" and i + 1 < n:
                cur.text += body[i : i + 2]
                if body[i + 1] == "\n":
                    line += 1
                i += 2
                continue
            if body.startswith("$PIPESTATUS", i) or body.startswith("${PIPESTATUS", i):
                cur.reads_pipestatus = True
            cur.text += ch
            if ch == '"':
                quote = None
            elif ch == "\n":
                line += 1
            i += 1
            continue

        # Unquoted.
        if ch == "\\" and i + 1 < n:
            if body[i + 1] == "\n":
                # Line continuation: the newline is not a boundary.
                line += 1
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
            if pending_heredocs:
                i, line = _skip_heredocs(body, i, line, pending_heredocs)
                pending_heredocs = []
                cur = Unit(line)
            continue

        if ch in "'\"":
            quote = ch
            cur.text += ch
            i += 1
            continue

        if body.startswith("<<<", i):
            cur.text += "<<<"
            i += 3
            continue

        if body.startswith("<<", i):
            i, delim, strip = _read_heredoc_delim(body, i)
            pending_heredocs.append((delim, strip))
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

        if ch in ";&":
            close()
            i += 1
            continue

        if ch == "|":
            cur.has_pipe = True
            cur.text += ch
            i += 1
            continue

        if body.startswith("$PIPESTATUS", i) or body.startswith("${PIPESTATUS", i):
            cur.reads_pipestatus = True

        cur.text += ch
        i += 1

    if quote is not None:
        raise Bail("unterminated quote in a shell body; cannot read its commands")
    close()
    return units


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
    """Consume every pending heredoc body. Its text is data, never code."""
    for delim, strip in heredocs:
        while True:
            j = body.find("\n", i)
            raw = body[i:j] if j != -1 else body[i:]
            candidate = raw.lstrip("\t") if strip else raw
            i = (j + 1) if j != -1 else len(body)
            line += 1
            if candidate.strip() == delim:
                break
            if j == -1:
                raise Bail(f"heredoc `{delim}` is never terminated")
    return i, line


def violations(body: str, first_line: int = 1) -> list[tuple[int, str]]:
    """Every `PIPESTATUS` read whose immediately preceding command is not a
    pipeline, as (line, why)."""
    units = scan_units(body, first_line)
    out: list[tuple[int, str]] = []
    for idx, unit in enumerate(units):
        if not unit.reads_pipestatus:
            continue
        if idx == 0:
            out.append((unit.line, "no command runs before this read"))
            continue
        prev = units[idx - 1]
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


def workflow_bodies(path: str, text: str) -> list[tuple[int, str]]:
    """Every `run:` shell body in a workflow file, as (first line, body)."""
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
        indents = [
            len(b) - len(b.lstrip(" ")) for b in body if b.strip()
        ]
        base = min(indents)
        bodies.append((i + 2, "\n".join(b[base:] for b in body)))
        i = j
    return bodies


def tracked_shell_files(root: str) -> list[str]:
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
    return sorted(
        p for p in out.split("\0") if p.endswith(SHELL_SUFFIXES)
    )


def workflow_files(root: str) -> list[str]:
    found: list[str] = []
    for d in WORKFLOW_DIRS:
        full = os.path.join(root, d)
        if not os.path.isdir(full):
            raise Bail(f"{d} is missing; this check has nothing to read")
        for name in sorted(os.listdir(full)):
            if name.endswith((".yml", ".yaml")):
                found.append(os.path.join(d, name))
    if not found:
        raise Bail("no workflow files found; this check has nothing to read")
    return found


def check_tree(root: str) -> tuple[list[str], int, int]:
    """Returns (failures, files read, PIPESTATUS reads checked)."""
    failures: list[str] = []
    reads = 0
    files = 0

    for rel in workflow_files(root):
        files += 1
        with open(os.path.join(root, rel), encoding="utf-8") as fh:
            text = fh.read()
        for first, body in workflow_bodies(rel, text):
            for unit in scan_units(body, first):
                reads += 1 if unit.reads_pipestatus else 0
            for line, why in violations(body, first):
                failures.append(f"{rel}:{line}: {why}")

    for rel in tracked_shell_files(root):
        files += 1
        with open(os.path.join(root, rel), encoding="utf-8") as fh:
            body = fh.read()
        for unit in scan_units(body):
            reads += 1 if unit.reads_pipestatus else 0
        for line, why in violations(body):
            failures.append(f"{rel}:{line}: {why}")

    return failures, files, reads


# --------------------------------------------------------------------------
# The mutant table. Every row names the failure it must catch; the check is
# that injecting it turns the tree red, and that the shapes which are CORRECT
# stay green. A row asserting only what the author just built proves nothing.
# --------------------------------------------------------------------------

MUTANTS: tuple[tuple[str, str, bool], ...] = (
    (
        "the correct spelling: read on the line after the pipeline",
        "cmd 2>&1 | tee out.log\nstatus=${PIPESTATUS[0]}\n",
        False,
    ),
    (
        "the historical defect: `|| status=$?` clobbers it first",
        "cmd 2>&1 | tee out.log || status=$?\nstatus=${PIPESTATUS[0]:-$status}\n",
        True,
    ),
    (
        "an intervening `echo` between the pipeline and the read",
        'cmd | tee out.log\necho "done"\nstatus=${PIPESTATUS[0]}\n',
        True,
    ),
    (
        "an intervening `[[ … ]]` test",
        'cmd | tee out.log\n[[ -s out.log ]]\nstatus=${PIPESTATUS[0]}\n',
        True,
    ),
    (
        "an intervening `let` arithmetic",
        "cmd | tee out.log\nlet n=n+1\nstatus=${PIPESTATUS[0]}\n",
        True,
    ),
    (
        "an intervening `local` declaration",
        "cmd | tee out.log\nlocal rc=0\nstatus=${PIPESTATUS[0]}\n",
        True,
    ),
    (
        "an intervening function call",
        "cmd | tee out.log\nannounce_group\nstatus=${PIPESTATUS[0]}\n",
        True,
    ),
    (
        "the read sunk into an `if` body, so `if` clobbered it",
        "cmd | tee out.log\nif true; then\n  status=${PIPESTATUS[0]}\nfi\n",
        True,
    ),
    (
        "index 1 rather than 0",
        'cmd | tee out.log\necho "x"\nstatus=${PIPESTATUS[1]}\n',
        True,
    ),
    (
        "the whole array",
        'cmd | tee out.log\necho "x"\nrc=(${PIPESTATUS[@]})\n',
        True,
    ),
    (
        "the unbraced spelling",
        'cmd | tee out.log\necho "x"\nstatus=$PIPESTATUS\n',
        True,
    ),
    (
        "a read inside double quotes still counts",
        'cmd | tee out.log\necho "x"\nif [ "${PIPESTATUS[0]}" -ne 0 ]; then :; fi\n',
        True,
    ),
    (
        "the preceding command is not a pipeline at all",
        "cmd >out.log\nstatus=${PIPESTATUS[0]}\n",
        True,
    ),
    (
        "nothing runs before the read",
        "status=${PIPESTATUS[0]}\n",
        True,
    ),
    (
        "same line, after `;` — still the immediately preceding command",
        "cmd | tee out.log; status=${PIPESTATUS[0]}\n",
        False,
    ),
    (
        "the pipeline spread over line continuations (render.yml's shape)",
        'FOO=bar \\\n  xvfb-run -a \\\n  demos/render.sh 2>&1 | tee "$out"\n'
        "status=${PIPESTATUS[0]}\n",
        False,
    ),
    (
        "the defect quoted in a comment is prose, not code (ci.yml's tombstone)",
        "cmd | tee out.log\n"
        "# ... | tee ... || status=$?\n"
        "# status=${PIPESTATUS[0]:-$status}\n"
        "status=${PIPESTATUS[0]}\n",
        False,
    ),
    (
        "the defect inside single quotes is a string, not code",
        "cmd | tee out.log\n"
        "echo 'status=${PIPESTATUS[0]:-$status}'\n"
        "true\n",
        False,
    ),
    (
        "the defect inside a heredoc body is data, not code",
        "cat <<'EOF'\nfoo | bar\nstatus=${PIPESTATUS[0]:-$status}\nEOF\ntrue\n",
        False,
    ),
    (
        "a `case` arm terminator is a command boundary too",
        "cmd | tee out.log\ncase $x in\n  a) true ;;\nesac\nstatus=${PIPESTATUS[0]}\n",
        True,
    ),
)


def selftest() -> int:
    bad = 0
    for name, body, want_red in MUTANTS:
        got = violations(body)
        red = bool(got)
        if red != want_red:
            bad += 1
            print(
                f"SELFTEST FAIL: {name}\n"
                f"  wanted {'a failure' if want_red else 'no failure'}, "
                f"got {got or 'none'}"
            )
        else:
            print(f"  ok  ({'red' if red else 'green'})  {name}")

    # The extraction half: a workflow whose `run:` bodies carry the defect must
    # be found through the YAML, and an unreadable shape must Bail rather than
    # pass.
    with tempfile.TemporaryDirectory() as tmp:
        subprocess.run(["git", "init", "-q"], cwd=tmp, check=True)
        wf = os.path.join(tmp, ".github", "workflows")
        os.makedirs(wf)
        with open(os.path.join(wf, "ci.yml"), "w", encoding="utf-8") as fh:
            fh.write(
                "jobs:\n"
                "  a:\n"
                "    steps:\n"
                "      - name: good\n"
                "        run: |\n"
                "          cmd | tee out.log\n"
                "          status=${PIPESTATUS[0]}\n"
                "      - name: bad\n"
                "        run: |\n"
                "          cmd | tee out.log || status=$?\n"
                "          status=${PIPESTATUS[0]:-$status}\n"
                "        env:\n"
                "          FOO: bar\n"
            )
        found = check_tree(tmp)[0]
        if len(found) != 1 or ":11:" not in found[0]:
            bad += 1
            print(f"SELFTEST FAIL: workflow extraction, got {found}")
        else:
            print("  ok  (red)  the defect in a `run: |` block, found through YAML")

        with open(os.path.join(wf, "ci.yml"), "a", encoding="utf-8") as fh:
            fh.write("      - run: >\n          folded\n")
        try:
            check_tree(tmp)
        except Bail:
            print("  ok  (bail) an unreadable `run:` shape fails rather than passes")
        else:
            bad += 1
            print("SELFTEST FAIL: a folded `run: >` scalar was read as agreement")

    if bad:
        print(f"\n{bad} selftest row(s) failed.")
        return 1
    print(f"\nselftest: {len(MUTANTS) + 2} rows, all as specified.")
    return 0


def main(argv: list[str]) -> int:
    ap = argparse.ArgumentParser(description=__doc__)
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
        "file(s) and workflow(s); each is taken on the command immediately "
        "after its pipeline."
    )
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main(sys.argv[1:]))
    except Bail as exc:
        print(f"check-status-capture: {exc}", file=sys.stderr)
        sys.exit(2)
