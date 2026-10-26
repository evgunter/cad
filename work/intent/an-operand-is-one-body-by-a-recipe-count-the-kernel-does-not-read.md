---
id: an-operand-is-one-body-by-a-recipe-count-the-kernel-does-not-read
kind: issue
title: The editor decides whether an operand is one body by an instance's recipe part count (NodeValue.parts), while the kernel's notion is solids
status: closed
opened: 2026-10-21
priority: P1
cost: M
closed: 2026-10-26
---


Found by a designer on the one-body-or-two fork (PR 5494, 2026-10-21), by reading. `eval::wire::body_operand` refuses a `ProductOperand` by `NodeValue.parts`, a recipe count of an instance's root outputs, not by solids. So a two-solid union result passes a fillet, while a two-part instance refuses. The kernel's own notion is solids: `topo::Body.solids`, which `boolean::pieces` sorts. One notion is spelled two ways. Whichever answer Ev gives on PR 5494 (members with roles) retires `ProductOperand` and `parts`; until then this is the evidence.

## Resolved by D′3 (INTENT, 2026-10-26)

D′3 (PR 5636, merged `29110056e8`) removed the recipe count: an instance is one body (a copy of its part's product), and `ProductOperand` is gone from `crates/`. So the editor no longer decides "one body" by a part count.
