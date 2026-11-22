(4c implementer) **REPORT, part 2 of 2: the changed PR-body paragraphs.** Your 18 November set stands for everything not listed. "The route carries the walk", "The shared-side rule and its bound", the count relation's exception, "where a split never makes a face refuse" and the mutants table replace the old ones; the rest are new.

**Correction to part 1:** `work.py territory --base origin/main` on e760867739 reads **11 paths**. pncad-py, `types.rs` and `walk.rs` left the list once they were back to main's text, and `cert.rs` and `route_rows.rs` joined.
- Double claims: cert (chord), chords (chord, tesslie), lib, sizing (chord), tessellate, trimmed (tesslie); tests all, reach_probes, route_rows, split_rulings (ciw, green).
- Owned elsewhere: `.config/nextest.toml` (green).

=====
**Integrated with main's shared-side rule and census (PR 7198).** Main now has one shared-side rule on every trimmed arm and a census where the patches join.
- The walk reads main's one test of a shared side. It is lifted, verbatim, from `TrimmedFace::emit` into `TrimPolygon::faces_holding_side`, which both the CDT arms and the walk call. A side qualifies when it joins two polygon points that are not neighbours on their loop and one face across holds both ends.
- The CDT arms cure such a side as before: they insert a point and rebuild the face. Their behaviour is unchanged.
- The walk has no triangulation to rebuild. It cuts the side at its chart midpoint, and only where the face across may reach it (`may_make`).
- The census (`tessellate::overused_side`, `OverusedSide`) is the backstop, never the cure. No walked body reaches it on any cell run here, at three ε. With the walk's cut turned off, the bored-ball row goes red by `OverusedSide { uses: 4 }`.
=====
**What the walk buys, and what its cut costs: the bored ball against main.** A ball of radius 1.05 with a bore of radius 1, placed obliquely, default ε. Triangle totals, main / this branch:

| δ | sign +1, unsplit (both) | sign +1, seam split at 0.5 | sign −1, unsplit (both) | sign −1, seam split at 0.5 |
|---|---|---|---|---|
| 3 | `OverusedSide` | 62 / 64 | `OverusedSide` | 62 / 64 |
| 1 | `OverusedSide` | 62 / 64 | `OverusedSide` | 62 / 64 |
| 0.3 | `OverusedSide` | 102 / 104 | `OverusedSide` | 62 / 64 |
| 0.1 | 132 | 152 / 220 | `OverusedSide` | 64 / 88 |
| 0.05 | 186 | 216 / 310 | 90 | 120 / 120 |
| 0.04 | 210 | 242 / 350 | 102 | 134 / 136 |
| 0.035 | 296 | 330 / 298 | 108 | 142 / 110 |
| 0.03 | 320 | 358 / 322 | 114 | 152 / 116 |

