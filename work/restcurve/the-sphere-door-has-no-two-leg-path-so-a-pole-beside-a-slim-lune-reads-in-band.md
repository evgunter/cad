---
id: the-sphere-door-has-no-two-leg-path-so-a-pole-beside-a-slim-lune-reads-in-band
kind: issue
title: The sphere door has no two-leg path, so a pole beside a slim lune reads in band and tier 3's pole side still reads locate
status: open
opened: 2026-11-01
priority: P1
cost: H
refs: [the-sphere-door-cannot-settle-a-pole-at-the-certified-scalar, a-loop-the-chart-singularity-walk-cannot-certify-is-exempt, import-reads-a-chart-singularitys-side-by-the-chart-walk-tier-3-is-to-leave, the-cone-door-reads-an-apex-generators-crossings-through-a-cut-angle]
---


## What

DESIGN `[topology.tiers]`: "whether a point lies in a face has one
reading, the point-in-face doors'". Tier 3's chart-singularity arm still
reads a sphere pole's or cone apex's side, off the boundary, by
`geom_brep::chart_singularity::locate` (its stereographic closed form,
or `walk_loop`). The sphere door now settles a pole's crossings at the
certified scalar
(`the-sphere-door-cannot-settle-a-pole-at-the-certified-scalar`, closed),
but with the switch built (the face's door at the point, every surface
cycle kept, `boolean::cycle_side`) tier 3 falsely refuses one
kernel-built body that the base passes:

- `crates/sweep/tests/blend4_r1_probes.rs`
  `p2_slim_skews_carve_valid_or_refuse_typed_never_worse` at ε 1e-6
  (f64), the 1° skew: `ChartSingularityUndecided` `Side(Banded)` on faces
  19 (North), 22 (South), 23 (North) and 34 (South), margins ±7.615e-6
  on `bool_sphere_region_roots_extreme` and `bool_sphere_region_span`.

Geometry (face 19, probed): a fillet corner sphere of radius
`R = 8.726e-4`, its patch a 1° lune from the equator down to the south
pole, a vertex of the face: two meridian arcs from the pole to the
equator, 1° apart in azimuth, and a 1° equator arc closing them. The
north pole is 90° off the face and plainly outside. Every ray the door
casts from it is a meridian:

- one at another azimuth meets the face only at the south-pole vertex,
  at its antipode, a sideless closest crossing, and grazes;
- one through the lune runs within 0.5° of both meridian arcs, whose
  residual extremes against its plane are `R·sin 0.5° ≈ 7.6e-6`, inside
  the band (1e-6, 1e-5).

So no great-circle ray from the pole decides, at that ε, a point no
distance puts near the face.

## Owed

1. **A two-leg path in the sphere door**, as the torus and cone doors
   have (`region::Paths`, `Path::Aimed`): a great-circle leg from the
   point to a corner, then a great circle through a target, which can
   cross a slim face's arcs head-on (here: down a meridian to the
   equator, then along the equator through the lune). Its rows: the
   blend4 body above at three ε, and the door's existing rows.
2. **Then the switch**: tier 3's chart-singularity side through the
   face's door, every surface cycle kept; an in-band reading refuses
   `ChartSingularityUndecided` with `SideUnread::Banded`; the at-a-vertex
   and inside-an-edge placements stay on `place_on_boundary`; tier 3's
   use of `walk_loop` and its area-error bound retires. The switch was
   built and reverted on `restcurve/pole-door` (commit `2608e936`'s
   `validate.rs` hunk, with its prose); the six interval rows of the
   closed item pass through it, and
   `props_sphere_cap_door::a_tilted_cap_holds_a_pole_by_its_region`
   and `a_pole_a_hair_either_side_of_a_tilted_rim_is_read_at_both_scalars`
   must keep passing through it. A body-level row for the in-band
   refusal is on that branch's history too (a zone whose rims are 3ε
   apart, both poles undecided on `bool_sphere_region_order`).
3. **Rows the switch must keep or turn green through the door**:
   `sweep one_region_rows::a_sliver_beside_a_pole_passes` (PR 6054's
   10 µm sliver 1 cm from a pole);
   `props_sphere_cap_door::a_pole_a_hair_either_side_of_a_tilted_rim_is_read_at_both_scalars`
   (the door at the pole beside tier 3, δ 1e-2/1e-3/1e-4 rad, f64 and
   Interval); and a near-pole row: PR 6252's review found about 22
   points 1e-6 and 1e-9 rad off an exact pole, off a partial
   hemisphere, that the door still escalates at `Interval`
   (`restcurve/review-quince-6252` @ `0392d9f31`,
   `crates/sweep/tests/one_region_rows.rs`).
4. `a-loop-the-chart-singularity-walk-cannot-certify-is-exempt` then
   closes for tier 3; import's half waits on FOREIGN's
   `import-reads-a-chart-singularitys-side-by-the-chart-walk-tier-3-is-to-leave`.

Filed by RESTCURVE's pole-door lane on the orchestrator's ruling.
