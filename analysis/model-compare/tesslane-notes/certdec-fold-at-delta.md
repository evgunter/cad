certificate-decides: the fold cure tried at δ with my correction in. Your 09:35Z, 10:33Z and 11:32Z notes all reached me only after the stage-2 report (my turn was running), so this answers them together. Nothing is committed or merged; my branch head is still 42d61a3806.

HOW: 4922edde4a does not merge onto my head cleanly (curved.rs 1 hunk, trimmed.rs 7, reach_probes.rs 2: it restructures the emit pass my correction sits in). So for the measurement I went the other way: a scratch checkout of 4922edde4a with my unit ported onto it by hand (headroom at 1.0, side-sag correction on every row, its own budget of 6 beside MAX_GRID_RETRIES, a per-face print of rounds). The rectangle lane's correction is not ported there. Dev profile, ε 1e-9.

THE FIVE CELLS: all mesh.
- `mesh` `reach_probes::ringed_curved_faces_that_still_refuse_answer_by_cause` passes as that branch pins it. The near-pole ringed sphere face: δ 2e-2 meshes after 5 fold rounds, δ 5e-3 after 2 fold rounds, δ 1e-3 with 0 fold rounds and 1 correction round.
- `sweep` `tilted_sphere_pair::a_circle_near_the_pole_meshes_at_a_coarse_delta` passes; its four sphere faces at δ 4e-2 mesh with 2 fold-or-shared rounds each and no correction round. (Your neighbour measured one fold round; my print does not split fold from shared-side rounds.)

CROSSING RODS: `reach_probes::crossing_rods_keep_no_side_a_neighbour_can_make` passes at 4922edde4a + my unit, every cylinder face in 1 to 3 shared-side rounds and 0 correction rounds. I could not check the cell you name: at this commit the shared-side insert is still in, so no face reads over δ. It needs the commit that retires the insert; tell me which and I will run it by name.

WHAT ELSE MOVED, over `sweep`'s 50 chart-lane tests (2,047 trimmed faces): 48 pass, 2 fail.
- `bisector_nodes::an_oblique_seam_near_a_node_builds` and `bisector_nodes::equal_cylinders_with_meeting_axes_build_every_operation_at_the_closed_forms`: both on the same body (60°, r = 0.5, ∩), both on the row's mesh-volume oracle, not a refusal: "the mesh encloses 0.76449, want 0.76980", 0.69% against the row's half a percent. The mesh is returned, certified and watertight at δ 2e-3. Both rows PASS on my own branch at δ (rowless cylinder merged), so the difference is that tree's cylinder lattice or its other changes, not δ sizing alone. I have not found which. The oracle's half-percent is the row's own number ("the chordal deficit at 2e-3 is a fraction of that"), and a full-sag inscribed polygon on r = 0.5 at 2e-3 is about 0.53% per cylinder, so it may be the oracle that is tight at δ rather than the mesh that is wrong; unmeasured.
- 6 face-meshings refuse `FoldedTriangle` after 6 fold rounds, all in `spheres_crossing_off_every_edge::a_trimmed_face_cut_beside_its_pole_builds` at δ 5e-3: the same six that fold on today's tree and on my branch (stage 1's logs), so not new and not cleared by the cure at δ.
- Correction rate on that tree is far higher than on mine: 1,051 of 2,047 faces take one round and 12 take two (mine: about 2% take any). That tree lays lattice rows on the cylinder (unit 4b is not under it), which is what 4b measured as the cause of over-δ triangles there. It should fall back once both are on main, but it is worth re-measuring then: if it does not, the work table in my report does not hold on the landed tree.
- No face is lost on the certificate; most rounds used 2.

NOT RUN on the ported tree: the whole `mesh` suite (its pins there are at the old target, so every count pin is red and says nothing), the other ε, `step-import`.

On your 09:35Z: the rectangle correction does insert through `cdt::lattice_point` (I took main's `insert` closure and added my points through it).

Next from me, on your word: merge main once join-split has landed (expect real conflicts in trimmed.rs's emit and loop), re-measure the five cells, the two bisector rows, the correction rate and the viewer pins, then report.
