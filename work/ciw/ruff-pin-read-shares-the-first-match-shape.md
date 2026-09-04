---
id: ruff-pin-read-shares-the-first-match-shape
kind: issue
title: check-python-lint.py reads ci.yml's ruff pin with the same first-match-at-any-indentation shape ci-pin.py replaced
status: open
opened: 2026-09-04
refs: [nightly-pin-reading-idiom-four-copies]
---

Found by the sweep for `nightly-pin-reading-idiom-four-copies`, which
fixed the five `nightly.yml` sites and did not fix this one because it
is outside that unit's territory.

`scripts/check-python-lint.py:68` reads ci.yml's ruff pin with

    PIN_RE = re.compile(r'^\s*RUFF_VERSION:\s*"([0-9]+\.[0-9]+\.[0-9]+)"\s*$')

and returns the FIRST line that matches
(`scripts/check-python-lint.py:170-171`). That is the same shape the
retired `sed -n 's/^ *NAME: *//p' … | head -1` had: `^\s*` matches at
any indentation, so a `RUFF_VERSION` under a job or a step is a
candidate, and position in the file — not scope — decides which one
wins. `scripts/ci-pin.py` now answers exactly this question, anchored
to ci.yml's workflow-level `env:` block and refusing on a second match.

**Its blast radius is smaller than nightly's was, which is why it did
not ride along.** A wrong read here does not install anything: this
script compares the pin against the ruff already on the box and either
fails the hosted row or skips loudly on a developer's. So the failure
mode is a spurious mismatch or a spurious pass on a version nobody
pinned as the workflow's, not a lane silently linting with the wrong
linter — the version that actually runs is still whatever the box has.
The regex is also tighter than the `sed` was (it requires the double
quotes and an `x.y.z` value), so an accidental second match is less
likely, though a per-job `RUFF_VERSION: "1.2.3"` matches it exactly.

**What a fix wants.** `scripts/check-python-lint.py` imports
`scripts/ci-pin.py`'s reader, or shells out to it, so there is one
answer to "which version does ci.yml pin" in the tree. The cost to
weigh first: check-python-lint.py is TIER_BLIND and runs in both halves
of CI, so a new import is a new coupling on the gate of record, and its
own self-test plants ci.yml fixtures
(`scripts/check-python-lint.py:443-455`) that the shared reader would
have to accept unchanged or the fixtures move with it.

`.claude/hooks/session-start.sh:104,123,124` is a THIRD thing and not
this: it restates the pins as literals rather than misreading them, and
`.claude/` is deleted at checkout by every hosted job by design. That is
a duplication, with its own argument for existing, and it is named here
only so the next sweep does not have to find it again.
