# M-c: points of a face's boundary outside its own cover

## What was computed, and against what

For each face a door read (f64, once per body and face per test; a body
mutated in place at the same address is read once, so this undercounts
in-place changes), against the face's own cover (`face_cover`, or
`face_cover_unlifted` on the lift's turns):

- each boundary vertex, and 17 points of each boundary edge (its certified
  carrier at `t₀ + k·Δt/16`): the point's distance outside the cover (the
  near end of `Cover::distance_range(Site::Point)`, zero inside), and for a
  point outside by more than 1e-9, its distance off the face's surface
  (`implicit_residual`, a first-order distance; poison on a spline face);
- each boundary edge's own cover's support against the face cover's along 14
  directions (the earlier `edge_cover` column).

A point outside by more than its off-surface distance (more than 3× it, plus
1e-9) would be a cover that misses a point of its own surface: a cover that is
not a cover. A point outside by about its off-surface distance is an edge not
on its face: the cover is right and the record is not the face it governs.

Coverage, ε = 1e-9: editor-core and step-import in full plus 454 sweep tests
(the first run, cut off by an outage), then topo and sweep in full (8,163
tests, 9 capped, 9 failing as on the base). The M-d "< 1" counts are from the
second run (topo and sweep).

## Result

**No cover that is not a cover.** Of every sampled point outside its face's
cover by more than 1e-9, none is outside by more than its own distance off
the face's surface: the cover trims to the surface, and every such point is
off the surface by about as much as it is outside (sampled cases: `out ≈ off`
to 4 digits where `off < 1e-6`). Nothing to file.

**Points outside by more than 1e-6, all off their face's surface**, by door
(all reads, f64):

| Door | Cylinder | Sphere | Torus | Cone | other |
|---|---|---|---|---|---|
| `euler::certify_edge_spec` | 27,696 | 19,285 | 9,739 | 79 | Spindle 21 |
| `sweep::blend::surgery` | 20,304 | 14,090 | 6,623 | | |
| `euler::certify_rebased_run` | 13,371 | 2,799 | 935 | 1,337 | |
| `step_import::adopt` | 1,251 | 1,014 | 773 | | |
| `chart_window` | | 1,544 | | | |
| `boolean::combine` (graft) | 633 | 20 | | | |
| `attach` (vouch, redescribe) | 140 | | | | |
| `cover::cone_may_hold_apex` (the lift's turns) | | | | 92 | |
| `validate` tier 3, at rest | 16 (sampled) and a few per test | | | | |

These are edges or vertices standing off the face they are filed under, by
0.03 to 700 m (largest: `reach_split_gate_pose::a_fixture_past_the_session_box_is_refused_by_it`),
mid-construction: an Euler op certifying an edge onto a face whose surface is
not yet the one it will carry (the face is still its seed's or its predecessor's
carrier), the blend surgery reading a strip's neighbour before the swap, a
rebased run's edges before the move lands, and step-import's adopt reading a
face against a surface the file states differently. Bodies, largest first:
`a_slit_tip_on_a_curved_face_reads_its_full_turn::a_full_height_slit_in_a_full_bore_meshes`,
`a_ball_with_no_room_at_its_rim_refuses_typed::*`, `blend5_r2_probes::a_bore_rim_hosts_its_plane_too`,
`a_smooth_join_imports_by_the_must_carry_rule::a_scaled_join_is_read_by_the_rule_at_its_new_size`
(15.4 m, Torus), `a_split_through_a_wrap_edges_vertex::*`,
`reach_continuation::*`, `a_ring_on_a_sphere_face::a_pierce_ring_beside_a_sphere_island_stays_outside`.

At rest (tier 3's door) every case past 1e-6 is a fixture built off its face to
be refused: `a_face_moved_off_its_edges_ends_as_their_incidence_read_at_rest::*`
(0.5 m and 5.5e-9 m, the latter its band-edge row),
`verbs_gate_r1_probes::*`, `one_region::tests::*` (check 9's crossed loops),
`verbs_shell::*`, `shell10_r2_probes::r2_e2e_axial_door_…`,
`validate::door_roster::a_horn_a_hair_past_r_…`, `…a_sheet_apex_at_a_face_vertex_passes`,
`tier3_tests::datums_inside_their_conventions_…`.

**Points outside by (1e-12, 1e-9]** (below ε), by door: tier 3 ≈ 570, Euler
≈ 800, boolean ≈ 750 (ops, combine, edge_join), splitting and pcurves a few
dozen. Their distance off the surface was not recorded below 1e-9, so a cover
rounding inward cannot be told from an edge certified within ε of its face
(certification admits that). Both are inside the band.

## The M-d "< 1" row, attributed

37,150 reads (topo and sweep, 1e-9) where an edge's sampled reach stands past
the bound from above: `euler::certify_rebased_run` 35,015, `certify_edge_spec`
956, the lift's turns under `cone_may_hold_apex` 578, `blend::surgery` 493,
`boolean::combine` 97, `step_import::adopt` 11 — the same doors as the table
above, so the same cause: a boundary edge off the face it bounds,
mid-construction. Not a sampling artefact: each sample is a point of the edge,
so sampling can only under-read an edge's reach, never place it past a bound.
None at tier 3's door.
