---
id: python-cannot-read-what-a-placement-places
kind: issue
title: Python has Doc.placements() but no read of which body a placement places, so a script cannot say "a placement reads X"
status: open
opened: 2026-10-08
---


Found by INTENT stage 2 unit C's Python lane (branch `intent/s2-c-world`).

`Doc.place(body, pose=None)` and `Doc.placements()` landed with the
world (`crates/pncad-py/src/py/doc.rs`), but nothing reads which body a
placement places, so `docs/guide/assembly.md`'s split example can only
count the remainder's placements
(`len(outcome.remainder.placements()) == 2`) where it means "a
placement reads `outcome.instance`". The spec asks for no such read;
a `Doc.placed_by(placement) -> NodeId` (or the node's operand read
through the existing node view) would let a script say it.
