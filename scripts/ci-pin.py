#!/usr/bin/env python3
"""Read one pinned tool version out of `.github/workflows/ci.yml`'s
WORKFLOW-LEVEL `env:` block.

`ci.yml` declares itself the single source of truth for every tool version CI
installs — `NEXTEST_VERSION`, `MATURIN_VERSION`, `TY_VERSION`, `RUFF_VERSION`,
`SCCACHE_VERSION` — and the lanes that are not `ci.yml` read the pin back from
there rather than restating it. This is the one reader they call:

    v="$(scripts/ci-pin.py NEXTEST_VERSION)"

WHAT THIS REPLACES, AND WHY IT IS NOT A TIDY-UP. The reader used to be a `sed`
one-liner pasted at each site:

    sed -n 's/^ *NAME: *//p' .github/workflows/ci.yml | head -1 | tr -d "\\"'"

`^ *NAME:` matches at ANY indentation, so it matches the workflow-level `env:`
block it is aimed at AND any `env:` under a job or a step that pins the same
name; `head -1` then takes whichever comes FIRST IN THE FILE rather than
whichever is in scope. The day someone pins a tool per-job above the workflow
block, every caller silently installs a different version and the lane goes on
reporting green. Each site carried a `test -n` guard, which catches an EMPTY
answer; nothing caught a WRONG one. The quoting is the other half: the idiom
has to smuggle both quote characters through a YAML scalar and a shell string,
and one site got that wrong and never ran at all until a person read a log.

SO THIS READER IS ANCHORED AND IT REFUSES. Anchored: the value comes from the
workflow-level `env:` block — the mapping key at column 0 — and from nowhere
else, so a job-level pin cannot be picked up by accident. Refuses: if the name
appears anywhere else in the file, at any indentation, this exits nonzero and
names every line it saw, because a second pin means the question "which version
does this lane install" has two answers and picking one is how the old idiom
broke. That refusal is deliberately blunt. A caller that legitimately wants a
per-job pin should read that pin where it is, not teach this script to rank
candidates.

The output is the bare value on stdout, unquoted once and correctly. Every
refusal goes to stderr and exits 2, so `set -euo pipefail` plus
`v="$(scripts/ci-pin.py NAME)"` is the whole of a caller's error handling — the
`test -n` guard each site used to carry is now this script's job and does not
need restating at the call site.

WHAT THIS DOES NOT REACH, AND IT IS MORE THAN IT LOOKS. This closes the copies
that READ the pin. It does nothing about the copies that RESTATE its VALUE:
`local-scripts/ci-local.sh` names `0.9.140` in its prereq note, in its
`nextest_check()` error text and in a comment, `local-scripts/gate.sh` names
sccache `0.16.0`, and `.claude/hooks/session-start.sh` names three pins as
shell literals. Nothing reconciles any of them with this file's `env:` block,
so they drift the day a pin is bumped and no gate says a word —
`work/ciw/local-half-restates-ci-pins-as-literals` carries that population.
It is worth knowing how they were missed: the sweep that produced this script
looked for the READING IDIOM and for `_VERSION` names, and a bare `0.9.140` in
an `echo` carries neither. **A sweep for this class starts from the `env:`
block — each pinned name and each pinned value — and asks where else in the
tree they appear.** Not from the shape of whichever idiom is in front of you.

    ci-pin.py NAME [--file PATH]
    ci-pin.py --selftest
"""

from __future__ import annotations

import argparse
import os
import re
import subprocess
import sys
import tempfile
from typing import NoReturn

CI_YML = ".github/workflows/ci.yml"

# A name this reader will look for. Deliberately narrow: an env key, not an
# arbitrary YAML path. A caller reaching for something else wants a YAML
# parser, and should say so rather than widening this.
NAME_RE = re.compile(r"^[A-Z][A-Z0-9_]*$")

# The workflow-level `env:` mapping key: column 0, nothing after it but an
# optional comment. `env:` indented under a job or a step does not match, which
# is the entire anchoring claim.
TOP_ENV_RE = re.compile(r"^env:[ \t]*(#.*)?$")


class Refuse(Exception):
    """Ambiguity, absence, or structure this reader does not recognise. Never a
    value: a reader that guesses is the defect this file exists to close."""


def die(msg: str) -> NoReturn:
    sys.stderr.write(f"ci-pin.py: {msg}\n")
    raise SystemExit(2)


