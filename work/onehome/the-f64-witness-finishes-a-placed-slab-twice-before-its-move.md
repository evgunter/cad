---
id: the-f64-witness-finishes-a-placed-slab-twice-before-its-move
kind: issue
title: The f64 witness of a slab placed then unioned finishes the slab twice, with one fingerprint, before the move: four finishes against the box lane's three
status: open
opened: 2026-11-14
priority: P3
cost: M
---


Found by BOXFINISH's M1 measurement (2026-11-14, main `b363f5fa`, with
the box-scalar switch turned off on the probe branch
`boxfinish/probe-switch-off`; its finish tally is env-gated there,
`BOXFINISH_TRACE=1`).

In `onehome_switch_parity_interval::a_slab_placed_then_unioned_certifies_in_one_leaf`
the `f64` witness finishes the slab twice, with one vertex fingerprint,
before the move: four finishes, against the box lane's three (the
door, the moved leaf's re-finish through `LeafBody for AtRestBody::moved_by`,
the union's result). The cause is not traced, and whether it shows with
the switch on is not measured. Filed here because the two lanes' logs
are compared row for row (`docs/predicate-dimension-audit.md` F34), and
a finish one lane runs and the other does not is where that comparison
breaks.
