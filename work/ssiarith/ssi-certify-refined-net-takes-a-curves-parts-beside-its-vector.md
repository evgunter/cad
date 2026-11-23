---
id: ssi-certify-refined-net-takes-a-curves-parts-beside-its-vector
kind: issue
title: ssi: certify's refined_net takes a carrier's weights and coordinate channels beside its knot vector, and returns the refined pair side by side
status: open
opened: 2026-10-23
---


Found by QUAD's sweep for `patch-doors-take-a-control-net-beside-two-vectors`
(the patch doors' net and weights beside two knot vectors): the same
shape on a curve, at `crates/geom-brep/src/ssi/certify.rs`.

- `refined_net(kv, weights, coords: &[Vec<Interval>])` takes a
  carrier's parts — the only caller, `box_chain`, hands it
  `carrier.knots_f64_interim()`, `carrier.weights_f64_interim()` and
  `carrier.certified_coords()` of one `NurbsCurve3` — and relates them
  by count alone: `coords` is refused when it is not three channels, and
  a channel of the wrong length "comes back refused, entry by entry"
  (`RingRefinement::refine`'s extent guard), which the row
  `refined_net_encloses_the_exact_refined_net` pins with a popped
  channel.
- It returns `(KnotVector, [Vec<Interval>; 3])`, the refined vector
  beside its refined channels, and `box_chain` re-pairs them with
  `kv.with_coeffs(...)` behind a refusal arm its own comment calls
  unreachable.

Disposition: take the carrier (whose constructor checked the counts)
and return the refined channels as one pair over the refined vector —
`geom_core::spline::TensorChannels` is the tensor form; the curve form
is `SplineCoeffs` per channel — deleting the count refusals and the
unreachable re-pairing arm with it. QUAD closed the patch-door
instance with a checked `PatchNet` (geom-brep `props::quad`) and by
having the rational lane's refinement return its base grid directly
(`refined_grid`), never a vector beside a net.
