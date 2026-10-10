---
id: nurbs-weights-and-knots-are-the-scalar
kind: issue
title: NURBS weights and knots are f64 by type, so every evaluation re-enters them into the symbolic lane as constants
status: open
opened: 2026-10-10
priority: P0
cost: H
refs: [a-computed-value-re-enters-as-a-constant, carriers-compare-in-canonical-form]
---


`geom`'s `NurbsCurve<T>` and `NurbsSurface<T>` hold their poles in `T`
and their weights and knots as `Vec<f64>` / `KnotVector`. Every
evaluation re-enters a weight or a knot through `Real::from_f64`, so at
`Sym` it is a constant. That is only true of a weight or a knot that IS
a constant of the model; a revolve's rational weight is `cos` of its
half-angle, and a fitted carrier's knots and weights are the fit's
output, so with an angle variable, or a fitted carrier over variables,
every identity over those carriers is a theorem about a frozen copy.
The `from_f64` audit lists these sites as
`stored-carrier-data(nurbs-weights-and-knots-are-the-scalar)` in
`scripts/gates/from-f64-sites.tsv` (91 sites across `geom`, `geom-brep`
and `topo`); they count as laundering, not as exempt.

The fix: weights and knots in `T`, so the carrier's whole definition is
one function of the variables (option (a) of the audit's FORK 1, the
orchestrator's ruling 2026-10-10). Cost estimate, ad hoc: H, a `geom`
refactor touching every NURBS constructor, the knot algebra
(`spline::CurvePlan`), the span locator (which reads knots as `f64`
to choose a span, so the choice stays an `f64` read of the value
channel) and every reader of `knots()` / `weights()`.

Considered and set aside: (c) an opaque atom keyed per minted carrier
datum (sound, keeps each carrier's self-identities, needs a
D9-deterministic key minted at construction) as the fallback if (a)
proves too large; (b) `from_computed` per read, which is sound but makes
two reads of one weight two unknowns, so no identity over a NURBS
carrier would discharge.

Whether this gates stage 4 C+D is the re-valuation row's to say: the
corpus scenes whose decisions reach a computed weight or knot are
pinned there as known reds naming this row.
