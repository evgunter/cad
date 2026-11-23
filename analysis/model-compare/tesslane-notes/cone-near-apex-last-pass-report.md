REPORT 3 (part 1 of 2) — cone-near-apex, the last small pass. Pushed head `14ef576db0159277456f029553122e13dffdf5d0`, confirmed with `git ls-remote origin refs/heads/tesslane/cone-near-apex`. Part 2 is the whole PR body.

**The branch moved under me.** My push of the last pass was refused: you had pushed onto the branch (a merge of main, `1304b9a1e7` the closing record, `b6ca992b45` the model-compare row). I merged those in (`14ef576db0`), no conflict, no source of `crates/mesh`, `tools/tess-meter` or the viewer in that merge. Your closing record is in the head unchanged.

**A. After main's ring unit (e3f8adfcad onward).**
- The gate: yes, `topology-reads-through-topo.sh` refused `once(face.outer).chain(face.rings…)`. `faces_across` now reads `body.face(fk).loops()`. `run-gates.sh` exit 0 on the final head.
- The rule on a ringed face: the polygon is every loop; "neighbours" is `TrimPolygon::next` on a point's own loop, and the neighbour map takes every loop's edges, so a ring's chain across one plane face and a side from a ring point to an outer point both read the same way: flagged where one face is across both ends' edges. I did not count how often it fires on a ringed face; what I measured is the outcome.
- Outcome, 2026-11-16: all six ops of the wedge poses (s `0.6 … 1e-3` × xf `0.5, 0.8, −0.5, 0`) at δ `2e-2, 5e-3, 1e-3`, 432 meshes through `check_mesh`: **0 non-manifold**. 108 have a ringed cone face: 90 mesh and pass `check_mesh` and `geometry`, 18 refuse `DomainUnread` (main's `s = 1e-3` answer). Unit 3's `a_cone_wall_holding_a_ring_meshes_on_the_cone_row` is green at three ε.
- **A new finding in that tally, not this unit's, filed:** 9 of the 324 without a ringed cone face (xf `0`, s `0.6` and `0.3`, both unions and `A ∖ B`) pass `check_mesh` and fail `reach_probes::geometry`: a cone triangle wound against the face, each with a corner at a low shared id. The same nine answer the same with the chart-box bound, uniform rows and the ear rule put back (main's three, emulated in my tree; main itself not rebuilt). Cause not measured: a fold the winding census misses, or the probe's own normal at a centroid beside the axis. `work/tesslane/a-cone-face-met-on-its-axis-meshes-a-triangle-wound-against-it.md`. Its priority is yours.
- The two re-pins. `a_ringless_cone_face_near_its_apex_meshes_sound` (was `…_meshes_an_edge_of_four`): `Meshed` in both orders, i.e. `check_mesh` and `geometry` pass; right because the cone face no longer keeps the chord its plane neighbour also makes, which is the row's own stated re-pin. `ringed_curved_faces_that_still_refuse_answer_by_cause`, the cone cells at s `0.1, 0.03, 0.01`: `Meshed` at all three δ in the three ringed ops; right because the refusal was the chart-box bound charging a span the triangle does not sag by, and the meshes pass the sampled distance check. s `1e-3` unchanged.

**B and C, each taken.** 1 the Rounding paragraph says the chord term holds to its own rounding for `|Δu|` up to a turn less a few `1e-8`, gives both reviewers' breaks, and says where each lane keeps a side under that (one column step on the curved lane; one chart box within a turn on the trimmed lane, by `DomainUnread` and `SeamTwice`); arithmetic unchanged. 2 and 3 on `work/tesslie/a-cone-face-near-its-apex-…md` (cure, dated counts with builders, the census still owed, the corner-only gap), and the ear row says the class reaches past the ear. 4 the heading reads "the cone arm; the other analytic arms read its nearest case, the ear". 5 no door reaches the seam arm on an accepted cone face: a seam walked twice has refused `SeamTwice` in `singular_row_shape` first; the comment says so (I cite that refusal, which I read in the code, not the spindle row). 6 done. 7 done. 8 split: `the_cylinder_and_sphere_certificates_read_a_nan_as_zero` holds the pin, and the other row is renamed `the_cone_torus_and_spindle_certificates_poison_on_an_input_that_is_not_finite`. 9 the doc's sentence was the wrong one; it now says that face has no theorem, its certificate holds it, and its rows are left alone.
10, a line each, nothing changed: `BoundaryEar` naming two conditions by arm: agreed it is one name for two tests, kept because both are "a triangle a neighbour may also make" and a second variant would grow a public error type for one arm; rename when the rule is lane-wide. `across.map_or_else(ear, shared_side)`: it is "is this the cone arm" spelled as "does this face carry the map", kept because the map's presence is exactly what the rule needs and a second flag could disagree with it. The metric `1.0` on the development: a placeholder for a field the cone's plane never reads; an `Option` would be truer and touches every arm's constructor, so I left it with its comment. The needle pin's table: indexed by the δ's place and panics on a δ it does not list.

