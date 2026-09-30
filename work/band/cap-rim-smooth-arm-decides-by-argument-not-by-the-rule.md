---
id: cap-rim-smooth-arm-decides-by-argument-not-by-the-rule
kind: issue
title: sweep: the cap-rim smooth arm decides a description by an in-code argument, not by the must-carry rule
status: open
opened: 2026-09-13
priority: P1
cost: D
---


## Finding

`geom_brep::must_carry_over_edge` is now the one home of the rule that
decides what description a definitely-smooth join carries, and the two
arms the item
`smooth-arm-siblings-disagree-on-the-in-band-case` named — extrude's
strut arm (`crates/sweep/src/extrude.rs`, the `DihedralClass::Smooth`
arm of the join pass) and revolve's latitude join
(`crates/sweep/src/revolve/upgrade.rs::upgrade_intersection`) — both
call it.

A THIRD smooth-description arm does not: `upgrade_rim`'s
(`crates/sweep/src/extrude.rs`, the `Ok(DihedralClass::Smooth)` arm,
~`:1225`) stores the conventional chart image unconditionally, and its
comment argues that the second-order rule has nothing to add there
because *"only plane pairs reach this arm (a cylinder wall's normal is
radial about the sketch normal, so it is perpendicular to the cap's ±n
at every rim point), and a plane pair's `κ_rel` is identically zero"*.

**The arm is unreachable, on BOTH doors** — which is the correction the
BLEND-9 dual review made to this item's first diagnosis. A cap–wall rim's
wall surface is swept along the extrusion vector `w`, so its normal is
perpendicular to `w` at every rim point (a plane wall's and a cylinder
wall's alike); the cap's normal is the sketch normal `n`, and the
`Extrusion::Vector` gate admits `w` only when its in-plane component is at
most ε while its normal component is at least K·ε — a tilt bounded by
`1/K`. So the angle between the two normals stays within `1/K` of a right
angle and `sin θ ≥ cos(1/K)`, which over a definite arm is a margin orders
above K·ε: no cap–wall rim classifies `Smooth` through a definite arm at
all. The arm is defence in depth, not a live path, and the description it
would store is unreachable rather than wrong.

**The secondary defect, still real.** The comment argues unreachability from
the wrong premise: *"a cylinder wall's normal is radial about the sketch
normal, so it is perpendicular to the cap's ±n at every rim point"*. Under
`Extrusion::Vector` the wall is radial about `w`, not about `n`, so the
premise is false on that door; the conclusion survives through the obliquity
gate above, which the comment does not name. One reviewer notes the
justification postdates the code it justifies.

That is still the shape the item objects to: a description decided by an
argument in a comment rather than by the shared rule, with the argument
naming a reason narrower than the one that holds.

## Disposition

Not this unit: BLEND-9's spec scopes the change to the strut arm and the
latitude join, and constrains `dihedral.rs` to the wrapper. The work, in
priority order:

1. Restate the comment's reason as the one that actually holds — the
   obliquity bound on `w` against the wall's ruling, both doors — rather
   than the sketch-normal premise that holds only under `Distance`.
2. Decide whether an unreachable arm should carry a description AT ALL, or
   whether it should escalate the way an unreachable state does elsewhere
   in this crate. Routing it through `must_carry_over_edge` is the cheap
   version and would make the gate, the stations and the K cost uniform
   across all three smooth arms; refusing is the other.

Either way the claim wants a row. Nothing fails today if the obliquity
bound changes, and the arm's unreachability is exactly the kind of fact
that is true until a door is added.
