---
id: the-declared-tangency-bracket-refuses-its-closing-junction-at-interval
kind: issue
title: Guided replay at Interval refuses declared_tangency's bracket: the junction after the tangent-inherited Line reads a turn margin of [-1.5, 1.5]
status: open
opened: 2026-10-14
priority: P2
cost: M
---


Found by Ev's ruling A on #5068 (the pinned profile lift retires, so
every `Interval` evaluation elaborates its profiles at `Interval`).

`crates/editor-core/tests/corpus/tangency.rs` (`declared_tangency`),
the hand-declared bracket on the `z = 4` plane: under the guided lift at
`Interval` with no box, the profile node refuses
`ProfileLaneReplay { loop_: 0, step: 8 }` with `path_junction_turn`
indeterminate over `[-1.5000000000000127, 1.5000000000000127]`, and the
extrude and its placement are poisoned through it
(`crates/editor-core/tests/m4_pr8_corpus_interval.rs`,
`every_document_evaluates_green_at_interval`). Step 8 is the
`LineTo((0, 3))` after `Line(1.5)`, which rides the direction the
`TangentArcTo` hands on. The inputs are points, so a turn margin three
units wide is not an enclosure of the nominal's (a quarter turn) but a
blow-up: the inherited direction, or the point it carries the line to,
is far wider at `Interval` than the arc it comes from. Under the pinned
lift this document carried the `f64` elaboration lifted, so the row never
saw the lane.

Owed: find which value widens (the tangent-arc end direction at
`Interval`, or `Line`'s endpoint along it) and make it an enclosure of
the nominal's width; the corpus row then evaluates green again.
