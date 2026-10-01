---
id: materole-has-no-display
kind: issue
title: MateRole has no Display, so no surface can say whether a mate placed its child
status: open
opened: 2026-09-30
priority: P1
cost: E
refs: [a-mate-row-does-not-say-whether-it-placed-its-child]
---


Found by AUTH-7 (PR 3528). A `Mate` node evaluates to
`ValuePayload::Mate(MateRole)` (`crates/editor-core/src/mate/solve.rs`,
`MateRole`): `Determining` means a tree mate placed its child,
`Declaring` means a non-tree mate that solved nothing and only
declares contact, and `Refused` means the mate refused. The type has no
`Display` and no word for a reader. So the viewer cannot tell a person
why the mate they just added moved nothing, unless it mints its own
sentence about the solver's role, and it does not.

The viewer consumer is `work/author/a-mate-row-does-not-say-whether-it-placed-its-child`,
blocked on this row. The kernel word comes first, so both surfaces say
it the same way.
