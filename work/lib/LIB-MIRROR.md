---
id: LIB-MIRROR
kind: unit
title: the derives match on both sides: twelve tags, McConfig, Denotation and the units gain Hash upward; seven records and SketchPlane stop hashing in Python; SketchPlane gains PartialEq via bit_eq
status: closed
opened: 2026-09-09
branch: lib/mirror
pr: 2271
refs: [the-tag-mirrors-hash-over-kernel-enums-that-derive-no-hash, the-value-records-hash-by-hand-over-kernel-types-that-derive-no-hash, sketchplane-compares-and-hashes-over-a-rust-type-that-derives-neither, the-unit-classes-hash-over-a-partialeq-only-newtype]
closed: 2026-09-09
---



Mechanical unit under Ev's ruling **(A), case by case** on `[ev]` PR
#2265, carrying all four rows LIB-HASH-2's two-way walk left open. The
rule is the one (B′) states — **a Python value class mirrors its Rust
type's derives** — read in BOTH directions: `Hash` is absent where the
kernel omits it for funnel reasons; elsewhere Rust may derive it and
both sides always match; and no work is spent adding hashing where
nothing plausibly keys.

Spanning both sides by construction, so every kernel file it enters
belongs to another program: announced on `bool`, `curved`, `docm`,
`fix`, `msolve` and `tcost`, one dated line each.

## Delivered

- **Fourteen upward derive words, Python unchanged.** `Hash` on the
  twelve tag mirrors' kernel enums — `Advisory`, `ArcSide`,
  `ArcSweep`, `AxisSense`, `CheckId`, `CheckKind`, `FlushRung`,
  `MateRole`, `MateSide`, `CarrierRelation` (Python's
  `PlaneRelation`), `Severity`, `SurfaceKind` — and on `McConfig`
  (`crates/editor-core/src/mc.rs:77`) and `Denotation`
  (`crates/editor-core/src/names/interrogate.rs:53`). Every one sat at
  the `file:line` the items' tables cite; both structs already derived
  `Eq` and every field of each derives `Hash`, so the ruling's stop
  condition never fired. `CheckId` keeps `PartialOrd, Ord`.
- **`crates/quantity`: `Hash` by symbol.** `impl Hash` on `UnitDef`,
  `LengthUnit` and `AngleUnit` over the row's symbol, with `Eq` beside
  each — derived on the two views, by hand on `UnitDef`, whose derived
  comparison reads an `f64` factor that is a finite literal on every
  row the seal admits. The seal makes the symbol determine the row, so
  the hash agrees with the derived `PartialEq`; the argument is the
  impl's own doc. Test:
  `crates/quantity/src/tests.rs`'s
  `a_unit_row_and_its_view_are_keys_that_hash_by_symbol`. Python's
  unit hashes and `test_a_unit_is_usable_as_a_dict_key` stay.
- **`ScalarUnit` deliberately left.** The third sealed view has no
  Python mirror and exactly one inhabitant, so it is the case the
  ruling's "nothing plausibly keys" clause excludes. Home: this list
  and the PR body; nothing is filed, because nothing is open.
- **Eight Python `__hash__`s removed, Rust unchanged for seven of
  them.** `Frame`, `DocParamValue`, `Distribution`, `McMeasure`,
  `McAssertion`, `FaceCensus`, `ValidationFinding`, and `SketchPlane`.
  Each `__hash__` went with the doc comment that justified it and with
  the fold it carried; `fold_zero` and
  `Distribution::fold_signed_zeros` both stay, still reached by
  `DocParam.__hash__`. The stub's eight `__hash__` lines are gone and
  the three trailing stub comments that described a hash now describe
  its absence.
- **`SketchPlane` — the one row where Rust moved to meet Python.**
  `impl PartialEq for SketchPlane<f64>` delegating to `bit_eq`
  (`crates/profile/src/lib.rs`), so `==` answers bit-for-bit on both
  sides. `f64` and not a generic `T`, because `bit_eq` is `f64`-only.
  `PartialEq` and no `Eq`: no hash on either side. Test:
  `crates/profile/tests/sketch_plane.rs`'s
  `the_partial_eq_impl_is_bit_eq_and_answers_the_same_on_the_two_zeros`.
- **Eight roster rows in the mirror's voice**, each naming its Rust
  type and the derives it mirrors, and each answering the reading that
  pulls the other way — `ValidationFinding`'s says the binding's own
  projection is unhashed to match the findings beside it. The roster
  is 21 entries and both guards are green over it.
- **Four suite rows that hashed a now-unhashable class** pin
  `assertIsNone(cls.__hash__)` instead of dropping the assertion:
  `test_distributions.py`, `test_document.py`, `test_placed_union.py`,
  `test_monte_carlo.py`.
- **The two-way walk re-run, and the four cells stated.** 163 classes,
  105 identity-group, 58 by value: **A 18 → 34**, **B 13 → 21**,
  **C 0 → 0**, **D 27 → 3**. Cell D is now exactly the (B′) carve-out
  (`DocParam`, `WrittenLength`, `WrittenAngle`) and cell C is empty —
  which is what the measurement LIB-HASH-2 recorded was asking. That
  record is left as the measurement it was; nothing references its
  table, so nothing in it is restated here.
- **No tag mirror lost a hash**: `TestEveryMirrorIsAKey` still tallies
  the whole mirror surface through one set and one dict.

## Deviations

- The brief says `impl PartialEq for SketchPlane<T>`; the impl is for
  `SketchPlane<f64>`, because `bit_eq` is defined on `SketchPlane<f64>`
  alone and a plane over another `Real` has no bit reading to compare.
- `Eq` was added beside `Hash` on the three `quantity` types. `Hash`
  without `Eq` makes no `HashMap` key, so the "a unit is a key" claim
  the ruling rests on is not spellable without it.
