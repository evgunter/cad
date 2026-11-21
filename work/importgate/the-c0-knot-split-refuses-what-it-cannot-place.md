---
id: the-c0-knot-split-refuses-what-it-cannot-place
kind: issue
title: the import's C⁰-knot split refuses a face whose trim crosses the knot line more than once, a crossed face with rings, or a face it cannot certify clear of the line within its budget
status: closed
opened: 2026-10-29
priority: P1
cost: M
refs: [import-splits-a-foreign-face-or-edge-at-each-c0-knot, tier-3-refuses-a-carrier-with-an-interior-c0-knot]
closed: 2026-11-17
pr: 7084
branch: importgate/knot-place
---


(FOREIGN implementer, `foreign/c0-knot-split`.)

`crates/step-import/src/knot_split.rs` splits a face chart along a crease knot's iso line where the line crosses the face (`crossing_knots`: projected boundary samples are evidence that the boundary reaches a side, and a side they do not see reached is certified clear by enclosures, `clear_of`, before the line is left out). It places the line only where it runs across the whole chart, from `Q₀` to `Q₁` (control points of the net), and parts the face's single loop at those two points: one chain on each sub-patch, or, on a chart closed across the split, one walk of the closed boundary each. A line outside the face's trimmed domain, or along its boundary, lies inside no cell and needs nothing (orchestrator's reading of "strictly inside its cell", 2026-10-29).

What still refuses `StepImportError::C0KnotUnsplit`, naming the face and the knot:

