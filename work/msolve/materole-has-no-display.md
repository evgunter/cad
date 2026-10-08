---
id: materole-has-no-display
kind: issue
title: MateRole has no Display, so no surface can say whether a mate placed its child
status: closed
opened: 2026-09-30
priority: P1
cost: E
parent: MSOLVE-11
closed: 2026-10-01
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

## Closed — `MateRole` reads in words (PR 3680)

`impl Display for MateRole` (`mate/solve.rs`) gives one sentence per
role:

- Determining: "places its child: the solve determined the pair
  through it"
- Declaring: "places nothing: it declares a contact, which the
  at-rest gate verifies"
- Refused: "places nothing: the solve refused it"

Each sentence is pinned by `msolve11_mate_log::a_mate_role_reads_in_words`.
AUTH's `a-mate-row-does-not-say-whether-it-placed-its-child` is
unblocked.
