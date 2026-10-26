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


## Duplication triage (INTENT, 2026-10-26)

Classified on Ev's ask, via the CLEAVE note on INTENT's log, against main `e92fb951` and DESIGN `[one-way.places]` / `[one-way.readings]`. The four classes:
- **design signal:** DESIGN names no single home, so a design conversation is owed;
- **mechanical:** DESIGN names the home; delete the off-home copy and repoint its users;
- **D10 debris:** a D10 stage retires one copy;
- **not duplication.**

**Mechanical** (cluster D). Repoint to `Span::contact` and delete `contain::point_on_circle`. Also off-home: `containment::circle_miss` lives in topo and the boolean reads it. DESIGN home: `[one-way.readings]` (geom). **Cluster D, carrier readings re-spelled outside their named home.** DESIGN names the home. Ev (2026-10-26): the first fix is making the canonical reading the path of least resistance (the API consumers need), with a gate only as a backstop. 

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P3 → P1: two rows for one point-on-carrier fact: two implementations of one underlying logic, P1 by `work/README.md` Priority.
- Moved to ONEHOME (`work/onehome/`) by `git mv`, id and body unchanged: the P1 track opened for CLEAVE's duplication rows; INTENT is sorting each into design signal or a mechanical deletion (`work/intent/log.md`, 2026-10-26), and the sort goes on the row. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
