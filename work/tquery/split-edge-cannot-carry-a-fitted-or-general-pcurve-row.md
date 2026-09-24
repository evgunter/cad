---
id: split-edge-cannot-carry-a-fitted-or-general-pcurve-row
kind: issue
title: split_edge carries only the Decide-door pcurve lanes; a Fitted/General row is left as found because its certification doors carry the PcurveFittedLane bound
status: open
opened: 2026-09-13
priority: P1
cost: H
---



The disclosed residue of this program's
`split-edge-children-lack-pcurve-rows-on-curved-charts`, given its own
file at the moment of disclosure (`work/README.md`).

`Body::split_edge` now carries a parent half-edge's stored pcurve row
to both children by restricting the parent's chart image to each
child's sub-interval and re-certifying it
(`crates/topo/src/pcurves.rs`'s `split_cache`). The re-certification
goes through `PcurveCache::certify`, which `geom-brep`'s
`crates/geom-brep/src/pcurve_cache.rs` declares in an
`impl<T: Decide> PcurveCache<T>` block, so the carry costs `split_edge`
no bound and no caller's bound moves.

**The two lanes it cannot reach.** `Pcurve::Fitted` and
`Pcurve::General` re-certify only through `PcurveCache::certify_fitted`
and `PcurveCache::certify_general`, and those two (with `recertify`)
sit in the NEXT impl block of the same file —
`impl<T: PcurveFittedLane> PcurveCache<T>` — because their shared body
`run_fitted_checks` is `T: PcurveFittedLane`. Reaching them from
`split_edge` widens that operator's bound from `Decide`, and with it
every generic caller's: `topo::splitting::classify`'s crossing
driver, `topo::boolean::reduce`'s `split_at`, `sweep::blend::surgery`'s
two split sites, and their own callers. `crates/mesh/src/curved.rs`
calls it too — `split_each_edge_then_place`'s walk and the two splits in
`split_and_placed_frustum_wedge` — from `mesh`'s own in-crate rows at a
concrete `f64`, so those sites cost the ripple nothing and are named
here because they are where a `General`-row split would first be
reached from outside `topo`. That is the same ripple
`topo::pcurves::mint_faces`'s `UnsupportedCarrier` arm banks in its
comment ("wiring it into this pass needs the `PcurveFittedLane` bound
on every constructor and is banked with that ripple").

So `split_cache` returns `None` for those two variants and the op
leaves that face exactly as it found it — the pre-fix behaviour,
unchanged: the two new halves are rowless and the parent's rows keep
the parent's interval, and the face is tier-3 invalid until a caller
runs `mint_pcurves`.

**Reach.** `Pcurve::Fitted` is never stored by the minting pass —
`mint_face` routes every non-`General` image through the closed-form
door, which refuses a fitted image `UnsupportedCarrier`, and
`mint_faces` then clears the face. `Pcurve::General` IS stored, by
`mint_face`'s own `certify_general` arm, for a spline-chart face whose
carrier is a `Curve3::Nurbs` (`derive_general_image`). A third route
exists for either: the public `Body::attach_pcurve`, whose rows are the
caller's. Nothing in the tree is known to split an edge of such a face
— the whole `topo`, `sweep` and `mesh` suites are green with the carry
in place — so the arm is unexercised today, which is why this is a row
and not a refusal.

**Reshaped by LANE-4 (evidence, 2026-09-24).** The bound this row
prices is gone. `PcurveCache::certify_fitted`, `certify_general` and
`recertify` now sit in an `impl<T: Decide>` block and take the fitted
door as a VALUE (`geom_brep::FittedLane<T>`; `recertify` takes
`Option<FittedLane<T>>` and the scalar's name), and `run_fitted_checks`
is `T: Decide`. The per-scalar answer is
`topo::AtRestPolicy::fitted_lane`. So `split_cache` can carry a
`Fitted`/`General` row without widening `split_edge`'s bound: it can
take `Option<FittedLane<T>>` as a parameter (H5 ruling 3's shape), with
`Some` handed in by a caller that holds `AtRestPolicy`. Or `split_edge`
re-spells `Decide` → `AtRestPolicy`, the ripple priced above, with one
policy trait in place of the deleted lane trait.
