---
id: pncad-py-spells-the-hash-signed-zero-fold-twice
kind: issue
title: pncad-py states its hash's signed-zero fold twice: doc.rs and quantity.rs each define fold_zero
status: open
opened: 2026-11-17
priority: P4
cost: E
---

Found by the EXPORT signed-zero disclosure sweep
(`work/export/three-crates-fold-signed-zero-with-one-body-and-three-unlinked-arguments.md`).

`crates/pncad-py/src/py/doc.rs` `fold_zero` (`pub(crate)`) and
`crates/pncad-py/src/py/quantity.rs` `fold_zero` (private) are the same
function: one body (`if v == 0.0 { 0.0 } else { v }`), one crate, and
one reason — a `__hash__` must agree with the IEEE `__eq__` it mirrors
(`FreeVar`'s hash in `doc.rs`; `Length`'s and `Angle`'s in
`quantity.rs`).

The cross-crate copies of this body are deliberately unshared, because
each serves a different invariant or would cost a dependency edge
(the EXPORT item above). Neither reason holds here: same crate, same
invariant, and `doc.rs`'s copy is already `pub(crate)`. The disclosure
comments the sweep added name each copy from the other; the repair is
one `fold_zero` (in `quantity.rs`, which `doc.rs` already imports from),
and those two comments shrink to the cross-crate pointer.
