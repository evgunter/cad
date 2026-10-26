---
id: the-census-at-a-union-of-assembles-is-not-carried
kind: issue
title: The census at a ⊔ runs in the gather, so a ⊔ under a union carries no records into it
status: open
opened: 2026-10-23
---


D′3 (spec §5′.3, rows 189–191) puts the at-rest census at each `⊔`'s
door: its operands are arrivals, and a meeting decided there is
carried by whatever reads the `⊔`. D′3 lands the census in the
GATHER instead: `product::assemble_gathers` and `product_recorded`
run `record_at_rest` over every `⊔` of the document (the product's
own and every other) at product and check time, and `wire_assemble`
(`eval/wire.rs`) grafts the pieces and carries each operand's own
records but decides no meeting between them.

So the records of `(bolt ⊔ plate) ∪ bead` are not carried into the
union: the union reads a `⊔` value whose `contacts` hold only what
its operands brought, and the bolt–plate meeting is decided only
where the gather reaches that `⊔`. A `⊔` read by nothing but a union
is still censused (every `⊔` is gathered), but its records do not
reach the union's value.

Moving the census into the wire needs `record_at_rest`'s inputs
(`product::Source` tables keyed by `names::member_view`, the
coincidence rows) at evaluation time, which the gather builds today
from the evaluation after the run.
