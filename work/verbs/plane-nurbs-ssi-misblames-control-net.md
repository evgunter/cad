---
id: plane-nurbs-ssi-misblames-control-net
kind: issue
title: ssi - plane_nurbs_ssi blames the wall's control net for the PLANE's own non-finite origin
status: open
opened: 2026-08-29
github: 1218
refs: [762]
---

## From GitHub issue 1218

Opened 2026-08-29; 0 comments.

Measured while writing the D286 fixture for the CERT-2 lane (SMELL scan Track Q, residue of issue 762). Tests-only scope from that lane's fence, so it is filed rather than fixed.

## What happens

`plane_nurbs_ssi(&plane, &wall, …)` with a **finite, ordinary wall** and a plane whose `origin` carries `+∞` (or `NaN`) in one coordinate refuses with:

```
ssi: the NURBS control-net enclosure poisoned over a cell — a weight so small that
the rational's own denominator underflows to zero, or homogeneous arithmetic that
does not stay finite over the net
```

The wall is the acceptance suite's own substrate net: order-1 control points, unit weights, a chart speed of ~25 m per parameter unit. Nothing about its control net poisoned anything. The plane did.

## Why

`sweep_chart_plane`'s cell predicate is `φ = n·(S(u,v) − p₀)`, built as

```rust
RingInterval::point(plane_normal.x) * (b.x - RingInterval::point(plane_origin.x)) + …
```

`RingInterval::point` is poison for a non-finite argument — deliberately, and its doc says why ("an infinite *point* would launder overflow into data"). So a non-finite `p₀` poisons `φ` on the first cell, and the chart sweep's poison arm answers. That arm's sentence describes the net, because the net is the only thing it was written about.

## Why it matters

This is the same shape as issue 762's third item, one lane over: **not a wrong answer, a wrong diagnosis.** A caller handed this refusal goes and audits the NURBS wall it names, and the wall is fine. The operand at fault is the one the message does not mention.

## Suggested shape of the fix

Refuse the plane at the door, where `plane_nurbs_ssi` already destructures `Surface::Plane { origin, normal, u_ref }` and already refuses `WrongLane` there. A plane whose origin or normal is not finite denotes no plane at all, and the operation can say so in its own terms before any sweep runs — the same argument the chart-speed guard ten lines below makes for the wall.

Note the D2-addendum row-0 question is live here: `Surface::Plane` admits a non-finite origin because `Point3<f64>` does. Whether that state should be representable at all is the prior question; a typed refusal at the door is the answer if it should not be.

## Home

`crates/geom-brep/src/ssi.rs` and `ssi/*` are in VERBS' `paths:` territory; the finding was measured by a SMELL Track Q lane whose fence is tests-only.
