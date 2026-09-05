---
id: two-parts-of-one-body-at-one-boolean-refuse-as-ray-exhausted
kind: issue
title: Two Parts selecting one body at one boolean are DM5-distinct inputs carrying one Arc; the boolean refuses as Containment(RayExhausted), not as the same body twice
status: open
opened: 2026-09-04
---

## What

`Node::Part` (DOCM-2, PR #1860) hands on the selected half's or
instance's own `Arc` (`crates/editor-core/src/eval/wire.rs`,
`wire_part`). Two Parts selecting the SAME half of one split — or
`Part(Instance(0))` beside its master, since instance 0 IS the input's
`Arc` (`wire_pattern`) — are two node ids, so DM5's pairwise-distinct
check on a boolean's inputs (`Node::input_fault`,
`crates/editor-core/src/node.rs`) admits them, and the boolean receives
the identical body twice: same allocation, identical `GeomSource`s on
every description.

## Measured

`docm/2-review-r2` @02d23644,
`tests/docm2_r2_probes.rs::r2p9_two_parts_of_one_half_at_one_boolean`:
`Boolean(Union)` of two `Part(Above)` of one split, and
`Boolean(Subtract)` of `Part(Instance(0))` from its master. Both refuse
TYPED — no panic, no assertion — but as
`NodeErrorKind::Boolean(Containment(RayExhausted))`: the diagnosis a
point-in-solid ray gives up with when every candidate face is its own
twin. Nothing says "the same body twice", which is what happened.

## What it is not

Not a DOCM-2 defect: the projection is right to hand on the Arc, and
DM5 is stated over node ids (a Part is a distinct node). Not the
boolean's either — it was handed a state nothing could produce before
this node existed.

## What a ruling decides

Whether "the same body twice" is a DM5 refusal at the edit door
(`InputFault` widened to a through-Part identity, which the door
cannot see without an evaluation) or a typed evaluation refusal at the
boolean (a `WrongOperand`-class arm naming both inputs, decided by
`Arc::ptr_eq` on the two operands before the kernel runs — cheap,
exact, and the one place both bodies are in hand). The second is
where the fact is readable; the first is where DM5 lives.
