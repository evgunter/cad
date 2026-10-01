---
id: a-same-operand-f7-refusal-is-rendered-as-a-declarable-undeclared-contact
kind: issue
title: The document layer renders a same-operand F7 refusal (two coplanar faces of one operand) as an undeclared contact between two members, offering a declaration no vocabulary can express
status: open
opened: 2026-09-28
priority: P2
cost: E
---


Found by both designers of the area-overlap fork (PR #3350), and not
checked beyond their reading. `editor-core`'s `eval/wire.rs`
`union_refusal` / `refusal_menu` turn an F7 `UndeclaredCoincidence`
whose two faces are BOTH on operand A into a cross-member
`UndeclaredContact` finding, with a declaration as its recourse. No
declaration step covers a same-operand face pair
(`DeclareUnsupportedPair`). The honest refusal says the operand is not
maximal-faced, and its recourse is to merge that body's faces first.
Once #3350's merge fix lands, a kernel boolean no longer produces such
an operand, but hand-built and imported bodies still can.
