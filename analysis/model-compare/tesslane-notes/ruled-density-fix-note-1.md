(4c lane) The MAJOR's cure is ready locally as commit 9908f164e6, one ahead of 4b9c52c3bc. Not pushed.

Reproduced: the 1° cone turned 0.05 at δ 3000 refuses UnresolvedWinding (height 1.6e-14 against an allowance of 1.8e-12). The cause is as the reviewer read it. On a one-chord face to the apex the walk writes the one triangle (a0, a1, apex), which holds BOTH sides. The first side's fan consumes it, so the second side's search finds its edge in a fanned triangle and fans from a point of the other side, a sliver when that point is near the apex. "An edge of exactly one triangle, the first or the last walked" was false on exactly that face; it holds on every other, since n ≥ 2 rim chords or a second row puts the two sides in different walked triangles.

Cure: ruled_walk Rows::zip_sides. When m = 0, n = 1 and both sides hold points, their points are joined to each other in order of distance from the apex, from the rim chord in (advance the side whose next point is farther from the apex), closed by the one triangle at the apex. All of these lie inside the walked triangle, in its plane, and each has a corner on each side, so none lies along a ruling. There are k + m + 1 of them: the count relation holds.

Restated:
- the comment at the fan;
- walk's doc;
- the module proof (zipped triangles divide the walked one in its plane);
- the ear argument's list of edge kinds (adds "an edge between a point of each side, on a one-chord face to the apex with both sides split"); its conclusion stands, since such edges join the two sides on a one-chord face, the case the argument already treats;
- cone_grid_step's paragraph.

Rows:
- a_one_chord_face_through_the_apex_with_both_sides_split_meshes: your body, 1°/5°/30°/45° × 0.05/0.5 rad, every line edge split at 1−g, 0.5, g, count relation. Default ε: height 1000, g 1e-8, δ 3000 to 3. 1e-6: g 1e-4, since split_edge refuses 1e-8 on the 1° cone there. 1e-12: height 1, δ 3 to 0.003, since a 1000 cone doesn't revolve there.
- Unit row a_one_chord_face_to_the_apex_zips_its_sides, which pins the zip's exact triangles.
- no_walked_triangle_has_an_edge_a_neighbour_makes widened: adds a one-chord apex cone turned 0.5 rad, and every line edge split at once (both sides of each one-chord face), at the middle and at 0.9/0.5/0.1. 218 cells. It and the new apex row go into the ci slow set (3.1 s and 1.0 s locally); its invariant does not survive a trim.

Mutants, rerun clean:
- zip off: the apex row reds at the default ε and 1e-12, and the unit row reds. The 1e-6 cells stay green: at the 1e-4 gap the wrong fan still divides the triangle in its plane, resolved, so no sliver.
- walk off: 8 rows.
- a triangle spanning two chords: 11.
- the fan from the wrong corner: 16.
- headroom kept: 7.

Also: mesh whole 525/525; clippy and the doc lints clean; split_rulings, ruled_walk and curved green at the three ε. Waiting for your word to push.
