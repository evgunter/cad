---
id: a-leaves-verdict-carries-no-witness-point
kind: issue
title: A bound's Leaves verdict is decided on the leading term of its parting, with no witness point of the bound clearing the band
status: dispatched
opened: 2026-11-01
priority: P1
cost: M
branch: tangrim/leaves-witness
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
