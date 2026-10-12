---
id: a-same-operand-f7-refusal-is-rendered-as-a-declarable-undeclared-contact
kind: issue
title: The document layer renders a same-operand F7 refusal (two coplanar faces of one operand) as an undeclared contact between two members, offering a declaration no vocabulary can express
status: closed
closed: 2026-10-10
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

## Parked on the D10 hold (2026-10-08)

The declare menu it renders (`editor-core` `eval/wire.rs` `union_refusal`, `refusal_menu`) is deleted when booleans glue on Zero. (CONTACT close-out triage; CONTACT's log (`docs/doc-ledger/contact-leaves-the-tracker.md` names the SHA it is read at).)

## Closed by INTENT stage 4 E (`intent/s4-e-glue-on-zero`) (2026-10-10)

E deletes the declare menu that rendered this refusal (`eval/wire.rs` `union_refusal` and `refusal_menu`), along with `UndeclaredCoincidence`. Two coplanar faces of one operand now reach the document as the boolean's own typed refusal (`NonMaximalFaces` at the operand gate), so no declaration is offered for them.
