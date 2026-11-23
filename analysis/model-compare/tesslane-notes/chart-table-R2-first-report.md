REVIEW — PR 6950, tesslane/chart-table, frozen head 3a44a00f24 (full review + style lane)
    
    **Verdict: NOT-MERGEABLE-AS-IS.** Confined to the cone apex arm. The torus wall, the ellipse-cut frusta, the chord pass and commit 1 I found right. Two demonstrated defects sit on claim 3, one of them a silent output defect on the unit's own pinned row.
    
    ## Findings
    
    **MAJOR-1 (DEMONSTRATED) — a through-apex cone face emits zero-area triangles, and its winding there is decided by rounding.** `trimmed.rs` emit pass / `chart_table.rs` singular-row rule. The CDT joins an apex copy `(u_apex, 0)` to two lattice points of one column `(u, v1), (u, v2)`: a proper chart triangle, three collinear points of one ruling in 3-D. Only "two corners on one mesh id" is dropped, so this one reaches the fold census, where `normal·outward` is ~1e-15 noise: negative → both grid corners dropped and a rebuild; positive → emitted.
    - Emitted: the body of `a_cone_face_through_its_apex_meshes_on_the_cone_row` (funnel, below) carries 2 such triangles at δ=1e-3 and 4 at 2e-4 (2·area/L² ≈ 1e-17); 21 of my 106 accepted meshes carry 1–10. `check_mesh`, the volume bracket and the winding test all pass them.
    - Refused: instrumented, 1274 fold events across my probes, every one with an apex corner. Where six retries do not clear them a valid face refuses `FoldedTriangle`: needle funnel (profile (0,0),(0.02,1),(0,1)) below at δ 1e-3 and 2e-4; funnel translated (700.3,−577.7,123.4) below at 2e-4; funnel (0,133.3),(612.3,911.1),(0,911.1) below at δ 0.18.
    - The module doc's "the triangles left at the point are a fan about it" is not what the lane produces on a cone.
    
    **MAJOR-2 (DEMONSTRATED) — "on the row" is 64ε absolute in slant metres, and a rigid motion moves the apex off it.** `chart_table.rs` `ON_ROW`, `SingularRow::holds`. The funnel under rotation 0.7 about (1,2,3)/|·| through (0.3,0.1,0.2) then translation (700.3,−577.7,123.4), inside the session box, split by the same posed plane: the two rulings' lifts read the apex at v = 5.0e-14 and 5.6e-15 (64ε = 1.4e-14). The polygon keeps the apex once; one half refuses `CertificateExceeded` (bound 0.039, not shrinking with δ) and the other `SphereTrimShape{SeamTwice}`, at every δ. With `ON_ROW = 1e-9` the same body meshes at all three δ, and no committed row goes red (104 selected rows green). The doc's "the cone's apex row is read at `v == 0.0` exactly" holds only for the unposed fixture. The gate's own list classes these two sites `pose`, "owed a decision at the band"; this unit extended them from radians to metres. I found no silent wrong mesh from it: the certificate stays conservative.
    
    **MINOR-1 (DEMONSTRATED) — no committed row pins the cone or torus certificate.** `ChartRow::cert` with the cone and torus arms returning 0 (both lanes read it): full mesh suite 455/455 green. Under that mutant my probes get accepted meshes at 2.07δ and 1.17δ true deviation, passing `check_mesh`, winding and the volume bracket.
    **MINOR-2 (DEMONSTRATED) — the window's soundness is unpinned.** `sizing::cone_face_rho_max` with the non-circle arm skipped: `a_cone_face_bounded_by_a_section_ellipse_meshes_on_the_cone_row` (the narrowing frustum the PR cites) stays green. What reds is faces with no circle at all, by `MissingEntity`.
    **MINOR-3 (DEMONSTRATED) — refusals at δ the rows do not pin.** Unposed funnel below at δ=2e-2: `CertificateExceeded` (0.029 &gt; 0.02). Rotated-only funnel below at 5e-3: same (0.0138), apex on its row. Cause not established for these two.
    **MINOR-4 (BY-INSPECTION + CI read) — the gate is not green.** `test (kernel)` failed in run 43584233101 at b69a146d45 (the branch has moved two commits past the frozen head; mesh diff between them is two doc lines in `curved.rs`). Three sweep eps rows: `band_strip_mouth_cups::a_cups_mouth…` [1e-6], `tilted_sphere_pair::a_section_passing_near_a_pole…` [1e-6], `full_turn_wall::both_doors_answer_the_bead…` [1e-12]. Each has a tracker row on main naming it red; I did not check main's own run. No mesh row failed. `python suite` was still running. Demos, lint, meter jobs green there. The frozen head has no run of its own.
    **MINOR-5 (BY-INSPECTION) — stale schedule row.** `work/oneround/site-rounding-charges-owed-a-re-posing.md:89` still names `mesh::trimmed` (`sphere_polygon_shape`, `at_pole`); the gate header says that file lists each `prod` row's file.
    **NOTE-1** The apex refusals of MAJOR-2 carry texts that are not their cause (`SeamTwice`: no seam is traversed twice).
    **NOTE-2** Headroom on the new arms is pinned only by the apex row: with cone/torus/spindle at δ instead of δ/2, that row alone reds.
    **NOTE-3** `sphere_face_widest_cos`'s `Ellipse | Nurbs | Spiric` box arm computes a window no mesh reads: every sphere face reaching it refuses next at `chart_image`.
    **NOTE-4** Cone faces sit on both nappes (the narrowing frustum's `v` is negative). `cert_cone` on a triangle spanning `v = 0` would not be the hull bound; I built no such face (an hourglass was not tried).
    
    ## Claims
    1. **Held.** Full mesh suite at c6632246c5 in a worktree, tests unedited, D9 goldens included: 434/434. Read every reader before and after (steps, certs, sphere steps, cylinder `hu`, `takes_sphere_arm` → `has_tilted_circle`). Not run at that commit: tess-budget-data, tour. "Read, not restated": corrected, Style S1.
    2. **Held.** A sixth variant fails to build at 11 sites. What still builds: a `debug_assert` predicate (`trimmed.rs:476`), the budget-meter gate (`:613`), test code. `lane_of`'s `_ =&gt; Curved` reaches only analytic charts; a non-iso circle on a cone or torus goes to the curved lane's guard as before.
    3. **Falsified** (MAJOR-1, MAJOR-2). Mutants: never keeping the side → 3 rows red; flags all false → 1 row (apex); no doubling in `pole_side_columns` → its unit row; `passed_by` and the 3π/2 test off → the sphere-shape unit row only, nothing for cone or spindle. Sphere-pole faces: unchanged predicate, not re-probed.
    4. **Held for values, corrected for pins.** True distance sampled (8×8 barycentric, both nappes): cone ≤ 0.75δ, torus ≤ 0.69δ on every accepted mesh, four δ per body. MINOR-1.
    5. **Held where measured; corrected for pins.** Ellipse past both rims (both frusta), ellipse-and-rulings only (funnel), plane 1e-2…1e-6 above the apex: all within δ. Circles: a circle on a right cone is a parallel; any circle on a sphere reads the harmonic form. Torus reads no window. `chart_box` enclosure itself: not executed. MINOR-2, NOTE-3.
    6. **Held (by inspection).** Old rule: cylinder if trimmed, sphere if `takes_sphere_arm`. New faces under headroom all refused before. No constant added.
    7. **Held off the apex; falsified at it** (MAJOR-1). On all 106 accepted probe meshes: every boundary segment used once by each side, no inward triangle. Mutants: inside-out cone/torus patch with the lane's fold check off → 5 rows red; fold check off alone → apex row; apex fan removed → apex row. A patch a period away: not run. The δ·area bracket is argued to first order and does not see MINOR-1's 2δ triangles.
    8. **Held.** Printed counts reproduce 46/7/7/46/4, 103/14/14/103/8, 229/31/31/229/17; `settle` takes the max, so the over-count is the carrier's and safe. The sphere-and-spline refusal site is right. Its text states a policy: the bound exists (1.4e12, per the filed row).
    9. **Held.** Ring guard untouched; all `reach_probes` rows green in my run.
    10. **Held by reading.** Sweep's and the tour's rows not run by me (demos job green at b69a146). The tsv row moved file with count 2 and reason `pose`, within the gate's rule; MINOR-5.
    11. **Held, and unreached.** Lemon: split refuses `CurvedBooleanUnsupported` at three planes; all six booleans against a brick refuse `CurvedPairUnsupported` at `OperandGate`. The spindle arm's `flat_points` and singular-rows rows can fail; its steps, cap, metric, certificate flags and emit path in the trimmed lane run nowhere, while `trimmed.rs`'s chart scope lists it as constructing.
    12. **Held.**
    13. Below.
    
    ## Spec deviations
    Reported, and corrections: no chart dispatch in `sizing`/`cert`; no door for a spindle face; circle-only window unsound; conic-only projected rule kept. Reported, a departure I read as better than the letter: circles keep their closed form.
    **SILENT:** the spec's row lists "the metric the triangulation runs in". `ChartRow` has none, and the PR's list of what the row holds omits it without saying so (S1).
    Filed rows: none is a defect this unit should have fixed, bar the naming row, which it caused and files. The sweep's pattern (a match with a fallback arm) cannot see exhaustive per-chart matches outside the table.
    
    ## Style
    Exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7, Q8 (`chart_table.rs` whole; `trimmed.rs` lines 1–2000 whole).
    - **S1 (Q1, sure).** Per-chart lists remain beside the table. `trimmed.rs:445-452` turns steps into lattice steps per chart (lever·h_u on ruled charts, `cap_angular` on torus/spindle) and `:496-503` picks the metric per chart; `curved.rs` `grid_counts` does the first again; `chords.rs` `FaceLattice::of` decides `pole_budget` and "reads a lattice at all" per chart. `ChartRow::steps` answers `+∞` on a ruling, which neither lane lays rows at.
    - **S2 (Q1, sure).** `ChartRow::singular_rows` is a third statement of `geom_brep::chart_singularity::singular_points` → `topo::chart::Chart::poles`. `mesh` already depends on `topo`'s chart, so the row could be read from it; instead a test ties them, and a `debug_assert` in `tessellate_trimmed` ties them again. The fix for "five rosters" minted a roster and a census.
    - **S3 (Q3, sure).** `the_rows_singular_rows_are_the_charts_poles` builds each row by a hand match that copies `ChartRow::of`, compares `v` only (never `end`), and its `holds` closure admits `w == -v` at π to absorb the horn, where the two lists already differ.
    - **S4 (Q3, sure).** `each_singular_row_keeps_its_point_once_at_each_u`'s "just off the row" is 0.25 off; nothing reads the threshold.
    - **S5 (Q1, likely).** `reach_probes::geometry` is a second copy of sweep's `assert_meshed_geometry`/`assert_outward`, and reads `implicit_outward_normal`, the mesher's own oracle, where sweep's spells the direction itself "on purpose".
    - **S6 (Q5, sure).** Stale in `trimmed.rs`: `tessellate_trimmed`'s doc names two certificates; `uniform_candidates` "of the cylinder and sphere arms"; "THE EAR CURE (both analytic arms)", "THE WINDING CENSUS (both…)"; the emit comment at ~1237 "no chart singularity"; the census comment at ~908 naming only the cylinder seam and sphere pole sides.
    - **S7 (Q5, likely).** `types.rs` `UnsupportedCurve`: "Every chart's trimmed face is a CONSTRUCTION lane, not a refusal" overstates (rings, spline-bounded analytic faces).
    - **S8 (Q7, likely).** The row is an enum with a match per method, not data. `outward` needs the `Surface` back because the cone and torus rows keep no frame. `is_spline()` collapses the roster to a bool passed to four per-chart decisions (images admitted, orientation checked, ears, refinement) that are then not in the table.
    - **S9 (Q7, unsure).** `singular_rows()` allocates a `Vec` and is rebuilt per edge side in the chord pass, as is the whole row with its loop walk.
    - **S10 (Q7, likely).** Nested `match row` inside an arm already narrowed to analytic rows carries `Spline` arms that cannot run (`trimmed.rs:451, 502`).
    - **S11 (Q2, likely).** The 64ε paragraph argues at length for a constant the gate lists as owed to the band.
    - **S12 (Q4, sure).** MINOR-5 is this class; `lift_probes`' cone branch still asserts `== 0.0`, the premise MAJOR-2 breaks.
    - **S13 (Q6, sure).** Measured claims without a guard: "64ε is far inside…", "read at v == 0.0 exactly", the slow-set timing.
    - **S14 (Q1, unsure).** A sphere pole with two same-meridian lattice points is the sibling of MAJOR-1 (a fin in the meridian plane); not probed.
    
    ## Rubric
    - Idiom/structure 3 — the table removes the roster refusal and readers are exhaustive; lever, cap, metric and budget stayed outside it.
    - Test quality 2 — reach rows pin right-looking answers at three δ on unposed bodies; zeroed certificates, a 5-order threshold move and an under-read window all stay green.
    - Doc/comment honesty 3 — deviations are stated plainly; the apex-exactness and fan sentences are false, and six lane comments still describe two arms.
    
    ## Executed / read only
    Executed: mesh suite at c6632246c5; 13 source mutants against 104 selected rows plus one on the full suite; a compile experiment; my probe module (true deviation, per-segment use counts, winding, degenerate-triangle count, lift ends) over frusta, funnels (8 shapes, 4 near-apex planes, 3 poses), vessel, elbow, lemon; fold instrumentation; the chord-count row with output. Probe file: /tmp/claude-0/-home-user-cad/3a35533e-2ea5-51c0-b401-ad945f3d5639/scratchpad/review_probes.rs. Two rows worth keeping: the posed funnel at three δ, and "no triangle of an accepted patch has 2·area/L² &lt; 1e-9" inside `geometry`.
    Read only: sweep and tour re-pins, the gate script, the filed rows, PR description and check runs. No other review's material seen; PR comment thread not read.
</cross-session-message>

0 notifications remain queued.
