REPORT 2 — cone-near-apex: the shared-side cure and the fix pass off both reviews. Head `26ce505802` (pushed, main merged). No unpushed work was lost in the outage: the tree was clean at `8f585255cf` and the container survived.

**The tools red (your interrupt).** Cause: not the cone. In the fix pass's item 3 I made `cert_cylinder`/`cert_sphere` answer NaN where `r − dist` is not finite. `r2_wedge_bytes`' frustum wedge has a cylinder face whose half-cell with a side on a ruling projects across the axis onto a segment; `dist_point_triangle` answers NaN on that triangle of no area, and main's `.max(0.0)` reads it as 0. So main certifies a sound triangle (true deviation 0.0341, its sibling's) at 0, and my poisoning refused the face. Reverted: both clamps are byte-for-byte main's. Held in the crate now by `cert::tests::no_certificate_reads_an_input_that_is_not_finite_as_a_bound`, which pins that exact triangle at 0 and its sibling at the sagitta (a poisoning clamp reds it), and filed: `the-cylinder-certificate-reads-a-ruling-sided-triangle-as-zero.md` (an under-reporting certificate on main; whether any accepted mesh is over δ by it is not measured). `tools/{k-lint,tess-lint,tess-meter}` fmt, clippy and tests are in my list and green.

**Fix pass, by item.** 1 built (below, PR body). 2 reproduced (60.53414746948693 against 60.534147538493464) and cured at the term: `sin_between` takes the azimuth difference error-free and adds `e·cos d`; all three reviewer poses now match their 300-bit values to 1e-12, and the family sweep holds 1e-9 either way against an oracle that shares no formula (integer-exact differences, plane-meets-axis by cross products, ternary search on the chords). `cone_scale`'s doc now says what it covers (`|u|·Δu ≤ 32π`), value unchanged. 3 done for the cone (ten pins); torus and spindle already answer NaN/+∞ (pinned); cylinder/sphere hits reported above, not fixed, filed. 4 verified, with a change: under the shared-side rule the needle cut at `h = 1e-3` now meshes at all three δ; `h = 1e-5` refuses `FoldedTriangle`, `BoundaryEar`, `FoldedTriangle`. Pinned, filed, and the family row and PR title name the unit funnel. 5a the lever has one home (`lattice_rows` reads `v_w` off the handed steps) and both docs say which lane reads what; 5b one spelling, `cone_chord_sag`; 5c the rows row reads geometry (all row mutants red it); 5d the dead count is gone (the cone passes no metric); 5e the doc separates what is proved from the rule's choice, with `|cos|`. 6 and 7 in the PR body and files. Left as they are, with reasons: `root` signed (one formula for a window on either nappe, and one ending at the apex); the axis block gated on no apex corner (with one, the axis meets the triangle at that corner, where f = 0; comment added). The finite-input guard at the top of `cert_cone` is belt only: with it reduced to a NaN check every pin still passes, because the arithmetic poisons by itself.
**Asked:** a cheap mesh-level row for the axis term: none, no lane emits a triangle about the axis. For the floor: none cheap; the rows row holds it by geometry. `geometry`'s 6-grid can miss a chord's peak near a small end, yes; I did not change the probe (the certificate rows hold exactness).
**`reach_probes` and `check_mesh`:** every `tessellate` site in the file checks `check_mesh` before the mesh is read; `got` cannot answer `Meshed` on a mesh it refuses. Nothing to fix.
**One thing the rule changed that you should know:** the `h = 1e-5` needle's `BoundaryEar` at δ 2e-4 is a new refusal *kind* on a cell that refused `FoldedTriangle` before; no cell that meshed on main refuses in any set I ran.

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

**`cert::cert_cone` is the flat triangle's own deviation, in closed form**: `max(D₁₂, D₂₃, D₃₁, A)`, `Dᵢⱼ = cos α·2ρᵢρⱼ·sin²(Δu/2)/(ρᵢ + ρⱼ + 2√(ρᵢρⱼ)·|cos(Δu/2)|)` a chord's largest sag, `A = sin α·`(the height at which the axis passes through the triangle). Derivation beside the function: `f = sin α·h − cos α·ρ ≥ dist` on the triangle (convex solid nappe); `f` affine along rays from the plane's axis point, so it peaks on an edge or the axis; the chord's concave maximum in closed form; the axis term in barycentric weights `sin(uₖ − uⱼ)`. It reads no chart box and never exceeds the old bound. Each term is a quotient of sums of one sign; the weights are read to the rounding of their own value (`sin_between`: the error-free difference, then `sin d + e·cos d`), since a bare `sin(uₖ − uⱼ)` near π loses up to seven digits and the term then under-reports. An input that is not finite answers NaN; corners on both nappes `+∞`.

