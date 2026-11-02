---
id: param-span-recourse-for-a-carrier-that-stalls-at-an-interior-parameter
kind: issue
title: geom-brep: the attach gate's ParamSpan lever ('runs forward over a clear length') for an edge whose carrier stalls at an interior parameter
status: open
opened: 2026-10-31
priority: P3
cost: E
---

Filed by CARVE's `carve/pinned-corners` lane, from its brief's
off-question (the hinged loft's measured refusal in
`work/carve/a-wall-pinned-between-two-loft-sections-refuses-at-the-wrong-door.md`).

## What

A loft hinged on one of its edges used to reach the attach gate with
seams that double back at the pinned corner, and refused
`Escalated { check: ParamSpan, margin −0.349 }` ending in "move the
geometry so this edge is not vanishingly short". That edge is not
short: its carrier (a wall's boundary iso) stalls at an interior
parameter and turns back.

Measured on main at commit c9c20ba
(`crates/sweep/examples/carve_pin_phase1.rs` on `carve/pinned-corners`)
the renders have moved since PR 5138:

- a v-degree-2 hinge refuses `SpanMeterCollapsed` ("…runs steadily
  forward, never stalling or turning back; if it does neither, the
  speed floor has reached its limit…"), which does describe a stall;
- a zero-length strut refuses `StatedInterval { Forward(Zero) }`,
  "runs forward over a clear length", which fits.

The loft door now refuses these bodies before assembly
(`sweep::LoftError::PinnedCorner`), so the loft no longer reaches the
gate with them.

## Question

Does any producer still reach `CertCheck::ParamSpan`'s own lever
(`crates/geom-brep/src/certify.rs`, `CertCheck::ending`) with a carrier
that stalls inside its stored interval rather than one whose interval
is short or reversed? If not, close this row; if so, the lever wants
the stall reading `SpanMeterCollapsed`'s already gives.
