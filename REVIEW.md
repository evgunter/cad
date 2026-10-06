# REVIEW — PR #4074, planar/trimmed pinch wedges (frozen head a1e90e6c)

IN PROGRESS (tour sweep pending)

**Verdict: APPROVE-WITH-FIXES.** MAJOR 0 · MINOR 2 · NOTE 5.
The wedge rule is right on every non-crossing chart I built: three cones, reflex and 1e-4° wedges, a pinch on the hull, and near-adjacent corners. It is also right on both hand-built self-touching bodies and on all 15 rows, with exact volumes.
No body without a pinch moves. The fixes are about honesty: one refusal arm fires on a non-crossing face while its docs say "the loops cross, a kernel bug", and two refusal arms have no row.

Probes are in `review-probes/`:
- `planar_pinch_probes.rs` is a `#[path]` child of `planar`'s tests.
- `pinch_review_probes.rs` is a module of `sweep/tests/all.rs`.
- `mutate.py` and `mutants.sh` hold the mutants.

## Claims

1. **Every pinch shape meshes right: holds, with one exception (MINOR-1).**
   - **Chart probes, executed.** The checker asserts: every walk edge used once; every other directed edge paired with its reverse; all triangles CCW; Σ area = the loops' shoelace.
     - three cones (ids 2/5/8) is watertight, and its crossed twin refuses `PinchWedge`;
     - a reflex corner of ~290° beside a thin one of 20°, 1°, 0.5°, 1e-2° and 1e-4° is watertight;
     - a pinch on the CDT hull, with the notch between the two corners 30°, 1°, 1e-3° and 1e-6° wide (corners adjacent in rotation, one outside sliver between them), is watertight.
   - **Bodies, executed.**
     - All 14 planar rows: `check_mesh` is clean and the mesh's `signed_volume` equals `mass_properties` to 1e-9 at δ 0.05 and 0.01.
     - Hand-built prisms over a self-touching profile, the PR's pinched square and my three-cone square: `AtRestBody::validate` OK, faces through two vertices at one point = 2, mesh volume exact (31, 52).
   - **Trimmed lane at finer δ, executed.**
     - `Lbot cyl` meshes `check_mesh`-clean at δ = 1, 0.5 and 0.2. Volume 202.10 / 202.11 / 217.64 against 223.48, a chordal deficit that converges.
     - At δ = 0.3, 0.1 and 0.05 it refuses `CertificateExceeded` (faces 8v3 and 7v5). The trimmed pinch is therefore unexercised below δ = 0.2 (NOTE-1).
   - **Not built:** `vee300 fib62 face` in the ruling's final form, which needs the pinch unit's construction. The hand-built prisms stand in for it.
   - **Pinch on a slit seam: refuses.** See MINOR-1.
2. **The refusal is right: partly falsified.**
   - Crossed charts refuse: the 2-pass and 3-pass crossings, executed. I found no crossed chart that slips past, because inside sectors come from parity and the wedge match needs both sides.
   - A non-crossing face does reach `PinchWedge` (MINOR-1), and a ring touching its outer loop does too (NOTE-2).
3. **No body without a pinch moves: holds, executed.**
   - `mesh --release -- --include-ignored`: main 370/370, head 372/372. The test sets differ only by the PR's two new rows.
   - `d9_mesh_goldens` (digests at 1 and 4 threads) passes on both.
   - Tour tess-budget sweep, `--sizing-only`: TOUR_RESULT.
   - `editor-core union_pinch_member_order`: 4/4 on head.
4. **The slit dedup is untouched: holds, executed and inspected.**
   - A handle met by one id only is skipped by `Pinches::of` (`planar.rs:603-608`).
   - All slit rows pass on main and head: `slit_traversed_twice_does_not_toggle_the_region`, `the_slit_annulus_cap_survives_the_written_zero`, `survives_concentric_slit_annuli`, `r2_slit_annulus_at_several_proportions`, `probe_slit_washer_rebuild`, `a_zero_width_slit_passes_the_door…`, and the d9 digests.
5. **The pins can see a regression: holds for the wedge choice, falsified for two refusal arms (MINOR-2).** All executed.
   - Go red: always the first id, always the last id, the other wedge (`sides != sector`), and the trimmed read switched off. Each turns all 15 sweep rows red (census panic at `tessellate.rs:592`); the first three also turn the unit row red.
   - Dropping the crossed refusal (`_ => Ok(None)`) turns only `crossed_corners_at_a_pinch_refuse_typed` red.
   - **Survive:** the one-id-twice arm (`planar.rs:612` → `continue`) and the no-bounding-constraint arm (`planar.rs:679` → `Ok(None)`). Nothing goes red, the probes included.
6. **The `curved` exclusion: holds by inspection (likely), not executed.**
   - Two distinct ids on one handle need two walk entries at one UV.
   - `require_swept_rectangle` (`curved.rs:771`) makes the walk the box's boundary. A rectangle's boundary revisits no UV point, and pole entries sit at distinct u.
   - A full-revolution face puts u0 and u0+2π at distinct UVs.
   - Nothing *checks* this: `curved.rs:299` still dedups silently (style S6).

## Findings

