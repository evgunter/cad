---
id: a-re-cut-booleans-verdicts-keep-the-re-cut-keys-its-rows-are-respelled-off
kind: issue
title: A boolean that re-entered on re-cut operands respells its rows to the input's cells but not its glue verdicts, which keep the re-cut keys
status: open
opened: 2026-11-23
priority: P2
cost: M
---

Filed by HELDRECORDS' `nary-lineage` lane (branch
`heldrecords/nary-lineage`), from the sweep for readers of a re-cut
boolean's naming that stop at the re-cut operand's keys.

## What

Where the extent scan asks for re-cuts, the pair door re-enters on the
cut and re-charted operands (`boolean_op_recut`, in the re-cut arm of
the join in `crates/topo/src/boolean/ops.rs`) and then respells the
result's `coincidences` to the input's cells
(`respell_through_cuts`): a cut-in piece reads as the face it was cut
from, and a one-face re-charted shell's turned face as its input face.
The result's `verdicts` were built inside the re-entry from the rows
before that respelling (`GlueVerdict::of_rows`, at each body the
re-entry returns) and are not respelled, so a glue verdict on a cut or
turned face names a re-cut key while its row names the input's.

Two readers would then go wrong:

- `verdict_row` (`crates/topo/src/boolean/nary.rs`) looks up each
  judgement verdict's row by `row.face_pair() == (verdict.a(),
  verdict.b())`; with the two in different keys it reaches its
  `unreachable!` ("glued … with no row recording it").
- `run_members` records the judgement's verdict faces as member faces
  (`PairVerdict::faces`), which then route the verdict through the
  member lineage by a key that names no face of the member.

**Unmeasured**: whether any glue row (`Rest`, `Tangent`,
`Continuation`) can stand on a cut-in or re-charted face. The re-cut is
asked for by a sphere crossing; a glue on the same shell needs a second,
coincident or tangent carrier on it. Where the re-chart keeps the
shell's keys (a member that is the ball alone) the keys agree and the
gap does not open; it opens where the turned shell was carved out of a
body of several shells and grafted back under new keys, or where a
cut-in divided the face.

## Way through

Respell the verdicts through the same cut and re-chart maps the rows
go through, or rebuild them from the respelled rows (with the tangent
lines re-keyed alike); a pin needs a glue row on a re-cut face, found
first.

## Evidence (2026-11-23, `orbit/one-sphere-escape-cut`)

That branch retires the re-chart: every escape, closed group or trimmed, is cut in, so `apply_recuts`, the `Recharts` map and the turned-face arm of `respell_through_cuts` are gone. The re-chart half of this row goes with them. The cut-in half stands unchanged: `respell_through_cuts` still respells the rows through the cuts' division rows and not the verdicts, so a verdict on a cut piece would still name the piece's key. The row stays open on that half, and stays unmeasured.
