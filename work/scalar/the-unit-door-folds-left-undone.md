---
id: the-unit-door-folds-left-undone
kind: issue
title: The unit door keeps four rounding vocabularies, three decades and three division routines that may be one each
status: open
opened: 2026-11-22
priority: P2
cost: M
---


Found by PR 7291's reviews; not built there (the owner's direction to skip polish).

- Rounding vocabularies: `quantity::Rounding {Nearest, Up, Down}` (public), `quantity::exact::Magnitude {Floor, Ceil, Nearest, NearestAway}` (rounding of a magnitude, with std's tie rule for shortest digits), `geom_core::told::Side {AtLeast, AtMost}` (the side an offer must type back on), each mapped to the next by hand (`units.rs` `Quotient::at`, `told.rs` `Float::toward`). `viewer::props::Rounding` was folded into `quantity::Rounding` in PR 7291.
- Three `decade`s: `UnitDef::decade` and `InUnit::decade` (the public question), `exact::Real::decade` (its implementation); `Decimal`'s was renamed `leading_exponent` in PR 7291.
- Three division routines in `crates/quantity/src/exact.rs`: `Nat::div_small` (Machin's small divisors), `Nat::div_rem` (a quotient below `2^64`, with a `u128` path) and `scaled_quotient`'s bit-serial `u128` loop (the rounding's quotient without allocating shifted naturals). The limb fold written twice (`to_u128`, `top_u128`) was folded in PR 7291.
- `UnitDef` holds its factor twice (`factor: f64` and the exact `Factor`), kept equal by `quantity::tests::the_unit_table_is_the_whole_closed_set_and_reads_as_data`.
- `UnitDef::quotient` and `number_of` are one function and its `Option` wrapper; the wrapper's one reader is `viewer::props::written`, the field's "no reading" arm that the editable-field unit retires.
