---
id: project-seed-evaluates-at-computed-places-under-a-bound-that-admits-a-dual
kind: issue
title: project_seed evaluates at computed places under a bound that admits a dual, and T::Value is not known to carry a bracket
status: open
opened: 2026-11-21
priority: P2
cost: M
---


Found by `a-place-is-an-f64-argument-answered-on-the-value-channel` and both reviews of PR 7250.

`geom::NurbsSurface::project_seed` (`geom/src/surfaces/projection.rs`) and the curves' `project_seed` (`geom/src/curves/projection.rs`) are bounded `T: Bounds`, which admits a dual, and evaluate their sweep samples at `T::from_computed(u)`: places entered as scalars. A dual's sample there carries a zero tangent, which the sweep never reads (it compares `mid(d·d)` at `f64` and returns `f64`).

Two readings of the `projection_policy` doctest `still_admitted` (it compiles `project_seed` at `NurbsCurve3<Dual64>`):
- A deliberate contract: its prose says `project_seed` is "deliberately NOT on that bound … returns `f64`, so it carries no tangent to be wrong about".
- A width no production caller uses: the one caller in `crates/*/src` is `project`, at `T: CertifiedBounds`. DESIGN `[unrepresentable]`: a bound is not widened to every scalar the body compiles at.

Under either, the sweep could take its samples as places on the geometry without its tangent (`value_view()` then `eval_at`), and the doctest stand. That needs the bracket on `T::Value`, and this is the wall: `Real::Value` is bounded `Decide<Value = Self::Value>` only (`geom-core/src/real.rs`, `type Value`), so `T: Bounds` does not give `T::Value: Bounds`, though every scalar's `Value` carries one (DL5.1: the bracket is the base scalar's, bit for bit). Adding `Bounds` to `Value`'s item bound is one bound, but it is the compound `Decide + Bounds` on one parameter that `scripts/gates/bounds-allowlist.sh` refuses and the `Bounds` scope rule reserves for ratification, and it would hand a bracket read to every generic body through `T::Value`. So it is a question for Ev, not a fix: either that bound, or a `where T::Value: Bounds` at each bracket-reading place site (the gate's grouping would read it as a second target, `T::Value`, carrying `Bounds` alone; untested), or the sweep left as it is with the doctest's reasoning written into DL5.2's place row as a named exception.