(Split at five points: the same shape, +8.)
- Main's chart lane meshes every split cell.
- The walk is coarser where it needs no cut (δ ≤ 0.035: sizing at δ, without main's headroom).
- It is denser at δ 0.1 to 0.04, where its cut over-cuts.
- On the unsplit bore, the census turns old main's silent non-manifold `Ok` into a typed refusal on 7 cells. That is main's, filed (below).
=====
**The shared-side rule and its bound.**
- **The rule.** `trimmed::may_make` excludes a side whose midpoint is decided, on the run's band (`geom_core::k_stats::decide`), to lie farther than δ·(1 + `cert::ALLOWANCE_SHARE`) from the surface of the face across. Undecided and decided-not-greater both cut. There is no margin constant in `mesh`.
- **Why it is sound.**
  - A passed triangle lies within δ(1+s) + a of its surface, where a is the certificate's rounding allowance: 8 ulps of the magnitude, and the verdict passes only while a ≤ sδ.
  - "Decided greater" puts the midpoint past δ(1+s) + Kε − r, where r is the distance's own short-chain rounding.
  - So the exclusion is sound while Kε > a + r, about 6·10⁻¹² m at the session box's edge.
  - Every ε the rows run has Kε ≥ 10⁻¹¹. The admission floor alone does not guarantee it at ε_min for a body at the box's edge; there, the census refuses what slips. This is stated at the site.
- **The undecided window.** `a_side_whose_reach_the_band_cannot_decide_is_cut` places a cell inside the band's undecided window on the bored ball at each ε. All 36 sides there are undecided and cut. A rule that reads undecided as "beyond reach" cuts none, and the cell is red.
=====
**The over-cut, stated and bounded.** Each cell of the two shared-side rows pins the number of sides the walk cuts (`holds_the_relation(…, cuts)`): the walked faces' interior points, two triangles each. A cut the rule does not ask for is red, and so is one it asks for and does not make.
- **Bored ball 1.05**, the same for both splits. sign +1: 15 at δ 3, 1; 25 at 0.3; 43 at 0.1; 69 at 0.04. sign −1: 15 at δ 3, 1, 0.3; 21 at 0.1; 16 at 0.04. None at δ ≤ 0.035.
  - The census needs the cuts at sign +1 δ 3 to 0.3 and sign −1 δ 3 to 0.1. With the cut off those cells refuse `OverusedSide`, as main does on the unsplit body.
  - The rest are over-cuts: sign +1 at δ 0.1 and 0.04, sign −1 at 0.04.
- **Bored ball 1.3** (a shell of 0.3). Its unsplit body meshes at every δ, so every cut is an over-cut. sign +1: 15 at δ 3, 21 at 0.5, 29 at 0.25. sign −1: 15 at δ 3 and 0.5, 7 at 0.25. None at δ 0.2 or 0.1. Its threshold falls between 0.25 and 0.2.
- **The band** (in the 218-cell row): 2 to 7 cuts on 24 cells at δ 1 and 0.1, all over-cuts. With the cut off, every cell of that row meshes manifold.
- **Why the over-cut stays.** Cutting exactly what the face across makes would make a face's patch depend on its neighbour's mesh. That is a design question, not this unit's.
=====
**The count relation's stated exception.** A walked face and its planar neighbours have the unsplit face's triangles plus one per traversal their loops gained, plus two per side the walk cuts, pinned per cell. Where main's census refuses the unsplit body (the bored ball's coarse cells), there is no unsplit count to compare against, and only a cell that cuts may pass. Its cut count, `check_mesh` and the geometry are read.
=====
**The route carries the walk, and its re-reads are bugs, not inputs** (`docs/DESIGN.md` `[failure.taxonomy]`). The lane has no typed router-defect refusal. Main's "(router defect)" sites are untouched. The walk's two re-reads of the route are `unreachable!`, each with its row-0 answer written at the site:
- **A walked face on a spline chart.**
  - Row 0 does not apply locally: whether a face's row is analytic is `ChartRow::of`'s run-time match, which every arm shares.
  - The route walks only where `walk::splits_a_straight_side` held. That holds only on a cylinder or a cone, whose row is analytic.
- **A trim polygon that does not read as the walk's two rows.** This is a re-derivation off the polygon the chord pass built after the route ran, of ONE decision:
  - The route and the lane read the apex through one function on the same arguments: `chart_table::at_a_singular_point` over `Chart::of(surface).poles()`.
  - Both read the vertex's point by topo's one read of it.
  - Both read at the run's one band.
  - Where the route's reading is undecided, it does not walk the face.
  - Re-run on this head: the 0.001° cone split 10⁻⁸ from its apex at three ε, and a sweep of the split gap from 10⁻¹² to 10⁻⁴ (81 gaps, δ 0.1 and 0.01). No panic. Typed refusals only:
    - default ε: `SingularUndecided` at gap 10⁻⁸;
    - 10⁻¹²: `UnresolvedWinding` at gaps 10⁻¹¹ to 6·10⁻¹⁰.
=====
**Where a split changes a refusal** (adding to "a split never makes a face refuse"):
- **A thin D-prism's planar cap: a regression against main the unit keeps.** A D-shaped prism of 0.5 rad at δ 0.1 and of 0.2 rad at δ 0.03, with every line edge split, gives 12 cells that mesh on main and refuse here.
  - The refusing face is a planar cap, not the walked face. With a point on its chord, the cap meets points the planar lane cannot wind (`UnresolvedWinding`).
  - The same prism with only its caps' chords split refuses the same way on both trees, and the unsplit prism meshes on both.
  - Main meshed the all-edges-split body only because its chart lane chorded the ruled face's rim finer (its headroom).
  - Pinned: `a_thin_d_prism_split_on_every_line_edge_refuses_as_its_split_caps_do`. Class: `work/tesslane/a-thin-arc-bounded-face-refuses-as-corrupt-geometry-at-a-coarse-delta.md`.
