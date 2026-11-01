---
id: held-value-equalities-decided-on-the-value-channel
kind: issue
title: Equalities between held NURBS values are decided on the f64 value channel, so at Sym they hold only at the witness
status: open
opened: 2026-10-11
priority: P2
refs: [4708, 4628]
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
  (`net.knots() != c.knots() || net.weight_channel() != c.weight_channel()`, now spelled with the `_f64_interim` accessors).
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

## More sites, and the NURBS speed meter (NURBS, PR 4834's review, 2026-10-12)

PR 4834's FULL review found two more sites of this class:

- `geom-brep/src/props/loop_area.rs` `nurbs_vector_area`: `*w != 1.0`
  on the channel sends a carrier down the polynomial lane;
- `geom-brep/src/nurbs_iso.rs` `interior_iso_u`: `along_u_constant`,
  decided on the channel, gives `omega = 1`.

The NURBS speed meter's arm choice (`NurbsCurve3::speed_lower_bound`,
`crates/geom/src/curves/nurbs.rs`) now reads
`SpanLocate::plain_constant` (PR 4834): the integral arm only when every
weight is the same plain constant (an `f64`, a point `Interval`, a
`Dual` with zero derivative, a `Sym` literal), the rational arm
otherwise. That is sound and stricter than a `T`-margin, so it should
move to this class's primitive when that lands, or the primitive should
subsume it.


## What the speed meter's plain-constant arm costs at `Sym` (NURBS, 2026-10-12)

Measured while making the meter's memo invisible to a symbolic session
(`work/nurbs/the-speed-meter-memo-is-visible-to-a-sym-session.md`). In
the sweep thread-count digest, `validate_geometric` on the two `Sym`
lofts reads `frozen` 1060 / 1177 when the meter is computed inside the
session, on every merge measured up to PR 4849. From PR 4834's merge
on, it reads 1914 / 2094. The meter's only change there is the arm
choice above, so carriers whose equal weights are not `Sym` literals
now take the rational arm. Its forms overflow the session's budget,
almost all of them in the contradiction checks of signs the numeric
channel decided. No decision moves, and the digest's wall time does
not either. A primitive that discharged the weights' equality in `T`
would put those carriers back on the integral arm.
