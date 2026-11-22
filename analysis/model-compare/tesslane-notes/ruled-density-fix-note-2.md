(4c lane) MAJOR-2 and the smalls are ready locally: 9908f164e6 (MAJOR-1), f775613fc8 and ee68db7ffe, three ahead of 4b9c52c3bc. Not pushed.

MAJOR-2, reproduced. Both bulge signs (sign +1 gives the torus neighbour, −1 the sphere): the seam split at [0.5] and at five points is non-manifold at δ 3 to 0.3, and to 0.1 with the sphere. The unsplit ball is non-manifold too, at its seam (0,1) (main's).

Cure, two steps:
1. The shared-side rule is restored in walk_face, read from faces_across (one reading, with neighbours on the polygon taken by mesh id).
2. A shared side is cut, not its triangle split. Splitting at the centroid, as my removed code did, keeps all three sides, so the shared edge survived in a sub-triangle (measured: edge (1,3) still four-use). The cure cuts the shared SIDE at the surface point at its chart midpoint (the apex's u left out), so each triangle on it becomes two, between the same rulings.
Every cell of the bored ball now meshes manifold and is the body's.

The count relation has a stated exception, Walked::SharedSidesCut: +2 per cut side, on the walked face alone (an even surplus); every other face stays exact. The ear argument in ruled_walk is restated to what is true.

COST, measured, for you to weigh: the rule reads which faces hold the points, not how they mesh. So on a ball bored at a fine δ it cuts sides the neighbour's interior rows would never make: the bore goes 80 → 240 triangles at δ 0.03 (sphere neighbour) and 38 → 114 (torus), where base meshes 80 and 38 manifold. It also cuts the one-chord band's rim-to-rim side, which the wide face never makes (+4 there). Narrower test I did not build, since it is a second fact: a neighbour can make the edge only in a triangle its certificate passes, so cut only where the edge lies within δ of the neighbour's surface.

Filed (main's): work/tesslane/a-strip-against-a-neighbour-holding-both-rims-makes-an-edge-of-four-triangles.md, P1, with the body and cells. It is the ear item's class, the chord across two chains instead of one chain's ear; not the coincident-edges item. The walk's cut would cure the rectangle lane's unsplit bore if read there, at the same density cost; not done here.

Rows:
- a_walked_face_whose_neighbour_holds_both_rims_shares_no_edge: both signs × 2 splits × δ 3 to 0.03, 20 cells, all three ε.
- no_walked_triangle_has_an_edge_a_neighbour_makes takes SharedSidesCut.

Mutants:
- shared-side cut off: reds the bored-ball row only (the band never needed it);
- zip off: 4 rows;
- walk off: 9;
- two chords: 13 (including the new sizing row);
- wrong corner: 17;
- headroom kept: 7.

SMALLS
- a_routed_face_is_chorded_for_the_chart_lane: deleted. What it held (the routed face's neighbours' counts, which the chord pass decides) is subsumed: a_split_side_adds_one_triangle_per_point asserts every face's count, neighbours included, equals the unsplit face's plus its gained points, so a neighbour chorded otherwise fails there. For the PR body.
- The duplicate-id continue in walk_face: removed. No walked triangle holds both copies of a seam: a full turn has at least 8 chords (MAX_ANGULAR_STEP). If one ever did, resolved_normal would refuse it typed.
- The singular flags to row.cert: kept. They are an equivalent mutant: cert_cone zeroes an apex corner's slant whether or not it is flagged, and every apex the lift places sits at v = 0 exactly, so no row can tell. They stay for the certificate's contract ("a corner on a singular row has no azimuth"); the cut's midpoint reads the same flag to leave the apex's u out.
- sizing::tests::a_walked_triangle_lands_at_delta (new): the real walk at the solved step on a cylinder, a frustum (the smaller rim two chords finer) and a face to the apex, three points on each side, sampled in 3-D. All triangles within δ, the worst at δ to 1%. The two-chords mutant reds it.
- rim_runs' filter is now one function (rim_run_starts).
- The fans take their triangles by construction: the first walked triangle holds the start side and the last the end side, n + m ≥ 2 when not zipped, and the corner is the triangle's third index. The search and its unreachable! are gone.
- Docs: sizing.rs' opening and trimmed.rs' module header now name the walk.
- The volume-unread arm now runs every geometry check but the volume (reach_probes::geometry_but_the_volume; it was winding alone). Each row asserts its count of such cells: the band 24 at every ε (the mef'd body never finishes, so its 24 cells had checked winding only), the large cone 3 at the default ε, the draft wall 6 at 1e-12, 0 elsewhere.
- Zero-body ε: the hair row now asserts each split_edge refusal by variant (at 1e-6 both NotInterior; at the default the 5° cone Escalated).
- Stale citations fixed: seven work/ files and reach_probes' cone-on-axis doc.
- 7054's walked rows, renamed and re-documented to what they hold (assertions unchanged):
  - a_run_of_ruling_points… → a_draft_cones_seam_split_within_its_developments_resolution_meshes;
  - every_run… → every_close_split_of_a_draft_cones_rulings_meshes (slow set and its comment updated);
  - a_run_at_a_corner… → a_split_beside_a_corner_of_a_draft_cone_meshes;
  - two_runs_anchored_by_the_apex_mesh → a_one_chord_face_to_the_apex_split_beside_both_rim_ends_meshes (it is MAJOR-1's shape, now zipped);
  - the frustum and draft-cone rows keep their names, with docs restated.

Checks: mesh whole 527/527; changed rows at 1e-6 and 1e-12 104/104; clippy, fmt, the doc lints, the eps inventory with budget, the topology gate and work lint all clean.
