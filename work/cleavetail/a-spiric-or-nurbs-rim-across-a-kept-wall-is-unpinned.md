---
id: a-spiric-or-nurbs-rim-across-a-kept-wall-is-unpinned
kind: issue
title: A stale spiric or NURBS rim across a kept wall reaches describe_edges' stale arm through no boolean today, so the arm's handling of them is unpinned
status: open
opened: 2026-10-13
priority: P3
cost: E
---


## Finding

Raised by PR 4856's review (NOTE-2 / Q6).

`boolean::ops::describe_edges`' stale arm restates a stale curved edge
of any carrier kind as a derived image in `s1`'s chart, using
`geom_brep::EdgeCurve::restated_in_chart`. For a spiric or NURBS carrier
the image comes from `chart_pcurve`'s spiric or projected lanes.

No boolean reaches the arm with either kind today. Every scene the
reviewer built refuses upstream:

| scene | refusal |
|---|---|
| torus ∩ slab | `SectionUnarmed` |
| loft ∩ slab | `CurvedPairUnsupported` |
| tube × perpendicular cylinder | `GermFrameUnsupported` |
| tube × off-axis ball | `CurvedBooleanUnsupported` |

So no row tests how the arm handles these kinds.

## Wanted

Once one of those doors opens, pin the scene where a spiric or NURBS rim
ends up across a kept wall. The stale arm should describe it as a
derived image in one of its faces' charts, in both member orders. The
pin to copy is `sweep/tests/a_ruling_lying_on_a_wall.rs`'s `across_a_wall`.

## Slate triage (2026-10-26, CLEAVE)

- Band P3: an arm no boolean reaches has no pin: a guard that cannot go red, P3 by `work/README.md` Priority.
- Priced `E`, the effort the body describes.
- Moved to CLEAVETAIL (`work/cleavetail/`) by `git mv`, id and body unchanged: the track opened for CLEAVE's P3 and P4 tail: refusal stories, guards that cannot go red, latent soundness and hygiene. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
