---
id: LIB-ZERO
kind: unit
title: the quantity classes mirror the newtypes' derives: IEEE comparisons, no hash on Length and Angle
status: closed
opened: 2026-09-09
branch: lib/zero
refs: [the-quantity-boundary-compares-and-hashes-as-if-poison-and-signed-zero-cannot-arrive]
pr: 2259
closed: 2026-09-09
---


Mechanical unit under Ev's `(B′)` ruling on
`the-quantity-boundary-compares-and-hashes-as-if-poison-and-signed-zero-cannot-arrive`:
the Python `Length` and `Angle` mirror the Rust newtypes' derives.
Binding-only — `crates/quantity` and every other crate are untouched.

## The mirror table

Every class in `crates/pncad-py/src/py/quantity.rs`, measured against
the rule the ruling states.

| Python class | Rust type | Rust derives | Python dunders before | Action |
| --- | --- | --- | --- | --- |
| `Length` | `quantity::Length` | `PartialEq, PartialOrd` | six comparisons via one `partial_cmp` with a bare `ValueError` arm; `__hash__` = raw `to_bits` | **changed**: comparisons answer IEEE, nothing raises; `__hash__` gone |
| `Angle` | `quantity::Angle` | `PartialEq, PartialOrd` | the same macro | **changed**: the same, one splice down |
| `Count` | `quantity::Count` | `PartialEq, Eq, PartialOrd, Ord, Hash` | six comparisons via `Ord::cmp`; `__hash__` = the `i64` | **left**: Python implements exactly the derives (checked: `hash(Count(3)) == 3`, `Count(1) < Count(2)`) |
| `WrittenLength` | `quantity::WrittenLength` | `PartialEq` | `__eq__`; `__hash__` folds `-0.0` then hashes bits + unit symbol | **left**: more than the derives, and (B′) names it and keeps it — recipe data past the funnel, already folding the zero |
| `WrittenAngle` | `quantity::WrittenAngle` | `PartialEq` | the same | **left**: `WrittenLength`'s reason |
| `LengthUnit` | `quantity::LengthUnit` (the sealed view) | `PartialEq` | `__eq__`; `__hash__` over the row's symbol | **left and filed** — `the-unit-classes-hash-over-a-partialeq-only-newtype` |
| `AngleUnit` | `quantity::AngleUnit` (the sealed view) | `PartialEq` | the same | **left and filed**, with `LengthUnit` |

## Delivered

- **The comparison, in the `continuous_quantity!` macro**
  (`crates/pncad-py/src/py/quantity.rs`). All six operators answer on
  the canonical floats exactly as the derived `PartialOrd` does: a
  pair that does not order — a NaN operand — is `false` on every
  relation but `!=`, and `±inf` orders normally. The bare
  `ValueError` arm is gone, and no typed refusal replaces it. The
  comment that replaces the old one states the invariant (the
  newtypes refuse no float; the funnel refuses non-finite where a
  value enters recipe data, so the boundary type does not re-decide
  it) and carries no history. A cross-dimension or foreign operand
  still refuses, unchanged: `DimensionError` with `op` `"<=>"`, or a
  plain `TypeError`.
- **`__hash__` removed from both classes**, no replacement. PyO3
  0.29 puts `__hash__ = None` in the class dict for a class defining
  the comparisons and no hash, so nothing had to be set explicitly —
  verified on the compiled module (`Length.__hash__ is None`,
  `Angle.__hash__ is None`, `vars(Length)["__hash__"] is None`), and
  `{1 * m}` / `{1 * m: …}` raise Python's own `TypeError`.
- **The invariant is stated where the API documents itself**: the
  `Length` and `Angle` `#[pyclass]` docs and their `pncad.pyi`
  stanzas say the comparisons are IEEE and the class is unhashable,
  naming `WrittenLength` / `WrittenAngle` as the records that key.
  Neither `docs/GUIDE.md` nor `crates/pncad-py/README.md` teaches
  quantity comparison or hashing at all, so neither carried a
  sentence to correct — the class docs are the door a reader reaches.
- **The stub's two `__hash__` lines are gone**; `__eq__` stays, which
  is what `test_stubs.py`'s
  `test_a_class_that_compares_by_value_declares_it` reads.
- **`test_hashability.py`'s `UNHASHABLE` roster gains `Length` and
  `Angle`** with the ruling's reason. Its `FILED` entries are
  untouched (LIB-HASH-2's nine).
- **Deviation from the brief, stated**: the brief reads the mirror
  rule as reaching `WrittenLength` / `WrittenAngle` ("apply the rule
  the same way"). It does not, because the ruling it implements
  carves them out by name — "(B′) … `DocParam`, `WrittenLength` and
  `WrittenAngle` keep their hashes (recipe data past the funnel;
  they already fold the zero)". They are left alone and the table
  says so.
- **Filed rather than decided**: `LengthUnit` / `AngleUnit` hash over
  Rust views deriving `PartialEq` only, which is the same
  more-than-the-derives shape — but a unit is a table row rather
  than a magnitude, its hash agrees with its equality through the
  `UnitDef` seal, and `test_notation.py` pins it as a door.
  `work/lib/the-unit-classes-hash-over-a-partialeq-only-newtype.md`.

## Pins moved

- `test_quantities.py::test_comparing_a_non_finite_quantity_raises_today`
  → `test_a_non_finite_quantity_compares_as_ieee`: a row per operator
  on a NaN operand, on each side, plus the `±inf` rows the old pin
  never had.
- `test_quantities.py::test_signed_zero_displays_apart_while_comparing_equal`:
  keeps the `format`/`==` relationship, loses its
  `assertNotEqual(hash(below), hash(above))` line — there is no hash
  to disagree.
- New: `test_a_quantity_is_not_a_key`,
  `test_the_angle_surface_compares_the_same_way`,
  `test_comparing_against_another_dimension_still_refuses`.
- `test_hashability.py`'s roster and both of its guards
  (`test_every_unhashable_class_is_on_the_roster_with_a_reason`,
  `test_the_roster_carries_no_stale_entry`) stay green with the two
  new entries; the stub-drift guard stays green with the two stub
  lines gone.
