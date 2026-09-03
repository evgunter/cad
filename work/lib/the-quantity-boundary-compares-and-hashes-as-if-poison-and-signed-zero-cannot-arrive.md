---
id: the-quantity-boundary-compares-and-hashes-as-if-poison-and-signed-zero-cannot-arrive
kind: issue
title: the quantity boundary compares and hashes as if poison and signed zero cannot arrive
status: open
opened: 2026-09-03
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
