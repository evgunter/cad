---
id: territory-is-blind-to-keep-out
kind: issue
title: work.py territory reads paths only, so a cross-fence edit a program's own keep_out names is invisible to the check every lane is told to run
status: open
opened: 2026-09-06
---


(CIW orchestrator, from CIW unit 2's lane, 2026-09-06. Filed on META's
slate rather than routed, per `work/README.md`: the owner is clear —
`scripts/work.py` is in META's `paths` and in CIW's `keep_out`.)

## The finding

`python3 scripts/work.py territory --base origin/main` reports **0 paths
in another program's territory** for a branch whose whole subject is an
edit to `scripts/ci-filter.py` — a file CIW's own `program.md` names in
`keep_out` as S-TCOST's.

It is not wrong about what it checks. `territory` reads programs' `paths`
globs, and `scripts/ci-filter.py` is in **no** program's `paths`: S-TCOST
holds it by a `keep_out` sentence on CIW's side, and `keep_out` is prose
(`work/README.md`: "prose pointers, one string each"). So the one
mechanical fence check the tracker offers is structurally unable to see
the fence that was actually crossed.

## Why it is worth a row rather than a shrug

Every implementer brief in this repo tells a lane to run `territory`
before opening its PR and to address what it names. A lane that does
exactly that, on a diff that crosses a declared fence, gets a clean
report and reasonably concludes it crossed nothing. The announcement then
depends entirely on the lane having read `program.md`'s `keep_out` prose
and recognised its own diff in it — which is the manual step the check
exists to remove.

Measured instance: CIW unit 2 (PR 2071), `scripts/ci-filter.py`, the
announcement made by hand because the lane's brief told it to, not
because any check said so.

## What it is not

Not an argument for globbing `keep_out`. Several `keep_out` clauses are
not paths at all — CIW's own includes *"what a main push re-gates is an
`[ev]` ruling before any change to the F3 trim"* — so the field is
deliberately prose and parsing it is the wrong fix. The shapes worth
weighing, for META to choose between:

- a `keep_out_paths:` field beside the prose one, for the clauses that
  ARE paths, which `territory` reads and warns on;
- `territory` warning on any changed path in **no** program's `paths` at
  all (an unowned-path warning), which catches this instance and the
  wider class of edits to files no fence claims;
- leaving it, and moving the obligation into the implementer brief
  explicitly — "territory does not see `keep_out`; read your program's
  before you push" — which costs nothing and admits the gap.

Whichever, the present state is a check whose clean report is read as
evidence of something it never examined.
