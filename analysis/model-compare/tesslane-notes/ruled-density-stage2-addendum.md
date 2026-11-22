(4c lane) Addendum to the stage-2 report, answering your 20:09Z message, which reached me after the report. Head unchanged at 4b9c52c3bc.

7054's re-split mutant (cut_along_runs returns no cut; throwaway, not committed): only a_run_on_a_cone_face_cut_by_an_oblique_plane_meshes and a_run_on_a_cut_cone_faces_seam_meshes go red, at both the default ε and 1e-12. All 13 other split_rulings rows stay green, because their bodies are walked and never reach developed_runs.

What 7054's rectangle rows assert under the walk, each unchanged and passing: a_run_of_ruling_points…, every_run_of_a_draft_cones_rulings_meshes, a_run_at_a_corner_is_anchored_along_its_ruling (its pinned count of 3 cone-face triangles holds), a_frustum_seam_split…_meshes, a_draft_cone_ruling_split…_meshes, two_runs_anchored_by_the_apex_mesh.

The needle pin (the size-1000 cone, split beside its rim end) meshes under the walk at every cell at all three ε, so it became a_split_beside_a_large_cones_rim_end_meshes. It holds the count relation, check_mesh and check_winding; the volume is read only where the body finishes, and at 1000 it is outside the session box. The other refusal row, the band shorter than the development resolves, became a_cone_band_shorter_than_the_developments_resolution_meshes, for the same reason. Neither assertion was weakened: each refusal is now meshing plus the count relation.
