---
id: ci-pin-quoted-branch-skips-the-bare-branch-rejection
kind: issue
title: ci-pin.py's quoted branch accepts a scalar its bare branch would refuse
status: open
opened: 2026-09-11
refs: [2327]
---

Found by the correctness review of PR 2327 (CIW unit 7) and **filed here on
the CIW orchestrator's instruction** — `scripts/ci-pin.py` is META's by
`work/meta/program.md`'s `paths`, and `implementer-discipline.md` §6 has a lane
report a cross-fence finding rather than file it. It is filed so it is
scheduled rather than living in a merged PR body; re-home or close it as META
sees fit.

`unquote()` in `scripts/ci-pin.py` has two branches:

- **bare scalar** — strips a trailing comment, then refuses on
  `re.search(r"[\s\"']", val)`: whitespace inside the value is not a version
  pin, and the reader says so.
- **quoted scalar** — takes everything between the matched pair, refuses text
  after the closing quote, and then checks only `if not val` (non-emptiness).
  **The whitespace rejection the bare branch applies is never applied here.**

Executed, not read. With `NEXTEST_VERSION: "0.9.140 extra"` in a scratch
`ci.yml`, `ci-pin.py NEXTEST_VERSION` exits 0 and prints `0.9.140 extra`, and
the caller built `https://get.nexte.st/0.9.140 extra/linux`.

**Contained today**, which is why it is an issue: nothing in the tree sets a
pin that way, the URL 404s rather than installing something wrong, and
`.claude/hooks/session-start.sh` now checks the shape of the answer itself
(PR 2327) precisely because it installs from it. That second check is a
caller defending itself against its reader, which is the wrong way round for a
file whose header says *"a reader that guesses is the defect this file exists
to close"*.

**The fix is one line** — apply the same `[\s]` rejection after stripping the
quotes — plus a fixture in `--selftest`, which is where this reader's other
refusals already live. The docstring's promise that *"`set -euo pipefail` plus
`v="$(scripts/ci-pin.py NAME)"` is the whole of a caller's error handling"* is
the reason to take it: callers are told to trust the answer's shape.
