---
id: MSOLVE-5
kind: unit
title: The at-rest gate refuses a mate read below a product root in the operand's voice, not as a vanished name
status: review
opened: 2026-09-06
refs: [assembly-gate-refuses-vanished-on-a-mate-read-below-a-pattern]
branch: msolve/5-read-below-a-root
pr: 2090
---



## Ruling (MSOLVE orchestrator, 2026-09-06)

`assembly-gate-refuses-vanished-on-a-mate-read-below-a-pattern`
(MSOLVE-1's correctness review, NOTE-4) is ruled IN as a unit that
changes what the refusal SAYS, not what is admitted. The gate keeps
refusing a mate read below a pattern — `mate1_member_vocab.rs::the_
master_name_spelling_still_refuses_vanished` pins that as ratified,
the canonical spelling being the `Instance(i)` head — but it stops
calling the name vanished: it asks the operand's own table, and a name
spelled there at a node the product does not list refuses
`RefusedRef::ReadBelowARoot { at }` in the operand's voice. `Vanished`
is then only ever a name that names nothing where the mate reads it.
The dead `NodeGone` arm goes with it. Minting on copy 0's row instead
is not this unit: it would reopen the pin, which is Ev's, and nobody
has asked for it.

## Spec (2026-09-06)

`docs/MSOLVE-5-SPEC.md`. Fence: `assembly.rs`, the Python projection
of `RefusedRef`, tests, `ASSEMBLY.md` A5. No consumer walk. Dispatches
from main once MSOLVE-3 (PR 2081) is in.