- **A trim that crosses the knot line more than once**, or crosses it away from the chart's two boundary rows. The line then parts the face into several pieces, or into pieces whose loops need outer-loop inference.
- **A crossed face with rings.**
- **A face the selection cannot certify clear of the line** on the side its samples do not reach (QUAD 6013 fix lane, 2026-10-30). `clear_of` boxes each boundary edge piece against the chart's enclosure past the line, set back `ε_in` and inflated by `ε_in`. It counts a piece as on the line where the piece lies within `ε_in` of it: across the line's chord (exact for a straight line, loose by the line's bend otherwise), or from an enclosed point of the line. It stops a piece no wider than `ε_in` that still meets the region, and the whole reading at `CLEAR_BUDGET` enclosures. So a trim crossing by under `ε_in` certifies, a shallow tangency included (`crossings_between_samples::pinned_near_misses`). What refuses:
  - **an edge running along a curved knot line** within `ε_in` of it, such as one half of a curved surface the exporter split at the knot into two faces. Neither reading settles it short of `ε_in`-long pieces, so the budget runs out (`a_face_along_a_curved_knot_line_runs_out_its_budget_and_refuses_typed`). Certifying it needs the edge's distance from the iso curve bounded along its length: a certified pcurve, or a curve-curve Hausdorff bound.
  - **a boundary within about `ε_in` of the line where the chart's speed across it varies.** There the region's inflation reaches past the line, and a piece at the size floor can meet it with a corner farther than `ε_in` from the line.

  Each such face is kept as crossing, and its split refuses, because one chain holds no corner, or the line meets the loop where it has no vertex. On the committed corpus at the fix (93 imports, `tier_gate::every_corpus_import_passes_the_shared_gate`), no knot reached certification at all.

Placing either needs the line's crossings with the trim: a curve-surface inversion (`chart::uv_of` has no NURBS arm) and the loop surgery that follows it. This is a capability gap, not a design question. Until it lands, such a file refuses typed. On the merge base it imported with the crease inside a cell, and the mesher refused it.

- 2026-11-02 — moved from FOREIGN to IMPORTGATE by `git mv` when FOREIGN merged into IMPORTGATE, id and body unchanged (the track budget now counts only P0, P1 and unbanded rows, and FOREIGN had fallen far under it).

## Placed (`importgate/knot-place`)

`split_face_once` now reads where the knot's iso line meets the trim through the kernel's curve-pair reader (`geom_brep::curve_pair::meet`, at the file's chart band). It reads an edge along the line by its iso pcurve (`PcurveCache::certify` with `Pcurve::iso_column` / `iso_row`). It splits an edge only where its side changes, and traces each side's pieces. So the three classes above import:

- **(a)** a trim crossing the line any even number of times, anywhere along it: one face per piece, each with its own outer loop (`a_trim_crossing_the_knot_line_four_times_imports_in_three_pieces`, `c_prism.step`);
- **(b)** a crossed face with rings: a crossed ring joins its pieces' outer loops, and an uncrossed ring joins the one piece on its side (unit rows in `knot_split`). No import reaches it yet: a NURBS face with rings refuses earlier, at the curved-face ring gate in `entities::Resolver::face`;
- **(c)** a face the selection cannot certify clear: the split itself finds no crossing, and the face stays whole, its chart narrowed to the sub-patch on its side (`a_face_bounded_by_a_curved_knot_line_imports_whole`, `half_ridge.step`).

What still refuses `C0KnotUnsplit`, each naming the face and the knot, one line per reason the code gives (`split_face_once`), with the row that reaches it where there is one:

- **no band**: the file's uncertainty makes no chart band (`chart::file_band` answers `None`).
- **the chart refused its split** at its own crease knot (`split_net`), or **the knot's iso curve, or the chart's boundary column, did not re-wrap as a curve**. Each is a structural refusal of the stored net.
- **a seam edge of a chart closed across the split that did not project onto its column** within the band.
- **a pair the curve-pair reader does not settle at the file's band**:
  - a crossing within a few ε_in of tangency (`a_trim_crossing_by_twice_eps_in_is_kept_and_refuses_typed`);
  - a curved iso meeting a circle at a span's end at a coarse band, where the walk exhausts (`work/cpair/the-curve-pair-walk-exhausts-on-an-end-meeting-at-a-coarse-band.md`).

  A straight iso is read as the line `recognize_curve::straight_map` certifies (the straight MAP: the crossing is carried back through the chord map), in closed form, which settles that case.
- **a boundary edge sharing a run with the line by a closed form**, or **an edge along the line its iso pcurve does not certify** (a parameter not affine in the line's, or a line beside a rational column). The latter goes to the walk, which reads no shared run.
- **a boundary piece between meetings that reads no side**: no chart foot within the band (`a_trim_off_its_chart_past_the_band_reads_no_side_and_refuses_typed`; the incidence premise, `import-splits-knot-lines-on-an-unchecked-incidence-premise`).
- **a boundary edge that refused its split** where the line crosses it (`NurbsCurve3::split_at`).
- **a loop that changes side away from the line** (`a_tube_loop_against_its_sense_changes_side_away_from_the_line_and_refuses_typed`).
- **loops on both sides of a line none crosses.**
- **a crossed line that closes on itself** (a chart closed in the other direction).
- **crossings that do not alternate into and out of the face** (`a_loop_turning_against_its_faces_sense_refuses_typed`).
- **a touch of the line, or an edge along it, inside a run of the line in the face**, where a piece would be pinched at a vertex (`a_notch_touching_the_line_inside_a_run_refuses_typed`, `a_ring_touching_the_line_inside_a_run_refuses_typed`).
- **a ring the line crosses while it does not cross the outer loop.**
- **a ring on a side the line parts into several pieces**: telling which piece holds it needs containment on a chart with no inverse; `chart::infer_outer` has no NURBS arm (`a_ring_on_a_side_parted_into_several_pieces_refuses_typed`).

Unexercised: a boundary within about ε_in of the line where the chart's speed across it varies. It now reaches the same reading, but no fixture builds it. This residue is filed as `the-c0-knot-split-residue-after-placement`.

## Closed

2026-11-17, PR 7084. The split reads each boundary edge's meeting with the knot's iso line by `geom_brep::curve_pair::meet` at the file's chart band (a two-control equal-weight iso through `recognize_curve::try_line`, now `recognize_curve::straight_map`), places a vertex at each crossing and parts the face, rings included, into the pieces the line cuts it into; edges along the line are certified by the iso pcurve, minted runs carry the whole iso, and an uncrossed face's chart narrows to its side's sub-patch. The three refused classes above now import. What still refuses `C0KnotUnsplit` is `the-c0-knot-split-residue-after-placement`.