**D. Ran on the final content.** On the pushed head `14ef576db0`: fmt, clippy mesh + geom-brep `--all-features`, `run-gates.sh` exit 0, tracker lint, mesh WHOLE at default ε 479/479. On `8146f62c14` (same sources; the two later merges bring only your records and 15 unrelated commits of main): mesh WHOLE 479/479 at 1e-6 and at 1e-12; the meter job's two steps (`clippy --features budget`, `cargo test --features budget`: 213 + 272); `tools` k-lint, tess-lint, tess-meter each fmt + clippy + tests; viewer clippy `--features app`; `cargo nextest run --profile ci -p viewer --features app`: 1551 of 1554, the three failures each panicking "NO WGPU ADAPTER" / "no wgpu adapter" (read from their output this time; CI installs lavapipe); sweep `--profile ci` 2957/2957 and its cone rows 24/24 at three ε; tour clippy and `ci-tour` suite 97/97; doc-gate OK. Slow-set check: the axis row is 0.14 s here at 2500 draws (0.32 s here when the runner read 0.637 s), so about 0.28 s there by that ratio; I did not run `check-fast-set-times.py`, which reads the runner's own times.
**Not run:** hosted CI; the wasm32 clippy step of the viewer job; `demos/wild`; the nightly-only rows.

REPORT 3 (part 2 of 2) — the whole PR body for PR 6965, head `14ef576db0`.

=====
## The cone arm certifies the triangle it emits; the unit funnel cut near its apex meshes at every δ; a cone face keeps no side its neighbour can make

Closes `work/tesslane/a-cone-face-ending-near-its-apex-refuses-at-fine-delta.md`.

### Measurement

The unit funnel cut by the tilted plane through `(0, h, 0)`, the half above the cut, δ `2e-2 … 5e-5` (the filed table, which reproduces to the digit under the old certificate):

| h | old: refuses at δ (bound) | flat triangle's true deviation, same mesh | now |
|---|---|---|---|
| `1e-2` | `1e-3 … 5e-5` (`1.25δ … 1.49δ`) | within δ | meshes |
| `3e-3` | `1e-3 … 5e-5` (`1.67δ … 4.0δ`) | within δ, but `5e-5`: **1.63δ** | meshes |
| `1e-3` | `5e-3 … 5e-5` (`1.04δ … 6.1δ`) | within δ, but `5e-5`: **1.23δ** | meshes |
| `1e-4`, `1e-6` | `5e-3 … 5e-5` (`1.03δ … 8.0δ`) | within δ | meshes |

- **The refused triangles** (`h = 3e-3`, δ `1e-3`) join two adjacent points of the first lattice row (`v = 0.0775`) to a section-ellipse point (`v = 0.005`), or one row point to two ellipse points, `0.45 … 0.6` rad apart in `u`: near in the development, wide in the chart. The old bound charges the triangle's whole azimuth span at its widest slant, `1.0δ … 1.67δ`; those triangles deviate `0.14δ … 0.27δ`.
- **Two cases are truly over δ** (sampled `1.63δ`, `1.23δ`): two row points at `v = 0.020` and an ellipse point at `v = 0.0046`, `0.38` rad round the axis. Nothing stands between the small end and the first row.
- **One centroid a round does not finish inside `MAX_GRID_RETRIES`.** Inserting each exceeded triangle's developed centroid: `191` triangles over at `1.67δ` → `165` at `12.4δ` → `106` at `4.9δ` → `84` at `7.9δ` → `26` → `17` → `2` at `2.5δ`, stopped at the round budget; the other case clears on the last round after `16.6δ`. Whether more rounds would end it is not measured.
- **The cone ∩ wedge slivers** (the T1 cone met by a box edge `1.5·s` under its apex): `tessellate` answers `Ok` and `check_mesh` refuses `NonManifoldEdge { count: 4 }`. The body is sound. The edge joins two points of one section-ellipse chain four or five chord steps apart; the plane face across that chain, whose triangulation has no interior point, uses it as a diagonal, and so does the cone face. It is the class of `work/tesslie/two-faces-cutting-one-ear-…`, one step past its ear, so the ear test cannot fire. Over 288 such meshes (s `0.6 … 1e-3` × xf `0.5, 0.8, −0.5, 0` × both orders × δ `2e-2 … 5e-4`): 20 under rows uniform in `v`, 52 under this PR's graded rows, every one an edge used twice by the plane patch and twice by the cone patch.

