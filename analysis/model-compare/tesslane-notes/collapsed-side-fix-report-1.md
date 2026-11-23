tesslane/collapsed-side — FIX PASS REPORT (part 1 of 2). HEAD 390036911f4ea3777b98434e4e7b3c1ef099bc06 (confirmed with ls-remote). No trailers.

COMMITS SINCE THE REVIEWED a87d047b5a:
- 52b6423b18: rows only (the v-degree-2 row and the lune row; the in-band row, later re-meant).
- 675f00031c: merge of main.
- 3d33c0b7ff: the banded read (the MAJOR).
- d6274e05aa: work-file wording.
- 85d314a002: merge of main (`cargo check -p mesh --tests` green after it).
- 390036911f: this fix pass.

EACH ITEM'S FATE
1. MAJOR: decision read once, name changed.
   - The lift does NOT store the decision. `Lift::read` reads stored rows. A joint at a singular point stores an ordinary deck element, like every joint (CHARTTAIL's ruling on PR 7287: "There is no JointElement::Reset"). So nothing on the lift says "turned here". Storing it would change topo's at-rest record, which that ruling settled; out of reach in this unit.
   - So the mesh calls the SAME function at the same band: `chart_table::at_a_singular_point` → `geom_brep::chart_singularity::vertex_at_singular_point` over the home's `singular_points`. That is what topo's `singular_at` calls, at the run's band. `face_polygon`'s doc says why.
   - mesh's type is now `SidePoints`; topo's `SideWalk` keeps its name.
   - Gates: `run-gates.sh` rc 0, the bit-identity gate list unchanged. This pass first tripped `knots-f64-interim`: `CollapsedSides::of` read `knots_u_f64_interim`. It now reads the held knots (`knot_values_u().domain()`), and the gate is green.
2. Bytes across ε, done. The pinned wedge and the corner pinned through three at v-degree 2 join `d9_mesh_goldens`, at δ [1e-2, 3e-2]. Both refused at 1e-2 on base. Their digests are identical at ε 1e-9, 1e-6 and 1e-12, and the corpus doc lists them.
   - Editing those two rows made them slow-set rows by the 0.25 s rule: 1.48 s and 1.58 s before, 1.91 s and 2.12 s after, one local run. Both are now in the ci filter, so per PR they run only when a diff touches them; the nightly runs them at every ε. Say if you'd rather keep them fast and drop the bodies to a coarser δ.
   - The body says that no committed row shows the faces WITHOUT a collapsed side unmoved, beyond the suite's goldens staying green.
3. Dropped two-copy triangles: stated once, in `SidePoints`' doc, for the spline walk and the sphere's and spindle's pole sides alike.
   - Covered by the promise: each dropped triangle's 3-D image is the segment from the vertex to its third corner, an edge of kept, certified triangles, so it adds no mesh point.
   - Measured, not proved: the surface over the dropped chart region against that segment is within 0.061 δ over 2,184 drops (the review of PR 7517).
4. On the side by construction. `CollapsedSide` carries `held`, the knot rectangle's end on the fixed axis. Every inserted point sets that coordinate to `held` exactly and interpolates only the free one. The copies themselves stay pcurve-evaluated. The pieces row now starts the walk from a copy an ulp off the side; the new mutant 8 (fixed coordinate taken from the arriving copy) reds it.
5. Loud.
   - A missing `shared[id]` is `unreachable!` with its reason.
   - A vertex on two sides, or undecided against one, keeps its copies. The site says so and cites the filed row, and the row now names the undecided case too.
6. Types.
   - The walk's entries are `Along { fixed: ChartAxis, held, step }`.
   - `face_polygon(&PolygonOf { .. })` replaces nine parameters and the `allow`.
   - `trim_polygon` keeps its `(singular, walk)` pair.
7. Docs.
   - `trim_polygon` and `flatten_traversals` say "leaves from another chart location", naming a pole's latitude and a collapsed side.
   - The emit comment says no shape check runs on the spline arm and that the census after the loop re-derives the fan.
8. Body text only (part 2): the count is the rim-chord rule's, finer than the certificate needs, and the class row does not pin it.
9. Coverage.
   - The loft builds no `v` side, and no import fixture holds a collapsed side, so a `v` side is covered by unit rows only.
   - The hinges in the first and last slab were already in the class (their hinge rim is a cap's).
   - Added corners pinned in the first and in the last slab of two; the loft builds both.
   - The class is now 11 bodies, and the digest row pins 11 values, the same at all three ε.
10. Tracker.
   - The corner row now has priority P3, cost M.
   - `chords::tests::witness_pass` is now the one chord-pass setup (`chords_of` reads it, and so do the rows); my copy is gone.
   - Filed `work/tesslane/the-mesh-crates-unit-rows-paste-the-chord-pass-setup-twelve-times.md` (P4, C) for the other twelve copies (four in `curved.rs`, six in `trimmed.rs`, one each in `tessellate.rs` and `walk.rs`).
   - `work.py lint` passes.

LOCAL RUNS:
- whole `mesh`, slow set included: 556/556 at ε 1e-9, 1e-6 and 1e-12;
- clippy -D warnings with and without `budget`, and fmt;
- `run-gates.sh` rc 0;
- check-slow-set-names: no dead entry;
- the mutants on the final code, each caught:
  1. sides never read: 6 rows red;
  2. copy kept, no points: 6 rows red, the v-degree-2 row among them;
  3. free axis swapped: 7 rows red;
  4. corner walked: the axis row;
  5. exact read in place of the band: the band row and the in-band row;
  6. one piece more: 4 rows red, the twin relation row and the digest among them;
  7. twice the step: 4 rows red, as for 6;
  8. fixed coordinate taken from a copy: the pieces row.

LEFT TO CI: every other crate.

SLOWED ME: a nextest listing I ran while a test run held the build left a mesh test binary that would not relink (`undefined hidden symbol`). `cargo clean -p mesh` cured it.

PR body in part 2.
