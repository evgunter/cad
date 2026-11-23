certificate-decides: your 16:22Z, 17:31Z and 19:23Z notes reached me together just now, after the list's result went out. Nothing is pushed; the tree is clean at 1cfa09180c (every mutant edit reverted).

On 16:22Z/17:31Z: I merged b448200999 and then main at a497926d13, which holds all of 4b; `common/rowless.rs`' `loop_edges` line is main's spelling. Nothing built on PR 7573.

(1) THE FOUR MUTANTS, re-run on 1cfa09180c (dev, local):
- correction off (both lanes): 5 rows red: `curved::tests::an_over_delta_grid_is_corrected_and_refuses_typed_without_the_correction`, `trimmed::tests::an_over_delta_first_guess_is_corrected_and_refuses_typed_without_the_correction`, `m5_pr11_trimmed::a_wide_tilted_cylinder_meshes_at_every_delta`, `rowless_cylinders::a_cut_wall_is_its_boundary_points_alone`, `rowless_cylinders::two_holes_across_a_column_and_a_half_mesh_by_the_correction`.
- correction's rounds also counted against `MAX_GRID_RETRIES`: 1 row red, the chart-lane row above (its own-budget assertion).
- point at the centroid: `sweep` `snowman::two_cut_ins_on_one_face_build_in_either_order` red, and now also the two `rowless_cylinders` rows (their pinned point counts).
- point at the middle of the longest free side: `sweep` `bisector_nodes::slanted_bores_build_every_operation_one_difference_pinched_to_itself` red, and the same two `rowless_cylinders` rows.
So since 4b's identity rows, each rule mutant is caught inside `mesh` too, which the body's table does not say yet (it names only the `sweep` rows). One thing it shows that is worth a reviewer's eye: `d9_mesh_goldens::every_corpus_body_meshes_to_its_committed_digest` PASSES under both rule mutants, so no corpus body at its two δ holds a corrected face; the goldens pin "no ε" for the sizing, not for the correction's point. The correction's ε-independence rests on the code reading only the verdict, and on `rowless_cylinders` passing at three ε with its corrected cells.

(2) WHICH SCRATCH-TREE MEASUREMENTS COULD CHANGE under PR 7573's reshaping (split point at the foot of the side's straight midpoint; census also where two faces each have a triangle of only shared points; fold cure unchanged):
- The 51% correction rate: taken on 4922edde4a WITHOUT unit 4b, so its cylinder faces laid rows; I expect most of it to be that and to fall once the integrated tree has 4b under it. The new census would add inserted points on more faces, and a split point off the chart midpoint changes the triangles beside a shared side, so which triangles read over δ can move either way. Unknown until measured; it is the number most likely to change.
- The two `bisector_nodes` volume-oracle rows (0.69% against half a percent): the enclosed volume is the mesh's, and those bodies' faces meet along shared sides where the split acts, so a moved split point and more splits both move it. Could pass or fail; must be re-read.
- The five fold cells meshing: rests on the fold cure, said unchanged, so I expect it to hold; the round counts may differ.
- The six `spheres_crossing` folds: the fold cure did not clear them at δ; unchanged unless the cure changes.
- The crossing-rods cell: not measured at all yet (needs the insert retired).

Holding for the reviews.
