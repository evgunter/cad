---
id: held-value-equalities-decided-on-the-value-channel
kind: issue
title: Equalities between held NURBS values are decided on the f64 value channel, so at Sym they hold only at the witness
status: open
opened: 2026-10-11
priority: P2
refs: [nurbs-weights-and-knots-are-the-scalar, carriers-compare-in-canonical-form]
---


Found by the review of PR 4703 (`nurbs-weights-and-knots-are-the-scalar`
u3), F3.

Since u2 a NURBS carrier holds its knots and weights in `T`, with their
`f64` value channel beside them for structure. Some decisions that read
the channel are not structure. They are value identities between held
weights or knots, decided bitwise on the channel, so at `Sym` they hold
at the witness and carry nothing for the variables:

- `geom-brep/src/pcurve_cache.rs`, the arc-rim class (`arc_rim_weight`'s
  caller): the column's `1, w, 1, w, …` pattern decides `wᵢ == w₁` for
  every interior weight on the channel. The decided margin then certifies
  `w₁` alone against `cos(h/2)`. A column whose other interior weights
  are different functions of the variables, equal at the witness, passes
  with nothing decided about them.
- `pcurve_cache.rs`, the seam class: `shared_weights` / `shared_knots`
  decide that the carrier is the chart's own column ("one spline space")
  by channel equality of two carriers' knots and weights.
- `pcurve_cache/projected.rs`: the projected image's shared-net test
  (`net.knots() != c.knots() || net.weight_channel() != c.weight_channel()`).
- `topo/src/offset_derive.rs`: `holds_row` (`w0 != w1` along the
  boundary row and the one next in) and `translates_along`
  (`w[at(i, j)] != w[at(i, 0)]`) decide that a net's weights are
  constant along a row or a column on the channel (found while closing
  `nurbs-weights-and-knots-are-the-scalar`, u4).
- `sweep/src/skin.rs`, the skin in `T` (`the-skin-computes-its-values-in-t`,
  #4726):
  - `skin_on`'s integral lane decides `w == 1` on every section
    weight's channel and enters the constant `1` for the output
    weights, so a held weight that is a function whose channel rounds
    to exactly 1 (an arc sweep near 1e-8 rad) loses that function;
  - `skin_on` takes the u knots from section 0's held values after a
    channel-equality check of every section's knots, so the other
    sections' held knot values are dropped;
  - `geom/src/curves/fit.rs`' `averaged_knots_in` merges averages
    that round onto one value into one run holding the first's value.

Each claims an identity between two held values. Decided on the channel,
it is a numeric coincidence at `Sym`, not a theorem. The fix is to decide
each such identity as a margin in `T` (one decision per pair, or a
`max` of `|a − b|` whose `Sym` form discharges as a theorem when the
values are one node). The counted decisions move, so the goldens that
count decisions re-cut. Nothing reaches these sites with a variable
weight or knot today: the skinned arc wall (`the-skin-computes-its-values-in-t`)
is the first that will.
