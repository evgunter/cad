---
id: a-two-chart-tube-cannot-prove-one-arc-along-a-chart-boundary
kind: issue
title: The two-chart tube cuts its boxes to the charts' domains, so a rim on a boundary row of both charts lies on a face of every box and limb 3 refuses it (lily wall 8)
status: open
opened: 2026-10-28
priority: P0
cost: H
refs: [a-plane-section-along-a-fits-window-edge-proves-no-one-arc, a-rim-between-two-spline-walls-is-certified-against-a-fitted-image-not-its-exact-one, a-pair-walk-end-at-a-domain-corner-refuses-undecided]
---

Lily wall 8's next stop, found by SPLICE's rim-exact-image lane once
`edge_nurbs::nurbs_pair_limbs` read the rim's limbs 1 and 2 through
each wall's exact `IsoArc` image. It is filed here because the ground
is the SSI tube's (`ssi/four.rs`, `ssi/certify.rs`); SPLICE's lily
witness waits on it.

## What

`nurbs_pair_limbs` certifies limb 3 through `ssi::certify::certify_branch`
with the two-chart tube (`probe_tube_pair`). `ssi::four::one_arc_pair`
cuts every chain box to the product of the two charts' knot rectangles
(`Box4::of_charts`) and asks each cut box for exactly two boundary roots
(`face_roots`). Lily's shared rim is the `v = 0` boundary row of BOTH
walls, so the whole arc lies on the `v_a = 0` and `v_b = 0` faces of
every cut box. Those faces hold a curve of solutions, not isolated ones,
so `face_roots` cannot resolve them and returns `None`, read as
`Shortfall::Undecided`.

The refusal is conservative, not unsound. It is the two-chart sibling of
`a-plane-section-along-a-fits-window-edge-proves-no-one-arc` (the chart
tube along a fit's window edge) and of the corner case in
`nurbs/a-pair-walk-end-at-a-domain-corner-refuses-undecided`.

## Measured

On `splice/rim-exact-image` (main plus the exact-image arm):

- `sweep::all a_sheath_grafted_on_its_blade::a_sheath_on_its_blade_unions_past_the_shared_rim`
  (5 stations), both orders: `PlaneNurbs(TubeNotOneArc { rungs: 16, cause:
  Undecided(ssi_tube_one_arc, MarginDiag(Invalid)) })` at ε 1e-9 and
  1e-12, `rungs: 8` at 1e-6. At 1e-9 the refusal is the same whether
  the tube is built about the exact image's polyline or the fitted
  image.
- demo-tour lily wall 8 (9 stations), default ε: `PlaneNurbs(TubeStraddles
  { verdict: Zero(margin 0.0), boxes: 16 })`. The graph margin's
  enclosure holds zero on the narrowest rung that answered, which is
  the same tube read about the same boundary arc.
- `geom-brep::all spline_pair_arc_rim::*`: a unit cylinder and a flat
  annulus meeting at right angles along their shared `v = 0` row refuse
  `TubeNotOneArc { rungs: 20 (14 at 1e-6), Undecided }` at every ε.

## What is open

A one-arc proof for a carrier that runs along a boundary of a chart, or
of both. Options include proving over uncut boxes against the charts'
own extension, or reading a domain face that holds the carrier as a
face the arc runs along rather than one it crosses. Either way the
section's accounting has to bank the same region the proof reads.
A question, not a fix (SPLICE orchestrator): when the arc is a shared
boundary row of both charts, is limb 3's one-arc question answerable
from that structure rather than from a box search? C5's "topology
decides first" rung is the echo. Each chart's certified `IsoArc` image
already states that the carrier is that chart's boundary row over the
edge's span. Unasked: whether that, with the two charts' transversality
along it, is enough to show that no other arc of the pair's locus lies in
the tube's region.

Until then every rim between two spline walls that is a boundary row
of either refuses at limb 3. That covers every loft or sweep body grafted
on another at a shared section.

