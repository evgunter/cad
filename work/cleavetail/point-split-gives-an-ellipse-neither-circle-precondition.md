---
id: point-split-gives-an-ellipse-neither-circle-precondition
kind: issue
title: reduce::split_other_at_point checks a circle's point on its carrier and its span within a period, and an ellipse gets neither
status: open
opened: 2026-10-12
priority: P3
cost: E
---



Found by PR 4782's sweep (`cleave/one-carrier-reads-ellipses`), and
raised again by its review.

## What

`reduce::split_other_at_point` checks two preconditions for a `Circle`
before reading the split parameter with `Curve3::param_near` anchored
at the span's midpoint:

- the point is ON the carrier (`point_on_circle`, row
  `bool_contact_arc`);
- the span is at most a period (`bool_split_span_period`), because
  past a period the azimuth aliases by 2π and `split_edge`'s
  interiority gate cannot see it.

An `Ellipse` reaches the same `param_near` read and is asked neither.
Since PR 4782, the interior question splits boundary ellipse edges at
`OnEdge` crossings, so this path is more reachable than it was.

## Direction

Ask both of an ellipse:
- the on-carrier distance, through the boundary pre-pass's conic arm;
- the period guard, levered by the larger semi-axis.

Alternatively, state why the boundary pre-pass's `OnEdge` decision
already covers the point, and put that statement at the site.

## Slate triage (2026-10-26, CLEAVE)

- Moved to CLEAVETAIL (`work/cleavetail/`) by `git mv`, id and body unchanged: the track opened for CLEAVE's P3 and P4 tail: refusal stories, guards that cannot go red, latent soundness and hygiene. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
