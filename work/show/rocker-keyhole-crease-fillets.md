---
id: rocker-keyhole-crease-fillets
kind: unit
title: the rocker plate gains a keyhole and a D-bore whose convex line-arc creases fillet to the closed form
status: open
opened: 2026-10-02
priority: P3
cost: M
---

## What

BAND PRs #3243, #3271 and #3715: the convex creases where a keyhole's
slot meets its disc — a line × arc crease running through a cap ring —
fillet to the closed form, as do a D-bore's creases, and a bore lying
inside the material a fillet removes refuses `RingClearance` instead of
answering wrong. Evidence: `crates/sweep/tests/review_band_ruled_ring_probes.rs`
(`a_keyhole_fillets_its_convex_ring_creases_at_the_closed_form`) and
`band_ruled_cap_ring.rs`.

`rocker` is the tour's fillet-construction plate (its six profile
corners filleted through the PATHS doors, the eye slot's arc×arc tip).
Those are PROFILE fillets, in 2-D before the extrude. Add the 3-D
counterpart beside them: a keyhole (a mounting keyhole is what a
rocker plate hangs on) and/or a D-bore, extruded as inner loops, their
convex creases rounded AFTER the extrude by `fillet_edges` on the
vertical crease edges. The cell then reads as the two ways the kernel
rounds a plate's corners.

## Oracle

The closed form the review_band row asserts, per crease; the plate's
volume moves by exactly the creases' corner-square excess. If a crease
the user would naturally pick refuses `RingClearance`, that is the
door working: say so, narrate it, and pick a radius that clears.
