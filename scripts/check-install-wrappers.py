#!/usr/bin/env python3
"""An install idiom this repo has WRAPPED is spelled only through its wrapper.

WHY THIS EXISTS. `apt-get update` reports one exit status for every source list
it was pointed at, and the runner image ships lists this repo never asked for;
when one of them publishes a broken index the step dies having fetched
everything the job actually needed. `scripts/apt-install.sh` is the answer —
it narrows the update to Ubuntu's own archive, retries a hang, and refuses
loudly when a package really is missing — and PR 2277 routed every apt call
site in `.github/workflows/` through it. Nothing kept them there. A step added
tomorrow spelling `sudo apt-get update && sudo apt-get install -y foo` inline
is exactly the state that PR removed, and every other check in the tree passes
it: `check-ci-mirror-parity.py` reads workflows for `scripts/` and `demos/`
INVOCATIONS, so a step that invokes no script is outside its claims 1-4 by
construction; its claim 9 is about jobs, not steps; and no shell linter runs in
the hosted gate at all.

WHAT THIS PROVES. In every shell body this repo's CI can run — every `run:`
under `.github/workflows/` and `.github/actions/`, and every tracked `*.sh` /
`*.bash` file and every tracked file with a `sh`/`bash` shebang — no command
whose head word is a WRAPPED IDIOM (`IDIOMS` below: today the apt family) runs
at all, except in a file `PATH_EXEMPT` names with its reason.

IT IS THE CLASS, NOT THE INCIDENT. The subject is *a preamble re-spelled
inline where a wrapper exists for it*; apt is the instance that has a wrapper
today. A sibling idiom earns a row in `IDIOMS` at the moment something in this
repo wraps it, and the check is then the same check. `IDIOMS` is not a list of
things that are bad — `pip install` inside a demo venv is fine and has no
wrapper — it is the list of doors that already exist.

HEAD WORD, NOT SUBSTRING, and that distinction is the whole recogniser.
`ci-local.sh` tells a developer `admesh not installed (apt admesh, or build
…)`; a check that matched the token anywhere would red on that sentence, and a
check that reds on correct text gets routed around and then detects nothing
(the lesson `check-ci-mirror-parity.py`'s `CMD_BREAK` comment records from
meeting it twice). So a command's words are walked left to right, assignments,
redirections, numbers, flags and the wrapper words in `PREFIX_WORDS` are
consumed, and the FIRST word that is none of those is the command being run.
`echo "apt admesh"` runs `echo`.

STRINGS AND HEREDOCS ARE CODE HERE, COMMENTS ARE NOT, and that is the exact
inverse of `check-status-capture.py`, whose header calls a heredoc body and a
single-quoted string data. Both are right about their own subject: a
`PIPESTATUS` read inside a heredoc never executes, while `bash -c 'sudo
apt-get install -y foo'` and `ssh host <<EOF … EOF` are how the preamble gets
written when the obvious spelling is refused. So quoting is removed rather
than obeyed, a `bash -c` string and an `eval` argument are re-scanned as shell,
and every heredoc body is scanned as shell too. A `#` comment is the one thing
that is prose: a tombstone quoting the shape this check forbids must stay
green, and every workflow in this tree carries paragraphs of them.

WHY A SEPARATE SCRIPT, weighed against the two shapes the item named and the
one that landed since. Not `scripts/gates/*`: that directory is Track K's, its
members read neither `.github/workflows/` nor `local-scripts/`, and its roster
argument (`gate-roster.sh`) is that the directory means `lib.sh`'s two-mode
bash contract. Not an arm in `check-ci-mirror-parity.py`: its subject is paths
and invocations and its header calls the opacity of a `run:` body deliberate —
an arm here would widen what a block scalar MEANS to every claim in a 4900-line
file whose claims share one tokenizer, so a change made for this property could
move another claim's answer. `check-status-capture.py` took the third road for
the same reason, blast radius rather than subject, and this file is its
sibling: its own reader, its own selftest, one `TIER_BLIND` membership entry,
and it fails alone. Its tokenizer is NOT reused, because the two disagree about
heredocs and quoting in exactly the places each one's property lives, and
teaching that file both answers is the coupling both arguments exist to avoid.

WHAT IT CANNOT SEE, stated because a disclosed blind spot is a work order:
  * A COMMAND NAME COMPOSED AT RUN TIME. `CMD=apt-get; $CMD install -y foo`
    leaves the head word `install`. `$sudo apt-get …` IS caught — an
    expansion in a PREFIX position is stepped over — but an expansion that is
    the command name itself cannot be resolved without running the shell.
  * WHAT A PIPE FEEDS A SHELL. `curl … | sudo bash` runs code this file never
    sees; so does `bash ./fetched.sh`. The idiom's own row is the guard for
    the spellings written down.
  * A `uses:` ACTION'S SIDE EFFECTS, and any shell in a file this population
    does not cover — an untracked script, a container image's entrypoint.
  * WHETHER THE WRAPPER IS USED CORRECTLY. That `scripts/apt-install.sh` is
    the only door is what is proved; the door's own behaviour is its
    `--selftest`, which ci.yml's `mirror` job runs beside this one.

AN UNREADABLE INPUT IS A REFUSAL, never an empty reading: an unterminated
quote, an unterminated heredoc, a folded `run: >` scalar, a `run:` with no
body, a population with no workflow in it, and a run that scans no command at
all each raise `Bail` and fail the check naming the site. The alternative — a
body that parses to nothing and is reported as "no apt-get here" — is the
green-over-nothing failure this program has now met three times.

THE EXEMPTION TABLE EXPIRES. An entry in `PATH_EXEMPT` whose file no longer
runs the idiom it excuses is an ERROR, not a fossil: the confession outlived
what it confessed and the next reader would take it for a live fact. Same for
`IDIOMS` itself — a wrapper path that is no longer tracked fails, because a
prohibition pointing at a door that has been bricked up is not a rule.

  check-install-wrappers.py [--selftest] [--root DIR]
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

# A GitHub expression can hold quotes, pipes and braces that are not shell.
# Masked to one opaque word before the body is read, exactly as
# `check-ci-mirror-parity.py` masks it, so it cannot open a string that never
# closes. The mask keeps a `$` so the word still reads as an expansion.
GH_EXPR = re.compile(r"\$\{\{.*?\}\}", re.DOTALL)
GH_MASK = "$__gh_expr__"

# --------------------------------------------------------------------------
# THE TABLE. `head word -> (wrapper, what the wrapper is for)`. A row says: an
# idiom this repo has a door for, and the door. Adding a row is adding a door,
# never adding an opinion — the argument for each is at the wrapper it names.
# --------------------------------------------------------------------------
IDIOMS: dict[str, tuple[str, str]] = {
    "apt-get": (
        "scripts/apt-install.sh",
        "the apt preamble. `apt-get update` fails for ANY source list on the "
        "runner image, including third-party lists this repo never asked for; "
        "the wrapper narrows the update to Ubuntu's own archive for one "
        "transaction, bounds and retries a hang, and refuses a missing "
        "package loudly instead of dressing it up as a mirror outage",
    ),
    # The same binary by its other names. `apt` is the interactive front end
    # and warns that it has no stable CLI; `aptitude` is a third resolver over
    # the same source lists. All three fail the same way for the same reason,
    # so a door that only covered the spelling in use today would be a door
    # somebody walks around by typing four fewer characters.
    "apt": ("scripts/apt-install.sh", "see `apt-get`"),
    "aptitude": ("scripts/apt-install.sh", "see `apt-get`"),
    # The other end of the same failure: this ADDS a foreign source list, so
    # every later `apt-get update` in the job inherits the outage the wrapper
    # exists to survive — and the wrapper would then set that list aside,
    # making the install that motivated it fail in a way nobody expects. A
    # third-party archive is a design decision, not a step.
    "add-apt-repository": (
        "scripts/apt-install.sh",
        "adding a third-party source list, which is the condition the wrapper "
        "is written against rather than a use of it",
    ),
}

# --------------------------------------------------------------------------
# Files allowed to run an idiom directly, with the reason. An entry is a
# confession, not a disposition, and `exempt_fossils` errors on one whose file
# no longer runs anything — see the header.
# --------------------------------------------------------------------------
PATH_EXEMPT: dict[str, str] = {
    "scripts/apt-install.sh": (
        "the wrapper itself. It is the one place in this repo that says "
        "`apt-get`, which is what every other entry in `IDIOMS` points at"
    ),
}

# Words that stand in FRONT of the command actually being run. Each either
# runs its remaining argv (`env`, `exec`, `xargs`, `time`, `nice`, `timeout`,
# `command`, `builtin`, and the privilege-raising pair) or is shell grammar
# that a `;`-split leaves glued to the command (`if`, `while`, `!`).
# Consuming them is what makes
# `DEBIAN_FRONTEND=noninteractive sudo -E timeout -k 10 60 apt-get install`
# one command whose head is `apt-get`.
PREFIX_WORDS = frozenset({
    "sudo", "doas", "env", "exec", "command", "builtin", "nohup", "setsid",
    "time", "nice", "ionice", "stdbuf", "timeout", "xargs", "then", "do",
    "else", "elif", "if", "while", "until", "!",
})
# NOT `eval`: its argument is one word to the outer shell and a whole script
# to the inner one, so it is a recursion in `offenders` rather than a word to
# step over. Stepping over it would leave the head word
# `sudo apt-get install -y foo`, which is no command at all.

# Shells whose `-c` argument is a script. Its value is re-scanned as shell.
SHELL_COMMANDS = frozenset({"bash", "sh", "dash", "zsh", "ksh"})

ASSIGNMENT = re.compile(r"^[A-Za-z_][A-Za-z_0-9]*(\[[^]]*\])?\+?=")
REDIRECTION = re.compile(r"^[0-9]*[<>]")
NUMERIC = re.compile(r"^[0-9]+(\.[0-9]+)?[smhd]?$")


class Bail(Exception):
    """A shape this script cannot read. Fails the run rather than passing it."""


class Word:
    """One shell word: the value quoting leaves behind, and whether an
    expansion contributed to it."""

    __slots__ = ("expands", "value")

    def __init__(self) -> None:
        self.value = ""
        self.expands = False

    def __repr__(self) -> str:  # pragma: no cover - diagnostics only
        return f"Word({self.value!r}, expands={self.expands})"


class Command:
    """One simple command, with the line its first word starts on."""

    __slots__ = ("line", "words")

    def __init__(self, line: int) -> None:
        self.line = line
        self.words: list[Word] = []

    def __repr__(self) -> str:  # pragma: no cover - diagnostics only
        return f"Command(line={self.line}, words={[w.value for w in self.words]})"

    def text(self) -> str:
        return " ".join(w.value for w in self.words)


def scan_commands(body: str, first_line: int = 1) -> list[Command]:
    """Split a shell body into the simple commands it runs, in order.

    Boundaries are every unquoted list operator (`;` `&` `&&` `||`), the
    newline, the pipe — each stage of a pipeline is a command of its own, so
    `curl … | sudo bash` names `bash` — and the grouping characters. Quoting
    is REMOVED rather than obeyed: the word `'apt-get'` is the word `apt-get`,
    because a check on the spelling of the quotes is not a check on what runs.
    A command substitution and a heredoc body are scanned as shell in their
    own right, at their own line numbers.
    """
    body = GH_EXPR.sub(GH_MASK, body)
    out: list[Command] = []
    line = first_line
    cmd = Command(line)
    word = Word()
    started = False  # the current word has characters, even if they are ''
    quote: str | None = None
    quote_line = 0
    # One entry per open `(` / `$(` (kind "p") or backtick (kind "b"), saving
    # what the outer command was. Inside a substitution the shell quotes
    # afresh, which is what makes `"$(cmd "$x")"` readable.
    nest: list[tuple[str, str | None, Command, Word, bool]] = []
    heredocs: list[tuple[str, bool]] = []
    line_start = 0  # where this logical line's commands begin in `out`
    i, n = 0, len(body)

    def end_word() -> None:
        nonlocal word, started
        if started:
            cmd.words.append(word)
        word = Word()
        started = False

    def end_command() -> None:
        nonlocal cmd
        end_word()
        if cmd.words:
            out.append(cmd)
        cmd = Command(line)

    def add(ch: str) -> None:
        nonlocal started
        word.value += ch
        started = True

    while i < n:
        ch = body[i]

        if quote == "'":
            if ch == "'":
                quote = None
            else:
                if ch == "\n":
                    line += 1
                add(ch)
            i += 1
            continue

        if quote == '"':
            if ch == "\\" and i + 1 < n:
                if body[i + 1] == "\n":
                    line += 1
                else:
                    add(body[i + 1])
                i += 2
                continue
            if ch == '"':
                quote = None
                i += 1
                continue
            if body.startswith("$(", i) or ch == "`":
                kind = "p" if ch == "$" else "b"
                if kind == "b" and nest and nest[-1][0] == "b":
                    end_command()
                    _, quote, cmd, word, started = nest.pop()
                    i += 1
                    continue
                word.expands = True
                started = True
                nest.append((kind, quote, cmd, word, started))
                quote = None
                cmd, word, started = Command(line), Word(), False
                i += 2 if kind == "p" else 1
                continue
            if ch == "$":
                word.expands = True
            if ch == "\n":
                line += 1
            add(ch)
            i += 1
            continue

        # ---- unquoted ----
        if ch == "\\" and i + 1 < n:
            if body[i + 1] == "\n":
                line += 1  # a line continuation joins the word; no boundary
            else:
                add(body[i + 1])
            i += 2
            continue

        if ch == "#" and not started and (i == 0 or body[i - 1] in " \t\n;&|("):
            j = body.find("\n", i)
            i = j if j != -1 else n
            continue

        if ch in "'\"":
            quote = ch
            quote_line = line
            started = True  # `''` is a word, and an empty one
            i += 1
            continue

        if ch == "\n":
            line += 1
            end_command()
            i += 1
            if heredocs:
                code = _feeds_a_shell(out[line_start:])
                i, line = _take_heredocs(body, i, line, heredocs, out, code)
                heredocs = []
                cmd = Command(line)
            line_start = len(out)
            continue

        if ch in " \t":
            end_word()
            i += 1
            continue

        if body.startswith("<<<", i):
            end_word()
            i += 3
            continue

        if body.startswith("<<", i):
            end_word()
            i, delim, strip = _heredoc_delim(body, i)
            heredocs.append((delim, strip))
            continue

        if body.startswith("$(", i) or ch == "`":
            kind = "p" if ch == "$" else "b"
            if kind == "b" and nest and nest[-1][0] == "b":
                end_command()
                _, quote, cmd, word, started = nest.pop()
                i += 1
                continue
            word.expands = True
            started = True
            nest.append((kind, quote, cmd, word, started))
            cmd, word, started = Command(line), Word(), False
            i += 2 if kind == "p" else 1
            continue

        if ch == "$":
            word.expands = True
            add(ch)
            i += 1
            if i < n and body[i] == "{":  # `${NAME…}` is one expansion
                depth = 0
                while i < n:
                    if body[i] == "{":
                        depth += 1
                    elif body[i] == "}":
                        depth -= 1
                        if depth == 0:
                            add(body[i])
                            i += 1
                            break
                    elif body[i] == "\n":
                        line += 1
                    add(body[i])
                    i += 1
            continue

        if ch == ")" and nest and nest[-1][0] == "p":
            end_command()
            _, quote, cmd, word, started = nest.pop()
            i += 1
            continue

        if ch in "()":
            end_command()
            i += 1
            continue

        if ch in "{}" and not started:
            end_command()
            i += 1
            continue

        if ch == "&":
            prev = word.value[-1:] if started else ""
            nxt = body[i + 1] if i + 1 < n else ""
            if prev in (">", "<") or nxt in (">", "<"):
                add(ch)
                i += 1
                continue
            end_command()
            i += 1
            continue

        if ch == "|":
            if i + 1 < n and body[i + 1] == "&":  # `|&`, one operator
                i += 1
            end_command()
            i += 1
            continue

        if ch == ";":
            end_command()
            i += 1
            continue

        add(ch)
        i += 1

    if quote is not None:
        raise Bail(
            f"line {quote_line}: a {quote!r} quote is never closed; the "
            "commands after it cannot be read"
        )
    if nest:
        raise Bail("an unterminated command substitution; cannot read its commands")
    if heredocs:
        raise Bail(f"a heredoc `{heredocs[0][0]}` with no body at all")
    end_command()
    return out


def _heredoc_delim(body: str, i: int) -> tuple[int, str, bool]:
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
    out = ""
    while i < len(body):
        c = body[i]
        if quote is not None:
            if c == quote:
                i += 1
                break
        elif c in " \t\n;&|<>()":
            break
        out += c
        i += 1
    if not out:
        raise Bail("a `<<` heredoc with no delimiter word; cannot find its body")
    return i, out, strip


def _take_heredocs(
    body: str,
    i: int,
    line: int,
    heredocs: list[tuple[str, bool]],
    out: list[Command],
    code: bool,
) -> tuple[int, int]:
    """Consume every pending heredoc body, scanning it as shell iff `code`.

    A HEREDOC IS CODE WHEN A SHELL EATS IT, and data otherwise, which is the
    one place this reader needs a question `check-status-capture.py` never
    asks (it skips every heredoc, because a `PIPESTATUS` read written into a
    file cannot execute). `bash <<EOF` and `cat <<EOF | bash` run what is in
    the body, so an install preamble hidden there is an install preamble.
    `cat > usage.txt <<EOF` does not, and the bodies in this tree that are
    not shell are Rust fixtures and usage text carrying `&'static str` and
    `input's own default` — unbalanced apostrophes that, read as shell, are
    an unterminated quote. Reading those as code would refuse three live
    files; reading them as data loses nothing, because nothing in them runs.

    The terminator must stand ALONE on its line whichever way it is read:
    bash accepts no leading whitespace at all, and `<<-` strips leading TABS
    only, so matching a stripped line would end the body early and read the
    rest of it twice.
    """
    for delim, strip in heredocs:
        start = line
        collected: list[str] = []
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
            collected.append(candidate)
        if code and collected:
            out.extend(scan_commands("\n".join(collected), start))
    return i, line


def _feeds_a_shell(line_commands: list[Command]) -> bool:
    """Does a shell read this logical line's heredoc?

    Any command on the line, because the heredoc is attached to one of them
    and `cat <<EOF | bash` attaches it to the stage that is not the shell.
    """
    for cmd in line_commands:
        head, _rest = head_of(cmd.words)
        if head is not None and os.path.basename(literal(head)) in SHELL_COMMANDS:
            return True
    return False


# Everything quoting and expansion leave behind. `${PREFIX}apt-get` keeps its
# literal tail and is read as `apt-get`; `$sudo` leaves nothing at all, which
# is what lets it be stepped over as a prefix rather than mistaken for the
# command.
EXPANSION = re.compile(
    r"\$\{[^{}]*\}|\$[A-Za-z_][A-Za-z_0-9]*|\$[-@*#?$!0-9]|\$"
)


def literal(word: Word) -> str:
    return EXPANSION.sub("", word.value)


def head_of(words: list[Word]) -> tuple[Word | None, list[Word]]:
    """The command a simple command actually runs, and its remaining words.

    Walks off the things that stand in front of it: assignments,
    redirections, the wrapper words in `PREFIX_WORDS` and their flags and
    durations, and words that are nothing but an expansion. `None` when the
    command is all prefix and no command.
    """
    idx = 0
    while idx < len(words):
        value = literal(words[idx])
        if not value:  # `$sudo`, `""`, a masked `${{ … }}`
            idx += 1
            continue
        if ASSIGNMENT.match(value):
            idx += 1
            continue
        if REDIRECTION.match(value):
            # `>out` carries its target; a bare `>` takes the next word.
            idx += 2 if value.rstrip("<>") == "" else 1
            continue
        if value in PREFIX_WORDS or value.startswith("-") or NUMERIC.match(value):
            # `command -v apt-get` LOOKS A BINARY UP and runs nothing. A
            # check that reds on the one idiom test a script may legitimately
            # write is a check that gets routed around.
            if value == "command" and idx + 1 < len(words) \
                    and literal(words[idx + 1]) in ("-v", "-V"):
                return None, []
            idx += 1
            continue
        return words[idx], words[idx + 1 :]
    return None, []


def offenders(body: str, first_line: int = 1) -> list[tuple[int, str, str]]:
    """Every command in `body` that runs a wrapped idiom: (line, name, text)."""
    found: list[tuple[int, str, str]] = []
    for cmd in scan_commands(body, first_line):
        head, rest = head_of(cmd.words)
        if head is None:
            continue
        name = os.path.basename(literal(head))
        if name in IDIOMS:
            found.append((cmd.line, name, cmd.text()))
            continue
        if name == "eval":
            # `eval "sudo apt-get install -y foo"` is one word to the outer
            # shell and a whole script to the inner one.
            found.extend(offenders(" ".join(w.value for w in rest), cmd.line))
            continue
        if name in SHELL_COMMANDS:
            script = _dash_c_argument(rest)
            if script is not None:
                found.extend(offenders(script, cmd.line))
    return found


def _dash_c_argument(rest: list[Word]) -> str | None:
    """The script a `bash -c` / `sh -lc` runs, if this invocation has one."""
    for pos, word in enumerate(rest):
        value = literal(word)
        if not value.startswith("-") or value.startswith("--"):
            continue
        if "c" in value[1:] and pos + 1 < len(rest):
            return rest[pos + 1].value
    return None


# --------------------------------------------------------------------------
# The population. Every shell body this repo's CI can run.
# --------------------------------------------------------------------------

def yaml_bodies(path: str, text: str) -> list[tuple[int, str]]:
    """Every `run:` shell body in a workflow or composite action, as
    (first line, body).

    A second reader of the same key as `check-status-capture.py`'s, and
    deliberately not a shared one: the two hand their bodies to tokenizers
    that disagree about heredocs and quoting, and one file raising the
    other's `Bail` is the coupling both scripts are separate to avoid.
    """
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
            bodies.append((i + 1, _plain(path, i + 1, stripped)))
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


def _plain(path: str, lineno: int, scalar: str) -> str:
    """The shell inside an inline `run:` scalar, YAML quoting removed.

    `run: "cargo test … m4_pr8_latency::"` is a QUOTED YAML scalar, and
    handing it to the shell reader with its quotes on makes the whole command
    one word — so the row runs no command this check can see, and
    `run: "sudo apt-get install -y foo"` would be read as a step that invokes
    nothing. A scalar that opens with a quote and does not close at its end is
    a YAML shape this reader does not know, and refuses.
    """
    if scalar[:1] not in ("'", '"'):
        return scalar
    quote, out, i = scalar[0], "", 1
    while i < len(scalar):
        ch = scalar[i]
        if ch == quote:
            if quote == "'" and scalar[i + 1 : i + 2] == "'":
                out += "'"  # YAML doubles a single quote to escape it
                i += 2
                continue
            if i == len(scalar) - 1:
                return out
            break
        if quote == '"' and ch == "\\" and i + 1 < len(scalar):
            out += scalar[i + 1]
            i += 2
            continue
        out += ch
        i += 1
    raise Bail(
        f"{path}:{lineno}: a `run:` scalar that opens with {quote!r} and does "
        "not close at its end; this reader does not know that YAML shape"
    )


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
    """Tracked `*.sh` / `*.bash`, plus any tracked file with a sh/bash
    shebang — `local-scripts/hooks/pre-push` carries one and no suffix."""
    found = []
    for rel in _tracked(root):
        if rel.endswith(SHELL_SUFFIXES):
            found.append(rel)
            continue
        try:
            with open(os.path.join(root, rel), "rb") as fh:
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
                    found.append(os.path.relpath(os.path.join(dirpath, name), root))
    if not found:
        raise Bail(
            f"no YAML under {' or '.join(YAML_ROOTS)}; this check has nothing to read"
        )
    return sorted(found)


def _read(root: str, rel: str) -> str:
    with open(os.path.join(root, rel), encoding="utf-8") as fh:
        return fh.read()


def scan_tree(root: str) -> tuple[dict[str, list[tuple[int, str, str]]], int, int]:
    """Every idiom invocation in the population, by path, with a census."""
    hits: dict[str, list[tuple[int, str, str]]] = {}
    files = 0
    commands = 0

    for rel in yaml_files(root):
        files += 1
        for first, body in yaml_bodies(rel, _read(root, rel)):
            commands += len(scan_commands(body, first))
            found = offenders(body, first)
            if found:
                hits.setdefault(rel, []).extend(found)

    for rel in shell_files(root):
        files += 1
        body = _read(root, rel)
        commands += len(scan_commands(body))
        found = offenders(body)
        if found:
            hits.setdefault(rel, []).extend(found)

    if not commands:
        raise Bail(
            f"{files} file(s) read and not one command scanned out of them; "
            "the reader is broken and a green here would mean nothing"
        )
    return hits, files, commands


# --------------------------------------------------------------------------
# The three arms, named. `check_tree` runs exactly these and the selftest
# asserts the set by name, so an arm dropped, renamed or duplicated reds
# saying which rather than passing at a shorter roster.
# --------------------------------------------------------------------------
CLAIM_NAMES = ("inline-idiom", "exempt-fossil", "wrapper-tracked")


def check_tree(root: str) -> tuple[dict[str, list[str]], int, int, int]:
    """Returns (failures by claim, files read, commands scanned, exempt uses)."""
    failures: dict[str, list[str]] = {name: [] for name in CLAIM_NAMES}
    hits, files, commands = scan_tree(root)

    # CLAIM `inline-idiom`: nobody outside the exemption table runs one.
    for rel in sorted(hits):
        if rel in PATH_EXEMPT:
            continue
        for line, name, text in hits[rel]:
            wrapper, why = IDIOMS[name]
            failures["inline-idiom"].append(
                f"{rel}:{line}: runs `{name}` directly — `{text}`\n"
                f"      {wrapper} is the door: {why}"
            )

    # CLAIM `exempt-fossil`: a confession outlives what it confessed.
    tracked = set(_tracked(root))
    exempt_uses = 0
    for rel, why in sorted(PATH_EXEMPT.items()):
        used = hits.get(rel, [])
        exempt_uses += len(used)
        if rel not in tracked:
            failures["exempt-fossil"].append(
                f"{rel} is exempt and is not a tracked file. Delete the entry."
            )
        elif not used:
            failures["exempt-fossil"].append(
                f"{rel} is exempt — \"{why}\" — and runs no wrapped idiom at "
                "all any more. The confession has expired: delete the entry."
            )

    # CLAIM `wrapper-tracked`: every door the table names still exists.
    for name, (wrapper, _why) in sorted(IDIOMS.items()):
        if wrapper not in tracked:
            failures["wrapper-tracked"].append(
                f"`{name}` is routed through {wrapper}, which is not a tracked "
                "file. A prohibition pointing at a door that is gone is not a rule."
            )

    return failures, files, commands, exempt_uses


# --------------------------------------------------------------------------
# THE MUTANT TABLE. Every row is a way the preamble could be re-spelled, and
# the verdict it must draw. A row is not a row until it has been watched go
# red: each is run through EVERY route a body reaches the parser by, on a
# tree holding that route alone, and its verdict asserted per route — a row
# that only ever arrives by one of them leaves the others deletable with
# nothing red, and one tree holding all four cannot say which one answered.
# --------------------------------------------------------------------------

GREEN, RED, BAIL = "green", "red", "bail"

MUTANTS: tuple[tuple[str, str, str, tuple[int, ...]], ...] = (
    (
        "the state PR 2277 removed, written inline again",
        "sudo apt-get update && sudo apt-get install -y foo\n",
        RED,
        (1, 1),
    ),
    (
        "the same pair split over a line continuation",
        "sudo apt-get update -qq \\\n  && sudo apt-get install -y -qq foo\n",
        RED,
        (1, 2),
    ),
    (
        "the WORD split over a line continuation: `apt-\\<newline>get`",
        "sudo apt-\\\nget install -y foo\n",
        RED,
        (1,),
    ),
    (
        "no `sudo` at all (a root container)",
        "apt-get install -y foo\n",
        RED,
        (1,),
    ),
    (
        "`apt` rather than `apt-get`",
        "sudo apt install -y foo\n",
        RED,
        (1,),
    ),
    (
        "`aptitude`, a third resolver over the same source lists",
        "sudo aptitude install foo\n",
        RED,
        (1,),
    ),
    (
        "adding a third-party source list, which is the failure itself",
        "sudo add-apt-repository -y ppa:someone/ppa\n",
        RED,
        (1,),
    ),
    (
        "an environment assignment in front of it",
        "DEBIAN_FRONTEND=noninteractive apt-get install -y foo\n",
        RED,
        (1,),
    ),
    (
        "the sudo spelled through a variable (session-start.sh's shape)",
        "DEBIAN_FRONTEND=noninteractive $sudo apt-get update -qq\n",
        RED,
        (1,),
    ),
    (
        "wrapped in `timeout`, whose flag and duration are not the command",
        "sudo -E timeout -k 10 60 apt-get install -y foo\n",
        RED,
        (1,),
    ),
    (
        "by absolute path",
        "/usr/bin/apt-get install -y foo\n",
        RED,
        (1,),
    ),
    (
        "with the command name quoted, which changes nothing about what runs",
        "sudo 'apt-get' install -y foo\n",
        RED,
        (1,),
    ),
    (
        "inside a `bash -c` string",
        "sudo bash -c 'apt-get update && apt-get install -y foo'\n",
        RED,
        (1, 1),
    ),
    (
        "inside a `sh -lc` string, whose flags are bundled",
        'sh -lc "apt-get install -y foo"\n',
        RED,
        (1,),
    ),
    (
        "inside an `eval`ed string",
        "eval \"sudo apt-get install -y foo\"\n",
        RED,
        (1,),
    ),
    (
        "inside a heredoc body, which this reader runs and does not file",
        "bash <<'EOF'\nsudo apt-get install -y foo\nEOF\n",
        RED,
        (2,),
    ),
    (
        "inside a `<<-` heredoc, whose terminator is tab-indented",
        "bash <<-EOF\n\tsudo apt-get install -y foo\n\tEOF\n",
        RED,
        (2,),
    ),
    (
        "as a pipeline stage",
        "echo y | sudo apt-get install foo\n",
        RED,
        (1,),
    ),
    (
        "inside a command substitution",
        "out=$(apt-get -s install foo)\n",
        RED,
        (1,),
    ),
    (
        "through `xargs`",
        "printf '%s\\n' foo | xargs sudo apt-get install -y\n",
        RED,
        (1,),
    ),
    (
        "in a `run:` step whose shell is a subshell",
        "( cd /tmp && sudo apt-get install -y foo )\n",
        RED,
        (1,),
    ),
    # ---- shapes that are CORRECT and must stay green. A check that reds on a
    # ---- correct change gets routed around, and then detects nothing at all.
    (
        "the wrapper, which is the whole point",
        "exec scripts/apt-install.sh --no-install-recommends foo bar\n",
        GREEN,
        (),
    ),
    (
        "the wrapper's own selftest row",
        "scripts/apt-install.sh --selftest\n",
        GREEN,
        (),
    ),
    (
        "the idiom in a `#` comment — a tombstone quoting what is forbidden",
        "# sudo apt-get update && sudo apt-get install -y foo\nexec scripts/apt-install.sh foo\n",
        GREEN,
        (),
    ),
    (
        "a trailing comment after a real command",
        "scripts/apt-install.sh foo  # not sudo apt-get install\n",
        GREEN,
        (),
    ),
    (
        "the idiom inside an `echo` argument (ci-local.sh's prereq sentence)",
        'echo "ERROR: admesh not installed (apt admesh, or build from source)"\n',
        GREEN,
        (),
    ),
    (
        "the idiom inside a single-quoted argument of another command",
        "echo 'sudo apt-get install -y foo'\n",
        GREEN,
        (),
    ),
    (
        "`command -v apt-get`, which looks a binary up and runs nothing",
        "if command -v apt-get >/dev/null 2>&1; then echo yes; fi\n",
        GREEN,
        (),
    ),
    (
        "`which`/`dpkg-query` naming it as an ARGUMENT",
        "dpkg-query -W apt-get\n",
        GREEN,
        (),
    ),
    (
        "a `${{ … }}` expression, masked before the shell is read",
        "${{ matrix.prefix }} scripts/apt-install.sh foo\n",
        GREEN,
        (),
    ),
    (
        "a path that merely CONTAINS the token",
        "bash scripts/apt-install.sh-selftest.sh\n",
        GREEN,
        (),
    ),
    # ---- refusals. An unreadable body is never an empty reading.
    (
        "an unterminated quote is refused, not read as agreement",
        'echo "unterminated\n',
        BAIL,
        (),
    ),
    (
        "an unterminated heredoc is refused",
        "bash <<EOF\nsudo apt-get install -y foo\n",
        BAIL,
        (),
    ),
    (
        "a heredoc with no delimiter word is refused",
        "bash << \n",
        BAIL,
        (),
    ),
)

# The rows this guard exists for, asserted present by NAME. A table is a claim
# about coverage; this is what stops a row being dropped or renamed out of it
# while the run still prints "all as specified".
REQUIRED_MUTANTS = (
    "the state PR 2277 removed, written inline again",
    "the same pair split over a line continuation",
    "no `sudo` at all (a root container)",
    "`apt` rather than `apt-get`",
    "an environment assignment in front of it",
    "inside a `bash -c` string",
    "inside a heredoc body, which this reader runs and does not file",
    "the idiom in a `#` comment — a tombstone quoting what is forbidden",
    "the wrapper, which is the whole point",
    "an unterminated quote is refused, not read as agreement",
)

# The routes a body reaches the parser by, asserted by name for the same
# reason `REQUIRED_MUTANTS` is.
ROUTE_NAMES = ("action", "shebang", "suffix", "wf")

_WF_STUB = "jobs:\n  a:\n    steps:\n      - run: true\n"
_WRAPPER_STUB = "#!/usr/bin/env bash\n# the door\nsudo apt-get install -y \"$@\"\n"


def _write(root: str, rel: str, text: str) -> None:
    full = os.path.join(root, rel)
    os.makedirs(os.path.dirname(full), exist_ok=True)
    with open(full, "w", encoding="utf-8") as fh:
        fh.write(text)


def _add(root: str, *rels: str) -> None:
    subprocess.run(["git", "add", "-f", "--", *rels], cwd=root, check=True)


def _as_workflow(body: str) -> str:
    indented = "\n".join("          " + ln for ln in body.split("\n"))
    return f"jobs:\n  a:\n    steps:\n      - name: s\n        run: |\n{indented}\n"


def _as_action(body: str) -> str:
    indented = "\n".join("        " + ln for ln in body.split("\n"))
    return (
        "runs:\n  using: composite\n  steps:\n    - name: s\n      run: |\n"
        f"{indented}\n"
    )


class Carrier:
    """One route a shell body reaches the parser by, in a tree of its own."""

    __slots__ = ("label", "name", "offset", "rel", "root", "wrap")

    def __init__(self, tmp, name, label, rel, wrap, offset):
        self.root = os.path.join(tmp, name)
        self.name = name
        self.label = label
        self.rel = rel
        self.wrap = wrap
        self.offset = offset
        os.makedirs(self.root)
        subprocess.run(["git", "init", "-q"], cwd=self.root, check=True)
        if rel != ".github/workflows/ci.yml":
            # `yaml_files` refuses a tree with no workflow at all, so every
            # tree carries one; only this carrier's route holds the body.
            _write(self.root, ".github/workflows/ci.yml", _WF_STUB)
        # Every tree carries the wrapper, so the `exempt-fossil` arm is
        # satisfied on all of them and a red can only be the body's.
        _write(self.root, next(iter(PATH_EXEMPT)), _WRAPPER_STUB)
        _write(self.root, self.rel, self.wrap("true\n"))
        _add(self.root, ".", self.rel)

    def plant(self, body: str) -> None:
        _write(self.root, self.rel, self.wrap(body))
        _add(self.root, self.rel)


def _carriers(tmp: str) -> list[Carrier]:
    """Every route `scan_tree` reads a body by, one fixture each.

    The two shell routes are separate branches of `shell_files` and each has
    live members: `demos/hosted-render-guard.sh` is in the population by
    SUFFIX alone, `local-scripts/hooks/pre-push` by SHEBANG alone. A fixture
    carrying both marks pins neither branch.
    """
    return [
        Carrier(tmp, "wf", "workflow `run: |`",
                ".github/workflows/ci.yml", _as_workflow, 5),
        Carrier(tmp, "action", "composite action `run: |`",
                ".github/actions/h/action.yml", _as_action, 5),
        Carrier(tmp, "suffix", "tracked `*.sh`, no shebang", "t.sh",
                lambda b: "# shellcheck shell=bash\n" + b, 1),
        Carrier(tmp, "shebang", "tracked shebang file, no suffix",
                "hooks/pre-push", lambda b: "#!/bin/bash\n" + b, 1),
    ]


def _outcome(root: str) -> tuple[str, list[str], dict[str, list[str]]]:
    try:
        failures, _files, _commands, _uses = check_tree(root)
    except Bail:
        return BAIL, [], {}
    flat = [f for name in CLAIM_NAMES for f in failures[name]]
    return (RED if flat else GREEN), [f.split(": ", 1)[0] for f in flat], failures


def _fresh(tmp: str, name: str) -> str:
    root = os.path.join(tmp, name)
    os.makedirs(root)
    subprocess.run(["git", "init", "-q"], cwd=root, check=True)
    return root


def selftest() -> int:
    bad = 0
    seen_claims: set[str] = set()
    wrapper = next(iter(PATH_EXEMPT))

    def fail(msg: str) -> None:
        nonlocal bad
        bad += 1
        print(f"SELFTEST FAIL: {msg}")

    with tempfile.TemporaryDirectory() as tmp:
        carriers = _carriers(tmp)
        routes = ["direct"] + [c.label for c in carriers]

        for name, body, want, lines in MUTANTS:
            results: list[tuple[str, str, list[str]]] = []
            try:
                got = offenders(body)
                results.append(("direct", RED if got else GREEN,
                                [str(ln) for ln, _n, _t in got]))
            except Bail:
                got = []
                results.append(("direct", BAIL, []))
            if results[0][1] == want == RED:
                seen_claims.add("inline-idiom")
                got_lines = tuple(ln for ln, _n, _t in got)
                if got_lines != lines:
                    fail(f"{name}\n  lines {got_lines} != {lines}")

            for car in carriers:
                car.plant(body)
                outcome, sites, _f = _outcome(car.root)
                if outcome == want == RED:
                    expect = [f"{car.rel}:{ln + car.offset}" for ln, _n, _t in got]
                    if sites != expect:
                        outcome = f"red, but not at {expect}"
                results.append((car.label, outcome, sites))

            if any(r[1] != want for r in results):
                fail(f"{name}  (wanted {want})")
                for where, outcome, detail in results:
                    print(f"    {where}: {outcome}  {detail}")
            else:
                print(f"  ok  ({want:5})  {name}")

        # ---- the table's own membership. A row dropped or renamed out of it
        # ---- leaves a shorter run reporting the same "all as specified".
        names = [m[0] for m in MUTANTS]
        dupes = sorted({n for n in names if names.count(n) > 1})
        if dupes:
            fail(f"the mutant table names {dupes} more than once")
        missing = [n for n in REQUIRED_MUTANTS if n not in names]
        if missing:
            fail(f"the mutant table has lost {missing}")
        if not dupes and not missing:
            print(f"  ok  (green)  every row in `REQUIRED_MUTANTS` is in the "
                  f"table, and no row is in it twice ({len(names)} rows)")

        route_names = tuple(sorted(c.name for c in carriers))
        if route_names != ROUTE_NAMES:
            fail(f"the carriers are {route_names}, not {ROUTE_NAMES}")
        else:
            print("  ok  (green)  every route in `ROUTE_NAMES` has a carrier")

        # Both branches of `shell_files` are pinned, one carrier each: a
        # fixture carrying a suffix AND a shebang would pin neither.
        preamble = {c.name: c.wrap("").encode() for c in carriers}
        by_suffix = [c.name for c in carriers if c.rel.endswith(SHELL_SUFFIXES)
                     and not SHEBANG_RE.match(preamble[c.name])]
        by_shebang = [c.name for c in carriers if not c.rel.endswith(SHELL_SUFFIXES)
                      and SHEBANG_RE.match(preamble[c.name])]
        if len(by_suffix) != 1 or len(by_shebang) != 1:
            fail("shell_files' branches are not pinned one each; by suffix "
                 f"{by_suffix}, by shebang {by_shebang}")
        else:
            print("  ok  (green)  each branch of `shell_files` has a carrier "
                  "of its own")

        # ---- the population and refusal rows.
        pop = _fresh(tmp, "pop")
        _write(pop, ".github/workflows/ci.yml", _WF_STUB)
        _write(pop, wrapper, _WRAPPER_STUB)
        _add(pop, ".")
        outcome, sites, failures = _outcome(pop)
        _f2, _files, _cmds, uses = check_tree(pop)
        if outcome != GREEN or uses != 1:
            fail(f"the wrapper's own `apt-get` is not exempt: {outcome} {sites}")
        else:
            print(f"  ok  (green)  `{wrapper}` runs the idiom and is exempt")

        # The confession expires with what it confessed.
        _write(pop, wrapper, "#!/usr/bin/env bash\necho nothing to see\n")
        _add(pop, ".")
        outcome, _s, failures = _outcome(pop)
        if outcome != RED or not failures["exempt-fossil"]:
            fail(f"an exemption for a file that no longer runs the idiom "
                 f"passed: {outcome}")
        else:
            seen_claims.add("exempt-fossil")
            print("  ok  (red  )  an exemption whose file no longer runs the "
                  "idiom is a fossil")

        # A door that has been bricked up is not a rule.
        gone = _fresh(tmp, "gone")
        _write(gone, ".github/workflows/ci.yml",
               "jobs:\n  a:\n    steps:\n      - run: echo hello\n")
        _add(gone, ".")
        outcome, _s, failures = _outcome(gone)
        if outcome != RED or not failures["wrapper-tracked"]:
            fail(f"a wrapper that is not tracked at all passed: {outcome}")
        else:
            seen_claims.add("wrapper-tracked")
            print("  ok  (red  )  a wrapper the table names and the tree does "
                  "not have")

        if seen_claims != set(CLAIM_NAMES):
            fail(f"the rows above exercise {sorted(seen_claims)}, not every "
                 f"claim in {CLAIM_NAMES}")
        else:
            print("  ok  (green)  every claim in `CLAIM_NAMES` was watched red")

        # An unreadable `run:` shape fails rather than passing.
        folded = _fresh(tmp, "folded")
        _write(folded, ".github/workflows/ci.yml",
               "jobs:\n  a:\n    steps:\n      - run: >\n          folded\n")
        _write(folded, wrapper, _WRAPPER_STUB)
        _add(folded, ".")
        if _outcome(folded)[0] != BAIL:
            fail("a folded `run: >` scalar was read as agreement")
        else:
            print("  ok  (bail )  an unreadable `run:` shape fails rather "
                  "than passes")

        # A population with nothing in it is a refusal, not a green.
        empty = _fresh(tmp, "empty")
        _write(empty, ".github/workflows/ci.yml",
               "jobs:\n  a:\n    steps:\n      - uses: actions/checkout@v4\n")
        _add(empty, ".")
        if _outcome(empty)[0] != BAIL:
            fail("a tree with no command in it at all was reported green")
        else:
            print("  ok  (bail )  a population with no command scanned out of "
                  "it refuses")

        # ---- the inline `run:` scalar, and the QUOTED inline scalar, are
        # ---- two more routes into the parser, and the second turns a whole
        # ---- command into one shell word if its YAML quoting is not removed.
        inline = _fresh(tmp, "inline")
        _write(inline, wrapper, _WRAPPER_STUB)
        _write(inline, ".github/workflows/ci.yml",
               "jobs:\n  a:\n    steps:\n"
               "      - run: sudo apt-get install -y foo\n"
               '      - run: "apt-get install -y bar"\n'
               "      - run: 'sudo aptitude install baz'\n"
               '      - run: "cargo test -- --nocapture m4::"\n')
        _add(inline, ".")
        outcome, sites, _f = _outcome(inline)
        want_sites = [f".github/workflows/ci.yml:{n}" for n in (4, 5, 6)]
        if outcome != RED or sites != want_sites:
            fail(f"the inline `run:` scalars: {outcome} {sites} != {want_sites}")
        else:
            print("  ok  (red  )  the idiom in an inline `run:` scalar, plain "
                  "and YAML-quoted, beside a quoted scalar that is innocent")

        # A quoted scalar whose quote does not close at its end is a YAML
        # shape this reader does not know, and it refuses rather than
        # guessing which half is shell.
        odd = _fresh(tmp, "odd")
        _write(odd, wrapper, _WRAPPER_STUB)
        _write(odd, ".github/workflows/ci.yml",
               'jobs:\n  a:\n    steps:\n      - run: "echo a" x "echo b"\n')
        _add(odd, ".")
        if _outcome(odd)[0] != BAIL:
            fail("a `run:` scalar whose quoting this reader cannot read passed")
        else:
            print("  ok  (bail )  a `run:` scalar this reader cannot unquote "
                  "refuses")

        noyaml = _fresh(tmp, "noyaml")
        _write(noyaml, wrapper, _WRAPPER_STUB)
        _add(noyaml, ".")
        if _outcome(noyaml)[0] != BAIL:
            fail("a tree with no workflow at all was reported green")
        else:
            print("  ok  (bail )  a tree with no workflow refuses")

    if bad:
        print(f"\n{bad} selftest row(s) failed.")
        return 1
    # NO COUNT OF THE ROWS ABOVE: a roster asserted by arithmetic goes on
    # printing "all as specified" after a row is dropped. What is asserted is
    # membership by NAME — `REQUIRED_MUTANTS`, `ROUTE_NAMES`, `CLAIM_NAMES` —
    # and each of those rows prints its own verdict above.
    print(f"\nselftest: {len(MUTANTS)} mutants x {len(routes)} routes "
          f"({'; '.join(routes)}), each asserted on its own tree, and the "
          "membership, population and refusal rows above — all as specified.")
    return 0


def main(argv: list[str]) -> int:
    ap = argparse.ArgumentParser(description="wrapped install idioms check")
    ap.add_argument("--selftest", action="store_true")
    ap.add_argument("--root", default=".")
    args = ap.parse_args(argv)

    if args.selftest:
        return selftest()

    failures, files, commands, uses = check_tree(args.root)
    flat = [f for name in CLAIM_NAMES for f in failures[name]]
    if flat:
        print(
            "An install idiom this repo has a wrapper for is spelled inline. "
            "The wrapper exists because `apt-get update` fails for any source "
            "list on the runner image — including third-party lists this repo "
            "never asked for — so an inline preamble reds every branch during "
            "a stranger's bad hour, indistinguishable from a real red.\n"
        )
        for f in flat:
            print(f"  {f}")
        print(
            "\nThe fix is to call the wrapper, which narrows the update to "
            "Ubuntu's own archive, bounds and retries a hang, and refuses a "
            "missing package loudly:\n"
            "    - run: exec scripts/apt-install.sh <package>...\n"
            "with `timeout-minutes: 8` on the step, which is what admits one "
            "retry (that script's header does the arithmetic)."
        )
        return 1
    print(
        f"install wrappers: {commands} command(s) across {files} workflow(s), "
        f"composite action(s) and shell file(s); none runs "
        f"{'/'.join(sorted(IDIOMS))} outside "
        f"{', '.join(sorted(PATH_EXEMPT))} ({uses} use(s) there)."
    )
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main(sys.argv[1:]))
    except Bail as exc:
        print(f"check-install-wrappers: {exc}", file=sys.stderr)
        sys.exit(2)
