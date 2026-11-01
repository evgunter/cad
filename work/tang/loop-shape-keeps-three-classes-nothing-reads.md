---
id: loop-shape-keeps-three-classes-nothing-reads
kind: issue
title: LoopShape's Polygon, ArcParity and NoWalk are computed on every call and read by no production code; SplitJoinError's doc still calls the engraving pose's cause unmeasured
status: closed
opened: 2026-10-01
priority: P4
cost: E
closed: 2026-10-02
pr: 3784
branch: tang/e-batch-docs-loopshape
---


## What

Two leftovers from `arc-aware-point-in-loop`, found by the TANG
re-measure (PR 3748). Each is one edit:

- `boolean::contain::loop_shape` classes every loop as `Disc`,
  `Polygon`, `ArcParity` or `NoWalk`, but only `Disc` has a production
  reader: check 9 decides two circles against each other with it. The
  other three are read only by test rows asserting a fixture's class
  (`validate.rs`'s lune row, `contain.rs`'s `LoopShape::Polygon` row).
  Shrink it to a disc-class reader (`Option<LoopCircle>`), drop the
  doc paragraphs that argue the polygon walk's domain, and re-spell
  those rows.
- `SplitJoinError::SectionLoopMixed`'s variant doc
  (`crates/topo/src/chord_join.rs`) says the engraving pose's cause is
  unmeasured. It is measured: it was the `point_in_solid` misread
  ATREST-9 fixed, and the pose now builds
  (`editor-core/tests/pierce_ring_engraving.rs`).

## Closed (2026-10-02, PR 3784)

`contain::loop_shape` and `LoopShape` are gone: `contain::loop_circle`
returns `Option<LoopCircle>`, the disc class check 9's arm 4 reads. The
lune row's class assertion is deleted (its polygon-`Out` assertion
already proves the fixture reaches the lune); the two disc rows assert
`Some`; the box row asserts a straight-edged loop is `None`.
`SplitJoinError::SectionLoopMixed`'s doc names the `point_in_solid`
misread as the measured source and cites the engraving pose's suite.
