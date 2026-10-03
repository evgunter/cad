---
id: limb3-at-rest-proves-the-graph-not-the-arc
kind: issue
title: at rest, limb 3 proves its chain a graph, not C2's one arc; a carrier along a wall side within the band cannot prove more
status: open
opened: 2026-10-03
priority: P2
cost: M
design: true
refs: [limb-3-tube-banks-a-second-arc-as-accounted]
---

## Found (limb-3 lane, branch `ssi/limb3-one-arc`, 2026-10-03)

`limb-3-tube-banks-a-second-arc-as-accounted` made limb 3 prove the
chain holds one arc wherever a search banks the tube. `certify_rung3`
(`ssi.rs`) is the at-rest door. It serves the plane × NURBS edge lane
(`edge_nurbs.rs`, `plane_nurbs_limbs`), the pcurve cache
(`pcurve_cache.rs`) and the adversarial suite, and it banks nothing,
so it passes `certify::Banked::No`. There limb 3 still proves only the
graph: at most one solution per slice of each box. C2 claims one arc
for every fitted `Intersection`.

The one-arc proof cannot simply be required there. The edge lane's
own class is a planar cap meeting a wall along the wall's boundary
iso-curve (`tests/m7_8_plane_nurbs_edge.rs`):

- The carrier runs along a domain side that lies on the plane within
  the band. `φ` along that side is zero up to rounding, so no walk of
  the box's boundary can count its simple zeros.
- The proof refuses at every rung. Requiring it broke all four m7_8
  certifying rows, four `r1_pxn_probes` rows and
  `d290_r2_e2e::plane_nurbs_limbs_image_lands_on_the_carriers_own_domain`.
- At ε scale such a locus need not be one arc: an edge within ε of the
  plane can cross it many times.

## What is open (a design question)

What limb 3 should claim at rest:

1. **The graph, stated as the at-rest claim.** C2 says limb 3 proves
   one arc where a search banks the tube, and the graph at rest.
   Nothing at rest reads the stronger claim today: D2's component
   selection takes the witness's component, and accounting is a
   search's.
2. **One arc, with a side within the band as the arc's own.** The walk
   accepts a box edge on the wall's domain side when that side lies on
   the plane within the band and the carrier runs along it. The claim
   weakens to "one arc, or the band-coincident side it runs along",
   which is what C3's `Side` region already says for a search.
3. **One arc at rest, refusing the band-coincident edge class** toward
   C7 or a boundary-curve rung. That would be a behaviour change for
   real B-rep edges.
