(4c implementer) Your notes queued behind my turn, and I only read them at 07:44Z. That includes the 01:32Z reorder. Nothing has been pushed since 7a6f5bfe6e. The Python suite had already run: 960 OK. The torus seed is lost: that log was overwritten by the rerun, and in the rerun the torus row passed while the window-oracle fuzz row failed (CAD_FUZZ_SEED=0x2809be1edc2b5ff5, ε 1e-12).

**One doc commit to propose before pushing.** `split_rulings::Walked::SharedSidesCut`'s doc (crates/mesh/tests/split_rulings.rs ~l.158) still says "whose midpoint lies within δ·(1 + ALLOWANCE_SHARE)". With the decision, it should read "whose midpoint is not decided, on the run's band, to lie farther than δ·(1 + ALLOWANCE_SHARE)". The bored-ball row's doc says "lies past" at δ 0.035, which still holds (decided). It is doc only, and I'll push it on your word.

**The mutant you asked about, "undecided read as greater":**
- It needs a cell whose margin |m − foot| − δ(1+share) falls inside the band (about 1e-6 to 1e-5 at the default ε).
- Buildable in principle. On the bored ball, the cut side's midpoint distance d is constant over a δ interval where the rim chord count does not change, so δ = d/(1+share) ± 5e-6 lands inside the band, provided that δ is not at a chord-count step.
- Not built yet. It would be a new row and so a push. Say if you want it.

The PR-body paragraphs follow, the full set. Part 1's needle bound loses its "maximised by" clause (unmeasured), and here it is stated as the measured cells.

