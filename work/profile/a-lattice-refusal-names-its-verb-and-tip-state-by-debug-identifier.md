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
"the {verb:?} verb is not a legal continuation of tip state {state:?}":
`Verb`'s and `TipState`'s `Debug` identifiers (`TangentArcTo`,
`DirectedPoint`) inside a sentence. `Verb` has a `Display` that gives
the authoring spelling; the comment above `ReplayError`'s `Display`
says the lattice arms keep the other rendering on purpose, since no
authoring surface produces such a program. `TipState` has no prose
form.

It prints no scalar, so the build pinned it rather than changed it:
`scripts/gates/told-quantity-fields.pins` holds
`crates/profile/src/path/program.rs 2`. Owed: either the two names get
a prose form and the pin goes to zero, or the pin's reason stands.
