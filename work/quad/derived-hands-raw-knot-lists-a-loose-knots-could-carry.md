---
id: derived-hands-raw-knot-lists-a-loose-knots-could-carry
kind: issue
title: Derived's SpanConstant and Discontinuous arms hand out raw knot slices, which props re-validates into a LooseKnots behind an unreachable!
status: closed
opened: 2026-10-22
priority: P1
cost: M
closed: 2026-10-22
pr: 5581
---


## Finding

`geom_core::spline::Derived` (`crates/geom-core/src/spline/hull.rs`,
`KnotVector::derived`) answers a derivative that no clamped vector can
hold with a raw slice: `SpanConstant(&[f64])` at degree 1, and
`Discontinuous { knots: &[f64], degree }` across a full-multiplicity
interior knot. Since D30 there is a type for exactly that shape,
`LooseKnots` (`crates/geom-core/src/spline/knots.rs`). Its contract is
end runs of `degree + 1`, any interior, degree 0 allowed. `Derived`
still does not produce one, so:

- `props::quad`'s `Loose::from_derived` (`crates/geom-brep/src/props/quad.rs`)
  copies the slice into `LooseKnots::new` and `unreachable!`s on a
  refusal. That re-validates, at every ladder build, a fact `derived()`
  established by construction. It is also reachable by a hand-built
  `Derived`, whose variant fields are public.
- The other consumers match the arms and discard the payload. Each
  spells the derivative's cause by the arm alone:
  `geom::curves::second_derivative`'s `nonrational_second_derivative_sup`,
  `mesh::nurbs_cert`'s `second`, `geom_brep::patch_bound`'s `of`, and
  `geom_brep::offset_fit`'s `build` (twice, plus its `lift` closure).
  They are unaffected by the payload's type, but any of them that
  later reads the list would read a raw slice with no contract.

**Shape of the fix:** `Derived::{SpanConstant, Discontinuous}` carry a
`LooseKnots`, or `Derived` gains a `loose()` that builds one without
re-checking. Either way, `from_derived`'s `unreachable!` goes. This
touches geom-core (knot and nurbs territory) plus the matching sites
above.

## Closed

PR 5581. `Derived<V>` is now `Vector(V)`, `SpanConstant(LooseKnots)` and `Discontinuous(LooseKnots)`, built by `LooseKnots::derivative_of`. It keeps two arms, because every consumer treats the two causes differently. quad's `Loose::from_derived` maps each arm directly, so its re-validation and `unreachable!` are gone. `Derived` loses its lifetime. Bits are unchanged. `knots-f64-interim` geom-core went from 28 to 27: the borrowed `&[f64]` field is gone. The review tier was the orchestrator's read: a type change the compiler checks, with no number moved.