### The change

**`cert::cert_cone` is the flat triangle's own deviation, in closed form**: `max(D₁₂, D₂₃, D₃₁, A)`, `Dᵢⱼ = cos α·2ρᵢρⱼ·sin²(Δu/2)/(ρᵢ + ρⱼ + 2√(ρᵢρⱼ)·|cos(Δu/2)|)` a chord's largest sag, `A = sin α·`(the height at which the axis passes through the triangle). Derivation beside the function: `f = sin α·h − cos α·ρ ≥ dist` on the triangle (convex solid nappe); `f` affine along rays from the plane's axis point, so it peaks on an edge or the axis; the chord's concave maximum in closed form; the axis term in barycentric weights `sin(uₖ − uⱼ)`. It reads no chart box and never exceeds the old bound. Each term is a quotient of sums of one sign; the weights are read to the rounding of their own value (`sin_between`: the error-free difference, then `sin d + e·cos d`), since a bare `sin(uₖ − uⱼ)` near π loses up to seven digits and the term then under-reports. An input that is not finite answers NaN; corners on both nappes `+∞`. The chord term reads the rounded `Δu`, which holds to its own rounding for `|Δu|` up to a turn less a few `1e-8`; the doc says so and where each lane keeps a side under that.

**A face's worst bound keeps a NaN once met** (`cert::worse`), on both lanes: the old fold let the next finite bound overwrite a NaN, so a face refused for a NaN reported a finite bound under its δ. No mesh or verdict moves.

