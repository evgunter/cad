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

Nothing in behaviour — the fit is correct either way. What VIEW owes
is that the cost stays where it is documented:
`DocSession::landed_body` and `LandedRun::body` both name this case,
and `crates/viewer/tests/landing_gathers.rs` counts the gathers of the
paths that do NOT pay it. A row for the refused-gate path would need a
mate that certifies its way into a refusal, which is `asm::bench`'s
territory and is worth adding when one exists.