- **Curved-lane neighbours of a walked face get coarser.** In `curved::tests::a_multiply_carried_iso_side_is_bitwise_straight_and_meshes_watertight` at δ 0.1, 24 patches (16 cone, 8 cylinder) sharing a rim with a walked face lose one or two triangles each. The walked face no longer asks the rim for the headroom's chords, so the unit's effect arrives through the shared rim, as it does on plane neighbours. Accepted.
=====
**One certificate verdict over a face's triangles.** `cert::FaceVerdict` (`read`, `worst`, `refusal`) holds the worst bound, whether any triangle exceeded δ, the largest unresolved allowance, and the face's one refusal from them.
- The rectangle lane, the CDT arms' emit and the walk all read it. Before, each spelled the accumulation itself.
- No byte moved: route_rows' main's-bytes rows, the D9 goldens, the digest rows, curved, errors, split_rulings and trimmed tests.
- The ε inventory re-pins `cert.rs`'s carriers 2 → 4: the band now passes through `FaceVerdict::read` to `verdict`. The read columns are unchanged.
- What stays per lane is the winding and fold reading: the CDT emit drops a fold and retries, while the walk and the rectangle lane refuse on it.
=====
**The K stream.** On a ruled face with a split side, the route also asks `walkable`: one `chart_singularity_at_vertex` per singular point of the chart per outer-loop vertex. The lane records one `walked side beyond its neighbour's reach` per side and face across asked. Both are named at `tessellate::FaceRoute` and pinned by `route_rows::a_split_side_body_records_the_walks_decisions`:
- a frustum wall: 36 apex decisions, 12 of them `walkable`'s;
- the bored ball at δ 0.1: 43 reach decisions, one per cut.
A memo hit records neither.
=====
**Filed.**
- `work/tesslane/a-strip-against-a-neighbour-holding-both-rims-makes-an-edge-of-four-triangles.md`, restated against merged main.
  - The unsplit bore now refuses `OverusedSide` on 7 cells: sign +1, the torus neighbour, at δ 3 to 0.3; sign −1, the sphere, at δ 3 to 0.1.
  - PR 7198's "the rectangle lane needs no rule" covers one chain on one iso line, not two chains held by one neighbour.
- `work/tesslane/a-walked-sides-reach-exclusion-is-read-only-by-the-walk.md` (P3).
  - `may_make`'s exclusion is true of every arm that certifies through `cert::verdict` at the run's δ.
  - How often main's rule fires where it would exclude: not counted.
- Dated lines on the thin-arc item (the D-prism cells) and the needle item.
- Stale citations fixed in two items.
=====
**Mutants** (each red under its own rows; filter: split_rulings, route_rows, errors, ruled_walk, sizing, reach_probes):

| mutant | red |
|---|---|
| zip off | 4: `ruled_walk::tests::a_one_chord_face_to_the_apex_zips_its_sides` (`--lib`) and 3 split_rulings rows (the two one-chord apex rows, the 218-cell row) |
| shared side never cut | 4: the bored-ball row by `OverusedSide { uses: 4 }` (the census), the 218-cell row (its pinned band cuts), the undecided-reach row, the K row |
| shared side always cut | 3: the bored-ball row, the undecided-reach row, the K row |
| undecided reach read as "beyond" | 1: `a_side_whose_reach_the_band_cannot_decide_is_cut` |
| route's walked fact off | 11: 10 split_rulings rows and the K row |
| walk reached by a face the route did not mark | 22: 17 reach_probes rows, route_rows' K stream and bytes rows, 2 split_rulings rows |
| 7054's anchor / three / two / resplit (earlier pass) | the two-runs row / run-of-three / two-runs / five rows |
=====
