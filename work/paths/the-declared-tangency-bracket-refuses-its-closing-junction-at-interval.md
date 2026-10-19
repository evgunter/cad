---
id: the-declared-tangency-bracket-refuses-its-closing-junction-at-interval
kind: issue
title: A LineTo heading along -x from a computed point reads its direction off atan2's branch cut at Interval, so its turn encloses the whole circle (declared_tangency's bracket refuses at Interval)
status: open
opened: 2026-10-14
priority: P2
cost: E
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

## Diagnosis (2026-10-19, #5068's last fix pass)

A defect, not an enclosure. `PartialPath::line_to_point`
(`crates/profile/src/path.rs`) takes the chord `d = p − at` and spells the
leg's director as an ANGLE: `Dir::from_angle(d.y.atan2(d.x))`. Step 8's
chord, from the end of `Line(1.5)` (a computed point, `(1, 1.5) + 1.5·unit(π/2)`)
to the authored `(0, 3)`, is `(−1, ±δ)`: `x < 0` and `y` a thin enclosure
straddling zero. That is `atan2`'s branch cut, and inari's interval `atan2`
answers that class (`N1_M`) with the whole `[−π, π]`
(`inari-2.0.0/src/elementary.rs`, `atan2_impl`); `x < 0` with `y` exactly
`[0, 0]` gives a narrow π, which is why the authored `(3, 1) → (1.5, 1)` leg is
fine. `unit` of `[−π, π]` spans `[−1, 1]` in each component, and the junction
turn reads a margin as wide as the leg (`±1.5`).

The fix: build a chord's director from its COMPONENTS (`Dir::from_unit` of
`d` normalised under the door's band, as `PartialPath::toward` does), which
keeps the ray an enclosure of the nominal's, and leaves `atan2` to the angle
arithmetic that needs a number (where the angle's own enclosure is narrow off
the cut). The same `from_angle(atan2(..))` spelling sits in the sibling at
`path.rs` `arc_angles`' chord and the second `line_to_point`-shaped door
beside it; each of those is the class.
