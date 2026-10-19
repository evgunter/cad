---
id: the-declared-tangency-bracket-refuses-its-closing-junction-at-interval
kind: issue
title: A LineTo heading along -x from a computed point reads its direction off atan2's branch cut at Interval, so its turn encloses the whole circle (declared_tangency's bracket refuses at Interval)
status: closed
closed: 2026-10-19
opened: 2026-10-14
priority: P2
cost: E
branch: paths/chord-director-components
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

## Outcome

A chord's director is built from its components: `path::chord_director`
divides the chord by its length, deciding and refusing nothing — the
lever gates and the segment shape validation already ask about that
length under their own names (a band question here pre-empted
`SeamArrivalLeverTooShort` on a sub-ε closing leg), and geometry past the
format's range is refused where its vertex is validated or drawn (a
`NonFiniteDirection` refusal here reddened eight `viewer` rows built on
that layering). An over- or underflowed length divides by the largest
component first; the zero chord stands in `+x`; a component past the
range keeps `atan2`'s heading.
`line_to_point` and `line_to_start` take it; `arc_angles` and the fused
`Bulge` and `Center` incomings (`path/family.rs`) turn its ray by half the
sweep (`path::turned`) instead of adding angles. At `Interval` every such
junction reads a ray narrow around the nominal's; `declared_tangency`
evaluates green, and its two corpus pins are gone.

At `f64`: the bracket now lifts bit-identically (the lift census moves
one row from value-equal to bit-identical), one stored arc centre in
`fillet_stored_tangency`'s corpus moves by ulps, and of the editor-core
corpus only `declared_tangency` moves (4 points, ≤ 3.3e-16). Under the
symbolic tier the directors are algebraic forms rather than `sin_cos`
of an `atan2`: the rounded pad proves 162 more of its decisions
symbolically (none gained or lost), the registry door buys nothing on
it any more, and its ceiling rises from 2.7783e3·ε (`line_span`) to
3.152e3·ε (`arc_span`); the slab and plate build fewer forms. Each pin
is re-baselined with its note, and the drive's ladder-receipt row takes
the pad with its first leg spelled as an angle, the measured document
no longer retrying.

Filed: `a-turn-reads-its-incoming-angle-off-atan2s-branch-cut-at-interval`
(`.turn(δ)` and the `Sweep` extension still build a ray from the stored
angle; measured for `.turn`), and
`a-zero-length-line-leg-refuses-as-a-tangent-junction` (kept bit for
bit here).
