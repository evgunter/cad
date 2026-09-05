---
id: declared-tangency-needs-the-registered-identity-door
kind: issue
title: a constructor-declared tangency (Fillet's carrier_line_circle) is a live consumer for the registered-identity door M10-8 left unbuilt
status: open
opened: 2026-09-05
refs: [M10-8]
---

**Found by M10-8's R2 review, by execution**, on a document the unit
never built: a rounded-corner pad with a central bore, its four corner
arcs authored as `ProgramStep::Fillet(corner_r)` on one chain
(`crates/editor-core/tests/m10_8_r2_probes_interval.rs`, `pad`), the
corner and bore radii both distributed, the study measuring the web
from the bore wall to a corner wall and asserting a floor on it.

## What was measured (R2, M10-8's frozen head fe649cadf)

- The pad certifies whole only below **2.083e-6 of its study**, with
  the tier on or off (the atom algebra does not reach it).
- The first refusal beyond that ceiling is **`carrier_line_circle`**,
  enclosure `[-8.1e-5, 7.7e-5]` — the tangency between a straight leg's
  carrier and the fillet arc's circle.

## Why this is a consumer for M10-8's §3 door

`carrier_line_circle` is not a coincidence the interval channel has to
discover: it is a tangency the CONSTRUCTOR declares. The `Fillet(r)`
step builds the arc tangent to both legs by construction
(`crates/editor-core/src/program.rs:115`, `Fillet(Expr)`), so the
identity "this circle is tangent to that line" is guaranteed by the door
that made the geometry, and a symbolic tier that could be TOLD it
(`session.assert_equal(a, b)`, ERROR-DESIGN E12's provenance reserve —
the door M10-8's spec §3 describes) would discharge it without any
normal form reaching it.

M10-8 declined to build that door for want of a consumer ("machinery
for zero certificate content", E6): the three documents it measured
were bounded by the arc family's `sqrt` shapes, which rules A0/C reach.
This document is bounded by something those rules cannot reach — a
declared tangency — and is the consumer the door was waiting for.

## What is owed

- The registered-identity door in `geom_core::sym` (a session-level
  `assert_equal` that folds two DAG nodes into one indeterminate, with
  the provenance recorded in the receipt as its own count — neither
  `symbolic_zero` nor `sign_gated`), and the `Fillet` constructor
  registering its two tangencies through it.
- The pad's ceiling re-measured with the door open, and the R2 probe
  row `r2_end_to_end_rounded_pad_study` re-cut as its pin.
