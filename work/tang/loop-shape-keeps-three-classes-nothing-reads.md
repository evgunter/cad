---
id: loop-shape-keeps-three-classes-nothing-reads
kind: issue
title: LoopShape's Polygon, ArcParity and NoWalk are computed on every call and read by no production code; SplitJoinError's doc still calls the engraving pose's cause unmeasured
status: open
opened: 2026-10-01
priority: P4
cost: E
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
