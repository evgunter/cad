---
id: measureexpr-has-no-display-or-as-primitive
kind: issue
title: MeasureExpr has no Display and no as_primitive, so a reader outside the crate cannot say what a measure measures
status: open
opened: 2026-09-30
priority: P1
cost: M
---


Found by AUTH-7 (PR 3528) and its review. The feature tree now shows a
measure's value (`Measure »  0.0125 m`). The parent row's example,
`Measure  distance  12.500 mm`, also names what was measured, and the
viewer cannot produce that word honestly:

* `MeasurePrimitive::verb()` is public, but
  `MeasureExpr::kind()` is `pub(crate)`
  (`crates/editor-core/src/measure.rs`, `MeasureKind`). A reader
  outside the crate cannot tell a bare `distance` apart from
  `distance - 5 mm`. `primitives()` lists the leaves and says nothing
  about the arithmetic above them.
* `MeasureExpr` has no `Display`, so a composed measure has no kernel
  spelling at all. A viewer that wrote one would be minting a second
  unparser.

Either door closes it: an `as_primitive() -> Option<MeasurePrimitive>`,
which lets the row name the verb only when it is the whole expression,
or a `Display` that `unparse` shares. The consumer is the tree row in
`crates/viewer/src/tree.rs` (`Measured`), and the viewer half is one
line once the door exists.
