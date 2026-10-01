---
id: boolean-door-passes-a-geometrically-open-result-the-backstop-cannot-see
kind: issue
title: "A boolean result missing a face passes the door: the gate runs tiers 1-2 only, and no volume inequality bounds a short intersection from below"
status: open
opened: 2026-10-01
priority: P1
cost: M
---

## What

Measured on PR #3627's dual review (the R1 probe's cap-straddling
bars, `crates/sweep/tests/reach_wall_chord_rows.rs` holds the poses). A
join defect (the chord taken for the section segment, fixed in that
PR) dropped one face of B's in-component. The zip then glued the two
free edges between the same pair of pierce vertices (a line and an
arc), so the result was topologically closed. The door
(`boolean::ops::boolean_op_recut`) let it out:

- `gate` runs tiers 1 and 2 only (its doc: "tier 3 is an at-rest
  posture with the PR 3 description gap"). The body failed tier 3
  (`PlanarBoundaryResidual`: an arc edge in a plane face), which no one
  ran.
- `volume_backstop` bounds ∩ only from above (`≤ vol(A)`, `≤ vol(B)`)
  and ∖ only from above. Of the 10 wrong ∩ bodies, 6 had NEGATIVE
  volume. PR #3627 adds a positivity arm (a bounded op's result has a
  non-negative flux volume), which refuses those 6. The other 4 were
  positive and short (for example 0.00367 against a truth of 0.01124),
  and no inequality over the three volumes can see that: the lower
  bound on `vol(A ∩ B)` is `vol(A) + vol(B) − vol(A ∪ B)`, a volume
  the op does not compute.

## What would close it

- Gate tier 3 (or the part of it that sees an off-carrier edge) at the
  boolean door, once the description gap its doc names is closed; or
- add the inequalities the operands do bound: `vol(A ∖ B) ≥ vol(A) −
  vol(B)` and `vol(A ∪ B) ≤ vol(A) + vol(B)`. These are cheap, but they
  would not have caught these 4.

Measure the cost of tier 3 on the door's corpus before choosing.

## Home

REACH (the backstop is #3611's), on `crates/topo/src/boolean/ops.rs`,
which CLEAVE and HONE share.
