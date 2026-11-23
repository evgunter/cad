---
id: a-step-given-pairs-and-re-charting-its-operand-reads-them-by-the-keys-the-re-chart-moved
kind: issue
title: A boolean given glued pairs that also re-charts an operand's sphere shell reads the declared faces by keys the re-chart may have moved
status: open
opened: 2026-11-23
priority: P2
cost: M
---

Filed by HELDRECORDS' `nary-lineage` lane (PR 7516), on its reviewer's
open question; not measured.

## What

The n-ary door hands each fold step its glued pairs in the input keys
of the step's operands (`given_pairs`, `crates/topo/src/boolean/nary.rs`):
an accumulation face and a face of the joining member. Where the same
step's extent scan asks for a re-chart (`Recuts::rechart`), the pair door
turns that sphere shell (`apply_recuts`) and re-enters on the turned
operands (`boolean_op_recut`, `crates/topo/src/boolean/ops.rs`) with the
same declarations. On the re-cut operand a declared face's key is one
of three things:

- gone, where the turned shell was carved out of a body of several
  shells and grafted back under new keys. `validate_declarations`
  (`crates/topo/src/boolean/mod.rs`) refuses a key that does not resolve
  ("declared face key does not resolve"), typed, if the re-entry runs it
  on the re-cut operands. Not checked.
- kept but moved, where the re-chart turned the whole operand in place.
  The key then names a turned face on the same sphere. A given pair
  decides a carrier pair, so the decision still holds, but the face it is
  attached to is a different region.
- untouched, where the declared face is not on the turned shell.

**Unmeasured**: whether any fold step is both given a pair on a sphere
face and re-charts that sphere's shell. Doing so needs a glue (a
coincident or tangent carrier) and a crossing on the same shell in one
step. The pin `a_verdict_on_a_turned_member_face_reaches_the_next_step`
(`crates/sweep/tests/the_nary_door_reads_a_re_charted_accumulation_through_its_shell.rs`)
hands a verdict to a face an earlier step turned. Its step refuses
`CoincidentShell`; whether that step's own scan asked for a re-chart is
not read.

## Way through

Measure first: find a step given a pair on a shell its own scan
re-charts. Then translate the declarations through the re-chart's
graft rows, or refuse them typed at the re-entry, stating the turned
shell.

## Evidence (2026-11-23, `orbit/one-sphere-escape-cut`)

That branch retires the re-chart, so the first two arms above go: no shell is carved out and grafted back under new keys, and no key is kept at a turned place. The re-entry now runs on operands that only the cut-ins changed (`apply_cut_ins`). A declared face that a cut divides keeps its key on one piece, and the cut mints the other piece under a new key that no declaration names. Whether a step is ever given a pair on a face its own scan cuts is still unmeasured, so the row stays open on that arm. The pin is renamed `a_verdict_on_a_cut_member_face_reaches_the_next_step` (`crates/sweep/tests/the_nary_door_reads_a_cut_in_accumulation_through_its_pieces.rs`); its step now cuts the crossing ball in, and it still refuses `CoincidentShell` given the judgement's verdict.
