---
id: r2-union-wall-probe-only-prints
kind: issue
title: r2_the_two_union_walls_on_my_operands prints four union refusals and asserts none — its stated claim about operand order is unchecked
status: open
opened: 2026-09-15
---


## What

`r2_the_two_union_walls_on_my_operands`
(`demos/tour/tests/verbs_teapot_r2_probes.rs`) runs four unions — a
torus tube against a plain cylinder, a cone frustum against the same
can, each in both operand orders — and `println!`s
`pncad::topo::union(..).err()` for each. It asserts nothing, so no
outcome can red it: the gate could stop refusing, refuse with a
different variant, or name the wrong operand, and the row stays green.

Its own doc states a falsifiable claim the prints were gathered to
support: *"The gate scans `Operand::A` then `B`, offenders in ARENA
order against the other body's faces in ARENA order, and returns the
FIRST padded box overlap — so swapping the operands must swap which
side is named."* That is exactly the kind of predicate a bug could
break, and nothing checks it.

## Why it is not covered by the row it was named in

Named as evidence in `teapot-walls-have-no-suite-row` (closed
2026-09-15 by SUITE's `D403`, PR 2626). That row's fix put the
TEAPOT's own walls under a by-variant assertion in the suite. This
probe is a different subject: its operands are the reviewer's own can,
tube and cone, and its claim is about the operand gate's SCANNING
ORDER, not about the teapot's frontier. So closing that row leaves
this standing, and it is filed rather than left in a PR body.

## Fix

Assert what the doc says: each union refuses with the expected variant,
and the two orders name opposite operands. Cheap — the four calls are
already made and the values are already in hand; what is missing is
`assert!`s over them. If the scanning-order claim turns out not to hold
for one of the two pairs, that is the finding and the row becomes a
kernel question rather than a test one.

## Home

CURVED — the claim is about the curved-boolean operand gate's own
behaviour, which is this program's register
(`docs/KERNEL-VERBS.md`'s scope limits), and the row it was named in is
CURVED's.