**The cone arm's rows are uniform in `√|v|`** (`AnalyticRow::lattice_rows`): the gap at each radius is the length of the chord the lattice's target allows along the surface there (`hv` at the widest radius), and no row lies where every chord certifies (`2·cos α·ρ ≤` the lattice's sag). Proved: a chord of that length sags within the lattice's sag to second order, up to twice it half a turn round. The rule's choice: laying that gap between rows; nothing bounds a triangle beside the boundary, so the arm keeps `UNPROVED_HEADROOM` and the certificate is the guarantee. At most twice the uniform count. The curved lane's split-side cone keeps its rows uniform in `v`: it has no worst-triangle theorem, its certificate holds it, and this PR leaves it alone; both lanes read the one row step.

**The cone arm keeps no triangle side a single neighbouring face holds both ends of**, unless the ends are neighbours on the trim polygon (`trimmed::faces_across`, the shared-side cure in `attempt`): such a triangle's inside point is inserted and the face rebuilt, and the budget's refusal is the existing `BoundaryEar`. The ear is its nearest case, and on the cone arm this one rule replaces it; the other analytic arms still read the ear. Every loop of the face is read, a ring as the outer loop, through `Body::face(k).loops()`. A seam (this face across its own edge) is left out; no accepted cone face reaches that arm, since a seam walked twice has refused `SeamTwice` first. Not in the map, and unwitnessed: a third face that holds both ends of a side but meets the cone face at corners only.

**Which geometry each decision reads on the cone arm.** Triangulation: the development. Lattice: the chart's columns, and rows a 3-D chord apart at their own radius. Certificate: the flat triangle in space, through its chart corners. Fold census: the triangle's 3-D normal. Shared sides: the polygon's order and the body's adjacency.

### The three forms of the shared-side rule, measured (2026-11-14, debug, default ε)

| set (meshes) | ear only | any two non-neighbour boundary points | two points of one edge's chain | **one neighbouring face holds both** |
|---|---|---|---|---|
| wedge grid (288): non-manifold / retry rounds / cone triangles | 52 / 192 / 119 016 | 0 / 192 / 120 464 | 0 / 80 / 118 928 | **0 / 80 / 118 928** |
| thin bands (32) | 0 / 60 / 13 148 | 0 / 142 / 24 112 | 0 / 0 / 12 944 | **0 / 0 / 12 944** |
| needles (12) | 0 / 0 / 1 019 876 | same | same | **same** |
| funnels and a frustum (40) | 0 / 2 / 173 976 | 0 / 2 / 173 992 | 0 / 0 / 173 972 | **0 / 0 / 173 972** |

No form refuses a mesh in these sets. "Any side" nearly doubles a thin band's triangles. The one-chain and one-face forms agree here: no body in the sets has two edges of one loop across one face, the case that separates them; that case is argued, not witnessed.

### What moved, and why the new answer is right

- **Every trimmed cone face's mesh**: its rows, and where it kept a shared side or an ear at a vertex between two neighbours. The funnel half above `h = 0.02`: `240 → 348` triangles at δ `2e-2`; above `h = 3e-3` at `5e-5`, `63 672` (over δ) `→ ~120 000` (within δ, sampled). Each is certified by the exact bound and passes `check_mesh` and `reach_probes::geometry`.
- **A needle funnel (rim `0.02`) cut `1e-3` above its apex** refused `FoldedTriangle` at δ `1e-3`, `2e-4` and `3e-5` on the merge base and now meshes at all three (the ear cure no longer splits ears no neighbour can cut). Cut `1e-5` above it still refuses (`FoldedTriangle`, `BoundaryEar`): pinned and filed.
- **Ringed cone faces near the apex** (main's ring unit): T1's box at `s = 0.1, 0.03, 0.01`, the three ringed ops, refused `CertificateExceeded` under the chart-box bound at the finer δ (at every δ by `s = 0.01`) and mesh at all three δ, passing `check_mesh` and the geometry check; `ringed_curved_faces_that_still_refuse_answer_by_cause` is re-pinned for them, and `s = 1e-3` still answers `Unread(Cone, "undecided")`. Main's pin of the four-use edge, `a_ringless_cone_face_near_its_apex_meshes_an_edge_of_four`, is `…_meshes_sound`: `Meshed` in both orders.
- **The needle row's shape numbers**: re-measured in its file; a needle's face through its apex reads thinner (`1.6e-8 → 2.5e-10` at δ `1e-6`), faces ending near an apex rounder.
- **Nothing else.** By a reviewer's digest of every patch of `mesh::all` on the merge base and on the first review head: cylinder, sphere, torus, spindle and spline patches, and curved-lane cone patches, are identical sets; trimmed cone patches all differ.

### Rows

- `cert`: `cert_cone_bounds_the_true_deviation_on_triangles_built_to_break_it` (fuzz, six shapes, exact distance on a grid); `…is_attained_on_a_chord_and_over_the_axis`; `…reads_the_axis_term_to_its_own_rounding` (three poses against 300-bit values to `1e-12`: the bare sine reads them `1.1e-8` and `5e-9` short and 14 % over; then two corners `1e-12 … 1e-4` rad apart with the third near opposite, `|u|` to 700, radii `1e-3 … 1e6`, against a geometric oracle that takes azimuth differences by integer arithmetic and searches each side and the axis, sharing the premise that the maximum is there and none of the closed forms, to `1e-9` either way); `the_cone_torus_and_spindle_certificates_poison_on_an_input_that_is_not_finite`; `the_cylinder_and_sphere_certificates_read_a_nan_as_zero` (a pinned gap, filed); `a_cone_chord_sags_within_its_larger_radius_sagitta`.
- `chart_table`: `the_cone_arms_rows_stand_a_certified_chord_apart` reads geometry, not the rule's arithmetic: a chord no longer than a gap sags within its share of the lattice's sag; no coarser row set does; every chord from under the floor certifies.
- `reach_probes`: `the_unit_funnel_cut_near_its_apex_meshes_at_every_delta`; `a_cone_face_keeps_no_side_its_plane_neighbour_can_make` (twelve slivers × three δ, `check_mesh` and `geometry`; at ε `1e-6` six are built, at `1e-12` two, the rest refused by the boolean and printed); `a_needle_funnel_cut_at_its_apex_refuses_typed`; `a_ringless_cone_face_near_its_apex_meshes_sound`.

| mutant | red |
|---|---|
| a chord's sag halved | five rows of `cert`, `chart_table`, `sizing` |
| the axis term dropped; `cos` for `|cos|`; the widest slant charged | the break-it sweep and/or the attained row; the last also the funnel row |
| a bare `sin(uₖ − uⱼ)` | the axis-rounding row |
| `f64::max` for the NaN-keeping fold | the non-finite row |
| a clamp that poisons on the cylinder or sphere | the cylinder-and-sphere pin (and `tools/tess-meter`'s `r2_wedge_bytes`) |
| the cone's rows uniform in `v` | the rows row; the funnel row (`h = 3e-3`, δ `5e-5`) |
| uniform at the graded count; half the rows; twice the rows; no floor; the floor doubled or halved | the rows row |
| the cone arm reads the ear alone | the sliver row |
| neighbours on the polygon flagged too | the sliver, funnel and through-apex rows |

Not held by a mesh row: the axis term (no lane emits a triangle about the axis: a face holding its apex inside refuses) and the row count (the funnel table still meshes at half the rows; the rows row holds the count).

### Measurements that are not rows

- 2026-11-16, after main's ring unit: all six ops of the wedge poses (s `0.6 … 1e-3` × xf `0.5, 0.8, −0.5, 0`) at δ `2e-2, 5e-3, 1e-3`, 432 meshes: none non-manifold. Of the 108 with a ringed cone face, 90 mesh and pass `check_mesh` and the geometry check and 18 refuse `DomainUnread` (main's `s = 1e-3` answer). Of the 324 without, 315 pass and 9 fail the geometry check on a triangle wound against the face (xf `0`, s `0.6, 0.3`, both unions and `A ∖ B`): the same nine with the chart-box bound, uniform rows and the ear rule put back, so not this PR's; filed.
- 2026-11-14: tilts `0.1, 0.3, 0.6, 0.75` × `h = 0.3 … 1e-6` (nine) × both halves × δ `2e-2 … 1e-5` (six) of the unit funnel: all 432 mesh and pass the geometry check, so on that grid no finer δ refuses where a coarser one meshes. It is the funnel's property: the needle above refuses.
- No pinned triangle count in `mesh`, `sweep` or the tour moved: read off green suites, not off a diff of pins.

### Sweep (§5)

Shape: *a certificate charging a triangle's chart box at its widest lever*; *a ruled lattice laid at one lever for the whole face*; *a `max`/`min` fold or a clamp that drops a NaN in a certificate*. Grepped `fn cert`, extents read off `uv` corners, `lever`/`lattice_steps`, `.max(`/`.min(` in `cert.rs`.

- `cert_cone`: fixed (box, fold, non-finite).
- `cert_torus`, `cert_spindle`, the spline cell certificates: box bounds, not this unit: each arm's CDT runs in an affine image of its chart. Non-finite inputs answer NaN or `+∞` (pinned).
- `cert_cylinder`, `cert_sphere`: read the 3-D triangle. Their `.max(0.0)` reads a NaN as 0, and one arises on a sound cylinder triangle with a side on a ruling. Not fixed: poisoning it refuses a rectangle-lane face that meshes on main. Pinned as today's answer and filed.
- The worst-bound fold of both lanes (`curved`, `trimmed`): fixed (`cert::worse`).
- `position_scale`, `uv_scale`, `cone_scale`: folds over magnitudes for the verdict's allowance, no bound.
- `lattice_steps`, cone: the trimmed lane's rows fixed; the curved lane's split-side cone rows stay uniform, held by the certificate (it has no theorem: `a-cone-face-with-a-split-side-has-no-worst-triangle-theorem`). Cylinder: one radius.

What the patterns cannot match: an arm whose triangulation plane disagrees with what its lattice or certificate reads without spelling an extent or a lever. Second pass: the `CdtPlane` sites (the development on the cone alone), and the mirror case on the sphere arm near a pole (a unit ball cut by a tilted plane `0.3 … 1e-4` under its pole, 60 meshes: 56 mesh and pass geometry, four refuse the stated `ChordBudget`).

### The other arms

They still read the ear alone, and the same defect is there: 12 of 120 cylinder slivers and 1 of 108 sphere slivers are non-manifold today. The counts, their builders, what this PR cures and leaves, and the rule's corner-only gap are on `work/tesslie/a-cone-face-near-its-apex-meshes-an-edge-of-four-triangles.md`; the ear row says the class reaches past the ear.

### Filed

- `work/tesslane/a-needle-funnel-cut-at-its-apex-refuses-folded.md`
- `work/tesslane/the-cylinder-certificate-reads-a-ruling-sided-triangle-as-zero.md`
- `work/tesslane/a-cone-face-met-on-its-axis-meshes-a-triangle-wound-against-it.md`
- `a-needle-cone-face-meshes-thin-triangles-beside-its-rulings.md` carries its re-measured table.
=====
