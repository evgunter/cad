---
id: pncad-py-spells-the-hash-signed-zero-fold-twice
kind: issue
title: pncad-py states its hash's signed-zero fold twice: doc.rs and quantity.rs each define fold_zero
status: closed
opened: 2026-11-17
priority: P4
cost: E
closed: 2026-11-17
parent: three-crates-fold-signed-zero-with-one-body-and-three-unlinked-arguments
---

Found by the EXPORT signed-zero disclosure sweep
(`work/export/three-crates-fold-signed-zero-with-one-body-and-three-unlinked-arguments.md`).

`crates/pncad-py/src/py/doc.rs` `fold_zero` (`pub(crate)`) and
`crates/pncad-py/src/py/quantity.rs` `fold_zero` (private) are the same
function: one body (`if v == 0.0 { 0.0 } else { v }`), one crate, and
one reason — a `__hash__` must agree with the IEEE `__eq__` it mirrors
(`FreeVar`'s hash in `doc.rs`; `Length`'s and `Angle`'s in
`quantity.rs`).

## Closed (2026-11-17): one home in geom-core

Closed by PR 7067, the EXPORT unit that filed it. The review of that
PR showed the cross-crate copies did not need to stay apart either:
every crate holding one already reaches `geom_core`. So the fold now
has one home, `geom_core::fold_signed_zero`. Both `fold_zero`s are gone,
and the three `__hash__` sites call it directly.
