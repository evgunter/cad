---
id: a-leaves-verdict-carries-no-witness-point
kind: issue
title: A bound's Leaves verdict is decided on the leading term of its parting, with no witness point of the bound clearing the band
status: open
opened: 2026-11-01
priority: P1
cost: M
---


Found by the stage-2 lane (`tangrim/sector-passes-descend`): item 4 of
`the-sector-passes-read-a-bounds-tie-first-order-beside-the-one-reading`
(the design's item 3, M2), not built in stage 2.

`bound_side::Tie::descend` decides a tied bound's side on the sign of
its parting, `(κ_b·n̂ − II_F(d̂, d̂))·l²/2` (`geom_brep::parting_side`,
predicate `tangent_sector_order2`), at the shorter of the bound's reach
and the face's lever. That is the leading term at a length, not a point
of the bound: the third-order remainder over the reach is unbounded
there, so a `Leaves` verdict need not agree with a finite-reach reader
of the same edge against the same carrier (`reduce.rs` `interior`,
`carrier_cross`, `carrier_touch::off_face`, the conic × quadric root
door) or with the result census. Only the conic-against-a-plane arm
(`geom_brep::conic_bend_side`) lifts a certified point of the edge.

Owed, per the design: the verdict stands only where a witness point of
the bound at its reach (`edge_extent_inf`, or the capped reach) stands
more than the band off the face's carrier on that side; where a root
door covers the edge and carrier kinds, its residual is the witness's
offset. The arms that now decide on the leading term only: a line or a
conic against a quadric or torus face, and a bisector against any face
(its own face's normal section stands in for its bend). The shipped
parting sites (`runs_into`, `crescent`, `corner_holds`, `walls_inside`,
`tangent_relative_side`, `tied_germ_turn`) are the same.

## The gap fallback this row retires (TANGRIM ruling, 2026-11-01)

Until the witness lands, a tied bound's parting that falls between the
zero band and the escalation band at its capped reach reads
`bound_side::BoundSide::Unresolved`, and every consumer reads it as the
first order's tie, `On`, as main did before stage 2 (`sectors::bound_read`,
`splitting::entry_side`). The arm goes when the witness reads these
bounds at the edge's extent. Its witnesses, each building on main and
refusing `Escalated { Coincidence(TangentSide), "tangent_sector_order2" }`
without the arm (band: zero `1e-9`, escalation `1e-8`):

| row | parting at the capped reach |
|---|---|
| `germ_plane_torus_frame::a_band_pose_under_the_tubes_top_reaches_the_donuts_door` | −2.0e-9 |
| `near_circular_sections::a_near_square_rod_minus_a_box_builds` | −5.0e-9 |
| `near_circular_sections::a_turned_near_square_rod_minus_a_box_builds` | −7.5e-9 |
| `pi_seam_and_kiss_through_the_boolean::a_rim_offset_inside_the_zero_band_answers_alike_in_both_member_orders` | −1.0000000827e-9 |
| `spheres_crossing_off_every_edge::a_trimmed_face_cut_beside_its_pole_builds` | 5.0e-9 |
| `tilted_sphere_pair::a_section_passing_near_a_pole_builds_and_one_through_it_refuses_typed` | 9.6e-9 |

`bound_side`'s `a_ties_parting_reads_zero_gap_and_decided_as_first_order_does`
pins the arm's rung; it changes with the arm.

**The uncertified arm (TANGRIM's third ruling, 2026-11-02).** A bend no
reading certifies (a spline face, a bisector of one, a spiric or spline
edge against a curved face) takes the same fallback,
`Unresolved::Uncertified`, read `On`: stage 2 does not move an outcome
on a reading it cannot certify, a refusal moving earlier included. Its
witness:
`a_sheath_grafted_on_its_blade::a_sheath_on_its_blade_stops_where_its_walls_touch_on_the_rim`,
a rim arc lying in the other lens's spline wall, which refused
`Escalated { ConicBend, "enters_material" }` without the arm and reads
as on main with it (the rim's `EdgeUndescribed`). That arm goes when a
spline face's and a spline edge's bend are certified.

**The lever.** The sector passes' datum takes no first-order charge
(`sectors::NO_CURVATURE`; the vertex-pair charge is
`vertex-vertex-side-codes-take-no-curvature-charge-on-curved-sector-faces`'s);
its descent reads no further than the face's own smallest radius of
curvature at the vertex (`FaceBend::Implicit::radius`). A cone's bend
tightens toward its axis within a long reach, which that radius does
not bound; the witness, read at points of the bound, would.

## Lane finding (2026-11-02, `tangrim/leaves-witness`)

The witness is built: `bound_side::Tie::descend` reads a parting off the
zero band through `Tie::witnessed`, the bound's own point at the arm the
parting was read at (`Reach::witness`: a line's chord point, a circle's
or an ellipse's arc point at an arc no longer than the arm, a
bisector's direction carried to its own face's carrier by the closed
foot `boolean::carrier_touch::distance` reads), its offset from the
face the carrier's own residual (`geom_brep::implicit_residual`; a
plane's, the height over the plane through the vertex). Decided on the
parting's side → `Leaves`; within the zero band → `Rides`; in the gap →
`Unresolved::Gap`; decided against the parting, poisoned, no point, or
its own rounding past the zero band's share (`Carrier::rounding`,
`tangent_sector_witness_rounding`: a residual that cancels in the
carrier's own size, as a 1e11 m ball's does at ε = 1e-12) →
`ReadFault::Witness`, refused `Coincidence(TangentSide)`.
`conic_bend_side` stays the conic × plane arm.

**The six gap rows do not leave the fallback.** Measured on the branch
(default ε, every tie of the suite logged with its witness):

| row | bound × face | parting at the arm | witness at the arm |
|---|---|---|---|
| `a_band_pose_under_the_tubes_top_reaches_the_donuts_door` | bisector (torus) × plane | ±2.0e-9 | ±2.0e-9 |
| `a_near_square_rod_minus_a_box_builds` | bisector (cylinder) × plane | ±5.0e-9 | ±4.22e-9 |
| `a_turned_near_square_rod_minus_a_box_builds` | bisector (cylinder) × plane | ±5.0e-9, ±7.5e-9 | ±4.22e-9, ±6.3e-9 |
| `a_trimmed_face_cut_beside_its_pole_builds` | bisector (sphere) × sphere | 5.0e-9 | 5.0e-9 |
| `a_section_passing_near_a_pole_builds_and_one_through_it_refuses_typed` | bisector (sphere) × sphere | 9.61e-9 | 9.61e-9 |
| `a_rim_offset_inside_the_zero_band_answers_alike_in_both_member_orders` | circle × cylinder | ±2.5e-10 (zero band) | ±5.0e-10 |

Every gap bound is a subdivision bisector, whose reach is its sector's
arm: the parting is already read at that length (no cap shortens it),
and its third-order remainder is a factor `(l/ρ)²/12` of it (2 % on the
rods, `l/r = ½`; under 1e-7 on the spheres). Its own point stands in
the same gap. These bounds genuinely stand 2–10 nm off the partner face
at their arm: neither within the zero band nor past the escalation band
at ε = 1e-9, so `Unresolved::Gap` (read `On`) stays for them, now meaning
the witness's gap, not the leading term's. The rim-offset row no longer
reaches the gap on main: since PR 6253's fix pass capped the descent at
the cylinder's radius its parting reads ±2.5e-10 (`Rides`); its witness,
k·zero exactly, agrees.

**What the witness changes.** In every body test of `-p sweep -p topo`,
each decided `Leaves` (39 894 readings in 371 tests: bisectors against
planes, spheres, cylinders, tori and cones, conic arcs against
cylinders, chords against cylinders) has a witness decided on its side;
none moves. The random
sampled test (`the_descent_matches_a_sampled_side_per_face_kind`) finds
arcs that leave a cylinder, a cone or a torus to one side and come back
across it within the arm; there the leading term at the arm was wrong
and the witness refuses (`ReadFault::Witness`).

**`Uncertified` stays**: no certified jet exists for a spline face, a
spline or spiric edge (`BendBounds::Unbounded`), or a bisector of a
spline face, so the sheath row keeps its fallback.

## `crescent` now mints topology (2026-11-02, PR 6375)

`sectors.rs` `crescent`'s leading-term verdict no longer only gates a
refusal: `sector_overlap` takes it where first order reads two opposed
plane faces touching, and `pair_search` records the pair as a coplanar
overlap (`PairRecord::coincident`), in every op. So its verdict now
decides topology.

That is safe without a witness point where `κ_a ≠ κ_b` (the review's
locality argument). The claim the record makes is local: the two
faces overlap in some neighbourhood of the vertex. The two bounds part
as `(κ_b − κ_a)·û·s²/2 + O(s³)` along the shared tangent, so once the
second-order term's sign is decided off zero, it dominates the
remainder for every `s` small enough. The crescent then exists near the
vertex, however thin, whatever either carrier does further out. The
reach the sign is decided at answers only whether the band resolves
it. A tie (`κ_a = κ_b` within the band, one circle bounding both)
reads `Zero`, and the pair stays touching, as first order read it. A
bend no reading certifies (a spiric section's) reads no bend
(`sectors.rs` `bound_bend`), so the pair stays touching there too,
as `runs_into` already read it. What a witness would add is the
crescent's extent at a finite reach, which nothing reads today.
