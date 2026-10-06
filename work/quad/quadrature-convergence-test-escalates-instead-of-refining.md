---
id: quadrature-convergence-test-escalates-instead-of-refining
kind: issue
title: the quadrature lanes escalate an in-band convergence test instead of refining another round, so a face measures in one boolean operand order and not the other
status: open
opened: 2026-10-01
---


## What

Each quadrature lane's round loop (`crates/geom-brep/src/props/quad.rs`,
`cylinder_cut_face_rounds` and its three copies, `props_quad_converged`)
decides whether the round's enclosure met the reporting target with
`classify_len(..)?`. When the convergence margin `target_len −
width_len` lands inside the band, the `?` escalates the whole face
(`PropsError::Escalated`) instead of treating "not definitely
converged" as "run another round". A width a hair from the target is
not a question about the model, and the next round, whose enclosure is
about 8× narrower, would decide it.

Measured (REACH, branch `reach/volume-backstop`, 2026-10-01):
- `union(base, boss)`, where base is `brick((-1, 1), (-1, 1), (0, 1))`
  and boss is a radius-0.25, height-1 three-arc cylinder on the sketch
  plane through `(0, 0, 0.5)` turned 0.5 rad about `x`, refuses
  `VolumeUnmeasured { operand: None, source: Face { source: Escalated
  { margin 3.13e-9, band (1e-9, 1e-8), predicate "props_quad_converged" } } }`.
- `union(boss, base)` builds and certifies at the analytic volume. The
  result is the same solid in a different arena order, so one of its
  wall faces lands its convergence margin in band in one order and not
  the other.
- At 0.45 and 0.55 rad both orders build.
- The GERM half-donut row
  (`work/germ/union-backstop-catches-a-suspect-body-from-a-tilted-rod-in-a-half-donut`)
  refuses through the same arm on its subtraction (margin −1.85e-9).

Both orders are pinned by `crates/sweep/tests/reach_volume_backstop.rs`
`a_boss_at_half_a_radian_measures_in_one_operand_order`. The refusing
order goes red when this lands, and should then build at
`penetrating_union(0.5)`.

## The shape of a fix

In-band at the convergence test means not converged: take the next
round, and refuse `QuadratureBudget` only at the last round, as a
definitely unconverged face already does. The four copies of the block
are the C3 finding (`work/quad/C3.md`), so the fix lands once if the
block is factored first.

The text the escalation renders is PROPS's
`props-escalation-renders-the-coincidence-menu-unlabelled`. The
backstop's own refusal now names the quadrature's convergence and
offers no coincidence (`topo::validate::classify_mass_props`).

## Second witness (BAND, branch `band/annulus-host-outer-metered`, 2026-10-01)

At `CAD_TOLERANCE_EPS=1e-12` only, subtracting a 45°-tilted block from
a revolved cone-then-cylinder shaft (profile
`(0,0) (0.5,0) (1,0.5) (1,3) (0,3)` about `y`, the block's underside
lowest on the cylinder at `y = 0.535` or `0.547`) refuses
`VolumeUnmeasured { operand: None, source: Face { source: Escalated {
margin 2.05e-12 / −8.52e-12, band (1e-12, 1e-11), predicate
"props_quad_converged" } } }` on the cut face, which the cut leaves
bounded by ellipse arcs alone. Both build at 1e-6 and the default eps.
With the cylinder ending at `y = 2` (a strip of the top cap stands, so
the cut face carries a chord too) every case builds at all three rows,
which is what `crates/sweep/tests/band_annulus_host_boundary.rs` now
uses. Two of two levels landing in band at 1e-12 suggests the width
stalls near the `1024·eps` target on that face rather than landing
there by coincidence.


## Also seen by CLEAVE's `cleave/steep-tube-eps` (2026-10-03)

`sweep`'s `review_cleave_wrongarc::steep_cuts_of_tubes_chord_inside_each_bore_face`,
pose `tube a 0.4 d 0 turn 1.0682 tilt 1.2 at [0, 0, 0.5] phi 0`, both
flips, at `CAD_TOLERANCE_EPS=1e-8` (off the gated rows): tier 3 refuses
`VolumeUncomputable { face: FaceKey(5v1), Escalated { margin
−4.733e-8, band (1e-8, 1e-7), predicate "props_quad_converged" } }`.
At the gated rows (default, 1e-6, 1e-12) the same sliver face runs out
its round budget instead (`QuadratureBudget`), which the row
tolerates as `volume refused`.

## 2026-10-06 — a split operand that does not finish at ε = 1e-6 (CLEAVE)

With the split's doors taking `AtRestBody` (branch
`cleave/split-operand-gate`), `sweep/tests/rehome_rings_lune.rs`
`an_oblique_cut_carries_a_lune_bore_with_its_half` splits the bored disc
on `z = 0.5 − 0.2y` and then finishes the lower piece to split it again.
At `CAD_TOLERANCE_EPS=1e-6` the at-rest gate refuses that piece
`VolumeUncomputable { source: Face { face 3v3, Escalated { margin
8.454e-6, band (1e-6, 1e-5), predicate "props_quad_converged" } } }`; at
the default ε and at 1e-12 it finishes. All three bore poses refuse
there.

That is a regression in what the split answers. On main the second split
of that piece answered at 1e-6, and now it refuses, because the split
serves finished bodies and the piece does not finish. The row pins the
refusal by type at exactly 1e-6 (`QUAD_ESCALATES_AT`;
`PropsError::Escalated { check: PropsCheck::Converged }`). At that ε it
asserts nothing past "the lower piece is non-empty". The pin goes red
when this lands, and the row's assertions then come back at 1e-6.

## 2026-10-06 — a counterbored tube's split half (CLEAVE)

Found by `cleave/tube-across-axis`'s sweep. The counterbored tube
revolved a full turn about `y` (profile `(0.3, 0)–(1, 0)–(1, 1)–(0.6, 1)–(0.6, 0.6)–(0.3, 0.6)`)
splits through `(0, 0.8, 0)` with its normal leaning 0.25 rad off `y`
toward azimuth 4 of `y`'s `orthonormal_basis` (`s = +1`). Both halves
pass tiers 1, 3 and 3′. `mass_properties` of the lower half refuses
`Face { face 11v1, Escalated { margin −2.936e-9, band (1e-9, 1e-8),
predicate "props_quad_converged" } }`. The other 59 poses of that sweep
measure.

## 2026-10-06 — an obliquely cut-off fillet band at ε = 1e-12 (BAND)

A cylinder band trimmed by elliptic end arcs (PR 4173). The
plane–plane fillet on the parallelogram leaning `s = 3`
(`band_planar_oblique_fillet.rs`), with a brick crossing its end arc:
subtract and union refuse `VolumeUnmeasured` on this arm at ε = 1e-12
and build at 1e-9 and 1e-6; pinned by
`a_brick_through_a_steep_elliptic_end_builds_in_every_op`. And the
D-profile rod of `fillet_h7_transverse_cap.rs` cut by the plane through
`(0, 0, 0.7)` with normal `(0.6 sin 0.4, 0.8 sin 0.4, cos 0.4)`, both
creases filleted: `mass_properties` of the result refuses
`props_quad_converged` (margin 5.6e-12, band `(1e-12, 1e-11)`) at
ε = 1e-12; not pinned.
