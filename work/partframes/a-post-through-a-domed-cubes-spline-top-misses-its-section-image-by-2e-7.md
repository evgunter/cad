---
id: a-post-through-a-domed-cubes-spline-top-misses-its-section-image-by-2e-7
kind: issue
title: A box post through a biquadratic dome top refuses its minted plane × NURBS section edge at ImageMiss, 2e-7 off at ε 1e-9
status: open
opened: 2026-11-27
priority: P2
cost: M
---


## What

Found by the LATHE teapot lane while it looked for a small join fixture
(2026-11-27, `lathe/teapot-spout-section-loop`). Take the unit cube with
its top on the biquadratic dome
(`topo::test_support_fixtures::domed_cube`: the centre
control point raised to `z = 1.4`, crown `z = 1.1`). Union it with the box
post `[0.4, 0.6] × [0.3, 0.7] × [0.5, 2]`
(`test_support_fixtures::prism_z`) through
`boolean::through_the_join`. At ε 1e-9 this refuses in the join:

`Join(BooleanJoin(Euler(DescriptionUncertified { edge: Minted, error: PlaneNurbs(Limb { limb: ImageMiss, margin: Value(1.97e-7) }) })))`

Each wall of the post is a plane meeting the dome in a smooth arc, well
inside both faces. A minted section edge whose image misses by about
200 ε is either an imprecise image or a limb reading that is too loose.
Which one is not yet known.

## Repro

Add to `join::holding_rows`:
`super::super::through_the_join(BooleanOp::Union, &domed_cube().0, &post, Tol::witness())`
(cube first; the other order was not run).

## The other order, and other posts

PR 7824's reviewer R2 ran the post first and got the same 1.97e-7
margin. Every plane × dome post R2 tried misses at rest by 2e-7 to
1e-6. So this is general to a plane meeting this dome, not one pose.

## Where it may belong

The miss is a limb's `ImageMiss` on a plane × NURBS section edge.
pcert's limb work found such misses on a curved foot path before: a
degree-1 chart image's chord error
(`pcert/a-limb-two-refusal-of-an-exact-closed-form-reads-as-an-off-surface-carrier`,
closed, `edge_nurbs::PXN_IMAGE_DEGREE`). If the minted edge's image is
that coarse image, the row belongs on pcert's slate with that limb
work, not here. Which image the minted edge carries is the first thing
to read.
