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

## 2026-11-23: the cut-in re-entry reads declarations by the pre-cut key too (ORBIT `orbit-carry`)

Found by the sweep of branch `orbit/cut-in-carry`; read, not measured.
That branch gives the re-entered op the cut-ins' face divisions
(`Pass::Recut`, `crates/topo/src/boolean/ops.rs`), so the carry and the
naming read a cut face through its pieces. Two readers of the caller's
`decls` still run on the re-cut operands by key alone, before any
division list exists:

- the glue door (`glue::decided_glue`, its `declared` check over
  `decls.pairs`): a pair declared on a face a cut-in divided names only
  the piece that kept the key, so the minted piece's overlap with the
  declared partner reads as undeclared and is decided afresh;
- the reduction's walls (`vtxfac::pierced_and_paired`, called from
  `boolean_reduce_declared_strategy` in `crates/topo/src/boolean/mod.rs`
  over `decls.records_a`/`_b`): an area record on a divided face is a
  wall only on the kept piece, so a pierce of the minted piece is not
  read as a pierce of the declared wall.

Once `orbit/one-sphere-escape-cut` routes closed sphere groups through
the cut-ins instead of the re-chart, this is the same question as the
row's: the declarations need the cut-ins' divisions (or the re-chart's
graft rows) at the re-entry's door, or a typed refusal naming the
divided face. Measure first, as above: a declared pair or an area
record on a sphere face that the same op cuts in.
