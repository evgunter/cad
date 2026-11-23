REVIEW — PR 6950 `tesslane/chart-table` @ frozen 3a44a00f24 (full review + style lane)
    
    **Verdict: APPROVE-WITH-FIXES** (one MAJOR, demonstrated. It fails loud, so the output is never wrong, but it breaks the unit's headline claim. The fix is small.) Gate: the frozen head has **zero** CI runs. The PR head is now 9313cff34d, which is 3a44a00f24 plus a merge of main that touches only viewer, editor-core and work, with no mesh/sweep/demos change. Its run 43582116731 was still in progress when I looked (test kernel, demos, lint). The verdict is conditional on that run going green.
    
    ## Claims
    1. **Held.** D9 goldens, docs/tess-budget-data and the tour pins are untouched by the diff. The full mesh suite (455 tests) is green at head; I ran it with a certificate-only mutant, which moves no bytes. Read before and after: the curved lane takes steps, window and cert from the same sizing/cert fns via ChartRow, and the cone/sphere windows are identical on circle/line loops. `lane_of`'s `takes_sphere_arm` → `has_tilted_circle` is equivalent, because `has_trim_carrier` is tested first. The one reader not on the table is `face_target` (Style S3).
    2. **Held.** There is no `_ =&gt;` over ChartRow anywhere, and `ChartRow::of` matches Surface exhaustively. `lane_of`'s `_ =&gt; Curved` reaches analytic faces with no ellipse/nurbs/spiric carrier, and spheres without a tilted circle. That set is unchanged by this unit, and nothing in it reaches the new arms. Some inner matches list unreachable variants instead (Style S6).
    3. **FALSIFIED on the cone.** `ON_ROW = 64ε` is absolute in the chart's v unit, which on a cone is slant metres. I scaled the funnel split (reach_probes' a_cone_face_through_its_apex):
       - meshes at ×0.01, ×0.1, ×1, ×10, ×30;
       - **refuses CertificateExceeded** below the cut at ×100 (bound 51.2 vs δ 0.5/0.1), ×300 (153.6) and ×1e3 (511.9), at every δ.
       - Changing `ON_ROW` to 1e-9 makes ×1e3 mesh (check_mesh ok, dev 0.59δ, fans 18/16). So the apex reads more than 64ε off v = 0 at that scale, loses its singular flag, and cert_cone takes the whole turn as Δu.
    
       The doc claim at chart_table.rs:46-53 ("64ε is far inside…", "apex read at v == 0.0 exactly") holds only at unit scale. The sphere and spindle use radians and are fine. It always fails as a typed refusal: no sliver, fold or wrong fan was seen at any scale.
       - Fans at the apex were manifold and check_mesh clean at every δ and scale that meshed; fan sizes were 4–82.
       - The apex probe pins only δ ∈ {5e-3, 1e-3, 2e-4}. At δ = 2e-2 the unit funnel refuses CertificateExceeded (0.029 &gt; 0.02), and so does ×3 at 1.5e-2: coarser refuses where finer meshes.
    4. **Held, with a coverage hole.** Sampled true distance (13×13 barycentric per triangle, closed-form cone/torus/sphere distance) on every new face: max 0.69δ (klein elbow), apex fans included.
       - Mutant "drop singular flags" in the trimmed emit: the apex row goes red.
       - **Mutant cone and torus row certs ×0.01: the whole mesh suite stays green.** The only red, every_suite_file_is_aggregated, was my scratch file. Nothing falsifies an under-reporting certificate on these arms, because the deviation sampler is spline-only (trimmed.rs:1188-1198).
    5. **Held.** Frusta cut at tilt 1.0, so the ellipse runs past both rims: mesh, dev ≤ 0.65δ. Steeper tilts give hyperbolas, and split refuses (OpenConicChord), so I could not build a face bounded only by non-circles beyond the funnel halves (ellipse plus rulings, fine). A torus split refuses (CurvedBooleanUnsupported), so the inner-side torus is covered only by the vessel and elbow. Circles on a cone or sphere are coaxial, so their closed form is exact (inspection). The cone box reads |v|·sinα, correct.
    6. **Held.** Every face whose target changed (trimmed cone/torus/spindle, and trimmed spheres with a trim carrier) refused before, and so did its whole body. No neighbour's chords moved on a body that meshed. No new constant.
    7. **Held.** `check_volume`'s δ·area bracket predates this unit; its argument is first-order and coarse. Mutant "inside-out cone patch" turns all three cone reach rows red. A patch a period away is the same 3-D points, so it is undetectable and harmless. One count per shared edge is check_mesh's watertightness, green everywhere.
    8. **Held.**
       - The sphere-spline row now refuses `chart_image` (ConicImageOnly) on the first edge reached. The refusal text is true. Which of its two refusals you see depends on edge order.
       - Refusals moved between sites, roster → chart_image: sphere/torus faces with a projected spline image. Nothing that meshed now refuses.
       - The vessel's 46-vs-7 over-count is max(carrier, UV), so it is finer and safe (inspection).
    9. **Held.** The ring guards are untouched, ring answers in reach_probes are unchanged, and `lane_of`'s edit is behaviour-neutral.
    10. **Held.**
        - The spiric_rim row and the tour vessel go from refusal to mesh; no kernel change was made for the demo.
        - The bare-rounding row moves trimmed.rs prod 2 pose → chart_table.rs prod 2 pose (`ON_ROW`, `passed_by`).
        - **But** the gate requires every pose row to be scheduled in work/oneround/site-rounding-charges-owed-a-re-posing.md. Its line 89 still names `mesh::trimmed` (`sphere_polygon_shape`, `at_pole`), both deleted, and does not list chart_table.rs. That line is stale.
    11. **Partly held.** My lemon/apple profile is refused by revolve (VertexCrossesAxis). The trimmed spindle arm is pinned only by a flat_points unit row (and cert.rs's pre-existing cert_spindle rows). Its lattice, metric, emit and fold paths are unreached code that reads as supported, and no row is filed for it.
    12. **Held.** The README re-word matches the code: cone, sphere, torus and spindle read ChartRows; the cylinder reads None.
    13. **Corrected.**
        - The circle-closed-form window, the conic-only projected rule, and "sizing/cert dispatch nothing" are corrections, not stops.
        - "No door builds a trimmed spindle face" is disclosed but unscheduled.
        - None of the four filed rows is a defect this unit introduced.
        - The sweep missed the Surface-kind roster in `face_target` and `Reads::ChartRows.pole_budget` (S2, S3).
    
    ## Findings
    - **MAJOR** chart_table.rs `ON_ROW`/`SingularRow::holds` (l.113-126, doc l.46-53): an absolute 64ε on the cone's metre-valued v. A funnel ≥100 units refuses at its apex at every δ. DEMONSTRATED: scale sweep, plus the ON_ROW mutant fixes it.
    - **MINOR** trimmed.rs emit l.1188-1198: cone/torus/spindle certificates are never sampled against true deviation, and a 100× under-reporting cert leaves all 455 tests green. DEMONSTRATED.
    - **MINOR** reach_probes `a_cone_face_through_its_apex…`: δ set skips 2e-2, where the same body refuses (cert 0.029). The row cannot show coarse-δ refusal. DEMONSTRATED.
    - **MINOR** work/oneround/site-rounding-charges-owed-a-re-posing.md:89: the schedule names deleted fns in trimmed and omits chart_table.rs. BY-INSPECTION.
    - **NOTE** The trimmed spindle arm is unreached by any body and no row is filed. BY-INSPECTION.
    - **NOTE** demos/tour/tests/torusvessel.rs module doc: "The scene's third panel" (there is none) and "[`sectioned`]'s only other caller is [`stops`]" (false: stops no longer calls it; the test is its only caller). BY-INSPECTION.
    
    Spec deviations: five reported. SILENT: none material. The unscheduled spindle reach is reported without a schedule.
    
    ## Style
    - S1 (likely, Q1): `ChartRow::singular_rows` (chart_table.rs:281) and `topo::chart::Chart::poles` (topo/src/chart.rs:213) are two lists of one fact, tied only by a test; this PR wrote the second copy. mesh already depends on topo, so the row could read the chart's poles and add the end. The curved lane still reads `Chart::poles` (walk.rs:1058), not the row.
    - S2 (likely, Q1/Q7): `Reads::ChartRows { pole_budget }` (chords.rs:399) is "is sphere" as a flag beside the table. The sphere-only `MAX_SPHERE_EDGE_CHORDS` is a per-chart fact the row does not hold.
    - S3 (sure, Q1): `trimmed::face_target` (l.267-276) matches a Surface roster of analytic kinds instead of reading the row. That is a second roster beside the table.
    - S4 (sure, Q5): chart_table doc l.33 says "Every lane reads the row the same way". The curved lane's pole and fan handling comes from walk/Chart::poles; it reads the row only for steps and cert.
    - S5 (likely, Q7): `ChartRow::outward` reads the row only to say spline-or-not, and recomputes from `surface`. The Cylinder and Sphere variants carry origin/axis/centre that duplicate the Surface. A row that held the surface reference (or a `normal` capability) would avoid the two-argument split. `is_spline` is a second exhaustive match for one bit.
    - S6 (likely, Q7): inner matches inside the analytic arm list `ChartRow::Spline` (trimmed.rs:449, 501), and the curved lane has `ChartRow::Spline =&gt; Err(MissingEntity)` (curved.rs grid_counts). Each is an exhaustive spelling of "can't happen" that answers silently for an unconsidered variant, the `_ =&gt;` problem in another form.
    - S7 (sure, Q3): see the MINOR on deflated certs. The "certificates certify" claim has no row that goes red when the bound degrades.
    - S8 (likely, Q2): the trimmed.rs l.1237-1262 and l.446-471 comments still argue "on the sphere arm… elsewhere the drop is all it is… no chart singularity". That is stale now that cone and spindle rows carry singular rows.
    - S9 (unsure, Q6): a `SphereTrimShape` name now covers cone and spindle; filed, OK. The `ThroughPole` text was widened, but only `ChordBudget` is sphere-only.
    - S10 (sure, Q5): the torusvessel test doc, as in the NOTE.
    
    Questions exercised: Q1, Q2, Q3, Q5, Q6, Q7, Q8. I read chart_table.rs whole, and trimmed.rs lines 1-2000: polygon, singular-row, lattice and emit passes. Q4 partly (the oneround schedule).
    
    ## Rubric
    - Idiom/structure 4: a real table, exhaustive everywhere. Residual rosters S1–S3.
    - Tests 3: the apex/flag mutant and inside-out mutant go red, but deflated certs go uncaught, there is no scale axis, and the δ set skips the refusing δ.
    - Doc honesty 3: false 64ε soundness sentence; stale "sphere arm only" comments; wrong torusvessel test doc.
    
    ## Executed
    - Built and ran the mesh suite under the cert ×0.01 mutant: 455 pass, plus my scratch lint red.
    - Mutants (each restored afterwards): drop singular flags, inverted cone winding, `ON_ROW` = 1e-9.
    - Scratch probe, not committed (crates/mesh/tests/zz_review_probe.rs; copy in my scratchpad, offered as a permanent row), run at δ 2e-2…2e-4. It covers funnel splits at three heights, tilt-0.3 and tilt-1.0 frusta, the vessel and elbow hollows, a funnel scale sweep from ×0.01 to ×1e3, and lemon/apple attempts. It reports sampled true distance against δ, apex fan size and check_mesh for each mesh.
    - Not run: the tour, sweep's spiric_rim, and a bisect at c6632246c5; for those I relied on the goldens being unedited and green at head.
    
    Read only: chords/sizing/curved/types/tessellate diffs, the sweep and demo diffs, the spec, plan, README, the PR description and check runs. I saw no other review's material and read no PR comments or `analysis/` branches.
</cross-session-message>

0 notifications remain queued.
