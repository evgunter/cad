---
id: point-on-circle-is-a-second-point-on-carrier-reading
kind: issue
title: contain::point_on_circle (bool_contact_arc) reads a split point's distance from its circle beside the point-on-span reading's gap row, which the boundary pre-pass now reads
status: open
opened: 2026-10-15
priority: P1
cost: E
refs: [5227]
---

Found by stage (c) of `the-curve-pair-reader` (closed, PRs 5154, 5162 and 5227), which moved `LoopEdge::contact`'s conic rows (`ConicArc::hit`) onto `geom_brep::curve_pair::Span::contact`.

`reduce::split_other_at_point` asks whether a split point lies on its circle carrier through `contain::point_on_circle`, row `bool_contact_arc`. That row's doc said the boundary pre-pass asks the same distance under the same row name. It no longer does: the pre-pass reads `Span::contact`, whose gap row `curve_pair_span_gap` reads the point's distance from the carrier's point at its foot. On a circle that is the same distance, under a second row.

Two rows of one fact can decide it differently in the band (D3, "One reading per question"). A route is to read the split point by `Span::contact` on the edge's span, with its parameter the foot: `On` is the split, `End` the end's vertex, and `Off`/`Carrier`/`Past` the broken invariant the caller already refuses.

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P3 → P1: two rows for one point-on-carrier fact: two implementations of one underlying logic, P1 by `work/README.md` Priority.
- Moved to ONEHOME (`work/onehome/`) by `git mv`, id and body unchanged: the P1 track opened for CLEAVE's duplication rows; INTENT is sorting each into design signal or a mechanical deletion (`work/intent/log.md`, 2026-10-26), and the sort goes on the row. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
