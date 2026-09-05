---
id: refused-a5-gate-eats-the-body-the-fit-then-regathers
kind: issue
title: a refused A5 gate consumes the landing's body, so the display fit gathers that document again
status: open
opened: 2026-09-05
---


Disclosed by `scene-gathers-the-landed-product-twice-more`, which
removed the other two re-gathers and could not remove this one.

## What happens

`DocSession::land` keeps the aggregate its one gather produced
(`LandedRun::body`), so `DocSession::landed_body` hands it to the
display fit for nothing. There is one landing shape where it cannot:
an assembly-shaped document whose A5 gate REFUSES.

`assemble_gathered` (`crates/editor-core/src/assembly.rs:564`) takes
`Product<T>` **by value**. A certification returns the same body on
`Assembly` (`:104-114`), so that path hands it back; a refusal returns
an `AssemblyError` and the body is dropped inside the gate. So for
that one landing shape `landed_body` gathers again — once, memoized
into the landing, but a whole gather.

Measured on this lane (dev profile, 165 roots / 990 faces): a gather
is 87 ms and handing an existing body on is 2.4 ms of `Body::clone`
— which is why the clone is NOT taken pre-emptively on every landing
of every assembly document. The clone is 2.7% of a gather but is paid
per LANDING, while the gather it would save is paid per opened
document: for an edit session on an assembly that is a net loss after
the first few edits, so the cost is left where it is rare (a refused
gate) rather than spread over the common case.

## Why it is not fixed here

The fix is a door change in `crates/editor-core`, which is DOCM's
ground and not VIEW's to edit: either `assemble_gathered`'s refusal
carries the product back (`Err((AssemblyError, Product<T>))`, or an
error variant holding it), or a borrowing variant exists for callers
that still want the body. Both are DOCM's call, and both are cheap
compared with the gather they save.

## What VIEW owes meanwhile

Nothing in behaviour — the fit is correct either way, and the cost is
now spelled at the consumer rather than hidden: `app`'s fit calls
`scene::product_of_evaluation` on this path and nothing else does.

**The path is covered, not deferred.**
`landing_gathers.rs::a_refused_a5_gate_eats_the_body_and_says_so_by_its_absence`
authors a Tangent mate over `asm::bench` (the fixture
`assembly_display.rs` already uses to red the badge) and pins both
halves: the gather succeeded (`product_fault` is `None`) and the body
is gone. So this item is about the COST, not about an untested
branch — when editor-core's door changes, that row is what says the
body came back.
