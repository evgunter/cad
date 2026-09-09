---
id: the-quantity-boundary-compares-and-hashes-as-if-poison-and-signed-zero-cannot-arrive
kind: issue
title: the quantity boundary compares and hashes as if poison and signed zero cannot arrive
status: open
opened: 2026-09-03
refs: [1668]
---

Banked at LIB-B-FORMAT. Found by BINDING the display formatter: it is
the first Python door that has to have an opinion about a non-finite
quantity, and asking that question of `Length` turned up two answers
its neighbours give wrongly. Neither is a formatter defect — the
formatter is the one door here that is right about both floats — and
neither is a kernel need: `crates/quantity/src/lib.rs:69-72` states
outright that "the newtypes are plain value wrappers and do not refuse
non-finite floats themselves; the fail-loud doors are where values
enter recipe data or the kernel". The kernel is deliberate. The
BINDING is what assumed otherwise.

Not fixed in that unit's diff on purpose: both are semantics calls on
`__richcmp__` / `__hash__`, doors LIB-B-FORMAT does not bind, and one
of them changes an observable raise.

## Finding 1: `==` on a non-finite quantity RAISES

`crates/pncad-py/src/py/quantity.rs`'s `continuous_quantity!` macro
routes all six comparisons through one `partial_cmp`:

```rust
match self.0.$canonical().partial_cmp(&rhs.0.$canonical()) {
    Some(ordering) => Ok(op.matches(ordering)),
    None => Err(pyo3::exceptions::PyValueError::new_err(
        "quantity comparison against a non-finite value",
    )),
}
```

`CompareOp::Eq` and `Ne` go through it too, so with
`nan = float("nan") * mm`:

```pycon
>>> nan == nan
ValueError: quantity comparison against a non-finite value
>>> nan == 1 * m
ValueError: quantity comparison against a non-finite value
```

A bare `float("nan")` answers `False` and `True` respectively, as IEEE
and the Python data model require. Raising instead is loud in the wrong
place: `x in some_list`, `assertEqual`, a dict lookup that collides,
and `==` inside any library code all become raises rather than answers
— and the raise is an untyped `ValueError` rather than a `PncadError`,
so it is outside the hierarchy every other refusal at this boundary is
in.

The comment above that arm claimed the arm was unreachable — "NaN
cannot arise from the constructors (the boundary refuses non-finite
input)". Both halves are false; nothing refuses, and
`float("nan") * mm` constructs. LIB-B-FORMAT corrected the comment in
place (it is in a file that unit touches) and pinned the behaviour as
it stands in `crates/pncad-py/tests/test_quantities.py`, so a change
here goes red rather than silent.

Ordering is a separate question from equality, and the current answer
may well be right for `<`: refusing to order poison is defensible, and
`sorted()` over a list containing one arguably should be loud. What is
not defensible is the same arm answering `==`.

## Finding 2: `-0.0` and `0.0` are equal quantities with unequal hashes

```pycon
>>> a, b = -0.0 * m, 0.0 * m
>>> a == b
True
>>> hash(a) == hash(b)
False
```

`__richcmp__` compares through `partial_cmp`, where `-0.0 == 0.0`;
`__hash__` is `self.0.$canonical().to_bits()`, where they differ. That
violates the data-model invariant Python relies on everywhere — equal
objects hash equal — so a `set` can hold both, and a `dict` keyed on
one misses the other.

The library already knows this fold is needed one layer up:
`crates/pncad-py/src/py/expr.rs`'s `Expr` docstring records that
"`DocParam` folds `-0.0` and hashes". The quantity newtypes do not.

The fix is one line per type — fold the zero before `to_bits` — and it
is the direction that keeps `__hash__` honest without touching `==`.
NaN interacts: `to_bits` also hashes distinct NaN payloads apart, which
is harmless only while `==` raises. Decide the two together.

## Where the formatter makes this visible, and why that is not a bug

`(-0.0 * m).format(m)` is `"-0 m"` and `(0.0 * m).format(m)` is
`"0 m"` — two quantities that compare equal, displayed differently.
That is correct and must stay: the formatter's pin is that the text
reads back to the value's exact BITS
(`crates/quantity/src/fmt.rs:7-14`), and `-0.0` and `0.0` are different
bits. `format` is the door that is right about the distinction; `==` is
the loose one and `__hash__` the inconsistent one. LIB-B-FORMAT pins
the trio as it stands, so the relationship is a checked fact rather
than a paragraph in this file.

## Shape of a fix

A boundary micro-unit on `py/quantity.rs`, not a kernel change:

