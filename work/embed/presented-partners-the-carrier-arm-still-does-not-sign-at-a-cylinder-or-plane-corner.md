---
id: presented-partners-the-carrier-arm-still-does-not-sign-at-a-cylinder-or-plane-corner
kind: issue
title: Eight census tests whose refusing pair has a reflex or lone-clip corner and a presented partner still refuse once the carrier's rate is read square to its axis, for causes not traced
status: open
opened: 2026-11-22
priority: P2
cost: M
---


## Finding

Found by EMBED's lane `embed/iso-end` (2026-11-22). With the cylinder and cone carriers' rate read square to the axis, the carrier arm reads a reflex or lone-clip corner against the partner's presentation. These tests keep a refusing pair at such a corner, and their partner is presented, but the arm still does not sign it; the cause is not traced.

**Cylinder bucket (3).** In the two `band_co_requested_boundary` rows the reflex face's carrier read along the partner's zero side is not zero (`zero_on=false`, Cylinder/Cylinder). In `axis_lap` no side read is printed at all.

Measured on `embed/iso-end-measure2` (`origin/embed/gate-measure` @ `ebb2e6f8e` plus `embed/iso-end`'s diff), where each refusal is the same pair, test by test, as on the trace run that named the exits (`embed/iso-end-measure` @ `030c75763`, `CV_TRACE`).

- `sweep::all band_co_requested_boundary::a_ruled_link_co_requested_with_an_annulus_rim_is_metered_at_its_trimline`
- `sweep::all band_co_requested_boundary::the_co_requested_rod_refuses_and_carves_on_its_closed_form_clearance`
- `sweep::all axis_lap::laps_off_the_rulings_build_at_the_analytic_volume`

**Plane/plane departure census (5).** The pair holds a reflex lean on one face and the partner is presented. In four of them no carrier side read is printed. In `p4_mixed_pair_refuses` the partner's span is 0.00195 and its departure straddles zero. These were measured on main `a29fd3695` with the trace prints on `embed/iso-end-measure`.

- `sweep::all join_pierce_runs_sweep::an_edge_split_on_the_corner_keeps_its_bits`
- `sweep::all join_pierce_runs_sweep::an_island_face_pinched_to_its_holes_ring_stays_its_own_face`
- `sweep::all join_rc_probes::reflex_corner_a_hair_off_flush_passes_its_own_census`
- `topo::all review_f7_pole_r1_probes::p3_inset_coplanar_patch_still_refuses`
- `topo::all review_f7_pole_r1_probes::p4_mixed_pair_refuses`
