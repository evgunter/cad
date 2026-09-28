---
id: check-6-planar-arm-skips-ellipse-and-nurbs-loops
kind: issue
title: check 6's planar arm does not examine a planar loop riding a spiric or NURBS carrier
status: open
opened: 2026-09-24
priority: P2
cost: D
refs: [sense-inversion-is-invisible-to-tier-3-on-arc-capped-lofts, m6-sense-gate-recorded-residuals, ATREST-13]
---


## The residue

Check 6's planar arm (`crates/topo/src/validate.rs`, the
`LoopRoleInverted` arm) examines every planar loop the shared winding
answers (`crates/topo/src/loop_winding.rs`, `Body::planar_loop_winding`):
loops of `Line`, `Circle` and `Ellipse` carriers. What it does not
examine is a planar loop riding a **NURBS or spiric** carrier: neither
has a closed-form area, and the shared winding answers `None`. A planar
face bounded so carries a stored `sense` bit no at-rest check falsifies.

## How the ellipse half closed (ATREST-13)

The arm reads the winding with no reach argument — the assigner and
the checker now answer on one carrier set by construction — and the
perimeter lever of an elliptic arc is `|Δ|` times the LARGER semi-axis,
so the upper bound holds for an ellipse stored with `minor > major`
(which mints: `an-ellipse-stored-minor-over-major-passes-tier-3`).

**Refusal surface re-taken** (CI run 36154432046, the winding logged
wherever the ellipse reach and the circle reach disagree, over every
`validate_geometric` call of the suite at all three ε rows, the tour's
tests and binary and the wild corpus): per ε row, 282 ellipse-bearing
planar loops, 280 wound `Positive` (honest), **0 escalate**, and **2
refuse — both deliberate inversions**: `cut_cylinder`'s section face
under `m6_6_sense_gate`'s own `flip_all`, and `tilted_cut_upper`'s cut
face inverted through `Body::set_face_sense` by
`m5_s10_face_sense`. The demos add 8, all `Positive`. The stop clause
(a refusal of a body the corpus builds on purpose) did not fire.

**Re-baselined:** `an_ellipse_bounded_planar_face_stays_outside_the_planar_arm`
is now `an_inverted_ellipse_bounded_planar_face_refuses_by_name`
(`crates/sweep/tests/m5_s10_face_sense.rs`): the inverted cut face
refuses one `LoopRoleInverted` naming it and its outer loop, as the
circle-bounded cap does. `cut_cylinder_conic_trim_wall_flip_is_caught_and_the_inversion_is_the_residue`
is now `…_and_the_inversion_refuses_at_both_planes`
(`crates/step-export/tests/m6_6_sense_gate.rs`): the whole-body
inversion refuses at the cap AND the section face.

## The flip

A closed-form or certified area for a spiric- or NURBS-bounded planar
region (the latter a quadrature the props lanes may already own), with
the same measure-first step over the corpora.
