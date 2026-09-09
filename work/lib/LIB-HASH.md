---
id: LIB-HASH
kind: unit
title: every comparable enum mirror hashes
status: review
opened: 2026-09-08
branch: lib/hash
refs: [pncad-py-comparable-enums-do-not-hash]
pr: 2242
---


Closes `pncad-py-comparable-enums-do-not-hash`: every comparable enum
mirror in `crates/pncad-py/src/py/` hashes, so a tally by tag is the
set or dict it always wanted to be.

## Delivered

- **All 24 fieldless mirrors carry `eq, eq_int, frozen, hash`**, and
  derive `Eq, Hash` beside `Clone, Copy, PartialEq`. Twenty-four, not
  the twenty-three the item listed: `AssertionDir`
  (`crates/pncad-py/src/py/measure.rs`) arrived after it was filed.
  Re-counted at the merge base with the item's own pattern.
- **`frozen` IS required**, established from the pinned pyo3 0.29.0
  rather than assumed: `pyclass_hash` refuses `hash` without both
  `frozen` and `eq`. A fieldless mirror satisfies `frozen` for
  nothing — it has no fields to mutate. Said once, at
  `crates/pncad-py/src/py/checks.rs`'s first mirror (`CheckId`), as a
  `//` comment rather than a `///` one so it does not become a
  Python docstring about Rust attributes.
- **None of the 24 is deliberately unhashable.** Every doc comment on
  every mirror read; not one mentions hashing, keys, or a reason to
  refuse them. The item's expectation ("no") holds.
- **`Denotation` hashes** (`crates/pncad-py/src/py/readback.rs`),
  hand-written beside its hand-written `__eq__` over the same pair
  `(tied, candidates)`. It IS a key: a caller tallies denotations —
  how many names in this evaluation are ties, and how wide.
- **`crates/pncad-py/tests/test_hashability.py`**, enumerated from the
  compiled module: every mirror member hashes, the whole surface goes
  into one set and reads back out of one dict, hash agrees with
  equality over every ordered pair, a mirror minted by a DOOR keys the
  same as the class attribute, and two door-minted `Denotation`s are
  one key. A `TestNothingComparesWithoutHashing` reads `__hash__` off
  every class in `vars(pncad)` and requires an unhashable one to be on
  a roster with a reason — so a 25th mirror added without `hash` fails
  two rows, and so does a new value class that compares without
  hashing.
- **Falsifiability measured, not asserted.** `frozen, hash` was
  removed from `SurfaceKind` alone and the crate rebuilt: the new file
  went to 2 failures and 3 errors, naming `SurfaceKind` in both
  rosters. Restored.
- **`crates/pncad-py/tests/test_face_frame.py`'s tally is a set**, and
  the comment explaining why it could not be is gone.
- **`crates/pncad-py/pncad.pyi` declares `Denotation.__hash__`.** The
  stub's convention is that a hand-written `__eq__`/`__hash__` pair is
  declared and the mirrors' pyo3-derived dunders are not (no mirror
  declares `__eq__` today either); `Denotation` was the one class
  declaring half a pair. The 24 mirrors are left alone, on that
  convention.

## Deviations and findings

- **Two findings filed, not fixed**, both inside LIB's fence and both
  out of this unit's scope by its own brief:
  `pncad-py-value-classes-compare-without-hashing` (nine value classes
  compare without hashing; `Expr` and `MeasureExpr` are the two that
  document why they do not) and
  `pncad-py-stub-omits-eq-on-three-mate-classes` (the stub omits an
  `__eq__` three classes define).
- **The filed item is named in the test WITHOUT its `.md` suffix**, and
  the reason is written at the constant that holds it
  (`crates/pncad-py/tests/test_hashability.py`'s `FILED`). A `.md`
  string literal in a python file under `crates/` is a page
  `scripts/ci-filter.py` must resolve to a repo path before it can
  decide the change tier, and it fails closed on one it cannot —
  correctly, since the failure it guards is a consumed page dropping
  into the docs tier where its suite stops running. Caught by the
  hosted gate on the first head, not by the unit's own verification
  list, which does not carry the selftest.
- **No kernel change.** `editor_core::Denotation` derives `PartialEq,
  Eq` and not `Hash`; adding it would have been the shorter fix and
  reaches outside the bindings, so the binding hashes the projection
  its own `__eq__` reads instead.
- **No `variant`/`kind` value or ordering moved.** `frozen`, `hash`
  and the `Eq, Hash` derives add slots; they touch no discriminant, no
  `eq_int` integer and no variant order.