def unquote(raw: str, name: str, lineno: int) -> str:
    """The value of one `NAME: value` line, with YAML's two quote spellings
    handled once, here, instead of through a shell `tr -d` at five call sites.

    `tr -d` deleted every quote character anywhere in the value, which happens
    to be right for `"1.2.3"` and is not a reading of anything. This strips
    exactly one matched pair, and refuses what it cannot read rather than
    handing back a mangled scalar.
    """
    val = raw.strip()
    if val[:1] in ('"', "'"):
        q = val[0]
        end = val.find(q, 1)
        if end < 0:
            raise Refuse(f"{name} at line {lineno} opens with {q} and never closes it")
        rest = val[end + 1:].strip()
        if rest and not rest.startswith("#"):
            raise Refuse(f"{name} at line {lineno} has {rest!r} after the closing quote — "
                         "this reader takes a plain quoted scalar and will not guess at that")
        val = val[1:end]
    else:
        # A bare scalar may carry a trailing comment. Anything else with a
        # space in it is a flow scalar, a list, or a mistake; all three are
        # things a version pin is not.
        val = val.split(" #", 1)[0].strip()
        if not val or re.search(r"[\s\"']", val):
            raise Refuse(f"{name} at line {lineno} is {raw.strip()!r}, which this reader does not "
                         "recognise as a version pin — quote it, or read it where it lives")
    if not val:
        raise Refuse(f"{name} at line {lineno} is empty. A pin nothing sets is a pin nothing "
                     "installs, and the caller would have gone on to install whatever was on PATH")
    return val


def read_pin(text: str, name: str, path: str) -> str:
    """`name`'s value from `text`'s workflow-level `env:` block.

    Two refusals carry the whole point of this file, so they are checked in
    this order: the name appearing MORE THAN ONCE anywhere in the file (the
    ambiguity the old idiom resolved by position), and the name being absent
    from the workflow-level block (the scope the old idiom never checked).
    """
    if not NAME_RE.match(name):
        raise Refuse(f"{name!r} is not an env-key name this reader will look for")

    lines = text.splitlines()
    # TEXTUAL, NOT STRUCTURAL, and knowingly so: a `NAME:`-shaped line inside a
    # `run: |` block or a comment counts as a setting here. That over-counts
    # towards a refusal, never towards a wrong value, which is the direction
    # this reader is allowed to be wrong in.
    key_re = re.compile(rf"^[ \t]*{re.escape(name)}:")
    hits = [i + 1 for i, ln in enumerate(lines) if key_re.match(ln)]
    if len(hits) > 1:
        raise Refuse(
            f"{name} is set {len(hits)} times in {path} — lines {', '.join(map(str, hits))}. "
            "This reader will not choose between them. The version a lane installs has to have "
            "one answer, so either the extra settings are per-job pins that belong where they are "
            "read, or one of them is the source of truth and the others are drift; the old "
            "`sed | head -1` took whichever came first in the file and said nothing")
    if not hits:
        raise Refuse(f"{path} does not set {name} anywhere. That pin is this caller's input, and "
                     "this repo keeps every tool version in that file's workflow-level `env:` block")

    tops = [i for i, ln in enumerate(lines) if TOP_ENV_RE.match(ln)]
    if len(tops) != 1:
        raise Refuse(f"{path} has {len(tops)} workflow-level `env:` block(s) (a mapping key at column "
                     "0). This reader is anchored to exactly one, because 'the version this "
                     "workflow declares' is only a well-formed question when there is one")
    start = tops[0]
    end = len(lines)
    for i in range(start + 1, len(lines)):
        ln = lines[i]
        # A COLUMN-0 COMMENT INSIDE THE BLOCK ENDS IT EARLY, for this scan.
        # YAML would keep reading; this stops. The consequence is a refusal
        # with the wrong diagnosis ("OUTSIDE the workflow-level `env:` block")
        # rather than a wrong value, so it fails closed — but a reader who
        # meets that message should check for a flush-left `#` first.
        if ln.strip() and not ln.startswith((" ", "\t")):
            end = i
            break

    lineno = hits[0]
    if not start + 1 < lineno <= end:
        raise Refuse(
            f"{name} is set at line {lineno} of {path}, which is OUTSIDE the workflow-level `env:` "
            f"block at line {start + 1}. A job- or step-level pin is scoped to that job or step; "
            "reading it from here would install it everywhere this reader is called")
    return unquote(lines[lineno - 1].split(":", 1)[1], name, lineno)


