join-split STAGE-2 REPORT. Head 5061ef0142 (confirmed by git ls-remote). Nothing running.

WHAT CHANGED, by commit (no merges listed; main merged at bb90c60104 and later, with one fix commit for main's renamed edge_curve):
- cba918e34b split at the join, rules still in: crates/mesh/src/join.rs (new: Book, census, split, the point), tessellate.rs (census before placement, split at each face's turn, PatchKeys::unstore, joined() panics on overuse), memo.rs (StoredPatch keeps the notes; Placement::shared_sides/splits), curved.rs, trimmed.rs, planar.rs (notes), types.rs + pncad-py (OverusedSide gone), tests (eps inventory re-pin, errors, split_rulings relation generalised).
- 7126c19cbc coincident-end floor (chords.rs), a cone's development point, the shell row.
- 0479765c01 notes kept only for triangles on a side the patch could split; the memo footprint counts them.
- 4922edde4a fold cure (a); 7918354b29 its own budget MAX_FOLD_ROUNDS.
- b197d12c7e retirement 1 (the arms' insert, UnclearedSharedSide); 0a0c273e2e main's spline-wall digests re-pinned under it.
- fd3bfc93a2 retirement 2 (walk's cut, may_make, faces_holding_side, faces_across).
- 09052ba6a8 retirement 3 (the memo key's faces-across entry; lib.rs sentence).
- 9e4c910a0d memo + editor-core rows for a split patch naming no entry; 1962b5fb37 join::tests (point, census, pieces; join::held factored out).
- 03fc81b236, 52328dbb13: the unit file's stage-1 section and rulings; items filed/closed/annotated; the stage-1 tables removed from the tree. 5061ef0142 a doc line.
Stage-1 instrument commits (95f5e58072, a9e9185ae1) stay in history; 290e2dce43 restored main's mesh crate before stage 2, so no switch is left on the head.

RUN LOCALLY (this head or the step named):
- mesh, whole suite incl. slow: default eps after every step; 1e-6 and 1e-12 on the final head: 564/564 each.
- sweep's 131 meshing modules incl. slow, default eps: 933/934, the one red bisector_nodes::the_near_node_poses_at_the_witness_band (main's, on your list).
- viewer review_pick_r2 + index_memo: 8/8 (7498's pins hold). editor-core pick rows: 24/24. pncad-py tag rows: 32/32.
- clippy -p mesh --all-targets --all-features -D warnings, fmt; work.py lint.
- Every mutant named in the body, each run against its row.
LEFT TO CI: clippy/fmt on editor-core and pncad-py (built and tested, not linted locally), the Python binding census test (test_binding_census.py edited, not run: no wheel built), demos/tools roots (no public signature they name changed; grep finds no use of the two retired variants outside mesh and pncad-py), the nightly's remaining eps rows of other crates.

SLOWED ME DOWN: your 09:11Z and 09:35Z notes reached me only at 10:31Z, with your 10:24Z check, so the δ-target table came after the first retirement rather than before; the stage-1 per-side offer turned out wrong for a triangle with two sides split (308 on the bored ball), which forced the on-demand split; no constructible body gives a memo-hit face split in one picture and not the next (three families searched).

Not done: a per-face digest of the whole suite at each step. Each step was read through every pin and golden and the stage-1 call-level diff; the body says so.

PR BODY between the lines:
=====
**A side two faces both make is split where the patches join, once, in every patch that has it inside, at a point of that face's own surface; the three rules that predicted the face across retire.**
Unit: `work/tesslane/the-mesh-cures-a-side-two-faces-can-both-make-three-ways.md` (decided on PR 7485).

## The defect and the cure

Two faces that share two boundary chains can each make, as an inside edge of its own triangulation, the same straight side between a point of one chain and a point of the other: an edge of four triangles. Three rules answered it by predicting the face across: the trimmed lane's CDT arms inserted a point and rebuilt (`UnclearedSharedSide` past the budget), the walk cut the side where the face across could reach it (`may_make`), and the rectangle lane had no rule, so the census at the join refused (`OverusedSide`). Each read its neighbour, and the patch memo's key carried the faces across.

Now no lane reads anything about the faces across its edges.

- **The census is the join's** (`mesh::join::census`, called in `tessellate_impl` before any patch is placed). On shared ids, each side used past twice per chord carrying it is split in every patch that has it inside: two of that patch's triangles on it, no chord of its own boundary along it. A face whose boundary chord carries the side keeps it. Nothing is meshed again, no patch is chosen, nothing reads storage order.
- **The split is the face's** (`join::split`). The side splits at the point of the face's own surface over its midpoint in the chart. An end at a singular point gives the other end's `u`. On a cone it splits at the side's midpoint in the development, read back to the chart. A plane splits at the 3-D midpoint, which is its chart's. A triangle with two sides split is split twice. Every piece is read as its lane reads a triangle (`join::held`): the face's own certificate (`ChartRow::cert`, the row its lane meshed with), its fold where the lane reads one, and the winding read where the face joins the mesh (`[mesh.winding]`). A failing piece refuses the face typed, as its lane would. The split reads the census, the face's chart and its certificate's verdict; no ε is read.
- **A patch is a function of its face and δ.** Each lane notes the chart corners of the triangles on a side it could split (`join::Book`; a side only one triangle has is its own boundary chord). A memo hit carries them. A patch the join split names no memo entry (`PatchKeys::unstore`), so nothing cached against the entry (the pick index's tables) is served for it.
- **`OverusedSide` is no longer a refusal.** A side still overused after the split is the join's own defect and panics (`[failure.taxonomy]` row 4). The variant leaves `mesh`, the Python binding and its tag inventory, and so does `UnclearedSharedSide`.

Beside it, two things every cure needs:

- **The coincident-end floor** (`chords::coincident_end_floors`, after the short-loop floor). Two edges joining one vertex pair are never both chorded to the one segment between them. Where two or more are one chord each, every one that is not a line takes two. The unsplit bored ball's torus seam beside the bore's straight seam was such a segment: the census passed it as two chords' and `check_mesh` refused it.
- **The fold cure** (`trimmed`'s winding census). Every folded triangle gets a point inside it and the face is rebuilt, on its own named budget `MAX_FOLD_ROUNDS` (6), beside refinement's `MAX_GRID_RETRIES` (6). A fold left after six rounds refuses `FoldedTriangle`. The lattice-corner drop goes. Measured over the `mesh` suite at three ε and `sweep`'s meshing modules, a face that meshes takes at most three fold rounds at the trimmed lane's headroom and five at δ. Six faces of `sweep`'s `spheres_crossing_off_every_edge` beside its pole spend all six and refuse `FoldedTriangle`, as on main. The shared-side insert had been curing folds: on `sweep`'s `tilted_sphere_pair` near-pole circle at δ 0.04, 133 of its 134 inserts were for folds with no lattice corner. The fold cure carries that work under its own name.

## What retires

- **PR 7198's rule on the CDT arms**, with `TrimPolygon::faces_holding_side`, `UnclearedSharedSide` and `TrimmedFace::across`.
- **The walk's cut**, with `may_make` and `trimmed::faces_across`. `walk_face` no longer takes the chord pass.
- **PR 7241's memo-key entry**: `EdgeInputs::across` and its fold, and `ChordPass::across`. The chord pass's holders are read only by the census.
- `lib.rs`' sentence on a patch's purity now reads: a patch is a function of its face and δ, and the join adds the split points.

## Patches whose bytes moved, and why

Per step. The `mesh` suite ran whole at the default ε after every step, and every moved pin has a base run on main's tree beside the new value on this head.

1. **The split, with the three rules still in** (`cba918e34b`). It fires only where the rules miss: the rectangle lane's unsplit bored ball, which main refuses `OverusedSide`, now meshes. No golden or pinned digest moves.
2. **The floor and the cone's point** (`7126c19cbc`). The unsplit bored ball's torus seam takes two chords. No golden moves.
3. **The fold cure** (`4922edde4a`, its budget `7918354b29`). Moved: the needle funnel's upper half at h 10⁻⁵ refuses `FoldedTriangle` at δ 2·10⁻⁴ (was `UnclearedSharedSide`; that insert was a fold's) and meshes at 3·10⁻⁵ (refused `FoldedTriangle`). The near-pole tube on the ball meshes at δ 2·10⁻² in every op at all three ε (refused `FoldedTriangle`). Goldens unmoved.
4. **Retirement 1, the arms' insert** (`b197d12c7e`, digests re-pinned after main's merge in `0a0c273e2e`). Main's spline walls beside a pinned corner lose the spline arm's insert for sides no face across makes. On every moved body the join splits nothing, `check_mesh` passes, the signed volume is unchanged to 10⁻⁹, and each spline face holds two points (four triangles) fewer; the lune's last slab's small face goes from 12 to 4 triangles.
   - `d9_mesh_goldens` `pinned_wedge`: `12ff02e05d1f4e27`/`b6d0baabeae4fad2` → `6698efdff7afcdc0`/`8c4e63f51bb8b49a`.
   - `d9_mesh_goldens` `corner_pinned_through_three`: `97897a1f84e0bbe9`/`656ebbf88ac8da74` → `b40716fbb8f76c75`/`ac62579558e13fb5`.
   - `trimmed::collapsed_rows`' class: ten of eleven digests move.
   - Base run: with `b197d12c7e` reverted on this head the old digests hold.
5. **Retirement 2, the walk's cut** (`fd3bfc93a2`). A walked face gains points only where its neighbour makes the side. On the 1.05 bored ball the bore gains 13, 13 and 20 at δ 3, 1 and 0.3 (sign +1) and 13, 13, 13 and 17 at δ 3 to 0.1 (sign −1), none at 0.04. Main cut 15 to 69 a cell, cutting where the neighbour lays a row between the rims. The 1.3 ball gains none (main cut 7 to 29), and so do the bands, cones and D-prisms (main cut 2 to 7 on the band). Goldens unmoved.
6. **Retirement 3, the memo key** (`09052ba6a8`). No mesh moves; trimmed faces' keys change.

These are the bodies measured to move. No per-face digest of the whole suite was taken step by step: each step was read through every pin and golden of the suite and through the stage-1 call-level diff.

## Relations and the rows that pin them

| Relation | Row | Red when |
|---|---|---|
| No side is used more than twice per chord after the join | `tessellate::…::a_mesh_whose_patches_overuse_a_side_does_not_join` (`should_panic`); every `check_mesh` row | — |
| A split patch has two triangles more per point | `split_rulings::holds_the_relation` (now reads each face's boundary points from both meshes and its points inside), in every `split_rulings` row | — |
| The shell thinner than δ meshes, split or not | `split_rulings::a_shell_thinner_than_delta_meshes_split_or_unsplit`: the bored ball of 1.05, both signs, unsplit and seam-split, δ 3 to 0.03, 28 cells, `check_mesh` | the floor is off; the census splits nothing |
| The shell's signed volume is positive on those cells | same row: 0.078 to 0.117 against the body's 0.137, measured on these cells, not a promise; main's seam-split body read −0.0715 to +0.0715 | the split is at the straight midpoint |
| Two faces that make one arc's sides mesh with no retry | `trimmed::tests::a_side_two_faces_make_is_split_with_no_retry` (the rod sliver on one attempt, δ 2·10⁻² and 5·10⁻³; main refuses `UnclearedSharedSide`) | the census splits nothing |
| The split is a function of body and δ | `split_rulings::the_join_splits_the_same_at_every_eps`: one digest at three ε | — |
| The split point (chart midpoint; apex end; a cone's development) | `join::tests::a_side_splits_at_its_charts_midpoint_and_a_cones_in_its_development` | the development is off; the apex rule is off |
| A boundary keeps its chord | `join::tests::the_census_splits_inside_and_a_boundary_keeps_its_chord` | the holders are not read |
| The pieces are held as the lane holds a triangle | `join::tests::a_split_piece_refuses_the_face_as_its_lane_would` (within δ held; past δ `CertificateExceeded`; against the face `FoldedTriangle` where the lane reads folds) | the verdict is skipped |
| A split patch names no memo entry | `memo::tests::a_patch_split_where_the_patches_join_names_no_entry`; `editor_core`'s `resolve::pick::tests::a_patch_split_where_the_patches_join_is_built_every_picture` (three pictures: split tables built each picture, the rest served, the index the fresh one) | `unstore` is not called |
| The fold cure | `reach_probes::ringed_curved_faces_that_still_refuse_answer_by_cause`, `reach_probes::a_needle_funnel_cut_at_its_apex_refuses_typed` | any fold refuses at once |

## The ε inventory

`the_eps_inventory_is_pinned` moves by carriers only: `join.rs` 0 → 2 and `tessellate.rs` 2 → 3. No read moves: a piece's verdict is the certificate's own (`cert::FaceVerdict::read`), as the lanes' triangles are.

## Measured, and what the measurements did not cover

**Stage 1.** The split was measured with the rules off, every call diffed against main. Over the `mesh` suite at ε 10⁻⁹, 10⁻⁶ and 10⁻¹² there were 1,846, 1,768 and 1,850 splits over 73, 57 and 73 meshes. Over `sweep`'s 131 meshing modules there were 276. No piece refused on certificate, fold or winding; the worst piece was 0.905 δ. No body that meshes on main was lost. The unit file has the table by pair of face kinds.

**Sides that could be split and never were.** On a side a lane could be asked to split, some pieces would fold (cones, spheres, tori) or fall under winding resolution (9,937 plane sides). No such side is made by a second patch in any body measured, and the face refuses typed if one ever is. Filed: `work/tesslane/a-side-the-join-could-split-has-a-piece-that-would-fold-or-fall-under-winding-resolution.md`.

**The lens** (`faces_that_bound_no_region::a_lens_of_two_short_arcs_meshes_at_a_coarse_delta`). Its two spline faces are one sheet, so each face's own-surface point is one position. Joined by position, main's mesh of it is already a sheet counted twice (6 coincident pairs of triangles, 7 edges of more than two); the split's has 4 and 4. Recorded on `work/ciw/the-admesh-gate-cannot-see-an-edge-of-four-triangles.md`.

**The pick memo.** No body was found whose face is a memo hit split in one picture and not the next; three families were searched: the rod sliver with its slab moved, an extruded circular segment, a spherical cap. The stored id is withheld from every split patch, so the pick memo never meets the case. The lost table hit is filed: `work/tesslane/a-patch-the-join-splits-is-never-a-memo-hit-for-the-pick-index.md`.

**At δ itself** (the sizing unit's target; `UNPROVED_HEADROOM` 1.0): the fold cure meshes the tilted pair and the near-pole tube. With the insert retired, `reach_probes::crossing_rods_keep_no_side_a_neighbour_can_make` refuses `CertificateExceeded` at that target; main meshes it there because the insert happens to shorten a triangle.

**Cost.** The split runs only where the census asks; a patch only notes chart corners. On `d9_mesh_goldens`' corpus (40 passes, process CPU time, three runs each, quiet machine) main takes 8.60 to 9.17 s and this head 7.91 to 8.17 s: fewer rebuilds without the shared-side insert. The notes grow that corpus's memo footprint from 1.24 MB to 1.44 MB (+16 %), on a coarse corpus where boundary-band triangles are a large share.

## §5 sweep

**Shape 1, a lane reading a face across its edges.** Grepped `crates/mesh/src` for `face_of_half_edge`, `twin`, `holders`, `across`, `sides_of`, `EdgeSides`, `he_plus`/`he_minus` in code lines.
- `chords.rs` reads an edge's two faces to size its shared chord (the one chord rule: a neighbour enters only through the shared chord points) and records the holders for the census. Kept, by design.
- `trimmed.rs` and `walk.rs` read their own half-edge's direction. Not this class.
- `curved.rs`' two hits are test code.
- Blind spot: a lane could reach a neighbour through the `Body` by a query none of these names. No other `Body` face query appears in the lanes' code.

**Shape 2, a cache keyed on a patch-memo identity.** `StoredPatchId`/`PatchKeys::stored` consumers: `editor_core`'s `PickMemo::table` and `keep_tables`, both covered by the withheld id. No other crate reads it.

## TESSLIE's rows

- `trimmed::tests::a_shared_side_the_budget_leaves_refuses_typed` → `a_side_two_faces_make_is_split_with_no_retry`. What it held, a refusal past the budget, has no rule left. The rod sliver now meshes on one attempt.
- `patch_memo::the_trimmed_lane_misses_when_only_the_face_across_an_edge_changes` → `the_trimmed_lane_hits_when_only_the_face_across_an_edge_changes`.
- `memo::tests`' rows for the faces across' identity structure and for an edge that becomes a seam are deleted with the field.
- `errors.rs`' message rows for the two retired variants are deleted.
- Evidence added (not closed, theirs): `work/tesslie/the-shared-side-rule-on-the-spline-arm-has-no-witness.md` (the rule is retired; the lens and main's spline walls are its witnesses), and `work/tesslie/memo-per-lane-input-lists-are-held-by-prose.md`.

## Filed and closed

- **Filed:** the two items above.
- **Closed:** `a-strip-against-a-neighbour-holding-both-rims-makes-an-edge-of-four-triangles` (held by the shell row) and `a-walked-sides-reach-exclusion-is-read-only-by-the-walk` (moot).
- **Evidence added:** the near-pole circle and needle-funnel items.

## What still refuses

- The needle funnel's upper half at δ 10⁻³ and 2·10⁻⁴: `FoldedTriangle`.
- `spheres_crossing_off_every_edge` beside its pole: `FoldedTriangle`, as on main.
- The near-pole circle's `ChordBudget` cells.
=====
