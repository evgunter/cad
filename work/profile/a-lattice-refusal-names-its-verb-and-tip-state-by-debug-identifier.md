---
id: a-lattice-refusal-names-its-verb-and-tip-state-by-debug-identifier
kind: issue
title: profile's Refused sentence names its verb and tip state by their Debug identifiers
status: open
opened: 2026-11-20
priority: P4
cost: E
---

Found by SCALAR's told-quantity build
(`work/scalar/the-told-quantity-type-and-profiles-refusals-on-it.md`),
whose gate reads every sentence-writing body of `crates/profile` for
`{:?}`.

`Refused`'s `Display` (`crates/profile/src/path/program.rs`) writes
"the {verb:?} verb is not a legal continuation of tip state {state:?}",
and `Takes`'s, its recourse, lists what the state takes the same way
(`{verb:?}`, `{mode:?}`, `{target:?}`): `Verb`'s, `TipState`'s,
`ArcMode`'s and the target forms' `Debug` identifiers (`TangentArcTo`,
`DirectedPoint`) inside a sentence, eight formats in all. `Verb` has a
`Display` that gives the authoring spelling; `Takes`'s doc says its
list is "in the transition table's identifiers", and the comment above
`ReplayError`'s `Display` says the lattice arms keep that rendering on
purpose, since no authoring surface produces such a program.

None prints a scalar. The sentences are unchanged, but they no longer
go through `Debug`: `Verb`, `ArcMode`, `TargetKind` and `TipState` each
have an `identifier()` that returns the declared name (the first three
from the declaring macro, `TipState`'s pinned against its `Debug` by
`path_program::every_tip_state_is_identified_by_its_declared_name`), so
`scripts/gates/told-quantity-fields.pins` holds nothing. Owed: whether
these names get a prose form, or the table's identifiers are what this
reader should see.