**The cone arm's rows are uniform in `√|v|`** (`AnalyticRow::lattice_rows`): the gap at each radius is the length of the chord the lattice's target allows along the surface there (`hv` at the widest radius), and no row lies where every chord certifies (`2·cos α·ρ ≤` the lattice's sag). Proved: a chord of that length sags within the lattice's sag to second order, up to twice it half a turn round. The rule's choice: laying that gap between rows; nothing bounds a triangle beside the boundary, so the arm keeps `UNPROVED_HEADROOM` and the certificate is the guarantee. At most twice the uniform count. The curved lane's split-side cone keeps rows uniform in `v` (its sides are iso lines); both read the one row step.

**The cone arm keeps no triangle side a single neighbouring face holds both ends of**, unless the ends are neighbours on the trim polygon (`trimmed::faces_across`, the shared-side cure in `attempt`): such a triangle's inside point is inserted and the face rebuilt, and the budget's refusal is the existing `BoundaryEar`. The ear is its nearest case, and on the cone arm this one rule replaces it; the other analytic arms still read the ear. A seam (this face across its own edge) is left out: that side is made once, by this triangulation.

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
- **A needle funnel (rim `0.02`) cut `1e-3` above its apex** refused `FoldedTriangle` at δ `1e-3` and `2e-4` on main and now meshes (the ear cure no longer splits ears no neighbour can cut). Cut `1e-5` above it still refuses (`FoldedTriangle`, `BoundaryEar`): pinned and filed.
- **The needle row's shape numbers**: re-measured in its file; a needle's face through its apex reads thinner (`1.6e-8 → 2.5e-10` at δ `1e-6`), faces ending near an apex rounder.
- **Nothing else.** By a reviewer's digest of every patch of `mesh::all` on the merge base and on the first review head: cylinder, sphere, torus, spindle and spline patches, and curved-lane cone patches, are identical sets; trimmed cone patches all differ.

### Rows

- `cert`: `cert_cone_bounds_the_true_deviation_on_triangles_built_to_break_it` (fuzz, six shapes, exact distance on a grid); `…is_attained_on_a_chord_and_over_the_axis`; `…reads_the_axis_term_to_its_own_rounding` (three poses against 300-bit values to `1e-12`: the bare sine reads them `1.1e-8` and `5e-9` short and 14 % over; then two corners `1e-12 … 1e-4` rad apart with the third near opposite, `|u|` to 700, radii `1e-3 … 1e6`, against a geometric oracle that takes azimuth differences by integer arithmetic, to `1e-9` either way); `no_certificate_reads_an_input_that_is_not_finite_as_a_bound`; `a_cone_chord_sags_within_its_larger_radius_sagitta`.
- `chart_table`: `the_cone_arms_rows_stand_a_certified_chord_apart` reads geometry, not the rule's arithmetic: a chord no longer than a gap sags within its share of the lattice's sag; no coarser row set does; every chord from under the floor certifies.
- `reach_probes`: `the_unit_funnel_cut_near_its_apex_meshes_at_every_delta`; `a_cone_face_keeps_no_side_its_plane_neighbour_can_make` (twelve slivers × three δ, `check_mesh` and `geometry`; at ε `1e-6` six are built, at `1e-12` two, the rest refused by the boolean and printed); `a_needle_funnel_cut_at_its_apex_refuses_typed`.

| mutant | red |
|---|---|
| a chord's sag halved | five rows of `cert`, `chart_table`, `sizing` |
| the axis term dropped; `cos` for `|cos|`; the widest slant charged | the break-it sweep and/or the attained row; the last also the funnel row |
| a bare `sin(uₖ − uⱼ)` | the axis-rounding row |
| `f64::max` for the NaN-keeping fold | the non-finite row |
| the cone's rows uniform in `v` | the rows row; the funnel row (`h = 3e-3`, δ `5e-5`) |
| uniform at the graded count; half the rows; twice the rows; no floor; the floor doubled or halved | the rows row |
| the cone arm reads the ear alone | the sliver row |
| neighbours on the polygon flagged too | the sliver, funnel and through-apex rows |

Not held by a mesh row: the axis term (no lane emits a triangle about the axis: a face holding its apex inside refuses) and the row count (the funnel table still meshes at half the rows; the rows row holds the count).

### Measurements that are not rows

- 2026-11-14: tilts `0.1, 0.3, 0.6, 0.75` × `h = 0.3 … 1e-6` (nine) × both halves × δ `2e-2 … 1e-5` (six) of the unit funnel: all 432 mesh and pass the geometry check, so on that grid no finer δ refuses where a coarser one meshes. It is the funnel's property: the needle above refuses.
- No pinned triangle count in `mesh`, `sweep` or the tour moved: read off green suites, not off a diff of pins.

### Sweep (§5)

Shape: *a certificate charging a triangle's chart box at its widest lever*; *a ruled lattice laid at one lever for the whole face*; *a `max`/`min` fold or a clamp that drops a NaN in a certificate*. Grepped `fn cert`, extents read off `uv` corners, `lever`/`lattice_steps`, `.max(`/`.min(` in `cert.rs`.

- `cert_cone`: fixed (box, fold, non-finite).
- `cert_torus`, `cert_spindle`, the spline cell certificates: box bounds, not this unit: each arm's CDT runs in an affine image of its chart. Non-finite inputs answer NaN or `+∞` (pinned).
- `cert_cylinder`, `cert_sphere`: read the 3-D triangle. Their `.max(0.0)` reads a NaN as 0, and one arises on a sound cylinder triangle with a side on a ruling. Not fixed: poisoning it refuses a rectangle-lane face that meshes on main. Pinned as today's answer and filed.
- `position_scale`, `uv_scale`, `cone_scale`: folds over magnitudes for the verdict's allowance, no bound.
- `lattice_steps`, cone: the trimmed lane's rows fixed; the curved lane's split-side cone rows stay uniform, held by the certificate (it has no theorem: `a-cone-face-with-a-split-side-has-no-worst-triangle-theorem`). Cylinder: one radius.

What the patterns cannot match: an arm whose triangulation plane disagrees with what its lattice or certificate reads without spelling an extent or a lever. Second pass: the `CdtPlane` sites (the development on the cone alone), and the mirror case on the sphere arm near a pole (a unit ball cut by a tilted plane `0.3 … 1e-4` under its pole, 60 meshes: 56 mesh and pass geometry, four refuse the stated `ChordBudget`).

### For TESSLIE's row: the other arms today (2026-11-14, not changed here)

Slivers of a rod, a ball and two torus shells against a tilted box, δ `2e-2 … 1e-3`: **cylinder** 120 meshes, 12 non-manifold (and 3 `Triangulation` refusals); of 180 accepted trimmed cylinder faces 14 keep a side one neighbour holds both ends of. **Sphere** 108 meshes, 1 non-manifold; 5 of 174 faces keep one. **Torus** 8 meshes, none; 0 of 12 faces. All 13 non-manifold meshes hold such a face.

### Filed

- `work/tesslane/a-needle-funnel-cut-at-its-apex-refuses-folded.md`
- `work/tesslane/the-cylinder-certificate-reads-a-ruling-sided-triangle-as-zero.md`
- `a-needle-cone-face-meshes-thin-triangles-beside-its-rulings.md` carries its re-measured table.
=====

**Ran on the final content** (before the last merge of main unless marked †, which ran after it): fmt†; clippy mesh + geom-brep `--all-features`†; viewer clippy (`-p viewer --features app`); tour clippy; mesh WHOLE default† 470/470, 1e-6 470/470, 1e-12 470/470 (cone, cert, funnel and needle rows again at both† 113/113); mesh `--features budget` clippy + tests 210 + 266; sweep `--profile ci` 2956/2956 and its cone rows 24/24 at three eps; tour `ci-tour` 97/97; doc-gate OK; `run-gates.sh` exit 0†; tools k-lint, tess-lint, tess-meter fmt + clippy + tests (tess-meter tests†). One caveat: a clippy lint in the needle row (a float match guard) was found by that battery and fixed after its mesh, budget, sweep and tour steps had run; the fix is a three-line rewrite of how the row spells its expected refusal, and clippy, fmt and the whole mesh suite ran again on it†. Slow set: funnel row 5.28 s, sliver row 1.27 s, needle pin 2.67 s. Left to CI: hosted runs, the nightly-only rows, `demos/wild`.
**Slowed by:** the outage; a disk allowance of about 39 G (incremental caches cleared twice mid-battery); two merges of main (702 and 93 commits).
