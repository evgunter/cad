---
id: decided-coincidence-carries-a-synthetic-invalid-margin
kind: issue
title: A coincidence decided Zero on every datum is reported with a synthetic MarginDiag::Invalid margin, as if a measurement had failed
status: open
opened: 2026-09-25
priority: P3
cost: M
design: true
---

## What

When the carrier ladder DECIDES two carriers coincident (every datum
`Sign::Zero`) with no identity rung, it refuses as an undeclared
coincidence, and the `Indeterminate` it attaches is minted, not
measured: `margin: MarginDiag::Invalid`, with the predicate filled in
by name.

- `crates/topo/src/boolean/plane_eq.rs`, rung 4 (`bool_plane_offset`
  decided `Zero` → `PlaneEqError::Undeclared { diag: Indeterminate {
  margin: Invalid, .. } }`);
- `crates/topo/src/boolean/carrier_eq.rs`, `data_rungs`'s fallback for
  an all-Zero list (names the kind's first datum).

It reaches users through `BooleanError::UndeclaredCoincidence { diag,
.. }` (`reduce.rs`'s `gate_maximal_faces` for the F7 same-operand pair,
and the classification sites). `Invalid` is what a poisoned
measurement reports, so the payload reads as a failed measurement on
exactly the pairs whose relation was decided cleanly. The same
synthetic shape appears in the contradiction arms
(`data_rungs`'s definite-nonzero `Contradicted` and the different-kind
arm). Measured on the GERM dumbbell: a `Tangent` declaration on two
coincident torus waists is refused `ContactContradicted { margin:
Indeterminate { margin: Invalid, predicate:
Some("carrier_torus_axis_parallel") } }`, and that axis margin was
decided zero, not invalid.

## Why it is not a local fix

The field is the public `Indeterminate` on `BooleanError`'s variants,
and a decided relation has no in-band margin to carry. Saying so needs
a payload change: an optional diag, or a decided-margin variant of
`MarginDiag`. That is a public API change across the ladder's errors,
beyond the drive-by that found it (GERM torus doors, from
`gate_maximal_faces`).

## Home

TANG (`carrier_eq.rs`); `plane_eq.rs` is the planar twin.

## Re-pointed from the D10 hold (2026-10-08)

Waits on `intent-stage4-is-built`, not on the whole program: the synthetic margins sit on the undeclared-coincidence refusal and ContactContradicted (plane_eq, carrier_eq), which become findings or retire at stage 4. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)

## Released by INTENT stage 4 E (`intent/s4-e-glue-on-zero`) (2026-10-09)

E deletes the row's first reach. Rung 4 now glues, so `PlaneEqError::Undeclared` and `BooleanError::UndeclaredCoincidence` are gone, and a decided Zero carries its margin through `CoincidenceMeasure::Zero` (`crates/topo/src/boolean/carrier_eq.rs:137`). The shape survives in two places:

- A pair decided one carrier at an arm the glue door did not glue escalates through `unglued_coincidence` (`crates/topo/src/boolean/mod.rs:1007`). It dresses the decided margin as an `Indeterminate`, and falls back to `MarginDiag::INVALID` where none was read (`recl.rs:151`, `vtxfac.rs:789`).
- The contradiction arms still mint `INVALID` (`contact_verify.rs:166` and its siblings; `vtxfac.rs:550`).

The payload question stands. It overlaps CLEAVE's `topo-mints-indeterminates-outside-the-funnel` step 3.
