---
id: an-operand-is-one-body-by-a-recipe-count-the-kernel-does-not-read
kind: issue
title: The editor decides whether an operand is one body by an instance's recipe part count (NodeValue.parts), while the kernel's notion is solids
status: parked
opened: 2026-10-21
priority: P1
cost: M
blocked_on: [assemble-is-an-operator-and-the-product-is-a-slot]
---


Found by a designer on the one-body-or-two fork (PR 5494, 2026-10-21), by reading. `eval::wire::body_operand` refuses a `ProductOperand` by `NodeValue.parts`, a recipe count of an instance's root outputs, not by solids. So a two-solid union result passes a fillet, while a two-part instance refuses. The kernel's own notion is solids: `topo::Body.solids`, which `boolean::pieces` sorts. One notion is spelled two ways. Whichever answer Ev gives on PR 5494 (members with roles) retires `ProductOperand` and `parts`; until then this is the evidence.
