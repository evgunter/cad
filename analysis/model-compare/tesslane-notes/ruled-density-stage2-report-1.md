UNIT 4c STAGE-2 REPORT, part 1 of 2 (part 2 is the PR body). Head 4b9c52c3bc9031258cc2c326665f7a8cb52022b4 (git ls-remote confirmed); main merged at e0f86b7092+ (7054 has landed; 7041 has not). No trailer, model or session line in any commit.

WHAT CHANGED, BY FILE AND COMMIT
- crates/mesh/src/ruled_walk.rs (new; a622c9bb9d, 0d3b0b3fde): Rows::of reads the face's two rows and its sides off the trim polygon; Rows::walk merges the rows by angle and fans each split side from the corner opposite. The module docs hold the proof and "No edge of four triangles". Two unit rows.
- crates/mesh/src/trimmed.rs:
  - `walked` (a622c9bb9d, cbf68f6506): no ring, no trim carrier, splits_a_straight_side, and the loop is rims and lines in one of the two shapes. One run of rims needs the apex on a side; two runs need it on neither. A cone face through its apex onto the other nappe, or one whose apex the band cannot decide, keeps today's path.
  - `walk_face`: per-triangle resolved_normal, outward check, cert::verdict.
  - `face_target`: no headroom on a walked face.
  - Docs: UNPROVED_HEADROOM.
- sizing.rs (5887116590): the walk's theorem at sagitta_step and cone_grid_step (the hull argument, the apex as a row of one point, the fan in its own plane); the module's line on the lanes without a theorem.
- chart_table.rs: a stale comment (the curved lane lays no cone rows).
- curved.rs: the split-and-placed wedge now has 28 triangles (was 40).
- tests/all.rs (995e6e0c6f): the ε inventory gives trimmed.rs 3 carriers (walk_face hands tol.eps to the verdict).
- tests/split_rulings.rs: the rows (below). reach_probes.rs: geometry_enclosing is pub(super). nextest.toml: the slow-set name.
- work/:
  - closed a-cone-face-with-a-split-side-has-no-worst-triangle-theorem and a-ruling-split-a-hair-from-its-end-folds-a-triangle;
  - dated note on a-run-anchored-on-a-needle-triangle-makes-triangles-f64-cannot-wind (its pinned bodies are now walked; the class stays open on non-rectangle cone faces);
  - the DevelopedMerge item takes main's (7054's) closed text.
  - The unit item's diff against main is your dispatch commits (51de818881, 14389dc621), not mine. I did not edit it, the README or the log.
- Stage-1 switches and probe: deleted (ruled_probe, CURE, ruled_cures_probe.rs).

YOUR POINTS
1. The walk is chosen inside the lane (trimmed::walked, read by face_target and by tessellate_trimmed after the polygon). tessellate.rs is untouched, so 7041 should merge cleanly. Faces of the class whose loop has neither shape: in the corpus and in mesh/sweep/step-import, none. Only a both-nappe cone face would, and no body built here makes one.
2. Only walked faces lose the headroom. In neighbours, only the chords of the edges they share with a walked face move. Patch digests, base main 35570b0552 against head 5887116590, over mesh + sweep + step-import (release, CAD_FUZZ_SEED=1):
   - 80,241 patches of other faces are byte-identical;
   - 121 walked patches moved (53,142 → 1,436 triangles);
   - 178 neighbour patches moved (2,792 → 2,390); 297 neighbour patches are identical;
   - every moved patch lost triangles; none meshes on base and refuses on head;
   - only three tests: curved's multiply-carried-iso-side and split-and-placed wedge, sweep's a_box_along_an_inflection_ruling, reach_probes' cone met on its axis.
   Not re-run after the 7054 merge or the apex check (that check moves no body built here).
3. The proof is at the step functions and in ruled_walk's docs. The worst certificate is 0.998 δ by design: every rim chord spans at most h (chords::tighten at the face's δ), h is solved so that one column's sag is δ, so the worst triangle, a chord of the largest rim against the next row, lands just under δ. The ceiling count rounds it down.
4. The ear question: no. A walked edge spans at most one rim chord. Two non-adjacent polygon points that one neighbour holds both of lie on the face's two different sides, a whole face apart, so the walk joins them only on a face one chord wide. Then the neighbour can't make the edge as well:
   - a plane holding both sides collapses the caps to a 2-gon;
   - a walked or rectangle neighbour on the same surface spans the rest of the turn;
   - a CDT cone face splits the triangle itself; on a CDT cylinder the certificate refuses it unless every chord is within δ (radius < δ/2), and even then the edge is its chart's whole diagonal across its lattice. No body here reaches that corner.
   I first had the walk carry the shared-side split. It never fired in any suite, and on the decisive body it would add 2 needless triangles, so it is gone. Row no_walked_triangle_has_an_edge_a_neighbour_makes (154 cells) holds the exact count relation and check_mesh on: a band (the wall cut by mef along a ruling 0.5 rad from its seam, so a one-chord face and a wide face each hold both of the other's sides); a D prism (at δ 0.03 and 0.01; at one chord its caps don't mesh on base either); a half cylinder and a half cone through the axis; a 0.5 rad wedge.
5. Rows:
   - a_split_side_adds_one_triangle_per_point: 330 cells, every face = unsplit + gained traversals, at all three ε.
   - The hair pin is a meshing row (20° at default and 1e-12; 5° at 1e-12; 1° builds at no ε).
   - The DevelopedMerge pins became a_ruling_split_within_two_to_the_minus_27_of_its_slant_meshes (frustum 1e-9 at 1e-12, the draft wall at all ε, your 2° cone at the default ε).
   - The thin planar neighbour is cured, in the 330-cell row (the slender wall at δ 0.1).
   Mutants, all caught:
   - walk off: 5 rows;
   - a triangle spanning two chords: the unit row's chord check + 5;
   - the side fan from the wrong corner: both unit rows + 6;
   - headroom kept: 4.
6. 7054's rows after the merge: a_run_on_a_cone_face_cut_by_an_oblique_plane_meshes and a_run_on_a_cut_cone_faces_seam_meshes still reach developed_runs (probed at the default ε and 1e-12). These go through the walk instead and pass unchanged: a_run_of_ruling_points…, every_run_of_a_draft_cones_rulings_meshes, a_run_at_a_corner…, a_frustum_seam…, a_draft_cone…, two_runs_anchored_by_the_apex_mesh. Two of its rows asserted refusals the walk cures; they now assert meshing plus the count relation, renamed:
   - a_run_anchored_on_a_needle_triangle_refuses_unresolved → a_split_beside_a_large_cones_rim_end_meshes (the volume is read only where the body finishes; the 1000 cone is outside the session box);
   - a_cone_band_shorter_than_the_developments_resolution_refuses → …_meshes.
   This changes 7054's rows, so please confirm. No row now pins a needle on a non-rectangle face.
7. One [mesh.sizing] sentence is now incomplete, quoted: "Depending on the lane, the worst triangle is half a grid cell, or a Delaunay sliver beside a point the grid did not place (a trim or chord point)." True instead: "…, a Delaunay sliver beside a point the grid did not place (a trim or chord point), or, on a ruled face the chart lane walks, the triangle on one rim chord." The headroom bullet stays true per face.
8. Done (above).
