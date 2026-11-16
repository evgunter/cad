---
id: one-carrier-answers-two-ways-whether-a-parameter-is-on-it
kind: issue
title: One carrier, two answers to whether a parameter is on it: NurbsCurve3's evaluators extend the end span past the domain, and the curve-pair polish iterates there, while a window refuses past its span
status: open
opened: 2026-11-16
priority: P2
cost: M
---

Found by the walk-stop fork's designers (2026-11-16); checked against main
`2c17deabf`.

One carrier answers "is this parameter on the curve" two ways:

- **Past the domain, the end span's polynomial.** `NurbsCurve3::ders1`
  (`crates/geom/src/curves/nurbs.rs:1700`) and `derivative_at`
  (`nurbs.rs:1726-1730`, "outside the domain, the end span, whose
  polynomial extension it returns as [`Self::ders1`] does") answer a
  parameter outside the domain with the end span's extension, which the
  module calls documented garbage-out (`nurbs.rs:52-58`).
- **The curve-pair polish iterates there.** `polish::pair`
  (`crates/geom-brep/src/curve_pair/polish.rs:51`) takes Gauss–Newton steps
  with no hold to the domain; the across-planes candidate calls it with
  `keep = |_| true` (`crates/geom-brep/src/curve_pair/pose.rs:892`), so an
  iterate past the domain is read on the extension.
- **A window refuses past its span.**
  `a_window_restricted_past_its_span_refuses_at_every_scalar`
  (`crates/geom-core/src/spline/blossom.rs:801`): "The piece's polynomial
  past its span is not the curve; at `f64` the default restriction answers
  NaN where the certification scalar refuses, rather than extrapolating."

## Done when

A parameter past the domain reads one way at every door that evaluates the
carrier: refused, or held to the domain where a reader steps past it.

**Read the walk-stop fork first.** The open design fork `work/onehome/the-curve-pair-walks-stop-verdict-rests-on-which-heuristic-foot-it-polishes.md` may change
the fix; whoever takes this row reads that fork's outcome before writing
anything.
