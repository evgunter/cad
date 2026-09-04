---
id: geometry-recourse-dead-at-line-ring
kind: issue
title: FILLET3_GEOMETRY_RECOURSE is front-door reachable at a non-circle ring and endorses a request the caller already made
status: open
opened: 2026-09-04
refs: [recourse-sentences-owe-followability-pin]
---

## The witness

A unit cube with a square pocket in its top face (subtract a 0.3-cube
translated to `(0.35, 0.35, 0.8)`), then `fillet_edges` on the twelve
OUTER edges. At every radius from 0.05 to 0.34 the door answers

    UnsupportedGeometry :: a ring edge's carrier is not a circle — the
    exact ring-clearance check covers circle rings only (at edge …) —
    blend edges whose supports are planes (for a fillet's rim, also a
    sphere cap) and whose stored carriers are lines and circles; the
    surgery's exact forms cover no other stored shape, and approximating
    one is not implemented

from `ring_circle` (`crates/sweep/src/blend/surgery.rs:1665`), carrying
`FILLET3_GEOMETRY_RECOURSE` (`crates/sweep/src/blend/mod.rs`). At 0.36
the clearance screen answers instead. Probe:
`crates/sweep/tests/review_fillet_e2_probes.rs`.

## Why this is a dead recourse (issue 1278's class)

The sentence tells the caller to request edges whose supports are
planes and whose carriers are lines and circles. Every requested edge
already is: twelve plane–plane edges on line carriers. The carrier the
refusal objects to belongs to a RING on a support face — a feature the
caller did not request and the sentence does not mention — so
following the sentence changes nothing, and no radius builds. A caller
cannot fillet the outer edges of a pocketed box at all, and is told
their request already satisfies the condition they are refused on.

## What PR 1753 says about it

PR 1753 files `FILLET3_GEOMETRY_RECOURSE` as **unreachable** ("the
chain-shape gate reads the support pair before the geometry frontier")
on one fixture, an arc-sided prism, and adds a doc comment at the
constant saying no caller has been handed the sentence. Both are false
of this witness. The row
`blend_recourse_followability::the_geometry_recourse_has_no_front_door_witness`
does not go red here because its premise (the arc prism) is not the
failing mode.

## The decision owed

Either `ring_circle` learns line rings (the exact clearance of a line
segment against a line trimline is a closed form the surgery already
has for lines), or this refusal stops carrying the geometry recourse
and names the ring and the lever that exists (move or remove the
feature; blend a face without a non-circular ring). Whichever is
chosen, the composed pin the class asks for is owed: the second request
executed and its outcome asserted.
