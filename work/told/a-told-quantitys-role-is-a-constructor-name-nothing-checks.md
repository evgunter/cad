---
id: a-told-quantitys-role-is-a-constructor-name-nothing-checks
kind: issue
title: A told quantity's role and dimension are constructor names nothing checks
status: open
opened: 2026-11-20
priority: P2
cost: M
design: true
---


Found by the review of the told-quantity build (PR 7159), as the class
behind its one MAJOR.

`geom_core::Told::given(&x, …)` makes any scalar "the caller's own
number", told to every digit and handed back by `Told::as_given`.
Nothing ties the constructor to where `x` came from. Two sites in
`crates/profile` told a derived number as given and were fixed on the
PR: an `ArcLen` leg's `len / r` (`path/verbs.rs`, `sweep_gates`, now a
`SweepDatum`), and a `Via` leg's bulge derived from its through-point
(`path/family.rs`, `bulge_carrier`). Both printed float noise
(`turns 6.999999999999999 rad`) and let a computed number out through
`as_given`.

The dimension is chosen the same way. `arc_fillet::carrier_tangent`
(`crates/profile/src/path/arc_fillet.rs`) tells the components of
`p − centre`, a displacement in metres, as ratios, because the refusal
variant it shares with the authored `toward(dx, dy)` door
(`PathError::NonFiniteDirection`, `UnderflowedDirection`) writes no
unit. Harmless at the magnitudes those refusals fire at, and false.

Every later per-crate sweep makes these two choices at dozens of sites,
by hand, with no reviewer aid. The sites the build checked are listed in
the PR body.

**The question.** What makes a given number given. Candidates: a given
quantity is built only at the door that received it (a newtype the door
mints from its own argument, so a derived scalar has no route to it); a
gate over `Told::given` call sites whose argument is not a parameter of
the enclosing door; or review alone, with the list of sites kept in each
sweep's PR.
