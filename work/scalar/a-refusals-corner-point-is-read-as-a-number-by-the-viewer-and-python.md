---
id: a-refusals-corner-point-is-read-as-a-number-by-the-viewer-and-python
kind: issue
title: A profile refusal's positions stay Point2<T>: the viewer draws them and the Python door exposes them as numbers
status: open
opened: 2026-11-20
priority: P1
cost: M
design: true
---

Found by the told-quantity build's measure-first pass
(`the-told-quantity-type-and-profiles-refusals-on-it`): who reads a
scalar of a `profile` refusal payload as a number.

`crates/geom-core/README.md`, "A number told to a person", says a
payload offers "no way to read a computed number back". Four fields of
`profile`'s refusals are positions, held as `Point2<T>`, and two of them
are computed and read by production code:

- `CornerRefusal::at` (`crates/profile/src/path.rs`), the derived
  corner. `viewer::sketch::drawn` (`crates/viewer/src/sketch.rs`,
  `refused_corners`) collects it to draw the corner a refusal names;
  `pncad-py`'s `path_err` (`crates/pncad-py/src/py/path.rs`) exposes
  `(c.at.x, c.at.y, reason)` as floats on the Python exception;
  `arc_fillet::resolve`'s presentation sort reads it (`anchor_span`,
  `crates/profile/src/path/arc_fillet.rs`).
- `PathNoCornerReason::TurnWithinTolerance::corner`, read by the same
  `viewer::sketch::drawn` arm.
- `PathError::CuspDoublesBackOverLeg::at` and
  `ArcMissesAuthoredPoint::point` / `ArcMissBelowSceneResolution::point`
  are read by no production code; the second and third are the
  author's own point.

The build left all four as `Point2<T>`, told through `Told` where a
sentence prints them (`told_point`), and that is why `PathError<T>` and
`CornerRefusal<T>` still carry `T`; `CornerReason`, `LegFit` and
`Recourse` lost it. `scripts/gates/told-quantity-fields.sh` cannot see a
scalar inside a type another crate declares, and names this row as its
blind spot.

**The question.** A corner the viewer draws is a place, not a number a
sentence quotes, and neither reader decides on it. Three answers were
visible and none was taken:

1. A told point in `geom-core` (two told lengths, with a door that
   hands a drawing reader the position), which empties `T` out of
   `PathError` and moves every signature that names `PathError<T>`.
2. The rule says positions a refusal locates are geometry the payload
   may hold as geometry, and the gate's blind spot becomes its stated
   scope.
3. The drawn corners leave the refusal for the channel the viewer
   already reads its construction from (`Construction::refused_corners`
   is built from the refusal today), and the Python door gets them from
   there.
