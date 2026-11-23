TESSLANE implementer — PR 6950 third pass. Head 507190de06 (origin/main merged first; no main change since).
    
    1. Pinned and filed, not cured. My own measurement (unit funnel, tilt-0.3 cut at (0,h,0), upper half):
    - h 1e-2 and 3e-3 refuse CertificateExceeded at δ 1e-3/2e-4/5e-5;
    - h ≤ 1e-3 also refuses at 5e-3 (1e-3, 1e-4, 1e-6 measured);
    - bounds run 1.03δ–8.0δ;
    - coarser δ meshes, every lower half meshes, and h = 0.02, 0.03, 0.1, 0.3 mesh at 2e-2…5e-5;
    - matches the reviewers'.
    With the cone certificate zeroed, all but two of the refusing cases (h 1e-6 not measured, see below) are within δ in truth; two are truly over (h 3e-3 at 1.63δ, h 1e-3 at 1.23δ, both δ 5e-5). The h 1e-6 cases stop at the shape check first, so their truth is unmeasured. The Δu≈0.44 small-end/first-row reading is stated as likely (the reviewer's), not re-verified. On main the face refuses UnsupportedCurve at every δ (trim_frontier: no trimmed cone arm). I read that from main's source; I did not rebuild main (3.6 GB disk free).
    - Row: reach_probes::a_cone_face_ending_near_its_apex_refuses_at_fine_delta (slow set, 1.7 s). It asserts the CertificateExceeded variant where the table refuses, and Meshed+geometry elsewhere. Its doc says it pins a gap and names the issue.
    - Per ε: within 1000ε of the apex the band reaches the cut. At ε 1e-6, h 1e-4 refuses at the split (KeptBoundaryUndecided) and h 1e-6 leaves no lower body; both are printed and skipped by name. h 1e-6 upper at 5e-3 meshes, held to geometry. Green at default, 1e-6 and 1e-12.
    - Filed work/tesslane/a-cone-face-ending-near-its-apex-refuses-at-fine-delta.md (P1): the table, verified vs likely mechanism, main's refusal, three cures unchosen.
    2. Row chart_table::tests::the_cone_and_torus_certificates_hold_the_true_deviation. Posed cone (apex triangles included) and ring torus, 4 spans × 3×3 places × 2 orientations; true distance at 300 barycentric points ≤ cert. Mutants run: cone ×0.01, cone ×0, torus ×0.01, torus ×0 — each reds it. geometry's doc now says it reds a mesh truly past δ, not an under-reporting certificate, and points at this row.
    3. The development's merge is SphereTrimShape { cause: DevelopedMerge } at the merging point's edge. It is decided once per face before the first attempt (trimmed::developed_merge), no longer Triangulation inside insert_points. I chose a cause over a new TessellateError variant because a variant reaches pncad-py's exhaustive tag match and binding census, outside this unit's files; SphereTrimCause projects to Python by name. Unit row updated (developed_merge → Some(1); none for the apex copies alone).
    4. is_singular returns Result&lt;bool, Indeterminate&gt;; singular_vertices refuses SphereTrimShape { SingularUndecided } at the vertex's edge before the polygon is walked. Its doc says so; the old "lift fails first" claim is gone. Separately, a loop with no lift (LiftGap) refuses UnsupportedCurve at the gap's own edge with the lift's cause in the note. It keeps "pcurve cache", which m5_pr11's missing-caches row asserts; that is a true lift gap. A half-edge missing from a successful lift is MissingEntity. Door: none tried reaches SingularUndecided. Body and mesh decide on the same stored points at the same band in one run, so it needs a body built at one ε and meshed at another, with a vertex within the band of the apex. Pinned at unit level: a point midway between band.zero and band.escalate reads Err, in the posed/scaled singular row.
    5. MIN_SHAPE is 1e-12, documented as "not on one line": a collinear ruling triple reads ~1e-17 (6.5e-18 measured); nothing bounds thin triangles. The const doc and the development doc say so. Measured: needle r=0.002 at δ 1e-5 is 9.0e-7/1.6e-7 (above/below) and at 1e-6 is 9.2e-8/1.6e-8; frustum small rim 2e-7/5e-7 at δ 5e-3 is 3.2e-8/7.9e-8 whole (curved lane) and 3.1e-7/7.6e-7 upper half; funnel cut 1e-6 above its apex at δ 2e-2 is 9.6e-7. Filed work/tesslane/a-needle-cone-face-meshes-thin-triangles-beside-its-rulings.md (P3), linked to item 1's issue but not the same row (these meshes are accepted). Mutant development()→None still reds at 1e-12 (6.5e-18).
    6. Five pieces:
    - AnalyticRow holds { surface, window, singular }, and kind() is derived from the surface. A kind/surface mismatch is unrepresentable; cert matches the surface directly.
    - AnalyticRow::new is the one constructor with the pole→RowEnd map (of reads the window, then calls new). It refuses plane/spline, so the remaining unreachable! guards only surfaces new refuses. tests::row is gone; every test uses new.
    - Azimuth::of(g.x, g.y) has a comment on (y, x) order.
    - lift_probes' on_singular_row and the apex/pole asserts are documented as the unposed fixtures' own chart readings, not the lane's rule; the stale SingularRow::holds reference is gone.
    - The exactness cut: the mutant "developed returns plain r·sin, r·cos" reds only the_development_keeps_apex_copies_one_and_mesh_vertices_apart, across the full mesh suite; that row's doc says so. No cheap end-to-end row (it needs a ruling side chorded into several chords).
    7. Nothing more.
    
    Ran, final: run-gates exit 0; doc-gate exit 0; fmt clean; clippy -D warnings clean (mesh, geom-brep, demos/tour); nextest -p mesh 462/462. Added and changed rows at ε 1e-6 and 1e-12: 30/30 each.
    Mutants: the four certificate ones (item 2); development()→None (6.5e-18); developed without cuts (only its unit row reds).
    
    PR body: replaces the earlier one; the "Rows" list changes and a "Not reached" section is new.
    =====
    One chart table, and the trimmed lane takes every analytic chart
    
    What the mesh knows about a curved face's chart is one row per chart (`crates/mesh/src/chart_table.rs`): `ChartRow::{Analytic(AnalyticRow), Spline}`, built once per face and read by the curved lane, the trimmed lane and the chord pass. An analytic row holds its surface and the face's window, and answers:
    - its kind;
    - its steps and lattice steps (lever and cap);
    - the metric its CDT runs in, or on the cone its development;
    - its certificate;
    - its singular points (`topo::chart::Chart::poles`, with where each row sits in the chart's `v` domain);
    - whether a vertex is one (`geom_brep::chart_singularity::vertex_at_singular_point` at the run's band, the lift's own decision; an undecided vertex refuses `SphereTrimShape { SingularUndecided }`);
    - its outward normal;
    - its per-edge chord budget.
    
    The trimmed lane meshes cone, torus and spindle faces as well as cylinder and sphere faces:
    - A cone face triangulates in the cone's development (`trimmed`, "The development"): the apex is one point and a ruling a ray from it, cut so every column is an exact multiple of one direction. No triangle lies along a ruling, and the CDT is the surface's own Delaunay triangulation.
    - A face whose points the development would merge refuses `SphereTrimShape { DevelopedMerge }`; one on both nappes refuses `BothNappes`.
    - `sin` and `cos` are `Real`'s libm; the read-back is `Azimuth::near`.
    - The lattice, the fold census, the ear test and every certificate read UV.
    
    The chord pass caches one `FaceLattice` per face. `sphere_face_widest_cos` refuses an ellipse or spiric edge typed. A loop with no lift refuses at its gap's half-edge.
    
    Rows:
    - `reach_probes::geometry` checks every analytic triangle against the surface's closed form: winding against its own normal, sampled true deviation ≤ δ, and not on one line (2·area/L² ≥ 1e-12).
    - The cone's and the torus's certificates are held to their triangles' sampled true deviation.
    - The funnel through its apex meshes at five δ, at scales 10⁻² to 10³, translated and turned; so do a needle funnel, a shifted funnel at δ 0.18, and two funnels apex to apex on both nappes. A typed revolve refusal below the default ε is named.
    - Frustum faces are pinned to their nappes.
    - A cone face's window reads its widest edge.
    - The new arms size at the lane's headroom.
    - Unit rows pin the development's vertex rule, its exactness, orientation, whole-turn seam and the both-nappes refusal.
    
    Not reached:
    - A cone face ending near its apex refuses `CertificateExceeded` at fine δ: the development triangulates and the chart's certificate reads the span it makes short. Pinned by `reach_probes::a_cone_face_ending_near_its_apex_refuses_at_fine_delta`; filed `work/tesslane/a-cone-face-ending-near-its-apex-refuses-at-fine-delta.md`. The face refuses on main too (no trimmed cone arm).
    - Thin triangles beside a ruling are certified and unbounded in shape: `work/tesslane/a-needle-cone-face-meshes-thin-triangles-beside-its-rulings.md`.
    - No door builds a trimmed spindle face: `work/tesslane/no-door-builds-a-trimmed-spindle-face.md`.
    - No door builds a whole-turn trimmed cone face; its seam is pinned at unit level.
    =====
</cross-session-message>

0 notifications remain queued.
