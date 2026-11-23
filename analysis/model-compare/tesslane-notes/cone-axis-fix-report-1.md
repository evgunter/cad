cone-axis-winding — FIX PASS REPORT, part 1 of 3. Pushed head: 58565e3f0d4d1dc2fb7df4dfd5204038e28b71c3 (confirmed with ls-remote). It merges PR 7043's closing head 0309707d31, which includes the re-pinned viewer rows 05a7f8dbfd. Main does not have 7043 yet, so I haven't merged main. route-by-domain is not merged. Fix commits: 9e9a2b151c and 3700a156cc.

Per item (citations are to the head):
A. SeamTwice. `trimmed.rs` `singular_row_shape` (~2203) takes `developed` and returns before the SeamTwice check when the CDT runs in the development (`a.development().is_some()` at the caller, ~550). The doc says why: the seam's two copies are the sector's two rays; a triangle with a corner at each copy of one point is outside the face above a half turn and dropped inside a narrower one, and the census re-derives the fan. The `faces_across` doc now names the case live. Rows are in the new `crates/mesh/tests/split_rulings.rs`:
- `a_ruled_face_with_a_split_ruling_meshes`, 243 cells, 4.6 s, slow set. One-face full turns (cylinder wall, frustum wall, slender wall, needle wall; outer and reversed inner faces) and through-axis halves (cones at 45°, 80° and 5°, frustum; half cylinder wall). Each line edge split at 1, 2 or 5 points, and every line edge at once; δ 0.1, 0.01, 0.001.
- It checks `got_enclosing`, which is `reach_probes::geometry` with the volume read off the unsplit solid, because a `split_edge` body is not finished (JoinableVertexAtRest). That covers resolved and outward winding (S6), triangles sampled within δ, and volume.
- Green at the default ε, 1e-6 and 1e-12. Mutant "SeamTwice restored" reds it.
- The old lib row is retired, and its "full bodies" claim goes with it (E).
- The residual (ruled (a)): the slender wall's outer seam split, δ 0.1, refuses Triangulation(Plane) on its 0.05-wide annulus. It is pinned for the three split patterns, and evidence is added to the thin-arc row.
B. The three surviving mutants each now red a row:
- (a) door confirmation dropped → `split_rulings::a_notched_wall_with_a_split_ruling_refuses_at_the_shape_door`: reach_probes' notched pipe wall, each line edge split; it must stay NotIsoRectangle.
- (b) guard deleted → `curved::tests::a_ruled_face_with_a_point_inside_a_side_refuses_here`: a hand-built call into the curved lane on a split-seam cylinder wall.
- (c) chord pass blind to the route (both its lane reads and its target) → `split_rulings::a_routed_face_is_chorded_for_the_chart_lane` pins per-patch counts. The cylinder wall at δ 0.01 is 314/64/64/64, and blind it gives 296/55/64/55. The thin-annulus cells of the class row also mesh under it, so that row reds too. So what the route's reading in the chord pass decides is density, and through density where the thin-arc class lands; the PR body says so.
C. Ruled (4). Filed `a-ruling-split-within-the-developments-resolution-merges-two-points` (P3) with the mechanism, the 2⁻²⁷ bound, both bodies, option (1) and unit 4c's removal of the limit. Pinned: `split_rulings::a_ruling_split_within_the_developments_resolution_refuses` (DevelopedMerge at 1e-12 at all three δ; at the default ε and 1e-6 it asserts split_edge refuses the split).
D. Pinned `split_rulings::a_ruling_split_a_hair_from_its_end_folds_a_triangle`: the 20° cone over 2π−0.3, line edge 1 split 3e-7, δ 0.01 → Folded(Cone). Same at 1e-12; at 1e-6 the split is refused and the row says so. Filed `a-ruling-split-a-hair-from-its-end-folds-a-triangle` (P3). The "every class face meshes" sentence is now scoped in the PR body.
E. Corrections are in the PR body (part 2): the frustum line, the T1 density row (81 and 340 only; 1581 removed), worst ratios 14.6× and 36×, and the row's full-turn claim.
F. One statement:
- The route reads loop vertices.
- The lane's guard is written once (`curved.rs` ~299 `inside_a_side`, also read by `meridian_interior`). It refuses `UnsupportedCurve { edge: the side's line edge, note: SIDE_CHORD_POINT }`, and its text names what can raise it: a spline neighbour's lattice raising the line's chord count.
- The const's doc points to the filed row `a-spline-neighbour-can-put-a-chord-point-on-a-ruled-faces-straight-side`, which has the mechanism by reading per arm, the three survey counts (236 / 80 / 3), the tour unsurveyed, and both cures.
- The PR body says the two predicates differ exactly on a non-vertex chord point.
G. `walk.rs` ~736: the order is a comparison, `(a < v && v < b) || (b < v && v < a)`. A far end at the junction's own v (a zero-length line along the ruling) continues no side through it.
H.
- S2: `chart_table::ruled(surface)` is the one home, read by the route and the lane.
- S3: the `grid_counts` doc (the ruled arms read neither `has_pole` nor `meridian_interior`) and the `pole_columns` doc ("Moot on the ruled charts") are fixed.
- S4: the band's consumers are named at `sizing.rs` `SizingTols::band`, `tessellate.rs` (~350) and `memo.rs` (~31).
- S6: see A.
- S7: it is UnsupportedCurve with true text, not MissingEntity/"router defect".
- S10: the re-pointed row is fixed. The unit file's own sentence (`a-cone-face-met-on-its-axis-…md:90`, "the chart lane's, which meshes every one of them today") is yours: my brief bars me from editing the unit's item file. Suggested text: "the chart lane's, which meshes them but for three residuals filed beside this row (a thin planar neighbour at δ 0.1, a split within the development's resolution at ε 1e-12, a fold beside a hair-split) …".
- S11: filed `a-spline-face-whose-boundary-row-collapses-to-a-point-has-no-singular-row` (P3), which says the grep was blind to the import and loft doors.
I. S8 (PR 7041's `Routes::of`) is said in the PR body.
J. The worked example is in the PR body; the viewer rows are named as 7043's.

The PR body follows in parts 2 and 3.
