REPORT — implementer, unit `the-cone-arm-certifies-the-triangle-its-development-makes`, branch `tesslane/cone-near-apex`
    
    **1. Measurement** (pushed first, commit 0056b6cee8, before any build)
    - The filed table reproduces to the digit under the old `cert_cone` (1.249δ … 7.988δ).
    - Refused triangles (h 3e-3, δ 1e-3): two adjacent first-row lattice points (v 0.0775) + one ellipse point (v 0.005), or one row point + two ellipse points, 0.45–0.6 rad apart in u; in the development ~0.07 long and a few 1e-3 wide. Old bound 1.0–1.67δ; the flat triangle's true deviation 0.14–0.27δ. The filed mechanism holds.
    - Truly over δ: exactly two cases, h 3e-3 δ 5e-5 (sampled 1.634δ, exact bound 1.666δ) and h 1e-3 δ 5e-5 (1.230δ / 1.282δ). The over triangles: two row points at v 0.020 and an ellipse point at v 0.0046, 0.38 rad round the axis.
    - Spec corrections: (a) h = 1e-6 is not "unmeasured": with the certificate zeroed it meshes and passes geometry. (b) "the fold census reads the chart": it reads the triangle's 3-D normal against the surface's; only its inputs are chart points. (c) Certificate-driven refinement (spec's first option) does not converge: 191 over at 1.67δ → 165 at 12.4δ → 106 at 4.9δ → 84 at 7.9δ → 26 → 17 → 2 at 2.5δ, refused at the round budget; the other case clears on the last round after 16.6δ.
    
    **2. The certificate, and the points**
    - `cert::cert_cone` = the flat triangle's own deviation: max of its three chords' sags `cos α·2ρᵢρⱼ·sin²(Δu/2)/(ρᵢ+ρⱼ+2√(ρᵢρⱼ)·|cos(Δu/2)|)` and, where the axis passes through the triangle, `sin α·`(its height there). Derivation beside the function, four steps: f = sin α·h − cos α·ρ ≥ dist on the triangle (convex solid nappe, foot on the generator); f affine along rays from the plane's axis point, so it peaks on an edge or the axis; closed-form concave maximum on a chord; the axis term in barycentric weights. Assumes: corners are chart points of one nappe (both nappes → +∞, NaN poisons). No chart box, no Δu ≤ π precondition. It is exact (attained), and never above the old bound, so `cone_grid_step`'s theorem and the curved lane are unmoved; `cone_scale` (the verdict's rounding magnitude) is kept, its doc restated.
    - Break-it sweep: seeded fuzz, needle (α 0.002) to near-flat (1.55), either nappe, six shapes (the filed table's to slant ratio 1e-6, needles, apex corner in any slot, about the axis, any three azimuths over 1.5 turns, a box), oracle = exact distance to the double cone on a 325-point grid, blind to the chart. 3.8·10⁶ triangles over four seeds at raised effort: none past. Tightness: equal to the searched maximum to 1e-12 on the named poses of `cert_cone_is_attained_on_a_chord_and_over_the_axis`.
    - Mutants (all run): sag halved → 5 rows red; axis term dropped → sweep + attained; cos for |cos| → attained; widest slant charged (old-style) → attained + the near-apex mesh row; no nappe guard / no NaN guard → the edge-case row.
    - Points: a lattice graded toward the small end, not refinement (measurement 1c). Cone rows uniform in √|v| (`AnalyticRow::lattice_rows`): the gap at each radius is the chord the lattice's target allows there (= today's `lever·h_u` at the widest radius, to second order), and no row within the radius where every chord certifies. No new constant. `[mesh.sizing]` unchanged; the cone arm still carries `UNPROVED_HEADROOM` (no theorem for a triangle beside the boundary).
    - Reads now: triangulation = development (unchanged); lattice = chart columns uniform in u, rows a 3-D chord apart at their own radius; certificate = the flat 3-D triangle via chart corners; fold census = 3-D normal (unchanged); ear test = polygon order, cure the developed centroid (unchanged).
    
    **3. What changed** (non-merge commits 0056b6cee8 … 340b76c1db; merges of origin/main at dcffa29464 and 6916292d76, chart-table having merged)
    - `crates/mesh/src/cert.rs`: `cert_cone` rewritten, `cone_chord_sag`, module/scale docs, 3 rows. `chart_table.rs`: `lattice_rows` + its row. `trimmed.rs`: rows from `lattice_rows`, `uniform_candidates` → `lattice_candidates(u, nu, rows)` (same arithmetic for the other arms), module docs. `lib.rs`, `sizing.rs`, `curved.rs`: doc sentences only. `tests/reach_probes.rs`: the pinning row is `a_cone_face_ending_near_its_apex_meshes_at_every_delta`; `MIN_SHAPE`'s note. `.config/nextest.toml`: the renamed slow-set entry, times of the three cone rows. `work/tesslane/a-needle-…md`: re-measured table; `one-element-grid-axes-drop-schedule.md`: the renamed symbol.
    - Moved: every trimmed cone face's mesh (rows; ≤ 2× the row count). No pinned count in mesh, sweep or the tour changed; nothing on another chart or lane. Detail and why-right in the PR body.
    
    **4. PR body**
    =====
    ## The cone arm certifies the triangle it emits, and a face ending near its apex meshes at every δ
    
    Closes `work/tesslane/a-cone-face-ending-near-its-apex-refuses-at-fine-delta.md`.
    
    ### Measurement
    
    The unit funnel cut by the tilted plane through `(0, h, 0)`, the half above the cut, δ `2e-2 … 5e-5` (the filed table, which reproduces to the digit under the old certificate):
    
    | h | old: refuses at δ (bound) | flat triangle's true deviation, same mesh | now |
    |---|---|---|---|
    | `1e-2` | `1e-3 … 5e-5` (`1.25δ … 1.49δ`) | within δ | meshes |
    | `3e-3` | `1e-3 … 5e-5` (`1.67δ … 4.0δ`) | within δ, but `5e-5`: **1.63δ** | meshes |
    | `1e-3` | `5e-3 … 5e-5` (`1.04δ … 6.1δ`) | within δ, but `5e-5`: **1.23δ** | meshes |
    | `1e-4`, `1e-6` | `5e-3 … 5e-5` (`1.03δ … 8.0δ`) | within δ | meshes |
    
    - **The refused triangles** (`h = 3e-3`, δ `1e-3`) join two adjacent points of the first lattice row (`v = 0.0775`) to a section-ellipse point (`v = 0.005`), or one row point to two ellipse points, `0.45 … 0.6` rad apart in `u`: near in the development, wide in the chart. The old bound charges the triangle's whole azimuth span at its widest slant, `1.0δ … 1.67δ`; those triangles deviate `0.14δ … 0.27δ`. The filed mechanism holds.
    - **Two cases are truly over δ**, by the sampled distance and by the exact bound alike (`1.63δ`/`1.67δ` and `1.23δ`/`1.28δ`): triangles on two row points at `v = 0.020` and an ellipse point at `v = 0.0046`, `0.38` rad round the axis. The development's Delaunay triangulation is the surface's own, and it prefers an ellipse point farther out and farther round to the one on the row point's ruling, because nothing stands between the small end and the first row.
    - **The filed row has `h = 1e-6` unmeasured**; with the certificate zeroed those cases mesh and pass the geometry check.
    - **Certificate-driven refinement does not cure the two.** Inserting each exceeded triangle's developed centroid: `191` triangles over at `1.67δ` → `165` at `12.4δ` → `106` at `4.9δ` → `84` at `7.9δ` → `26` → `17` → `2` at `2.5δ`, refused at the round budget; the other case clears on the last round after `16.6δ`. A centroid is one more off-lattice point.
    
    ### The change
    
    **`cert::cert_cone` is the flat triangle's own deviation, in closed form.** For corners on one nappe at radii `ρᵢ` and azimuths `uᵢ`, `max(D₁₂, D₂₃, D₃₁, A)` with `Dᵢⱼ = cos α·2ρᵢρⱼ·sin²(Δu/2)/(ρᵢ + ρⱼ + 2√(ρᵢρⱼ)·|cos(Δu/2)|)` and `A = sin α·`(the height at which the axis passes through the triangle). The derivation is beside the function: (1) `f = sin α·h − cos α·ρ ≥ dist(p, cone)` on the triangle, by convexity of the solid nappe; (2) `f` is affine along rays from the point where the triangle's plane meets the axis, so it peaks on an edge or on the axis; (3) on a chord `f = cos α·(ρ̄ − ρ)` is concave with that closed-form maximum; (4) the axis term in barycentric weights. It assumes the corners are chart points of one nappe (both nappes: `+∞`; NaN poisons). It reads no chart box and has no `Δu ≤ π` precondition. At one radius `D` is the rim chord's sagitta `cos α·ρ·(1 − cos(Δu/2))`, and it never exceeds the larger radius's, so `sizing::cone_grid_step`'s theorem and the curved lane's no-rows argument read the same number as before.
    
    **The cone arm's rows are uniform in `√|v|`** (`AnalyticRow::lattice_rows`), at the lattice's step: the gap at each radius is the chord the lattice's target allows there (`hv·√(v/v_w)`, the present `hv` at the widest radius, to second order in the step), and no row lies within the radius at which every chord certifies (`2·cos α·ρ ≤` the lattice's own sag). Chosen over refinement by the measurement above, and over rows uniform in `v` because those hold the chord rule at the widest radius alone. At most twice the uniform row count. No constant is introduced; `[mesh.sizing]` is unchanged, and the cone arm still carries `UNPROVED_HEADROOM`: the rows shape the triangulation, and nothing yet bounds a triangle beside the boundary (`one-containment-lemma-for-an-oblique-boundary`).
    
    **Which geometry each decision reads on the cone arm.** Triangulation: the development (unchanged). Lattice: the chart's columns, uniform in `u`, where the cell bound's azimuth step lives; rows a 3-D chord apart at their own radius. Certificate: the flat triangle in space, through its chart corners. Fold census: the triangle's 3-D normal against the surface's (unchanged). Ear test: the polygon's order, its cure the developed centroid (unchanged).
    
    ### What moved, and why the new answer is right
    
    - **Every trimmed cone face's mesh**: its rows. The funnel half above `h = 0.02`: `240 → 348` triangles at δ `2e-2`, `3380 → 5612` at `1e-3`; above `h = 3e-3` at `5e-5`, `63 672` (over δ) `→ 120 268` (within δ, sampled). Each is certified by the exact bound and passes `reach_probes::geometry` (volume, winding, sampled distance, not on one line).
    - **`reach_probes::a_cone_face_ending_near_its_apex_refuses_at_fine_delta` is `…_meshes_at_every_delta`**: every half that is built meshes at every δ, at all three ε.
    - **The needle row's numbers** (`a-needle-cone-face-meshes-thin-triangles-beside-its-rulings.md`, re-measured in the file): faces ending near an apex read rounder (`3.1e-7 → 8.5e-6`, `9.6e-7 → 1.4e-5`); a needle's face *through* its apex reads thinner (`1.6e-7 → 7.9e-9` at δ `1e-5`, `1.6e-8 → 2.5e-10` at `1e-6`), because the triangles beside a ruling fan from its far end to consecutive first-column points and graded rows stand closer near the apex. Certified and within δ; the row stays open.
    - **Nothing else**: no D9 golden, no `docs/tess-budget-data` byte, no cylinder, sphere, torus, spindle or spline face, no rectangle-lane cone face (the certificate only gates there, and it is at most the old bound). The tour's suite and `sweep`'s are green with no re-pin.
    
    ### Rows, and the mutants each catches
    
    | mutant | red |
    |---|---|
    | a chord's sag halved | `cert_cone_bounds_the_true_deviation_on_triangles_built_to_break_it`, `…is_attained…`, `a_cone_chord_sags_within…`, `chart_table`'s `the_cone_and_torus_certificates_hold_the_true_deviation`, `sizing`'s `cone_grid_step_lands_the_worst_triangle_at_delta` |
    | the axis term dropped | the break-it sweep, the attained row |
    | `cos` for `|cos|` (past half a turn) | the attained row |
    | the widest slant charged on every chord (a bound that only dominates) | the attained row; the near-apex mesh row |
    | no both-nappes guard; no NaN guard | `a_cone_chord_sags_within_its_larger_radius_sagitta` |
    | the cone's rows uniform in `v` | `the_cone_arms_rows_step_evenly_in_the_root_of_the_slant`; the near-apex mesh row (`h = 3e-3`, δ `5e-5`) |
    | uniform at the graded count; half the graded rows; no floor; the floor doubled | the rows row |
    
    The break-it sweep (seeded fuzz, shape 1): needle to near-flat cones, either nappe, the filed table's shape to slant ratio `1e-6`, needles, an apex corner in any slot, triangles about the axis, any three azimuths over a turn and a half; the oracle is the exact distance to the double cone on a 325-point grid. `3.8·10⁶` triangles over four seeds at raised effort, none past its certificate. Tightness: the bound equals the sampled maximum to `1e-12` on the attained row's poses (a chord at one radius and at radii `1e-3` apart across `0.4`, `2.5`, `π`, `4` rad, `α = 0.3`, `1.2`; a triangle about the axis at `α = 0.4`).
    
    **A finer δ never refuses where a coarser one meshes, on this family**: tilts `0.1, 0.3, 0.6, 0.75` × `h = 0.3 … 1e-6` (nine) × both halves × δ `2e-2 … 1e-5` (six): all 432 mesh and pass the geometry check (a one-off run, not a row).
    
    **What the table does not witness**: with half the graded rows (the uniform count, graded) the table still meshes. The count is the chord rule's, held by the rows row, not by a mesh.
    
    ### Sweep (§5)
    
    Shape: *a certificate charging a triangle's chart box at its widest lever*, and *a ruled lattice laid at one lever for the whole face*. Grepped `fn cert`, extents read off `uv` corners, `lever`/`lattice_steps`, in `crates/mesh/src`.
    
    - `cert::cert_cone`: fixed.
    - `cert::cert_torus`, `cert_spindle`, `NurbsCellGrid::cert`, `NurbsFaceBound::cert`: box bounds; not this unit: each arm's CDT runs in an affine image of its chart, so what is near in its plane is near in its chart.
    - `cert_cylinder`, `cert_sphere`: read the 3-D triangle, no box.
    - `AnalyticRow::lattice_steps`, cone, trimmed lane: fixed (`lattice_rows`). Curved lane (`curved::grid_counts`): not this unit, and not to move: its triangles are its strips', which its theorem bounds. Cylinder: one radius, so one lever is exact.
    
    What the pattern cannot match: an arm whose triangulation plane disagrees with what its lattice or certificate reads without spelling an extent or a lever. Second pass: the `CdtPlane` construction sites (the development on the cone alone), and the mirror case on the sphere arm, whose chart spreads one neighbourhood of a pole over every `u`: a unit ball cut by a plane tilted `0.3` through a point `h = 0.3 … 1e-4` under its pole, both parts, δ `2e-2 … 5e-5`: 56 of 60 mesh and pass the geometry check, and the four at `h = 1e-4`, δ `≤ 2e-4` refuse `SphereTrimShape { ChordBudget }`, the stated budget (`MAX_SPHERE_EDGE_CHORDS`). No instance.
    
    ### Filed
    
    Nothing new. `a-needle-cone-face-meshes-thin-triangles-beside-its-rulings.md` carries its re-measured table and why it moved.
    =====
    
    **5. Ran locally** (final head, after the last merge of main, unless noted)
    - `bash scripts/run-gates.sh`: exit 0 (first run was red on two gates in my own lines, a bare `f64::EPSILON` in a test and `sin * sin`; both re-posed).
    - `scripts/doc-gate.sh --workspace-pass --skip-viewer-toolkit`: OK (before the last merge). `cargo fmt --all --check`: clean.
    - clippy `-p mesh -p geom-brep --all-targets --all-features -D warnings`: clean; `demos/tour` clippy: clean (before the last merge); mesh `--features budget` clippy + `cargo test`: 208 + 264 passed (before the last merge).
    - `cargo nextest run -p mesh` WHOLE: default 466/466 (final head); 1e-6 466/466 and 1e-12 466/466 (before the last merge); the cone and certificate rows again at 1e-6 and 1e-12 on the final head: 23/23 each.
    - Tour: `cargo nextest run --cargo-profile ci-tour --profile ci`: 97/97 (its 9 nightly-only rows not run). Sweep: `--profile ci` 2939/2939 at default ε; its cone rows (`reach_cone_split`, `cone_join_lane`, `a_ring_on_a_cone_face`, `a_conical_ended…`, `pinch_faces_tessellate`) 24/24 at each of default, 1e-6, 1e-12. (Tour and sweep before the last merge.)
    - Slow set: `a_cone_face_ending_near_its_apex_meshes_at_every_delta` 5.98 s, `…through_its_apex…every_scale_and_pose` 2.98 s, `…section_ellipse…` 1.32 s (alone, 4 vCPU). New fast rows: 0.28 s, &lt; 0.01 s ×3.
    - Left to CI: everything on hosted hardware; sweep's slow set and the rest of the workspace at 1e-6/1e-12; the nightly-only tour rows; `demos/wild`; k-lint; renders.
    
    **6. Head**: `6916292d76` (pushed).
    
    **7. For you to weigh / what slowed me**
    - **Row count is the chord rule's, not witnessed by a mesh**: with half the graded rows (the uniform count, graded) the whole filed table still meshes. I kept the derived rule (gap = the target's chord at that radius) rather than a tuned factor; it costs up to 2× triangles on a trimmed cone face reaching its apex (about 1.2× on a 2:1 frustum). If "as coarse as possible" should win, the derivable alternative is the same rule at δ instead of the lattice target (×0.71 rows); that is a `[mesh.sizing]`-flavoured choice, so I did not make it.
    - **A needle's face through its apex meshes thinner triangles than before** (2.5e-10 at δ 1e-6, was 1.6e-8; `MIN_SHAPE` is 1e-12). Cause and numbers are in the needle row. Not a soundness matter, but it is closer to the probes' floor.
    - I did not close `a-cone-face-ending-near-its-apex-refuses-at-fine-delta.md` (status left open; the unit says it closes it) and did not touch the unit's item or `log.md`.
    - Blinding: none of my commits carries a trailer or a session line. The container's git identity is the author name on them, as on every earlier commit of the branch; I did not change it. The dispatch commit 233ea49097 (not mine) carries a trailer.
    - Slow: one mesh suite pass is ~220 s and the brief asks three eps plus budget, tour (15 min compile) and sweep; two gate rules surfaced only at the first full gates run; main moved 132 commits between my verification and push, costing a second pass.
</cross-session-message>

0 notifications remain queued.
