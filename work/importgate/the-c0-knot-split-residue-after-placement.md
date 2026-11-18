---
id: the-c0-knot-split-residue-after-placement
kind: issue
title: what the import's C0-knot split still refuses after it places the knot line's crossings with the trim
status: open
opened: 2026-11-17
priority: P3
cost: M
---

(Filed by the IMPORTGATE implementer of PR 7084, `importgate/knot-place`, which placed the knot line's crossings with the trim. The item it closes against is `the-c0-knot-split-refuses-what-it-cannot-place`.)

## What still refuses

The import's C⁰-knot split (`crates/step-import/src/knot_split.rs`, `split_face_once`) refuses `StepImportError::C0KnotUnsplit`, naming the face and the knot, for each reason below. Rows are in `knot_split::tests`.

- no chart band for the file's uncertainty (`chart::file_band` answers `None`);
- the chart refused its split at its own crease knot, or the knot's iso curve or the chart's boundary column did not re-wrap as a curve;
- a seam edge of a chart closed across the split that does not project onto its column within the band;
- a pair `geom_brep::curve_pair::meet` does not settle at the file's band:
  - a crossing within a few ε_in of tangency (`a_trim_crossing_by_twice_eps_in_is_kept_and_refuses_typed`);
  - a curved iso meeting a circle at a span's end at a coarse band (`work/cpair/the-curve-pair-walk-exhausts-on-an-end-meeting-at-a-coarse-band.md`);
- a boundary edge sharing a run with the line by a closed form, or along the line where its iso pcurve does not certify (a parameter not affine in the line's, or a line beside a rational column);
- a boundary piece between meetings with no chart foot within the band (`a_trim_off_its_chart_past_the_band_reads_no_side_and_refuses_typed`);
- a boundary edge that refused its split where the line crosses it;
- a loop that changes side away from the line (`a_tube_loop_against_its_sense_changes_side_away_from_the_line_and_refuses_typed`);
- loops on both sides of a line none crosses;
- a crossed line that closes on itself (a chart closed in the other direction);
- crossings that do not alternate into and out of the face (`a_loop_turning_against_its_faces_sense_refuses_typed`);
- a touch of the line, or an edge along it, inside a run of the line in the face (`a_notch_touching_the_line_inside_a_run_refuses_typed`, `a_ring_touching_the_line_inside_a_run_refuses_typed`);
- a ring the line crosses while it does not cross the outer loop;
- a ring on a side the line parts into several pieces, which needs containment on a chart with no inverse (`chart::infer_outer` has no NURBS arm; `a_ring_on_a_side_parted_into_several_pieces_refuses_typed`).

## Unexercised

- **A boundary within about ε_in of the line where the chart's speed across it varies.** The selection's `clear_of` could not certify it. It now reaches the split's own reading, but no fixture builds it.
- **A crossed face with rings, through an import.** The split places it (unit rows), but a NURBS face with rings refuses earlier, at the curved-face ring gate in `entities::Resolver::face`.
