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

None prints a scalar, so the build pinned them rather than changed
them: `scripts/gates/told-quantity-fields.pins` holds
seven lines of `crates/profile/src/path/program.rs`. Owed: either the names get a
prose form and the pin goes to zero, or the pin's reason stands.