def main(argv: list[str]) -> int:
    ap = argparse.ArgumentParser(add_help=True, description=__doc__.splitlines()[0])
    ap.add_argument("name", nargs="?", help="the env key to read, e.g. NEXTEST_VERSION")
    ap.add_argument("--file", default=CI_YML, help=f"the workflow to read (default {CI_YML})")
    ap.add_argument("--selftest", action="store_true", help="run this reader's own tests")
    args = ap.parse_args(argv)

    if args.selftest:
        return selftest()
    if not args.name:
        ap.error("a name to read is required (or --selftest)")
    try:
        with open(args.file, encoding="utf-8") as fh:
            text = fh.read()
    except OSError as exc:
        die(f"cannot read {args.file}: {exc}")
    try:
        print(read_pin(text, args.name, args.file))
    except Refuse as why:
        die(str(why))
    return 0


# --------------------------------------------------------------------------
# The self-test. Fixtures only: it writes miniature workflows to a temp
# directory and asks this script, as a subprocess, what it says about them — so
# what is exercised is the command line the workflows actually run, not an
# in-process shortcut past it.
# --------------------------------------------------------------------------

FIXTURE = """name: CI
on:
  push:
    branches: [main]
jobs:
  build:
    steps:
      - run: echo hi
env:
  # a comment inside the block
  SCCACHE_VERSION: "0.16.0"
  NEXTEST_VERSION: "0.9.140"
  BARE_VERSION: 1.2.3 # with a trailing comment
  SINGLE_VERSION: '4.5.6'

defaults:
  run:
    shell: bash
"""


def _run(args: list[str], cwd: str) -> tuple[int, str, str]:
    p = subprocess.run([sys.executable, os.path.abspath(__file__), *args],
                       cwd=cwd, capture_output=True, text=True)
    return p.returncode, p.stdout.strip(), p.stderr.strip()


