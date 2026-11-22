---
id: a-chain-whose-structure-forces-a-degenerate-arc-passes-the-door
kind: issue
title: A chain whose structure forces its arc degenerate at every value (line_to, line, arc_onto(StartArriving)) passes the door
status: open
opened: 2026-11-18
priority: P3
cost: M
---

Found by PR 7116's last pass, while the random differential
(`crates/profile/tests/chain_shape_random.rs`) learned to compare
`DegenerateArcChord` through redraws of a chain's values.

`at(a), line_to(b), line(l), arc_onto(StartArriving)` refuses
`DegenerateArcChord` at its last step at every value: `line` continues
`line_to`'s direction, so the tip, `b` and the entry `a` are collinear,
and an arc leaving the tip along that line and arriving at `a` along
the first leg's direction is the full turn behind it. The redraw rule
saw every one of eight redraws refuse there. The value-free door
(`profile::chain_shape`, `crates/profile/src/path/shape.rs:655`) admits
it, because the walk reads step order, and this refusal is geometry
that the order happens to force.

The differential now compares only an exactly zero chord
(`chain_shape_random.rs:290`, `zero_chord`): a closing arc from the
entry itself, which the walk refuses as `OrderFault::ClosesOnItsEntry`
(`crates/profile/src/path/order.rs:135`). This full-turn case has a
non-zero chord and stays evaluation's.

## Done when

It is decided whether `[recipe.doors]` asks the door to refuse geometry
that structure forces (collinearity a step order fixes), and the door
does so or this row records that it is evaluation's. A door answer
would need a structural reading of which legs are collinear by
construction (a `line` after a straight leg, a `continue_to`), stated
once beside `order.rs`'s predicates and asked by both the replay and
the walk, as the order faults are.
