---
id: MSOLVE-5
kind: unit
title: The at-rest gate refuses a mate read below a product root in the operand's voice, not as a vanished name
status: closed
opened: 2026-09-06
refs: [assembly-gate-refuses-vanished-on-a-mate-read-below-a-pattern]
branch: msolve/5-read-below-a-root
closed: 2026-09-06
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

## Closed (2026-09-06, PR 2090)

Landed: `RefusedRef::ReadBelowARoot { at }`, raised by the at-rest gate
when the product's table is silent and the operand the mate reads at
spells the name with a face at a node the product does not list as a
root; `Vanished` now means neither table answers, and its sentence
says so; a non-face entry at the operand — unique or tied, the kind
taken from the name's own kind — refuses `NotAFace { kind }` before
the root question (the stop-clause ruling: a root's BODY row reached
"operand answers, root, product silent" because `carry_names` drops
body rows); a face row at a root with the product silent is
`carry_names`'s invariant, asserted in debug. The operand's table is
read through `interrogate::value_of`; no consumer is walked. The dead
`NodeGone` arm and its Python tag are gone; `ref_read_below_a_root`
and an `at` getter are in. Nothing admitted moved: the two ratified
pins still refuse, with the operand's word. Rows: the issue's
document, the nested document, an empty-boolean root, a tied face and
a tied edge below a root, a poisoned operand (the gather refuses
first), the display contract for every arm. The asymmetry the ruling
leaves — the product's own rows answer a tie before kind, the
operand's answer kind before a tie — is pinned by value and filed
(`work/issues/product-table-answers-a-tie-before-kind-the-operand-the-reverse`).
