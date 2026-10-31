---
id: an-edges-parameter-ends-enter-a-carrier-piece-as-constants
kind: issue
title: An edge's parameter ends enter a carrier piece's knots as f64 constants, read through their bracket
status: open
opened: 2026-10-11
priority: P2
refs: [4708, held-value-equalities-decided-on-the-value-channel]
---


Found by the review of PR 4708 (`nurbs-weights-and-knots-are-the-scalar`
u4), R2's F3. These sites were deferred to "u4" by #4703's fix pass, and
u4 does not move them.

The geom-brep sites that cut a carrier to an edge take the edge's
parameter ends as `f64`, through the bracket of the edge's parameter
enclosure, and enter them into the piece's knots with `T::from_f64`:

- `geom-brep/src/edge_nurbs.rs`, `edge_piece`: a piece end that is no
  carrier breakpoint is `T::from_f64` of `t0.lo()` / `t1.hi()` (the
  `Knots::restructure` fresh entry), from `analytic_rung3`.
- `geom-brep/src/pcurve_cache/projected.rs`, `net_param`: the same for a
  projected image's piece ends (shared with partition and halving
  breaks, which are genuine constants).

At `Sym` the edge's parameter is a value in `T`, so this is a computed
value entered as a constant. The gate lists both as
`launders(an-edges-parameter-ends-enter-a-carrier-piece-as-constants)`.
A related, sound case: `edge_nurbs.rs`' `on_carrier_domain` re-expresses
the chart image on the carrier's `f64` domain and enters its knots
through `from_computed`, so the image's end knots are unknowns rather
than the carrier's held ends, and an identity between them cannot
discharge.

The fix: hand the edge's parameter ends (and the carrier's held domain)
to these sites in `T`, and build the piece's end knots from them. A
certification-ring caller that has only the bracket keeps the bracket's
ends, as the certificate's own data.
