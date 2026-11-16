---
id: a-reachs-pivot-is-chosen-per-kind
kind: issue
title: Reach::foot_on chooses its pivot by the carrier's kind, while the lever reads the span's cover
status: closed
opened: 2026-11-14
priority: P1
cost: M
closed: 2026-11-15
branch: boxes/reach-region
---

`geom_brep::Reach::foot_on` (`crates/geom-brep/src/extent.rs`) picks the axis point a lever is read from by matching on the carrier's kind:
- a conic takes the foot of its centre;
- a line or a NURBS takes the minimax of its ends or its whole control net (`minimax_on_axis`).
`reading_point` invents a second point (the carrier at `t0`).

Since PR 6596, the lever and the axial window read the span's cover, so `Reach::Span` describes one region twice. Measured: the two descriptions disagree by up to 5.75× on sub-spans, and by up to 1000× on a short arc read on an axis through its centre. Choosing a pivot per axis also splits a parallel pair's feet: `d` reads 3.61 for axes 3 apart.

The `cover-kind-match` gate (PR 6957) pins the site. The design fork is the `[ev]` PR on branch `boxes/ev-footon-pivot`.

Settled on branch `boxes/reach-region` as approved on PR 6968. `Reach::Span` is `Reach::Region(Cover)`, built by `Reach::of_span` (a typed `Uncovered`) or from topo's `edge_cover`. `foot_on` reads a face at the foot of its `at` and every other reach at the middle of its window along the axis (`range_along`). `reading_point`, `span_points` and `minimax_on_axis` are gone. A parallel pair's `d` is the greater of each foot's distance to the other line. The `cover-kind-match` pin for `extent.rs` is 14.
