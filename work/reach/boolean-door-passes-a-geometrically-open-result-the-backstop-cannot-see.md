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
  volume; the other 4 were positive and short (for example 0.00367
  against a truth of 0.01124), and no inequality over the three volumes
  can see that: the lower bound on `vol(A ∩ B)` is
  `vol(A) + vol(B) − vol(A ∪ B)`, a volume the op does not compute.

A positivity arm (a bounded op's result must not enclose negative
volume) was tried on PR #3627. With the join defect reverted it refused
the 6 negative bodies; but it also refused a VALID sliver result at
ε = 1e-12 (`topo/tests/contact9_side_codes.rs`
`a_vertex_pair_reads_a_dipping_chord_at_its_far_vertex`, the ∩ of its
1 m-edge control): the volume door reads that 6e-19 m³ sliver as
−3e-16 m³, the error filed as
`work/contact/volume-door-reads-a-tiny-valid-boolean-result-wrong`, and
the sliver's `V/A` lands past the 1e-12 band. So the arm was not
shipped; it waits on that measurement fix, or on a floor derived from
the door's own error bound.

## What would close it

- Gate tier 3 (or the part of it that sees an off-carrier edge) at the
  boolean door, once the description gap its doc names is closed; or
- add the positivity arm once the sliver measurement is fixed (above),
  and the inequalities the operands do bound: `vol(A ∖ B) ≥ vol(A) −
  vol(B)` and `vol(A ∪ B) ≤ vol(A) + vol(B)`. These are cheap, but
  none of them would have caught the 4 short positive bodies.

Measure the cost of tier 3 on the door's corpus before choosing.

## Home

REACH (the backstop is #3611's), on `crates/topo/src/boolean/ops.rs`,
which CLEAVE and HONE share.

## Resolution (`reach/door-backstop`)

Tier 3's cost on the door's corpus was measured first: about 15 % of op
time on the topo and sweep suites. Gating it, though, refuses 52
results the door ships today, from tier-1/2-only operands and from
slivers whose minted edge keeps a scaffold description. That is the
description-gap decision. It is filed with the measurement as
`boolean-door-tier-3-waits-on-the-description-gap` and not taken here.

The backstop half ships:

- **The positivity arm** (`encloses_material`): a bounded ∩ or ∖ result
  must enclose material. It is tier 3's +V invariant read at the door,
  with the same exemption ("zero and escalated exempt"). Only a negative
  that the interval re-derivation certifies refuses. The ε = 1e-12
  sliver that blocked it (−3e-16 m³ read for 6e-19 m³) straddles zero
  in interval arithmetic and passes. It is the arm the 6 negative ∩
  bodies would meet. They were not re-measured here: the join defect
  that made them is fixed, so a negative result is planted instead
  (`boolean::ops::tests::volume_backstop_joint_and_sign_arms`).
- **`vol(A ∪ B) ≤ vol(A) + vol(B)` and `vol(A ∖ B) ≥ vol(A) − vol(B)`.**
  They are tight on correct results whenever the operands touch, so
  each margin carries what the door may move there. That allowance is
  `band.escalate()` × the smaller face of each declared pair. It is the
  door's own error bound, not a constant.

The 4 short positive ∩ bodies stay out of reach of any inequality over
the three volumes. They are the residue item's.
