---
id: offset-axial-zero-refusals-state-an-exact-tie
kind: issue
title: topo: offset_axial's Zero refusals state an exact tie or an in-plane vector, and print no margin
status: open
opened: 2026-10-13
priority: P3
cost: E
---

(Filed by the ENCL implementer of `zero-arm-payloads-overclaim-and-print-no-margin`, from PR 4976's review of its sweep's blind spot: a Zero verdict carried as a bare `Sign::Zero`, whose refusal states the zero as an exact fact.)

A `Sign::Zero` from `decide` is a band-decided reading — an exact zero, a straddling enclosure, or a nonzero size inside the zero band — not a proof of equality. The refusals below state it as exact, hedge nothing, and print no margin (D4 ¶1 (i): the margin rides the payload). geom-brep's Zero-arm payloads now say "… is not shown … (margin … m)".

## What

`crates/topo/src/offset_axial.rs`:
- the `decide("offset_axial_branch", …)` call over the two azimuths' distances from the moved corner, `Ok(Sign::Zero)` arm: "the two azimuths stand the same distance from the corner being moved, so which one the offset keeps is not determined";
- `reauthor`'s `decide("offset_axial_reauthor_rise", …)`, `Ok(Sign::Zero)` arm: "an extruded point whose extrusion vector lies in its own sketch plane, so no station along it is read".

## Repair shape

Carry the decided margin into the refusal (`refuse` takes a static string today) and say "is not shown … (margin … m)" of the reading.
