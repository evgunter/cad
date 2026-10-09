---
id: conic-plane-root-at-an-arc-end-reads-the-minor-meter
kind: issue
title: A conic root near an arc end is read against the span at the minor semi-axis, so a steep ellipse's crossing is dropped as an endpoint
status: open
opened: 2026-10-08
---


Found by CONTACT-12's review (the census twin is fixed there, on branch
`contact/12-ef-crossing-cuts`).

`crates/topo/src/splitting/classify.rs` `span_roots` (the span half of
`conic_plane_meet`, which `plane_crossing_lane` serves to the splitting
lane and to `boolean/reduce.rs`) reads each conic root against the
edge's span through `split_conic_crossing_root`. It meters the gap as
`Margin::metered(t − t₀, InfSpeed(min(|s_u|, |s_v|)))`. Its Zero arm
treats the root as "at an endpoint vertex (already swept), nothing to
insert". On an ellipse, though, a Zero bounds only `Δt·minor ≤ ε`, and
the root can lie up to `(major/minor)·ε` from the end along the arc.
That end vertex is then read definitely off the plane by
`split_vertex_side`, so the crossing is neither swept nor inserted.

Witness shape (measured on the census twin): the cylinder
`x² + y² = 0.01` cut by the plane `z = 20x` gives an ellipse with
semi-axes `√4.01` and `0.1`. An arc that ends `8ε` in parameter short of
where it meets the plane crosses it `0.8ε` from the end by the minor
meter and `16ε` from it along the arc.

The census twin no longer reads the parameter: it places each carrier
root on the arc with `LoopEdge::contact` (metres along the carrier).
That needs a `ConicArc`, which this lane does not hold. The fix here is
either that, or metering the interiority by an arc-length bound that
is sound in both directions. Unverified in the splitting lane itself:
no splitting row was written.