def selftest() -> int:
    fails: list[str] = []

    def check(cond: bool, msg: str) -> None:
        if not cond:
            fails.append(msg)

    with tempfile.TemporaryDirectory() as t:
        def write(body: str, name: str = "wf.yml") -> str:
            with open(os.path.join(t, name), "w", encoding="utf-8") as fh:
                fh.write(body)
            return name

        # 1. THE HAPPY PATH, in all three scalar spellings the block may hold.
        wf = write(FIXTURE)
        for name, want in (("NEXTEST_VERSION", "0.9.140"), ("SCCACHE_VERSION", "0.16.0"),
                           ("BARE_VERSION", "1.2.3"), ("SINGLE_VERSION", "4.5.6")):
            rc, out, err = _run([name, "--file", wf], t)
            check(rc == 0 and out == want,
                  f"{name}: wanted {want!r}, got rc={rc} out={out!r} err={err!r}")

        # 2. A SECOND MATCH ANYWHERE IN THE FILE. This is the defect: the
        # job-level pin sits ABOVE the workflow block, which is exactly where
        # `head -1` would have taken it from without saying so.
        body = FIXTURE.replace("      - run: echo hi\n",
                               '    env:\n      NEXTEST_VERSION: "0.9.99"\n')
        wf = write(body, "second.yml")
        want_lines = [str(i + 1) for i, ln in enumerate(body.splitlines())
                      if ln.strip().startswith("NEXTEST_VERSION:")]
        rc, out, err = _run(["NEXTEST_VERSION", "--file", wf], t)
        check(rc != 0, f"a second NEXTEST_VERSION setting returned a value anyway: {out!r}")
        check(out == "", f"a second setting still printed something to install: {out!r}")
        check("set 2 times" in err, f"the refusal did not say the name was set twice: {err!r}")
        check(f"lines {', '.join(want_lines)}" in err,
              f"the refusal did not name lines {want_lines}: {err!r}")

        # A second match BELOW the workflow block refuses just the same: the
        # rule is one answer in the file, not one answer before the anchor.
        wf = write(FIXTURE + '\nsomething:\n  env:\n    NEXTEST_VERSION: "0.9.98"\n', "below.yml")
        rc, out, err = _run(["NEXTEST_VERSION", "--file", wf], t)
        check(rc != 0 and out == "", f"a later duplicate was resolved by position: {out!r} {err!r}")
        check("set 2 times" in err, f"the later duplicate refused for another reason: {err!r}")

        # 3. A NAME THAT IS ABSENT. The old idiom returned empty here and each
        # caller carried its own `test -n`; this refuses centrally.
        wf = write(FIXTURE)
        rc, out, err = _run(["ABSENT_VERSION", "--file", wf], t)
        check(rc != 0 and out == "", f"an absent name yielded {out!r}")
        check("ABSENT_VERSION" in err, f"the refusal did not name what was missing: {err!r}")

        # 4. PRESENT, BUT OUT OF SCOPE — the only setting is under a job. The
        # value exists, and `sed | head -1` would have printed it happily.
        wf = write('jobs:\n  b:\n    env:\n      NEXTEST_VERSION: "9.9.9"\nenv:\n  OTHER: "1"\n',
                   "scoped.yml")
        rc, out, err = _run(["NEXTEST_VERSION", "--file", wf], t)
        check(rc != 0 and out == "", f"a job-scoped pin was read as the workflow's: {out!r}")
        check("OUTSIDE" in err, f"the refusal did not say it was out of scope: {err!r}")

        # 5. NO WORKFLOW-LEVEL BLOCK AT ALL, and TWO of them: both are
        # structure this reader does not recognise, and neither is a value.
        #
        # THE NAME IS SET IN THE no-block FIXTURE, and that is the case, not
        # decoration: with no setting anywhere the ABSENT-name refusal fires
        # first and this arm passes without reaching the block count at all —
        # which is how it was written, and is the shape of defect this whole
        # unit is about. Every arm below asserts the refusal's REASON, because
        # a nonzero exit alone cannot tell a case that fired from a case that
        # was never reached.
        wf = write('jobs:\n  b:\n    env:\n      NEXTEST_VERSION: "9.9.9"\n', "noenv.yml")
        rc, out, err = _run(["NEXTEST_VERSION", "--file", wf], t)
        check(rc != 0 and out == "", f"a file with no env block yielded {out!r}")
        check("0 workflow-level" in err,
              f"the no-env-block refusal was not the one that fired: {err!r}")
        wf = write('env:\n  A: "1"\nenv:\n  NEXTEST_VERSION: "2"\n', "twoenv.yml")
        rc, out, err = _run(["NEXTEST_VERSION", "--file", wf], t)
        check(rc != 0 and out == "", f"two workflow-level env blocks yielded {out!r}")
        check("2 workflow-level" in err, f"the refusal did not count the blocks: {err!r}")

        # 6. A VALUE THAT SETS NOTHING. The two spellings reach two DIFFERENT
        # refusals — the bare one is unreadable as a pin, the quoted one is a
        # pin that is empty — and each is asserted by its own words so neither
        # can quietly start answering for the other.
        for body, want, why in (
            ("env:\n  NEXTEST_VERSION:\n", "does not recognise", "an empty bare pin"),
            ('env:\n  NEXTEST_VERSION: ""\n', "is empty", "an empty quoted pin"),
        ):
            wf = write(body, "empty.yml")
            rc, out, err = _run(["NEXTEST_VERSION", "--file", wf], t)
            check(rc != 0 and out == "", f"{why} yielded {out!r}")
            check(want in err, f"{why} did not refuse with {want!r}: {err!r}")

        # 7. A NAME THIS READER WILL NOT LOOK FOR. `NAME_RE` is the narrowness
        # of the contract — an env key, not a YAML path — and an unasserted
        # guard is a guard nobody has run.
        wf = write(FIXTURE)
        for bad in ("nextest_version", "jobs.build.env", "9LIVES"):
            rc, out, err = _run([bad, "--file", wf], t)
            check(rc != 0 and out == "", f"{bad!r} was accepted as an env-key name: {out!r}")
            check("not an env-key name" in err,
                  f"{bad!r} was refused for the wrong reason: {err!r}")

        # 8. A MISSING FILE is a refusal, not an empty string. The callers run
        # under `set -e`, so this is what stops them installing from PATH.
        rc, out, err = _run(["NEXTEST_VERSION", "--file", "nope.yml"], t)
        check(rc != 0 and out == "", f"a missing workflow yielded {out!r}")
        check("nope.yml" in err, f"the refusal did not name the file: {err!r}")

    if fails:
        sys.stderr.write("ci-pin.py --selftest FAILED\n")
        for f in fails:
            sys.stderr.write(f"  - {f}\n")
        return 1
    print("ci-pin.py --selftest: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
