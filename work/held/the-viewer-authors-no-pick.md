---
id: the-viewer-authors-no-pick
kind: issue
title: The viewer authors no pick: a form over one piece of an instance's copy reads the whole copy
status: open
opened: 2026-10-23
priority: P2
cost: M
refs: [5636]
---

Found by D′3 (`assemble-is-an-operator-and-the-product-is-a-slot`), closing `the-viewer-authors-no-read-of-one-copy`. An instance defines one body, the copy of its part's product, and a body of several parts is an ordinary operand, so every form the viewer offers over an instance now reads its copy whole and is admitted. Reading ONE piece of that copy needs a pick (`Operand::Pick { placement, of }`, `VarDef::Pick`): the path from the copy through the part's placements and `⊔` operands to the piece. No viewer gesture writes one. `crates/viewer/src/world.rs:155` (`on_body`) answers the operand a form reads from the node a face pick lands on, and that is always the copy whole.

So a boolean, fillet or split meant for one part of a multi-part instance takes every part of it. Shape of a fix: when a face pick lands on an instance's copy whose part's product is a `⊔`, read the face's name (`From { read, of }` keyed at the part's `⊔`) for the member it came through and write the pick of that member. A pick authored twice is two variables, so the form should offer the existing pick where one is held (DESIGN `[placement.placement]`).

## Slate triage (2026-10-28, INTENT)

Moved from `work/intent/` by `git mv`, id and body unchanged: residue of INTENT's D10 build that belongs to DOORS's charter (the viewer's authoring doors). INTENT keeps D10's core (G and its riders, the stage-3 umbrellas, the carry, the refactor rows, the roots rename); Ev asked for INTENT's slate to be cut to its charter and its residue handed on (in chat, 2026-10-28).

## Moved (2026-11-01)

Cut from DOORS into HELD (DOORS over budget at 52 of 30 once D′3's triggers fired; `work/doors/log.md`, 2026-11-01). Id and body unchanged. (DOORS orchestrator)
