---
id: from-affines-identity-fast-path-cannot-change-its-answer
kind: issue
title: Frame::from_affine's identity fast path is a no-op: is_identity_bits() is true exactly when the value it discards already has IDENTITY's bits
status: closed
pr: 2499
opened: 2026-09-12
closed: 2026-09-12
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

**Answer 1, taken.** The branch is deleted; `from_affine` is the copy.

The justification is NOT that the output is unchanged —
`memories/output-stability-as-justification.md` is explicit that an
argument of that shape justifies nothing. It is that the branch
**cannot change the answer**: `is_identity_bits()` is
`bit_eq(&IDENTITY)` over every stored coordinate and `Frame` has no
other field, so the arm fires exactly when the value it discards
already carries `IDENTITY`'s own bits. That is a property of the code,
and the adopted row pinned it before the deletion rather than after.

Answer 2 was refused: nothing plans a `Frame` field `bit_eq` does not
compare, and the branch would not defend against one anyway — a field
outside the comparator would be outside the copy too.

### The adopted row did not survive the deletion, and should not have

`from_affines_identity_branch_agrees_with_the_branch_free_copy`'s
subject was the branch. With the branch gone, `from_affine` and the
row's `copy_only` were the same twelve lines transcribed twice, and the
mutation the row was verified red against — a changed identity arm —
no longer exists. Nothing a bug could do makes that assertion false:
the one scenario it claimed to keep open, a `Frame` field `bit_eq` does
not compare, would stop `copy_only`'s struct literal COMPILING rather
than redden it. `docs/prompts/implementer-discipline.md` §2 governs,
and deleting is the repair.

What survives the deletion is a different claim, and it is real:
`from_affine` CARRIES the affine's coordinates and snaps nothing. A
door that rounded, or that admitted a near-identity by anything but
bits, would break it — so it has runtime values that falsify it.
`from_affine_carries_the_affines_bits_and_snaps_nothing` keeps that,
over the same six inputs, with the non-snapping assertions as the
teeth. Verified red-first against a tolerance-snapping `from_affine`:
red at the `-0.0 translation` case, the other five rows in the module
green.

Citations accurate at `846def72a`.
