---
id: the-unit-classes-hash-over-a-partialeq-only-newtype
kind: issue
title: pncad-py: LengthUnit and AngleUnit hash over Rust views that derive PartialEq only
status: open
opened: 2026-09-09
---


Found by LIB-ZERO while measuring every class in
`crates/pncad-py/src/py/quantity.rs` against the rule Ev's (B′) ruling
states: **the Python class mirrors the Rust type's derives.** Two
classes in that file implement MORE than their Rust type derives and
the ruling does not name them, so this row asks rather than decides.

## What the mirror says

`quantity::LengthUnit` and `quantity::AngleUnit` are the sealed views
minted in `crates/quantity/src/units.rs:446-456`, and both derive
`Debug, Clone, Copy, PartialEq` — no `Eq`, no `Hash`, no ordering.
The Python classes carry a hand-written `__eq__` AND a hand-written
`__hash__` over the row's symbol
(`crates/pncad-py/src/py/quantity.rs:402-414` and `:449-460`).

Read as the mirror rule reads `Length` and `Angle`, that is a hash the
Rust surface does not have, and the same sentence that removed
`Length.__hash__` would remove these.

## Why it is not obvious the rule reaches them, and why LIB-ZERO left them

Three reasons pull the other way, and none of them was ruled on:

- **A unit is a tag, not a magnitude.** The ruling's reason for
  dropping the quantity hashes is that "a quantity is a magnitude, not
  a key". A `LengthUnit` is a table row — the closed, interned set the
  enum mirrors live in — and `crates/pncad-py/tests/test_hashability.py`
  opens by asserting the opposite premise for exactly that shape: "a
  comparable value is a KEY".
- **The hash is pinned as a door, deliberately.**
  `crates/pncad-py/tests/test_notation.py:188-195`
  (`test_a_unit_is_usable_as_a_dict_key`) tallies authored values by
  the unit read back off them. Closing the B-EXPR-READ census gap
  bound "equality and hashing on `LengthUnit` / `AngleUnit`" as one
  act (`crates/pncad-py/tests/test_binding_census.py:1682-1684`).
- **`UnitDef`'s seal makes the hash honest.** The symbol DETERMINES
  the row (`crates/quantity/src/units.rs:71-82`), so hashing the
  symbol agrees with the derived `PartialEq` it mirrors — this is not
  the `-0.0`/NaN inconsistency the quantity hashes had.

The reading that resolves it, if it is the right one: the derives are
mirrored for VALUE types, and the Rust omission here is a `#[derive]`
list nobody needed on the Rust side rather than a statement that a
unit is not a key — in which case the repair is upward, on
`crates/quantity`, adding `Eq, Hash` to the three view types and to
`UnitDef`. That is a kernel change and outside LIB-ZERO's fence.

## The same question, already answered, for the neighbours

`WrittenLength` and `WrittenAngle` are the other MORE-than-Rust pair
in that file — `crates/quantity/src/written.rs:71,80` derive
`PartialEq` only, and both Python classes hash
(`crates/pncad-py/src/py/quantity.rs:564` and `:628`, both through `fold_zero` at `:468`). They
are NOT this row: (B′) names them and keeps their hashes, because
they are recipe data past the funnel and already fold the zero.
