---
id: sketchplane-compares-and-hashes-over-a-rust-type-that-derives-neither
kind: issue
title: pncad-py: SketchPlane compares and hashes over a Rust type that derives neither PartialEq nor Hash
status: open
opened: 2026-09-09
needs_ev: true
---



Found by LIB-HASH-2's two-way walk. Every other class in the "Python
implements more than the Rust type derives" cell adds a HASH to a type
that derives `PartialEq`. `SketchPlane` adds both halves, so it is its
own shape.

## What the mirror says

`profile::SketchPlane<f64>` (`crates/profile/src/lib.rs:548`) derives
`Clone, Copy, Debug` — no `PartialEq`, no `Eq`, no `Hash`. The Python
class (`crates/pncad-py/src/py/doc.rs:1353`) carries both:

- `__eq__` (`:1451`) compares through `bit_eq` — the twelve `f64`
  words of origin, `u`, `v` and normal, bit for bit.
- `__hash__` (`:1458`) hashes those same twelve bit patterns, so it
  agrees with the comparison by construction and `-0.0` keeps its own
  bucket deliberately.

That last property is why the pair is INTERNALLY consistent and was
recorded as such: LIB-ZERO's survey of the boundary's twelve
hand-written hashes counted four that hash raw bits, "one on purpose
(a sketch plane whose `__eq__` also compares bits)".

## The question

The mirror rule says a Python value class mirrors its Rust type's
derives. Here the Rust type declares no comparison at all, so a
faithful mirror would make `SketchPlane` compare by identity — which
would silently change an observable answer (`plane == plane` on two
reads of the same sketch frame) rather than merely removing a key.
Three readings, none ruled on:

- The Rust omission is deliberate: a plane carrying floats has no
  correct `PartialEq`, and `bit_eq` exists precisely because the
  derived one would be wrong. Then Python inventing an IEEE-free
  bit equality at the boundary is the same judgement made once more,
  and the mirror rule is the wrong lens.
- The Rust omission is a `#[derive]` list nobody needed, and the
  repair is upward: `bit_eq` becomes a `PartialEq`/`Eq`/`Hash` triple
  in `profile` and Python mirrors it.
- The rule reaches it and `SketchPlane` should compare by identity in
  Python. This is the reading with an observable cost, and the one
  that needs Ev.

Filed rather than fixed for that reason: it is the only row in the
cell where applying the rule mechanically would change what a
comparison ANSWERS, not just whether a value can be a key.

## Question for Ev (2026-09-09, LIB orchestrator; `[ev]` PR)

Asked as one question with three siblings — the full text is on `the-tag-mirrors-hash-over-kernel-enums-that-derive-no-hash`. For this item under the recommended (A): the Python pair stays (bit equality and a bit hash, internally consistent, the one deliberate boundary invention), with an upward option for `profile` to make `bit_eq` its `PartialEq/Eq/Hash` if that program wants the mirror exact. Under (B): `SketchPlane` compares by identity in Python — the only row where the rule changes what `==` answers.