**MINOR-1. A non-crossing face refuses as "loops cross — a kernel bug".** Executed: `p4_one_vertex_twice_and_another_once`.
- Shape: three corners at one point, the first two the same vertex (ids 2, 2, 8). One cone holds two corners of the face, a manifold self-touch, and another cone holds the third.
- The chart is the watertight three-cone chart, so nothing crosses. `Pinches::of` refuses it at `planar.rs:612`.
- `id_in` refuses it too: with the `of` arm mutated away, P4 still returns `PinchWedge`. Wedges are keyed per id, not per pass (`planar.rs:587`, `:634`).
- Control: the same chart with all three passes one id meshes (`p4c`).
- The docs misdiagnose it: `types.rs:207-212` and the Display at `types.rs:577` ("their corners overlap … kernel bug"), and `planar.rs:600` ("a slit through a pinch").
- Whether the ruling's booleans build this shape is unsure. It is typed (fail-loud) and no regression, since main panicked here. The text is still false, and the row naming the arm is missing.

**MINOR-2. Two refusal arms are unpinned** (claim 5). The `of` one-id-twice arm (`planar.rs:612`) and `id_in`'s unbounded-rotation arm (`planar.rs:679`) survive mutation with every committed row green. The second is likely unreachable for an inside triangle; neither has a row.

**NOTE-1.** The trimmed pinch is pinned at δ = 1.0 only (`pinch_faces_tessellate.rs:313`). It meshes at 0.5 and 0.2 too, and refuses `CertificateExceeded` at ≤ 0.3. The PR body says the same for 0.05.

**NOTE-2.** A ring touching its outer loop at a pinch (two loops, distinct vertices) refuses `PinchWedge` (`p3`, executed).
- It is not a crossing. Each sector is bounded by one outer side and one ring side, so no single id owns it.
- The ruling plans Check 9 to refuse two loops meeting, so the refusal may be right; the "loops cross" text is not.

**NOTE-3.** The sweep rows assert `check_mesh` but not volume. My probe shows the volumes are exact, so this is a cheap upgrade to the pin.

**NOTE-4.** `note_side` fills the first empty slot and silently drops a third side touching a wedge (`planar.rs:634`). No refusal fires; `id_in` happens to refuse later.

**NOTE-5.** The dispatch premise checks out against the tree: 15 rows, 0.45 s runtime, and two lanes sharing `Pinches` (`trimmed.rs:112,371,532`).

## Style (questions exercised: Q1 Q2 Q3 Q4 Q5 Q6 Q7; Q8 partly — read planar.rs's header and pinch region, not all 1824 lines)

- **S1** `planar.rs:575-589`, sure. `Pinches`'s doc paragraph runs straight into `type Wedge`'s, so rustdoc attaches both to `Wedge`, and `struct Pinches` has no doc at all.
- **S2** `types.rs:207`, `types.rs:578`, sure. Both say "a planar face", but the trimmed lane, a cylinder wall in row 15, emits `PinchWedge` too (`trimmed.rs:371,532`).
- **S3** `planar.rs:407-431` and `trimmed.rs:356-377`, likely (Q1). The `meta` / `ids_at` pair is pushed in step by hand, twice, with an identical `ids_at` block at each site.
  - `meta`'s comment "handle index -> the patch corner" is no longer true at a pinch handle: `Pinches` overrides it.
  - The class: every lane that keeps a per-handle side table. Look at `curved.rs:289-301` too.
- **S4** `trimmed.rs:91-98`, likely (Q4, doc rot). "split sections and boolean seams mint simple loops" no longer holds: the ruling mints trim loops that touch at a vertex, and this PR exists to mesh them. The same premise sits in `SelfTouchingTrimLoop`'s Display ("No at-rest construction mints a self-touching trim loop").
- **S5** `types.rs:207-212`, `planar.rs:597-601`, likely (Q2). The comments assert an invariant, "corners overlap ⇒ loops cross", that MINOR-1 and NOTE-2 falsify. The code is fail-loud either way, so this is doc rot rather than a latent defect.
- **S6** `curved.rs:299`, likely (Q6). The `curved` exclusion is a reading in the PR body. A `debug_assert` that each curved handle receives one id was one line, and would turn the reading into a check. As it stands it is a disclosed narrowing with no scheduled guard.
- **S7** `planar.rs:587,672-686`, unsure (Q7). Matching sectors to wedges by sorted `Option` pairs, with one wedge per id, is the data shape that makes MINOR-1 inexpressible. Wedges per pass, with ordered `(out, in)` sides, would make the natural query direct. This is taste, not a proven defect.
- **S8** `pinch_faces_tessellate.rs:343`, unsure (Q3). The rows can go red: mutants confirm it. But a refusal-arm regression that refused *more* valid pinches would surface only as `tessellate refuses`, which the rows do report. Fine. No class issue found.
- **S9** Prose sweep (`verbatim|mirror of|ported from|twin of|by hand|kept in step|must match`) over the added lines and over `planar.rs`/`trimmed.rs`: no disclosed copy (the hits are the chart frame's "verbatim" anchor). The undisclosed copy is S3, found by its data rather than its prose. I ran no constants grep. Blind spot: copies outside `crates/mesh`.

REVIEW COMPLETE