=====
**What 4c changes.** A ruled face (cylinder or cone) whose straight side holds a split point no longer goes to the chart lane's lattice and CDT. The trimmed lane walks it: `ruled_walk::walk` merges the face's two rows of boundary points by their position along the rulings, so every triangle spans at most one chord of each row. That puts it inside one column of the rectangle lane's grid, and the rectangle lane's theorem (`sizing::sagitta_step`, `sizing::cone_grid_step`) bounds it. The face is sized at δ, not at UNPROVED_HEADROOM. A split face now meshes with the unsplit face's triangles plus one per point its side gained. Its plane neighbours do the same.
=====
**MAJOR-1 (one-chord apex face with both sides split), cured.** A cone face that runs from one section chord to its apex is a row of two points against a row of one (the apex). With both straight sides split, the fan from the opposite corner put the two fans on one triangle. `Rows::zip_sides` now walks the two sides against each other: k + m + 1 triangles for k and m side points. Pinned exactly by `ruled_walk::tests::a_one_chord_face_to_the_apex_zips_its_sides` (the triangle list) and on bodies by `split_rulings::a_one_chord_face_through_the_apex_with_both_sides_split_meshes` and `a_one_chord_face_to_the_apex_split_beside_both_rim_ends_meshes`.
=====
**MAJOR-2 (a bored ball's edge of four triangles), cured.** On a ball bored through, the sphere or torus face round the bore holds both of the bore's rims. Where it lays no point between them, its own triangles join the rims as the walk's do, so a walked side between two rim points can be an edge of four triangles. The walk now cuts such a side at the surface point at its chart midpoint, so each triangle on it becomes two (see the shared-side rule). Pinned by `split_rulings::a_walked_face_whose_neighbour_holds_both_rims_shares_no_edge`. The unsplit bored ball is not manifold on the rectangle lane at δ 3 to 0.1. That is main's defect, filed as `work/tesslane/a-strip-against-a-neighbour-holding-both-rims-makes-an-edge-of-four-triangles.md`, with this rule as its candidate cure.
=====
**The route carries the walk.** `tessellate::route_of` decides each face's lane once, and the lane does not re-derive it.
- A ruled face whose outer loop is an iso rectangle with a split straight side (fact 4, `walk::splits_a_straight_side`) is `Trim::Walked` (`Lane::Walked`) when `trimmed::walkable` answers yes. That needs one of two shapes: two runs of rims with no vertex at the apex, or one run whose sides meet at the apex.
- Split-side faces that `walkable` answers no stay `Trim::Structural` and keep the lattice and the CDT:
  - a loop through the apex onto the other nappe (no body built here makes one);
  - an apex the band cannot decide (refuses `SingularUndecided`, below).
- A face with a ring, a non-iso stored image or a spline chart is structural before fact 4 is read, as on main.
- A face the route marks walked but which the lane cannot walk is `TessellateError::RouterDefect { face, what }` ("the route misread a face (a kernel defect)"): a spline chart, or a boundary that does not read as the walk's two rows. No body reaches it. Mutants show the route's decision is load-bearing:
  - with the walked fact off, 8 split_rulings rows go red;
  - with the walk reached by a face the route did not mark, 19 rows go red across reach_probes, route_rows and split_rulings.
=====
**The shared-side rule and its bound.**
- The walk cuts a side whose two ends one neighbouring face holds (`trimmed::faces_across`, adjacency by mesh id) only where that face may make it (`trimmed::may_make`).
- A triangle the neighbour emits passes its certificate, which is at most δ·(1 + `cert::ALLOWANCE_SHARE`) from its surface. So where the side's midpoint is farther than that from the neighbour's surface (`geom::Surface::foot`), the neighbour cannot hold the side.
- "Farther" is decided on the run's band through `geom_core::k_stats::decide`. Decided greater means no cut; decided not greater, or undecided, means cut. A surface with no foot means cut. There is no margin constant in `mesh`.
- On the bored ball (radius 1.05, bore 1, shell 0.05; sphere and torus neighbours; seam split at its middle and at five points):
  - at δ 3, 1, 0.3, 0.1 and 0.04 the side is cut, and the bore's strip of 2n triangles becomes 6n;
  - at δ 0.035 and 0.03 the bore keeps the count relation exactly.
  - 0.04 and 0.035 are the cells nearest the threshold, and the same holds at the default ε, 1e-6 and 1e-12.
- The rule reads where the neighbour lies, not how it is meshed, so it also cuts where a same-surface neighbour would not make the side. In `no_walked_triangle_has_an_edge_a_neighbour_makes`, the corners of a one-chord band face beside a wider face of its cylinder are cut: +4 triangles, two cuts, on 24 band cells.
- Mutants: never-cut and always-cut each turn the bored-ball row red.
=====
**The count relation, and its stated exception.** A face that holds no interior point has n − 2 triangles over its n boundary points, and a split adds a point to each traversal of its edge. So a walked face and its planar neighbours have the unsplit face's triangles plus the traversals their loops gained. `split_rulings::holds_the_relation` checks this on every cell. The one stated exception is the shared-side cut, which adds two triangles per side cut, on the walked face alone:
- `Walked::SharedSidesCut` allows an even surplus;
- `Walked::Cut` requires a positive even surplus.
=====
[stage 1, `[mesh.sizing]` ×3, deleted row, "a split never makes a face refuse", needle pin moved: as sent in part 1, unchanged.]
=====
**Residuals, as bounds.** No face of the walked class refuses on any cell measured, at three ε. What still refuses after a split is outside the class:
- **A needle triangle in the development.** A full-turn 2° cone, cut near its rim with its apex kept, with its first ruling split beside the section end. It is not walked and refuses `UnresolvedWinding`. Measured cells:
  - default ε: a 1e-8 gap refuses at δ 0.01, and δ 10, 1 and 0.1 mesh;
  - 1e-12: a 1e-10 gap refuses at δ 10 to 0.01;
  - 1e-6: a 1e-5 gap meshes at every δ.
  No wider family was measured.
- **An apex the band cannot place.** A 0.001° cone, split 1e-8 from its apex. It refuses `SingularUndecided` at δ 0.1 and 0.01 at the default ε, and meshes at 1e-12.
- **Density, not refusal:** the band's +4 above. Also, on the curved lane, neighbours of a walked face get coarser (the digests: 24 curved patches, all in one row). That is the open STOP for your ruling.
=====
**Filed.**
- `work/tesslane/a-strip-against-a-neighbour-holding-both-rims-makes-an-edge-of-four-triangles.md` (P1, main's unsplit bore; candidate cure `may_make`).
- Closed by this unit: `a-cone-face-with-a-split-side-has-no-worst-triangle-theorem`, `a-ruling-split-a-hair-from-its-end-folds-a-triangle`.
- Dated notes on `a-run-anchored-on-a-needle-triangle-makes-triangles-f64-cannot-wind` (its row on the cut family) and on `a-needle-cone-through-its-apex-takes-most-of-an-hour-to-mesh-one-cell` (the head answers that cell in ~0.1 s; the item stays open).
=====
**Mutants** (each red under its own rows; filter split_rulings, route_rows, errors, ruled_walk, sizing, reach_probes):
| mutant | red |
|---|---|
| zip off | 4: `a_one_chord_face_to_the_apex_zips_its_sides`, `a_one_chord_face_to_the_apex_split_beside_both_rim_ends_meshes`, `a_one_chord_face_through_the_apex_with_both_sides_split_meshes`, `no_walked_triangle_has_an_edge_a_neighbour_makes` |
| shared side never cut | 1: `a_walked_face_whose_neighbour_holds_both_rims_shares_no_edge` |
| shared side always cut | 1: the same row (its exact cells at δ ≤ 0.035) |
| route's walked fact off | 8 split_rulings rows |
| walk reached by an unmarked face | 19 across reach_probes, route_rows (k-stream included), split_rulings |
| 7054's anchor / three / two / resplit (earlier pass) | the two-runs row / run-of-three / two-runs / five rows |
| undecided read as greater | not built (above) |
=====

Next: `mesh` whole at 1e-6, then at 1e-12, one at a time, each sent as it finishes.
