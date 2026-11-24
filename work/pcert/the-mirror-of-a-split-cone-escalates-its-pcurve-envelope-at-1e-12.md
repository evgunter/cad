---
id: the-mirror-of-a-split-cone-escalates-its-pcurve-envelope-at-1e-12
kind: issue
title: The mirror of a split cone escalates its pcurve envelope at ε = 1e-12 at Interval: an enclosure of 1.08e-12 against a band of 1e-12, on a cone cut through its wall alone
status: open
opened: 2026-11-23
priority: P2
cost: M
---


## Finding

From `embed/conic-symmetry` (2026-11-23), on main `1cf86c9e96`.

A frustum revolved a full turn about `y` and split by a tilted plane
through its wall alone, the side the normal points to, then mirrored
across the plane through `(0.2, 0.1, −0.3)` with normal
`(3/7, −6/7, 2/7)` by `topo::mirror_body`: at `CAD_TOLERANCE_EPS=1e-12`
the mirror door refuses

`Pcurve { source: Certify { half_edge: HalfEdgeKey(21v1), error: Escalated { check: Envelope, sample: 0, cause: Indeterminate { margin: MarginDiag(Enclosure { lo: 0.0, hi: 1.0753598283836185e-12, … }), band: Band { zero: 1e-12, escalate: 1e-11 }, predicate: Some("pcurve_envelope") } } } }`

for both cuts tried: radii 1 and 0.5 over a height of 1 cut at
`(0, 0.5, 0)` tilted 0.25 toward `+x`, and radii 0.7 and 0.31 over 0.9
cut at `(0, 0.43, 0)` tilted 0.3 toward the azimuth 0.7. The payload
above is the first's; the second refuses on the same half-edge with an
enclosure of `1.2774216889551278e-12` (the review of PR 7640 read both;
this row first said the two were the same to the digits printed, which
was wrong). The cut out
through the top cap mirrors. At the default ε and at 1e-6 all three
mirror at both scalars.

It is the `Interval` build that refuses; the `f64` mirror builds at
1e-12 and is asked. Which
edge `21v1` lies on was not read.

## Pin

`self_intersection_proves_injectivity::a_split_cone_refuses_on_a_pair_holding_its_wall`
skips the `Interval` twin of these two placements at ε below 1e-11,
matching the refusal by variant and printing `SKIPPED` with it, asks
the `f64` body all the same, and fails on any other refusal or on this
one anywhere else.
