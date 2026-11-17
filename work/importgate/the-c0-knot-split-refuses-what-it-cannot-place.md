---
id: the-c0-knot-split-refuses-what-it-cannot-place
kind: issue
title: the import's C⁰-knot split refuses a face whose trim crosses the knot line more than once, a crossed face with rings, or a face it cannot certify clear of the line within its budget
status: open
opened: 2026-10-29
priority: P1
cost: M
refs: [import-splits-a-foreign-face-or-edge-at-each-c0-knot, tier-3-refuses-a-carrier-with-an-interior-c0-knot]
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

What still refuses `C0KnotUnsplit`, each naming the face and the knot:

- **a pair the curve-pair reader does not settle at the file's band.** One case is a crossing within a few ε_in of tangency (`a_trim_crossing_by_twice_eps_in_is_kept_and_refuses_typed`). Another is a curved iso meeting a circle at a span's end at a coarse band, where the walk exhausts (`work/cpair/the-curve-pair-walk-exhausts-on-an-end-meeting-at-a-coarse-band.md`). A straight iso is read as a line in closed form, which settles that case.
- **an edge along the line its iso pcurve does not certify**: one whose parameter is not affine in the line's, or a line beside a rational column. It goes to the walk, which reads no shared run.
- **a touch of the line, or an edge along it, inside a run of the line in the face**: the piece would be pinched at a vertex.
- **crossings that do not alternate into and out of the face** along the line, or a loop that changes side away from it.
- **a crossed line that closes on itself** (a chart closed in the other direction).
- **a ring on a side the line parts into several pieces**: telling which piece holds it needs containment on a chart with no inverse (`chart::infer_outer` has no NURBS arm).
- **a boundary piece whose middle has no chart foot within the band** (the incidence premise, `import-splits-knot-lines-on-an-unchecked-incidence-premise`).
