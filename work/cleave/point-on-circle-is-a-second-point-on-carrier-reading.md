---
id: point-on-circle-is-a-second-point-on-carrier-reading
kind: issue
title: contain::point_on_circle (bool_contact_arc) reads a split point's distance from its circle beside the point-on-span reading's gap row, which the boundary pre-pass now reads
status: open
opened: 2026-10-15
priority: P3
cost: E
refs: [the-curve-pair-reader]
---

Found by stage (c) of `work/fuse/the-curve-pair-reader.md`, which moved `LoopEdge::contact`'s conic rows (`ConicArc::hit`) onto `geom_brep::curve_pair::Span::contact`.

`reduce::split_other_at_point` asks whether a split point lies on its circle carrier through `contain::point_on_circle`, row `bool_contact_arc`. That row's doc said the boundary pre-pass asks the same distance under the same row name. It no longer does: the pre-pass reads `Span::contact`, whose gap row `curve_pair_span_gap` reads the point's distance from the carrier's point at its foot. On a circle that is the same distance, under a second row.

Two rows of one fact can decide it differently in the band (D3, "One reading per question"). A route is to read the split point by `Span::contact` on the edge's span, with its parameter the foot: `On` is the split, `End` the end's vertex, and `Off`/`Carrier`/`Past` the broken invariant the caller already refuses.
