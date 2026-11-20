---
id: certificate-bound-fields-hold-limb-bounds-as-t
kind: issue
title: SSI and edge certificates hold limb 2 and limb 3 bounds in T fields, so their decisions lift the bound
status: open
opened: 2026-11-20
---

Found building `a-bound-to-decide-against-never-becomes-a-t` (DL5.2, `crates/geom-core/README.md`).

## What

Limb 2's floor and supremum and limb 3's clearance are `f64` bounds, and the certificate that reports them holds them as `T`:

- `geom_brep::ssi::certify::hull_limb` (`crates/geom-brep/src/ssi/certify.rs`) lifts `hull.sup` through `T::from_computed`, decides it as `Margin::of` and returns it into `SsiCertificate::hull_sup`. Its `floor` is decided and dropped, and takes `Margin::bound`.
- `tube_transversality` (same file) lifts `clearance` and returns `Margin::value()` of the levered product into `SsiCertificate::tube_transversality`; `certificate` maps `rung.tube` through `T::from_computed`.
- `edge_nurbs::PlaneNurbsLimbs` copies both fields (`hull_sup: T`, `tube_transversality: T`), and `certify.rs`'s edge certificate folds `limbs.hull_sup.abs()` into `max_residual: T` beside the sampled residuals, which are honestly `T`.

The `sup` and clearance decisions alone could take `Margin::bound` and `Margin::bound_levered` today. They were left, because the lifted value is also the certificate's reported field, so moving the decision alone keeps the lift and adds a second spelling of one number.

## The change

Re-type the bound fields of `SsiCertificate` and `PlaneNurbsLimbs` to `f64` (they are a bound read off an enclosure, DL5.1's "leaves"), decide them through `Margin::bound` / `Margin::bound_levered`, and give `EdgeCertificate::max_residual` a way to report an `f64` bound beside its `T` residuals (two fields, or the bound folded at `f64` after a bracket read of the residuals). Every reader of these fields is in `geom-brep` and `pair_section`'s tube (`pad_and_reach` already takes `hull_sup: f64`).

## Same shape elsewhere

`topo::replace_face`'s `corner_of_root` lifts a section's `near_gap` (an `f64` bound) into `ReplaceFaceError::ReanchorPastCarrierEnd::gap: T`, because that field also holds the walking re-anchor's measured distance `carrier.eval(t_new).distance(point)`, which is honestly `T`. The decision beside it already takes the bound as `Margin::less`. Re-typing the field to `f64` means a bracket read of the measured gap where it is reported, a `Bounds` question.
