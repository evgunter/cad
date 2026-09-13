---
id: from-affines-identity-fast-path-cannot-change-its-answer
kind: issue
title: Frame::from_affine's identity fast path is a no-op: is_identity_bits() is true exactly when the value it discards already has IDENTITY's bits
status: open
opened: 2026-09-12
---


## Finding

Found by WIRE's `placement-prose` lane while giving
`crates/editor-core/src/placement.rs`'s exactness rule one home — the
paragraph over this branch was one of the nine, and re-homing it meant
asking what the branch buys. It buys nothing.

`Frame::from_affine` builds `out` by COPYING twelve `f64`s out of the
`Affine3`, then:

```rust
if out.is_identity_bits() { Self::IDENTITY } else { out }
```

`is_identity_bits` is `bit_eq(&Self::IDENTITY)`, which compares all
twelve stored coordinates by `to_bits()`, and `Frame` has no other
fields. So the guard is true **exactly when `out` already equals
`Frame::IDENTITY` bit for bit** — and the two arms return the same
twelve bit patterns. The branch cannot change the function's answer,
observably or otherwise.

Contrast `Frame::compose`, where the same-shaped fast path IS
observable: it skips an `Affine3` product, and the product does
arithmetic that moves bits (`placement.rs`'s `sample()` fixture carries
`-0.0` precisely so that `x + 0.0` separates a copy from a rebuild, and
its doc says the `-0.0` is load-bearing for exactly this). `from_affine`
has no arithmetic to skip.

The retired comment claimed more than the branch does:

> an unmated instance's solved relative pose must land on the identity's
> own bits, or the mate-less document's evaluation would differ from the
> pre-mate one by a rounding step that never happened

The obligation is real — `crates/editor-core/src/mate/solve.rs` feeds
solved poses through this door. What meets it is the **copy**, not the
branch: a pose that is bit-exactly identity arrives as identity's bits
whichever arm runs, and a pose that is merely *close* to identity is not
snapped by this branch either, because the test is bitwise. So the branch
is not the mechanism for the thing it was documented as protecting.

## Settled by a row, not by reasoning (2026-09-12, PR 2475's fix pass)

The argument above is now an instrument.
`crates/editor-core/src/placement.rs`'s
`from_affines_identity_branch_agrees_with_the_branch_free_copy` — written
by PR 2475's reviewer and adopted verbatim, authorship recorded here
because a comment is not the place for it — asserts that `from_affine`
and a branch-free copy of its body agree BY BITS at the exact identity
(where the branch fires), at values a bitwise test deliberately does not
snap (`-0.0`, a subnormal), and on the module's three fixtures. It then
asserts the branch is not vacuous in the other direction: the copy really
does test identity on `IDENTITY` and really does not one bit away from it.

Verified red-first: mutating the identity arm to return a different
frame fails the row at the `identity` case, with the other four rows in
the module still green.

So the branch's inertness is now a claim the suite keeps rather than a
paragraph anyone has to re-derive, and the one scenario this row left
open — *a `Frame` field that `bit_eq` does not compare* — is exactly what
the row goes red on.

## Disposition

Left in place by the lane that found it: WIRE's placement-prose unit was
a prose unit, and deleting a branch — even a provably inert one — on a
D-3 bit-exact path is a code decision the unit was not asked to make. It
is three lines and the argument above is complete, so it is cheap for
whoever takes it.

Two answers, and the first looks right:

1. **Delete the branch**, returning `out`. Nothing observable changes;
   the file loses a fast path that is not one, and `is_identity_bits`
   keeps its one real caller in `compose`.
2. **Keep it and say why at the site** — e.g. as a defence against a
   future `Frame` gaining a field that `bit_eq` does not compare, which
   would make the arms differ. Nothing today plans such a field, and a
   `#[serde(deny_unknown_fields)]` struct whose comparator walks every
   field is not the shape that acquires one quietly.

Citations accurate at `846def72a`.
