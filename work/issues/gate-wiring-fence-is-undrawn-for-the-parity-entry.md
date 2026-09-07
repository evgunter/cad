---
id: gate-wiring-fence-is-undrawn-for-the-parity-entry
kind: issue
title: a new gate's wiring is one announced line in CIW's files by written convention, but nothing extends that to the TIER_BLIND parity entry it cannot land without
status: open
opened: 2026-09-07
---


Raised by VIEW's `view/all-gate` unit, which stopped before pushing
because `python3 scripts/work.py territory --base origin/main` flagged
four paths. Three of the four had a written answer; the fourth did not,
and the fourth is the one a gate cannot ship without.

## What is written

`work/gates/program.md`'s `keep_out` says `.github/workflows/*` and
`local-scripts/*` are CIW's — **"a new gate's wiring row is one
announced line there"**. That clause is about the CLASS of edit rather
than the identity of the editor, and it is how every gate in
`scripts/gates/` reached CI.

`work/ciw/program.md`'s `paths` also claims `scripts/check-*.py`, which
holds `scripts/check-ci-mirror-parity.py`. **Nothing extends the
one-announced-line convention to it**, and no keep_out clause on either
side mentions it.

## Why the omission bites

A gate whose allowlist lives in a document is TIER-blind, so it belongs
in the `mirror` job — the argument that sited
`scripts/gates/viewer-module-kinds.sh` there. A gate in that job with
no `TIER_BLIND` entry in `scripts/check-ci-mirror-parity.py` does not
merely go unchecked: parity itself reds. **So the entry cannot be split
into a follow-up PR by its owning program** — it lands with the gate or
the gate lands broken. The convention that covers the two files a gate
CAN be split from does not cover the one file it cannot.

## What is proposed

Extend the written clause so the next gate author does not re-derive
this fork: name `scripts/check-ci-mirror-parity.py`'s `TIER_BLIND` row
alongside `.github/workflows/*` and `local-scripts/*` as the wiring a
new gate announces rather than hands off. That is an edit to CIW's or
GATES' `program.md`, which is those programs' text to change — this
file is the announcement and the argument, not the change.

## The crossing this file announces

VIEW's PR wiring `scripts/gates/viewer-vocab-declared-once.sh` touches
`.github/workflows/ci.yml` (one step), `local-scripts/ci-local.sh` (two
lines and a `HOSTED MIRROR:` citation) and
`scripts/check-ci-mirror-parity.py` (one `TIER_BLIND` tuple entry).
Nothing on either program's slate is edited and no file moves.

The gate file itself is not part of this question. `scripts/gates/*` is
GATES' territory whole, but VIEW already authors a gate there:
`scripts/gates/viewer-module-kinds.sh` was created and revised entirely
by `view:` commits (`14f7d270f`, `daa56c0e7` — *"site the gate where it
can fire"*, the same wiring work — `16a7c113d`, `041d7d613`,
`b5d9ec7f7`). GATES' charter is a named backlog of gate fixes, not
authorship of every future gate, and a gate whose subject is
`crates/viewer/src` and whose allowlist is `crates/viewer/README.md` is
VIEW's subject in the directory where gates live.
