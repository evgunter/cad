---
id: a-limb-two-refusal-of-an-exact-closed-form-reads-as-an-off-surface-carrier
kind: issue
title: a limb-2 refusal of an exact closed-form carrier reads at rest as 'the declared carrier is off the surface' when the miss is the kernel's degree-1 image
status: open
opened: 2026-10-14
priority: P2
cost: M
---


Found by the review of PR 5097 (`pcert/spline-pair-any-carrier`).

## What happens

Since the spline-pair edge certificates take a closed-form carrier, an
EXACT circle on a spline wall (the arc rim of
`sweep`'s `an_intersection_circle_on_a_spline_wall_is_refused_at_the_door`)
refuses at limb 2 on its degree-1 chart image's chord error
(`edge_nurbs::PXN_IMAGE_DEGREE = 1`, about 4.6e-4 m there). The refusal
renders through `PlaneNurbsRefusal`'s `Limb` arm and `topo::validate`'s
`classify_certify` as a declared carrier that is off its surfaces
("the certificate cannot show the carrier is on them" / the BOUND
reason), and the at-rest recourse speaks to the carrier. The carrier is
exact; the miss is the kernel's own image, which is too coarse for a
curved foot path.

## What closes it

A wording (and an ending) that tells "the carrier is off the surface"
from "the certificate's image of an on-surface carrier is too coarse",
which needs the limb to say which of the two its bound is about, or a
finer image (degree or schedule) for curved closed forms.
