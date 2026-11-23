TESSLANE implementer — PR 6950 fix pass. Pushed head 72af97e31f (tesslane/chart-table; includes the merge of origin/main).
    
    A. What decides: whether a loop vertex IS a singular point is the body's fact, decided where the lift decides it: geom_brep::chart_singularity::vertex_at_singular_point(vertex, point, SizingTols.band), via AnalyticRow::is_singular, read by mesh id (singular_vertices → HashSet&lt;u32&gt;). ON_ROW, SingularRow, on_singular_row and is_spline are gone. Why: it is the same decision topo::pcurves' singular_at takes to turn the lift, so the lane reads the lift's own structure with no threshold of its own, in a 3-D distance against the run's band that moves with neither scale nor pose. Rows: a_singular_point_is_decided_at_the_band_at_every_scale_and_pose (cone/sphere/spindle posed at (700.3,−577.7,123.4), axis (1,2,3), scales 1e-2/1/1e3) and the scaled/posed funnel row (below). No sphere face that meshes on main moved.
    
    B. Mechanism: a cone ruling is straight; the chart holds the apex as a row of copies, and the chart CDT joins one apex copy to two points of one lattice column, which is three points on one ruling (zero area). Sphere-pole degenerates: 0 (worst 2A/L² 0.030 over the tilted pair and the z-balls; meridians are arcs). Built option (1), as you ruled: the cone face's CDT runs in its development, r = σ·v, θ = σ·sin α·u, point r·(sin θ, cos θ). This is an isometry, orientation-preserving on either nappe (Jacobian r·sin α).
    - By construction: direction components are cut to 26 significant bits and r to 27, so every product is exact. Each column point is an exact multiple of one direction, so the exact predicate reads any three points of a ruling (or two and the apex) as collinear and no triangle can lie along one. Three cone points on one line lie on a ruling, so no other zero-area triangle exists. Pinned by fma exactness in the_development_keeps_apex_copies_one_and_mesh_vertices_apart.
    - Cond. 1 (one CDT vertex): every apex copy maps to the origin. The development rounds, so it is not injective. A polygon point that would share a CDT vertex with one at another UV location refuses typed (Triangulation); a grid candidate that would is dropped, as in the chart. ids_at_two_uvs still reads UV, so it names the apex id; the fan census then counts the apex's single fan (edges used ≤2). The dedupe key is cdt_point's answer. cdt_point_floors_then_scales_the_growing_axis is unedited and green: cdt_point(&amp;self,u,v) stays as a cfg(test) wrapper over the free cdt_point(plane,u,v,singular).
    - Cond. 2 (replay): sin and cos are &lt;f64 as Real&gt;'s, the pure-Rust libm (D9). The read-back of an inserted point is geom::Azimuth::near, the angle door, so there is no new atan2 site. gates: atan2-sites unchanged; bare-rounding-sites lowered chart_table 2→1 (ON_ROW gone).
    - Cond. 3 (sector): negative nappe — the narrowing frustum row now asserts its faces lie on v&lt;0, and the apex-to-apex union row (lower funnel on v&lt;0, through apex). Through apex — the apex row. Whole turn — no door builds a whole-turn trimmed cone face (revolve makes half faces), so it is pinned at unit level (the_development_keeps_orientation_and_a_whole_turns_seam_apart: seam at u=0 and 2π gives two rays for α 0.02…1.5; orientation sign kept on both nappes). A loop meeting the apex twice is two wedges touching at the origin; even-odd reads it; argued in docs, no body. A face on both nappes refuses SphereTrimShape{BothNappes} (new cause), pinned at unit level.
    - Cond. 4: only trimmed cone faces move (all new in this PR). Lattice, creases, orientation, fold census, ear test and certificates read UV. Ear-cure insert = developed centroid read back (TrimmedFace::inside_point). Main's D9 goldens and the tour pins are green.
    - Cond. 5: needle (1e-3, 2e-4), translated (2e-4), shifted funnel (0.18), ×300 at fine δ: all mesh, with 0 degenerates anywhere (probe: worst 2A/L² 2.9e-5 on needle-below at 2e-4). reach_probes::geometry asserts 2A/L² ≥ MIN_SHAPE = 1e-6 on every analytic chart.
    
    Items:
    1. Taken (test side): geometry() samples true distance on a 28-point barycentric grid of every analytic triangle (closed-form distance per kind) and asserts ≤ δ. Production dev sampling stays spline-only (budget-consumer coverage gap, unchanged).
    2. Taken: a_cone_faces_window_reads_its_widest_edge (window ⊇ sampled widest radius, ≤1.2×; the narrowing frustum's upper halves are decided by the ellipse arm).
    3. Taken. Cause: at coarse δ the chart-metric CDT made 2-column triangles beside the trim that exceed. The development cures it (measured: with refinement back to spline-only every coarse funnel row still meshes). I reverted my interim "refinement on every arm" so no sphere or cylinder path moves; an_over_delta_sphere_arm_triangle_refuses_typed is restored to head. Pinned: the apex row at 2e-2 and 1.5e-2.
    4. Taken: lattice_steps (lever/cap), metric, chord_budget (MAX_SPHERE_EDGE_CHORDS) and development are AnalyticRow answers. face_target's roster is one predicate (non-spline trimmed face), no list.
    5. Taken: singular points come from Chart::poles (+RowEnd); the tying test and debug_assert are gone.
    6. Taken: filed work/tesslane/no-door-builds-a-trimmed-spindle-face.md (the lemon meshes whole on the curved lane; split refuses CurvedBooleanUnsupported{Spindle}, booleans CurvedPairUnsupported{OperandGate,Spindle}; it says what is unreached and what would reach it). Docs made true: trimmed.rs chart scope; types.rs UnsupportedCurve.
    7. Measured: no door builds a both-nappe face (an axis-crossing profile refuses at validation; the apex-to-apex union keeps one face per nappe). Typed refusal pinned anyway (BothNappes).
    8. Taken: re-pointed to mesh::chart_table (SingularPoint::passed_by).
    9. Taken. (a) The apex row runs every δ incl. 2e-2/1.5e-2. (b) Headroom is pinned directly: the chord-step row asserts face_target = δ·UNPROVED_HEADROOM bitwise on every trimmed cone/torus face. (c) geometry's outward is the surface's own closed form (outward()), not implicit_outward_normal; it caught my own nappe-sign slip on the narrowing frustum.
    10. Ellipse|Spiric dropped to a typed UnsupportedCurve (neither lies on a sphere); the Nurbs arm is kept and justified (sphere–cylinder section). The posed funnel now meshes, so there are no refusal texts left to check.
    11. Taken: every listed stale doc fixed. The torusvessel module doc is corrected (no panel; that row is sectioned's only caller). The SizingTols.band doc names the trimmed lane.
    Judgement: analytic rows are their own type (AnalyticRow; the curved lane and grid_counts have no Spline arm). outward reads the surface rather than copying it (kind keeps only the numbers steps/cert read). is_spline is gone (matches! on ChartRow). The chord pass caches FaceLattice per face; singular_points() returns a slice, so there is no per-edge-side rebuild.
    
    Departures: refinement stays spline-only (see 3). A whole-turn face is pinned at unit level only (no door builds one). The scale/pose row prints and skips a typed revolve refusal below the default ε (at ε 1e-12 the ×300, ×1e3 and shifted funnels refuse KernelLimit at revolve, as issue303's rows do), and asserts none is skipped at the default ε. Typed refusals added: SphereTrimCause::BothNappes; Triangulation on a developed merge.
    
    Lines (vs b69a146d45, mine): mesh src ≈ +1320/−700 (trimmed.rs +865/−316 incl. ~350 test lines; chart_table +338/−307); tests/reach_probes +360/−35; work/ +50 (new issue, oneround line); gates/nextest 3.
    
    Ran (on the merged head):
    - run-gates.sh: exit 0.
    - doc-gate --workspace-pass --skip-viewer-toolkit: exit 0.
    - fmt: clean.
    - clippy -D warnings: mesh, geom-brep, demos/tour all clean.
    - nextest -p mesh: 460/460.
    - demos/tour mesh/pin rows: 22/22 (incl. torusvessel, p1_mesh_pin, chaintol). The full tour suite exceeded its 30-min background limit and was stopped.
    - Changed/new rows at CAD_TOLERANCE_EPS 1e-6: 14/14; at 1e-12: 14/14 after the documented skip.
    Mutants:
    - development()→None reds a_cone_face_through_its_apex_meshes_on_the_cone_row (2A/L² 6.5e-18).
    - Headroom off for cone/torus reds the_cone_and_torus_arms_boundary_chords_step_within_their_lattice.
    - The rho_max ellipse arm skipped reds a_cone_faces_window_reads_its_widest_edge.
    - Refinement spline-only reds only the (then) every-arm sphere row (cause for 3).
    Slow set: added reach_probes::a_cone_face_through_its_apex_meshes_at_every_scale_and_pose (2.3 s); check-slow-set-names is clean for mesh.
    Filed: work/tesslane/no-door-builds-a-trimmed-spindle-face.md.
    Reviewer probes received by message: not committed.
    
    =====
    One chart table, and the trimmed lane takes every analytic chart
    
    What the mesh knows about a curved face's chart is one row per chart (`crates/mesh/src/chart_table.rs`): `ChartRow::{Analytic(AnalyticRow), Spline}`, built once per face and read by the curved lane, the trimmed lane and the chord pass. An analytic row answers:
    - its steps and lattice steps (lever and cap);
    - the metric its CDT runs in;
    - on the cone, its development;
    - its certificate;
    - its singular points (from `topo::chart::Chart::poles`, with where each sits in the chart's `v` domain);
    - whether a vertex is one (`geom_brep::chart_singularity::vertex_at_singular_point` at the run's band, the lift's own decision);
    - its outward normal;
    - its per-edge chord budget.
    
    The trimmed lane meshes cone, torus and spindle faces as well as cylinder and sphere faces:
    - A cone face triangulates in the cone's development (`trimmed`, "The development"): the apex is one point and a ruling a ray from it, cut so every column is an exact multiple of one direction. No triangle lies along a ruling, and the CDT is the surface's own Delaunay triangulation.
    - Points the development would merge refuse typed.
    - `sin` and `cos` are `Real`'s libm.
    - The read-back is `Azimuth::near`.
    - A face on both nappes refuses `SphereTrimShape { BothNappes }`.
    - The lattice, the fold census, the ear test and every certificate read UV.
    
    The chord pass caches one `FaceLattice` per face. `sphere_face_widest_cos` refuses an ellipse or spiric edge typed.
    
    Rows:
    - `reach_probes::geometry` checks every analytic triangle against the surface's closed form: winding against its own normal, sampled true deviation ≤ δ, and 2·area/L² ≥ 1e-6.
    - The funnel through its apex meshes at five δ, at scales 10⁻² to 10³, translated and turned; so do a needle funnel, a shifted funnel at δ 0.18, and two funnels apex to apex on both nappes.
    - Frustum faces are pinned to their nappes.
    - A cone face's window reads its widest edge.
    - The new arms size at the lane's headroom.
    - Unit rows pin the development's vertex rule, orientation, whole-turn seam and the both-nappes refusal.
    
    Filed: `work/tesslane/no-door-builds-a-trimmed-spindle-face.md`; no door yet builds a trimmed spindle face.
    =====
</cross-session-message>

0 notifications remain queued.
