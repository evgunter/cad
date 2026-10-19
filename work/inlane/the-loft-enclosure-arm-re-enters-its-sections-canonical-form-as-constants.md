---
id: the-loft-enclosure-arm-re-enters-its-sections-canonical-form-as-constants
kind: issue
title: The loft's enclosure arm re-enters its sections' f64 canonical form as constants (ValidatedProfile::lift_onto), laundering at Sym
status: open
opened: 2026-10-19
priority: P2
cost: E
---


`ValidatedProfile::lift_onto` (`crates/profile/src/validate.rs`) enters an
`f64` canonical form at a lane scalar through `Real::from_f64`, a literal at
`Sym`. Its one production caller is sweep's loft at an enclosure scalar
(`crates/sweep/src/loft.rs`, the `enclosure` arm of the skin's build): it reads
its sections' value channel down to `f64`, selects the skin's structure there,
and lifts that canonical form back onto its lane planes. At `Sym<Interval>` the
sections are functions of the document's variables, so the lifted loops are
constants of a function the document does not compute. A `Zero` proved over
them beside the same variables' symbols elsewhere is the E12 hazard
(`the-pinned-profile-lift-launders-at-sym`, whose evaluator half closed when
the pinned lift retired: Ev's ruling A on #5068).

`scripts/gates/from-f64-sites.tsv` lists `crates/profile/src/validate.rs`'s four
sites as owed to this row.

The fix this row owes: the enclosure arm reads its sections at the lane, as its
planes already are (`self.places`), or enters the canonical form through
`Real::from_computed`, `ConstructedLoop::lift_computed`'s convention. Either
retires the four gate sites.
