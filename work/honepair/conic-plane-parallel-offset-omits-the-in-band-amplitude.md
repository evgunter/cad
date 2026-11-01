---
id: conic-plane-parallel-offset-omits-the-in-band-amplitude
kind: issue
title: a conic read parallel to a face plane serves its offset without the in-band amplitude beside it
status: open
opened: 2026-10-07
priority: P2
cost: M
---

## What

`split_conic_plane_parallel` (`crates/topo/src/splitting/classify.rs:507`)
routes a conic whose amplitude (its tilt times its radius) reads Zero
to `ConicPlaneMeet::Parallel { offset }`. `topo::boolean::reduce` then
serves "the edge lies in the face plane" on
`bool_conic_face_plane_offset` Zero (`boolean/reduce.rs:1318`) and
"clear" on `bool_arc_plane_side` Positive (`reduce.rs:3163`), neither
with the in-band amplitude beside the offset.

Found by the sweep of the TANG unit that closed
`cylinder-axis-rows-decide-tilt-and-gap-one-at-a-time` (`work/tang/`),
and filed on this slate, whose ground it lands on.

## The shape of a fix

Decide the served verdict on one margin carrying both terms, as the
section classifiers' `decide_across` (`crates/geom-brep/src/intersect.rs`)
and `carrier_cyl_reach` (`crates/topo/src/boolean/carrier_eq.rs`) do:
the zero side on `|datum| + tilt·lever`, a definite side on the datum
shrunk toward zero by the tilt, the tilt row kept only to route.

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P3 → P2: offset decided without the in-band amplitude beside it: margins decided one at a time, or one verdict read off two decisions: interval and error propagation, P2 by `work/README.md` Priority.
- Moved to HONEPAIR (`work/honepair/`) by `git mv`, id and body unchanged: the P2 track opened for one verdict decided on one margin at its own reach: rows that decide two margins one at a time, compose two readings, or lever a reading at the wrong extent. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