1. Answer `Eq`/`Ne` without `partial_cmp` — plain IEEE equality on the
   canonical floats — and either keep the refusal for the four
   ordering operators or make it a typed `PncadError` arm rather than
   a bare `ValueError`.
2. Fold `-0.0` to `0.0` before `to_bits` in `__hash__`, the fold
   `DocParam` already does.

Both are behaviour changes on doors that already carry pins, which is
why they wait for a unit that owns them rather than riding a family
sweep.

## Question for Ev (2026-09-08, LIB orchestrator; `[ev]` PR)

Two semantics calls on the quantity boundary, both pinned as they
stand: `==` on a non-finite quantity RAISES a bare `ValueError` (IEEE
and the Python data model say `False`), and `-0.0 * m == 0.0 * m` is
`True` with unequal hashes (a `set` can hold both). The kernel is
deliberate — the newtypes do not refuse non-finite floats; the
fail-loud doors are where values enter recipe data — so the binding
assumed what the kernel does not promise.

- **(A) `==`/`!=` answer plain IEEE equality and `__hash__` folds the
  zero before `to_bits`; ORDERING on a non-finite quantity keeps
  refusing, as a typed `PncadError` arm rather than a bare
  `ValueError`.** Equality and hashing obey the data model everywhere
  library code relies on it; the one loud door left is the one that
  is defensible (sorting poison). Recommended.
- **(B) Everything answers IEEE** — ordering too (`<` on NaN is
  `False`); quiet where the rest of the boundary is loud.
- **(C) Leave both**, and document the raise.

Recommendation: **(A)**.

### Where it happens, and the class — added 2026-09-09 after Ev asked

Not a test: it is the runtime meaning of `==`, `<` and `hash()` on a
Python `Length`/`Angle` — `crates/pncad-py/src/py/quantity.rs`'s
`continuous_quantity!` macro routes all six comparisons through one
`partial_cmp` and hashes raw bits. The tests pin it as it stands.

The class is confined to the binding (the kernel is deliberate: IEEE
`PartialEq`, bits only in `bit_eq`). Twelve hand-written `__hash__`
sites over floats exist at the boundary; `fold_zero`
(`quantity.rs:465`) is used by `WrittenLength`, `WrittenAngle` and
`DocParam`, and four sites still hash raw bits — one on purpose (a
sketch plane whose `__eq__` also compares bits). **(A) as the class**:
one shared fold-then-bits hash for every class whose `__eq__` is
IEEE, bit-hash only where `__eq__` is bit-eq, and a row in the
module-enumerated `tests/test_hashability.py` asserting, for every
float-constructible class, that the `-0.0` and `0.0` forms hash equal
whenever they compare equal; plus the one macro arm so `==` on NaN
answers `False`. Ordering on a non-finite quantity keeps refusing,
typed.

### (B′), added 2026-09-09 after Ev noted the kernel omits these in favour of the funnel

Correct: Rust `Length`/`Angle` derive `PartialEq` and `PartialOrd`
and NOT `Hash` or `Ord` (`crates/quantity/src/lib.rs:89-95`), and
non-finite is refused at the funnel (`Expr::literal`'s door), never
by the newtype. The faithful mirror is **(B′)**: `==` and the four
orderings answer IEEE exactly as Rust's `PartialOrd` does (every
comparison on NaN is `False`; nothing raises — (B)'s answer), and
`__hash__` is REMOVED from `Length`/`Angle`, because Rust has none
and a quantity is a magnitude, not a key — which dissolves the
`-0.0`/NaN hash question for the newtypes instead of folding it.
`DocParam`, `WrittenLength` and `WrittenAngle` keep their hashes
(recipe data past the funnel; they already fold the zero). The two
newtypes go on `tests/test_hashability.py`'s `UNHASHABLE` roster with
that reason. Cost, stated: `{1 * m}` stops working; nothing in the
suite does it today, and a caller who wants a key has
`WrittenLength`. Recommendation revised: **(B′)**.

## Ruled (2026-09-09, Ev on `[ev]` PR #2233): (B′)

The Python `Length` and `Angle` mirror the Rust newtypes' derives.
`==`, `!=`, `<`, `<=`, `>`, `>=` answer as Rust's `PartialEq`/
`PartialOrd` do — IEEE: a non-finite quantity compares `False` on
every relation (no `ValueError`, typed or bare), and `-0.0 * m ==
0.0 * m` stays `True`. `__hash__` is removed from both classes, as
the newtypes implement no `Hash`, and the two join
`test_hashability.py`'s `UNHASHABLE` roster with that reason: the
funnel refuses non-finite at the doors where a value enters recipe
data, so the boundary type does not re-decide it. Mechanical unit
LIB-ZERO.
