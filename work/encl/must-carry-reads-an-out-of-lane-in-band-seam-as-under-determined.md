---
id: must-carry-reads-an-out-of-lane-in-band-seam-as-under-determined
kind: issue
title: must_carry_over_edge answers UnderDetermined for an out-of-lane smooth join whose sagitta is in band, which tier 3 refuses SliverDihedral
status: open
opened: 2026-10-02
---

## Finding (FUSE's sweep for the boolean rebuild's second-order fold)

`geom_brep::must_carry_over_edge` (`crates/geom-brep/src/dihedral.rs`)
reads the second-order sagitta only on a pair inside
`tangent_certificate_lane`; an out-of-lane pair whose stations all read
`Smooth` answers `MustCarryVerdict::UnderDetermined`, so every caller
stores the conventional description. Tier 3's must-carry arm
(`crates/topo/src/validate.rs`, the `all_smooth` branch of check 4)
reads `decide("tangent_second_order", …)` at the same stations on EVERY
definitely-smooth edge, lane or not, and pushes
`ValidationError::SliverDihedral { check: WedgeCheck::SecondOrder, .. }`
on an in-band sample. The lane only gates the DEMAND there
(`TangentNotIntrinsic`). So an out-of-lane smooth join whose sagitta is
in band is built conventional by any constructor that asks the rule and
then refused at rest: the shape
`work/fuse/boolean-rebuild-folds-an-in-band-second-order-into-conventional.md`
fixed in the boolean, one level down.

Measured at the rule (not through a constructor): the plane tangent to
a 45° cone (apex at the origin, axis `z`) along the ruling
`(sin 45°, 0, cos 45°)`, a `Line` carrier the lane refuses, over
`[100, 100 + L]` with `L` chosen so the sagitta `|κ_rel|·L²/2`
(`κ_rel = 0.01`) is the band's geometric mean at `Tol::witness()`:
`must_carry_over_edge` answers `UnderDetermined` in both argument
orders, while `tangent_second_order` at the middle station answers
`Err(Indeterminate { predicate: "tangent_second_order", margin ≈ 3.16e-9 })`.

Who can reach it: the four callers of the rule — `sweep::extrude`'s
strut and cap rim, `sweep::revolve::upgrade`, `sweep::blend::surgery`'s
contact edge, and `topo::boolean::ops::seam_must_carry`. Whether any
constructor mints an out-of-lane smooth join today is not measured.

## Repair shape

Either the rule reads the sagitta's in-band arm on every pair (escalate
`InBand` out of lane, keep `UnderDetermined` for a definite reading
only), or tier 3 gates its second-order reading by the same lane. The
first keeps tier 3's F6 stance; the second changes what tier 3 refuses
and is a design question.
