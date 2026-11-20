---
id: suites-read-a-told-quantitys-number-through-a-test-only-door
kind: issue
title: About ninety suite rows read a computed refusal number back, through a test-only door on Told
status: open
opened: 2026-11-20
priority: P3
cost: M
design: true
---

Found by the told-quantity build's measure-first pass
(`the-told-quantity-type-and-profiles-refusals-on-it`).

Moving `profile`'s payloads onto `geom_core::Told` broke 162 lines over
25 files of `crates/profile` (its suites and `path.rs`'s own test
module): every row that destructured a refusal and compared a scalar
field. Eighteen of those reads are of a **given** number and now go
through `Told::as_given`. The rest read a **computed** one — a turn
margin's sign, a setback against a closed form to 1e-12, a radius
lever's boundary and far edge — which the rule gives no accessor for.

The build kept them by adding a door no production build has
(`crates/geom-core/src/told.rs`, the `#[cfg(any(test, feature =
"test-support"))]` impl: `Told::pinned_ends`, `Told::pinned_edge`, and
`geom_core::test_support::told`), read at 56 sites in 22 files of
`crates/profile` (most through `crates/profile/tests/common/mod.rs`'s
`LegNumbers` and `LeverNumbers`). `demos/tour/tests/lily.rs`'s row,
which is outside the workspace and cannot enable the feature, now
asserts the refusal's kind and its sentence.

**The question.** Whether a suite may pin a payload against the scalar
its door computed. The rows that do are the ones that would catch a
door putting the wrong number in a refusal (`fillet_radius_at_anchor`
re-derives the lever from the carriers; `bool12_probes` checks the
margin is the authored miss), and a sentence told to four figures
cannot carry that. The alternatives are to make those doors return the
number beside the refusal for the suite to read, or to accept the
rendered text as the only observable and drop what it cannot show.
