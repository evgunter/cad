---
id: coefficient-vector-pairing-survivors
kind: issue
title: The loose (knot vector, coefficient array) shape survives outside hull: evaluators, tensor grids, composition, a public green-integral door
status: open
opened: 2026-09-05
---


The shape `coefficients-carry-their-knot-vector` closed at `hull` — a
coefficient array beside a knot vector, or beside a span, related by
length alone — survives at these sites, none of them a hull door. Cites
are at the head of the fix-pass PR (`props/coeffs-fixpass`); each line
carries its one-line disposition. The unit's own sweep reported the
first three groups for placement; the dual review added the rest.

**Evaluators (the retired shape exactly)**

- `crates/geom-brep/src/props/quad.rs:610` `bspline_eval_ring(kv,
  coeffs, t)` — de Boor on a slice beside a vector, length-checked
  (`coeffs.len() != kv.control_count()` → poison). Disposition: an
  evaluator door on `SplineCoeffs` / `CoeffWindow` closes it, the same
  move `hull` took; PROPS' after S-CERT's exit (`quad.rs` is cert's
  until #1924 merges).
- `crates/geom-brep/src/props/quad.rs:1084` `bspline_eval_ring_in_span(
  coeffs, span, t)` — a slice beside a `Span`, the shape the unit
  retired from `hull`. Disposition: the `CoeffWindow` evaluator above.

**The ladder and the direction (owned pairs travelling loose)**

- `crates/geom-brep/src/props/quad.rs:691` `DerivLadder::build(kv,
  coeffs)` with `levels: [Option<(Option<KnotVector>,
  Vec<RingInterval>)>; 3]`, and `:1264` `collapse_1d(dir, coeffs, op)`
  with `Dir::Kv(kv)` beside a channel — each mints at the door it
  reaches (`range_hull`, `KnotVector::difference_coeffs`), so every
  refusal arm is dead by construction. Disposition: a level type that
  owns the pair, once the evaluators move; not before.
- `crates/geom-brep/src/props/quad.rs:756` `pub fn
  bspline_green_integral(kv, u_coeffs, v_coeffs, weights, a, b,
  pieces)` — a PUBLIC door taking the loose quadruple (weights refused
  unless all `1.0`). Disposition: take two `SplineCoeffs` minted by the
  caller (the pcurve's channels against its own vector), dropping the
  weights parameter with the refusal it feeds.

**Tensor grids (the tensor case of the same pairing)**

- `crates/geom-core/src/spline/net.rs:310` / `:317`
  `TensorNet::diff_{u,v}_knots(&self, kv)` — a net beside a vector,
  related per line by count (a short line poisons).
- `crates/geom-brep/src/props/quad.rs:1130` `PatchGrid::base(kv_u,
  kv_v, control)` and `:1552` `build(kv_u, kv_v, net)` — a grid beside
  two vectors.
- `crates/geom-core/src/spline/compose/tensor.rs:287`
  `tensor_channel(ku, kv, grid)` and
  `crates/geom-core/src/spline/compose/patch.rs:126`
  `PatchSpans::decompose(ku, kv, grid, extra_u, extra_v)` — the same,
  in composition.
  Disposition for the group: a `TensorCoeffs<'a>` borrowing both
  vectors beside the net, minted by count once — `SplineCoeffs` in two
  directions — with `diff_{u,v}_knots` its doors; S-CERT ground for
  `geom-core`, PROPS' for `quad.rs`, one unit when scheduled.

**Composition (S-CERT ground)**

- `crates/geom-core/src/spline/compose.rs:331` `to_bezier_spans(kv,
  coeffs)` and `:342` `to_bezier_spans_extra` — the Bernstein
  conversion takes the loose pair. Disposition: take a `SplineCoeffs`.
- `crates/geom-core/src/spline/compose.rs:1033` `linear_composite(data,
  coeffs, offset)` — matched by the reviewers' grep; **not the shape**:
  `coeffs` here are the linear form's per-channel weights, related to
  `CurveRingData::dims()` and to no knot vector. Disposition: none;
  recorded so the next sweep does not re-derive it.
- `crates/geom/src/surfaces/nurbs.rs:797` `map_u_columns(build: impl
  Fn(&KnotVector, &[f64]) -> …)` — the higher-order form of the
  knot-algebra plans' `(kv, weights)`, which the unit classed as a
  different family (plans, not bounds, each through `check_weights`).
  Disposition: goes with that family if it ever moves; not a hull door.

**The sweep's blind spot, stated.** The unit's patterns (A)/(B) match a
vector and a slice on ONE line and (C) recovers `coeffs:`/`weights:`
parameters with `-B3` context. They cannot match: a multi-line
signature whose vector and slice are three or more lines apart; a
parameter named `grid`, `net`, `control`, `line` or `u_coeffs` on a
line without the vector; a closure capturing a vector and taking a
slice (`DerivTake`, `TensorNet::diff_*` steps); a struct field holding
an owned `Vec` beside a vector (`DerivLadder::levels`, `Dir::Kv`); and
a higher-order parameter (`impl Fn(&KnotVector, &[f64])`). The sites
above under those heads were found by reading `quad.rs`, `net.rs`,
`compose/` and `surfaces/nurbs.rs`, not by the grep; a re-sweep owes a
pattern over `&\[.*\]` parameters within a signature's brace span.
