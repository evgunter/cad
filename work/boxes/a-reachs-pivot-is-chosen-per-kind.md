---
id: a-reachs-pivot-is-chosen-per-kind
kind: issue
title: Reach::foot_on chooses its pivot by the carrier's kind, while the lever reads the span's cover
status: open
opened: 2026-11-14
priority: P1
cost: M
---

`geom_brep::Reach::foot_on` (`crates/geom-brep/src/extent.rs`) picks the axis point a lever is read from by matching on the carrier's kind:
- a conic takes the foot of its centre;
- a line or a NURBS takes the minimax of its ends or its whole control net (`minimax_on_axis`).
`reading_point` invents a second point (the carrier at `t0`).

Since PR 6596, the lever and the axial window read the span's cover, so `Reach::Span` describes one region twice. Measured: the two descriptions disagree by up to 5.75× on sub-spans, and by up to 1000× on a short arc read on an axis through its centre. Choosing a pivot per axis also splits a parallel pair's feet: `d` reads 3.61 for axes 3 apart.

The `cover-kind-match` gate (PR 6957) pins the site. The design fork is the `[ev]` PR on branch `boxes/ev-footon-pivot`.
