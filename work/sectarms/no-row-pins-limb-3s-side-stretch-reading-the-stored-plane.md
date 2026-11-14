---
id: no-row-pins-limb-3s-side-stretch-reading-the-stored-plane
kind: issue
title: No row pins limb 3's side stretch reading the stored plane's sign
status: open
opened: 2026-11-11
priority: P3
cost: M
---


Found by PR 6830 and its review.

**What is unpinned.** In `crates/geom-brep/src/ssi/section.rs`,
`Stretch::sign` reads the sign of the stored plane's φ along a stretch
of a side: the refined sign of `h + lift·W`, its `lift` carried from
the lifted form (`ImplicitForm::lifted`). `stretch_by`
(`crates/geom-brep/src/ssi/boundary.rs`) reads that sign for `clears`,
the same exact empty answer the boundary pass reads through
`Pieces::sign`. Drop the stretch's lift (set `Stretch::lift` to zero)
and every row stays green. PR 6830 ran that mutant, and so did its
review.

**Why no row reaches it.** Limb 3 reads stretches only through
`one_arc`'s `side_stretch` / `reaches_end`. That needs:
- a branch traced along part of a side;
- in a plane × spline pose read at a pivot;
- where the pivot plane's sign along the stretch is decided, and
  decided on the other side from the stored plane's.

None of the probe or review fixtures has that shape.

**What closes it.** A row with that shape that the mutant turns red,
or an argument written beside `stretch_by` that the case cannot arise,
with the lift then dropped from `Stretch`.
