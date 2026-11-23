DELTA RE-REVIEW — PR 6950 @ frozen 72af97e31f (fix commits b69a146d45, cd2d65a192, 5565b8f044, d422985b60; the merges of main are not reviewed)
    
    **Verdict: APPROVE-WITH-FIXES.**
    - My first-round MAJOR (the 64ε apex threshold) is closed.
    - The fix pass brings in a new MAJOR, DEMONSTRATED. A funnel cut close above its apex now refuses at fine δ where the previous head meshed it. The development causes it (bisected by mutant).
    - It fails loud: every mesh I accepted is right.
    - I took the CI state from your brief and did not re-check it.
    
    ## Claims
    1. **Held.** My probe at this head: the funnel split below and above its apex at scales 1e-2, 1e-1, 3, 7, 1e2, 3e2, 1e3, plus a translated and turned pose at ×1 and ×0.01, at relative δ 2e-2, 5e-3 and 1e-3. Every split meshes. Results:
       - check_mesh ok;
       - apex fans 8/16/36 per face, the same at every scale;
       - true deviation at most 0.47δ;
       - no folds.
    
       Mutant "is_singular forced false" turns 4 rows red: the band row, tilted_pairs, and both apex reach rows.
    
       Undecided zone (by inspection): `is_singular` reads `Err` as not singular. The doc says the lift fails first, and the face then refuses in `trim_polygon` with `UnsupportedCurve` "carries no stored pcurve cache — caches mint in the split/boolean pipelines". That is a wrong cause to show a user (NOTE).
    
       A non-vertex point on the apex row: a cone edge through the apex mid-edge runs onto both nappes, so it refuses BothNappes. A chord point at v = 0 that is not a vertex develops to the origin. If the apex vertex is also in the polygon, it refuses `Triangulation`; otherwise its cert keeps its u. Both are by inspection, and both refuse typed.
    2. **Corrected.**
       - Degenerate triangles: none in any accepted mesh. Folds: 0 over 122 probe meshes. The three needle bodies that refused FoldedTriangle with the development off now mesh.
       - Minimum 2·area/L²: needle below 5.98e-5, funnel cut 1e-5 above the apex at δ 2e-2 8.9e-6, the rest at least 6e-4.
       - Exact collinearity along one column holds by construction: 27+26 bits make an exact product, the apex sits at the origin, and every point of a column has a bit-identical u. Underflow is unreachable.
       - The 1e-6 floor is the test's observation, not a guarantee. A near-ruling triangle that is not on a ruling, or a 3-D sliver, is bounded by nothing.
       - With the development off, the near-apex face's floor falls to 1.6e-7, so the floor is close to live.
    3. **Held** on what I could build: one nappe, apex to apex (your row), whole-turn halves.
       - Sampled true deviation on every accepted cone mesh, coarse δ included, is at most 0.65δ.
       - The `inside_point` read-back is right: `Azimuth::of(y = g.x, x = g.y)` matches `developed`'s (sin θ, cos θ).
       - I did not build a loop reaching the apex twice.
    4. **Corrected.**
       - I reached no `Triangulation` collision: merging needs two polygon points within 2⁻²⁵·r of each other, and chord steps are far coarser.
       - It reads badly to a user, though: `Triangulation`'s Display says "a non-finite or out-of-range chart coordinate, i.e. corrupt geometry" (types.rs:741), which is false for a valid face (MINOR).
       - The rounding is not what costs; the development's triangulation is (finding M1).
    5. **Held.**
       - `developed` uses `geom_core::Real::sin_cos`.
       - The read-back uses `Azimuth::of`/`near`, the kernel's angle door. There is no new `atan2`; `f64::sin/cos` appears only in the sphere metric, which moved from trimmed unchanged.
       - `significant` is a bit mask.
       - bare-rounding: chart_table prod 1 pose (`passed_by`); the ONEROUND line names it.
    6. **Held** (by inspection).
       - `face_target`: `spline_chart().is_none() &amp;&amp; lane == Trimmed` equals the old roster, since planes never route Trimmed.
       - `lattice_steps` and `metric` are the old arithmetic verbatim; curved `grid_counts` is equivalent.
       - The chord budget is sphere-only, as before.
       - The per-face `FaceLattice` cache is the same value per face.
       - No fix commit touches the D9 goldens.
    7. **Held, with residue.** Mutants, each running the 49 reach_probes / trimmed:: / chart_table rows:
    
       | Mutant | Result |
       |---|---|
       | (a) cone and torus cert ×0.01 | **all green** |
       | (b) cone and torus cert ×0 | **all green** |
       | (c) window's non-circle arm skipped | 4 red, including `a_cone_faces_window…` |
       | (d) headroom off | 6 red |
       | (e) development off | 3 red |
       | (f) singular false | 4 red |
       | (g) refinement on every arm | 1 red (`an_over_delta_sphere_arm…`); the probe also shows FoldedTriangle on the near-apex funnel |
    
       On (a) and (b): output deviation is now pinned by `geometry()`, but nothing pins the soundness of the certificates themselves on these arms.
    8. **Partly falsified** (BY-INSPECTION).
       - At the default ε, every body must build: held.
       - Below it, the skip accepts any revolve `Err` (stringified) and asserts only `eps &lt; DEFAULT_EPS`. It does not check the refusal is the typed `KernelLimit` build limit.
       - Skipping is the right reading only for that refusal.
    9. **Held.**
       - Singular points read `Chart::poles`.
       - Budget, lattice, metric and development are row answers.
       - No `Spline` arm is unreachable.
       - Residue (style below): `AnalyticRow` carries both `kind` and `surface`, and `cert` has an `unreachable!` for a mismatch between them. The test helper `row()` re-spells `of`'s sign→`RowEnd` mapping.
    10. **Held.**
        - The stale torusvessel doc is fixed.
        - The 64ε paragraph is replaced by "decided at the band".
        - The spindle row `work/tesslane/no-door-builds-a-trimmed-spindle-face.md` is filed.
        - The ONEROUND line names `chart_table::SingularPoint::passed_by`.
    11. **My first findings:**
        - MAJOR (64ε): closed.
        - Uncertified-arms MINOR: closed for output (`geometry()` samples true deviation); open for certificate soundness (mutants (a), (b)).
        - δ-set MINOR: closed (2e-2 and 1.5e-2 pinned).
        - ONEROUND, spindle and torusvessel-doc items: closed.
        - Style S1, S3, S4, S6: closed. S2 is closed by `chord_budget`.
    
    ## Findings
    - **M1, MAJOR, DEMONSTRATED. The development refuses a cone face whose section passes near the apex.** Funnel `revolved[(0,0),(1,1),(0,1)]` split by the tilt-0.3 plane through (0, h, 0), upper half:
    
      | h | refuses CertificateExceeded at δ |
      |---|---|
      | 1e-2 | 1e-3, 2e-4 |
      | 3e-3 | 1e-3, 2e-4 |
      | 1e-3 | 5e-3, 1e-3, 2e-4 |
      | 3e-4 | 5e-3, 1e-3, 2e-4 |
      | 1e-4 | 5e-3, 1e-3, 2e-4 |
      | 1e-5 | 5e-3, 1e-3, 2e-4 |
    
      - The bound runs from 1.03 to 4δ (e.g. h 1e-4: 0.00515/0.005, 0.00204/0.001, 0.00081/0.0002). Every h meshes at 2e-2.
      - Here a finer δ refuses where a coarser one meshes.
      - At 3a44a00f24 the h = 1e-4 face meshed at all four δ (true deviation 0.39–0.49δ).
      - With the development mutated off at this head, it meshes at all four, but with slivers down to 2A/L² 1.6e-7. So neither plane serves this face.
      - Likely cause: the development's Delaunay joins far-apart columns around the narrow tip, and the UV-box cone cert (v_max·Δu) exceeds δ. The cause is not verified.
      - No row covers a section passing near the apex; h = 0.02 meshes.
    - **MINOR, BY-INSPECTION.** `Triangulation`'s Display blames corrupt geometry for the development's injectivity refusal (types.rs:741-745).
    - **MINOR, DEMONSTRATED.** Cone and torus certificates ×0 survive every row: certificate soundness is unpinned on these arms.
    - **MINOR, BY-INSPECTION.** The scale-and-pose row skips any revolve `Err` below the default ε, not specifically `KernelLimit` (reach_probes, `a_cone_face_through_its_apex_meshes_at_every_scale_and_pose`, the `build`/`Err(e)` arm).
    - **NOTE, BY-INSPECTION.** An undecided singular vertex refuses with the "no stored pcurve cache" text (trimmed.rs `trim_polygon`, the lift's `unwrap_or_default`).
    
    ## Style (delta)
    - `sure`, Q1: the `chart_table::tests::row` helper re-derives `AnalyticRow::of`'s pole→`RowEnd` mapping verbatim. It is a second copy that a test exercises instead of the real constructor.
    - `likely`, Q7: `AnalyticRow { kind, surface }` holds one fact twice. `cert` matches `(kind, surface)` and needs an `unreachable!` for the impossible pairings: the "can't happen" arm in a new form.
    - `likely`, Q7: `geom::Azimuth::of(g.x, g.y)` passes x as y on purpose because `developed` puts sin in x. It is correct, but it reads as a swapped-argument bug, and no comment says so.
    - `sure`, Q5: the `Triangulation` Display text, as in the MINOR.
    - `unsure`, Q2: the `developed` doc says "no other triangle of zero area exists to make" (three collinear points of a cone lie on a ruling). That is true of exact zeros, but the comment reads as a sliver guarantee, which only the 1e-6 test floor observes.
    - `unsure`, Q3: the 2A/L² ≥ 1e-6 floor sits 60× under the needle's measured 6e-5, so it is near-live. It would red on the near-apex face with the development off (1.6e-7), but no row builds that face.
    
    Questions exercised: Q1, Q2, Q3, Q5, Q7. I read chart_table.rs whole, plus trimmed.rs's development, insert_points, singular_vertices and singular_row_shape.
    
    ## What I did
    - **Executed:**
      - a mesh test build;
      - a scratch probe (not committed; copy in my scratchpad as zz_review_probe_v2.rs) over 122 mesh calls, measuring sampled true deviation, apex fan size, 2A/L², fold count and check_mesh;
      - a near-apex h sweep;
      - seven mutants (a–g), each followed by the 49 reach/trimmed/chart_table rows, all restored.
    - **Read only:** the diffs of chords, curved, sizing, tessellate and types; the reach_probes delta; the work files; the torusvessel test.
    - **Not run:** the tour, sweep, and the full mesh suite. I relied on the CI state you reported for those.
    - **Other reviews:** I saw no other review's material and did not read any PR comments or `analysis/` branches.
    - **Tree state:** the working tree is clean; nothing committed or pushed.
</cross-session-message>

0 notifications remain queued.
