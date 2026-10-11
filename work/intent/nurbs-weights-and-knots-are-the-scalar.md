---
id: nurbs-weights-and-knots-are-the-scalar
kind: issue
title: NURBS weights and knots are f64 by type, so every evaluation re-enters them into the symbolic lane as constants
status: dispatched
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
`scripts/gates/from-f64-sites.tsv`, across `geom`, `geom-brep` and
`topo`; they count as laundering, not as exempt.

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

Whether this gates stage 4 C+D was the re-valuation row's to say. On
the whole corpus at unit 1's merge, no document showed a laundering
attributable to a weight or a knot (PR 4543's body), so it does not
gate C+D on that evidence. The evidence is silence: no counter says
which listed sites a document reaches, so a scene that reaches a
stored weight without moving it is indistinguishable from one that
never reaches it. This unit's fix owes its own pin, as
`*-keeps-computed-values-in-t` do. No revolve builds a rational weight
(a spline profile's revolve takes `Surface::nurbs_placeholder()`, and
lines and arcs revolve to analytic carriers), so the pin is
`geom::test_support::held_arc::HeldArc`. It is a rational arc built in
`T` at `Sym<f64>` over a variable sweep `θ`, with weights `cos(Δ/2)`.
The pin owes two halves through the NURBS carrier, landed in u3 once
u2's carriers hold their weights in `T`: the carrier's point re-values
with `θ`, and `pcurve_cache.rs`'s arc-rim margin `w − cos(h/2)` is a
symbolic Zero. Each half is red on the stored-`f64` weight. The skinned arc wall
takes over as the door-level pin once `the-skin-computes-its-values-in-t`
lands.

Four `stored-carrier-data` rows are values computed from the stored
data rather than the stored data itself: `curves/nurbs.rs`'s `wp_lo` and
`wp_hi` (a bound on the stored weights' derivative) and `net.rs`'s `bw`
(the weights' deviation under a re-fit) and `rw` (a re-fit net's weight).
Once the weights are `T`, each is a value this crate computed, so it
stays in `T` or enters through `from_computed`. It does not stay as
stored data re-read. Re-class them as each is touched. (Raised by PR
4589's review, unverified.)

Closing this removes a way for the coincidence door to report a false proof: the door's symbolic rungs (`carriers-compare-in-canonical-form`) are theorems about the expression the `Sym` replay saw, and a listed site re-enters a computed value there as a constant.
