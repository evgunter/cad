---
id: a-body-level-crossing-through-a-cone-apex-along-a-planes-second-ruling-is-unbuilt
kind: issue
title: No body pins the end-vertex plane's refusal of a crossing: a plane face sharing a ruling with a cone face and holding the cone's second ruling through the apex, which no public door builds
status: open
opened: 2026-11-21
priority: P2
cost: M
---


Filed by lane `embed/apex-separator` (2026-11-21).

`departure`'s plane through the pair's one shared vertex, each face
presented against it alone (`at_end_vertex`,
`crates/topo/src/injective.rs`), is pinned against a crossing at unit
level only:
`injective::presentation_rows::a_wedge_plane_holding_the_cones_second_ruling_is_not_parted`,
a hand-built plane face whose corner at the apex runs on past the axis
and holds the cone's ruling at `u = π`. With the separator accepted
where it signs one face only (the lane's mutant M2), that row goes red,
and so does
`self_intersection_proves_injectivity::a_cone_wedge_past_a_half_turn_is_embedded_at_its_apex`
in its turned frame, a valid body; no crossing body in the suite does
(`a_seam_crossing_cylinder_pierced_by_a_face_on_its_edge_never_holds`
stays green under M2: its flap crosses the wall off the strut's ends).

**What the lane tried.** A reasoning pass, not a build:
- a revolve refuses a profile across its axis
  (`RevolveError::VertexCrossesAxis`, `ArcCrossesAxis`), so its wedge
  plane never runs on past the axis to the cone's second ruling;
- every Boolean door certifies its result.

**A candidate not tried:** the shape of the pierced cylinder's flap
(`topo::construction`, `mev` and `mef` on a sheet), on a cone wall
sheet: a planar flap on one ruling, in the plane of that ruling and the
axis, run on past the axis so it holds the ruling half a turn away. The
flap crosses the wall along that ruling from the apex out, and only
cells at the apex are near both the shared ruling and the crossing. No
cone wall sheet fixture exists (`cyl_wall_sheet_keyed` is the
cylinder's, and a cone's bottom collapses to the apex).
