---
id: a-uniform-ray-about-a-curved-edge-is-still-set-aside-on-a-faces-own-boundary
kind: issue
title: Rays about curved edges in uniformly random directions are still set aside on a crossing in band of a face's own boundary, about 1 in 100
status: open
opened: 2026-10-25
priority: P3
cost: M
refs: [a-ray-meets-a-curved-faces-edge-in-band-by-each-faces-own-trim]
---


Filed by the curved edge-meeting ray's fix pass (PR 5697). Its second
review counted the rays.

**Measured.** `crates/sweep/tests/curved_edge_ray_probes.rs`,
`probe_uniform_rays_about_the_corpuss_edges` (ignored, pinned seed 11).
Rays are aimed at and about every edge of the curved corpus
(`curved_edge_corpus::corpus`), within 3Kε of the edge and 2Kε off it, in
uniformly random directions, both ways out: 25 920 rays per ε. None
reads the wrong side. Set aside on a crossing in band of a face's own
boundary (`SetAside(Crossing)`): 231 at ε 1e-9, 364 at 1e-6, 223 at 1e-12.
The second review measured 88–236 of about 21 600 on the PR's first head
(`flush/curved-edge-ray-review-r2`, `review_5697_r2_rays`).

The pin `rays_in_band_of_a_curved_edge_read_it_once` asserts none only
for aims near the face's normal. These rays are set aside by a face's
own trim or in-face walk at a crossing no met edge stands for: a root
or plane crossing in band of an edge the ray passes clear of at the
crossings, or a region read whose metric differs from the edge reading's
at the band's edge. Each is a lost read, never a wrong one.

**What would close it.** Sort the 231 by the face reading that refused
(`point_in_face`, `ruled_region`, `sphere_region`, `torus_region`) and
by whether an edge of that face was read clear at a crossing, then
decide each class: a seed for the edge reading, or a region reading that
defers to it.

## The pin's aims on the band's edge now land in this class (2026-11-14, BOXFINISH's aimed-ray unit)

`rays_in_band_of_a_curved_edge_read_it_once` aims at `0.3Kε` and at
exactly `Kε` from each curved edge's middle. An aimed ray is now read
from its aim (`side_reader::aimed_ray`, `geom::Ray::starting`), so its
crossing of the aimed face is the aim to an ulp, where it was
`from + d·t` and landed a few ulps off. At `Kε` exactly, on the cone
frustum, the edge reading passes the edge clear and the face's own
trim reads the same distance just under the band's edge:

- ε 1e-9: three rays, `curve_carrier_gap` at `9.99999982734255e-9`
  (twice) and `bool_ruled_region_edge` at `9.999999999999997e-9`,
  against an escalate of `1e-8`;
- ε 1e-6: one, `curve_carrier_gap` at `9.999999999777484e-6`;
- ε 1e-12: one, `curve_carrier_gap` at `9.999824259158329e-12`.

With the ray as it was built before, the same twelve cone-frustum aims
at `Kε` read `Out` eight times and grazed four at 1e-9, read `Out` six
times and grazed six at 1e-6, and read `Out` twelve times at 1e-12:
which side of the band's edge they landed was rounding's then too. The
pin asserts "none set aside on a crossing" of every aim but the cone
frustum's at `Kε`, which it counts against its floor. The other bodies
of the pin did not move.

## A straddle beside a shallow spiric edge joins this class (2026-11-15, BOXFINISH's curved-straddle unit)

`curved_edges::passes` read a carrier's escalation as the edge met in
band whatever it bracketed. It now reads one that reaches past the
band's edge as unplaced, no reading, and sets the ray aside
(`boolean/solid_contain/edge_gap.rs`). At `f64` the only such
escalation is the carrier's straddle (`curve_carrier_straddle`), and the
only place it was found is beside a spiric edge where the plane meets
the torus at a dihedral sine under about 0.12: a spiric's lower bound on
a point's distance is the greater of its distances from the plane and
from the torus (`geom::Curve3::clearance`), which is under the zero band
a few bands from the edge there while the foot's gap is past the band.

**Measured** on a torus (major 2, minor 0.5) cut by the plane
`x = 1.5 − depth` through `topo::intersect`, rays aimed 0 to 10⁴ bands
either way from sampled points of its spiric edges whose dihedral sine
is under 0.2 (2304 rays a body), ε 1e-9. Rays whose reading changes,
each from `Out` or a graze to `SetAside(Crossing)` on the straddle: 4 at
depth 1e-2, 24 at 1e-3, 44 at 1e-4, 64 at 1e-5, 76 at 1e-6, all between
12 and 30 bands from the edge. On every one of them the old reading
(the edge met) and the faces' own crossings (the edge read clear) agree,
so each is a lost read, never a wrong one. The reader's own aims and
6400 containment queries about those points answer the same either way.
The pin was `sweep` `a_ray_in_a_shallow_spiric_edges_straddle_is_set_aside`
(since 2026-11-17 `a_ray_beside_a_shallow_spiric_edge_is_read_by_the_span_walk`, below).
No row of the `topo`, `sweep` or `editor-core` suites reached a straddle
at ε 1e-9 but `curved_edges`' own pin, as measured on 2026-11-15 by a
log line at the arm that is not in the tree (the unit's review took the
same count with its own). It is a measurement of that date, and nothing
guards it: a new row or a new body can reach one unannounced.

**What would give the reads back.** Either a spiric lower bound that is
the distance from the curve rather than from each surface (no straddle
then arises a few bands out), or the straddle read as an edge nothing
decided, each face reading its own crossing by its own trim (the
measurement above says those trims decide on every such ray).

## The spiric straddle is read by the walk over the span (2026-11-17, ONEHOME fork 275 step 3b)

`geom_brep::curve_pair::Span::read` now reads a point whose carrier
reading neither decides on nor off against the span itself: off the
span where the piece walk over the span clears it (`Span::clear`, a
non-nearest carrier only). Beside a shallow spiric edge that walk
certifies the rays' points clear, so the straddle no longer sets them
aside. On the pin's body at ε 1e-9, the 192 aimed rays read 88 `Out`
and 104 grazes, where with the clearance step off they read 82 `Out`,
106 grazes and 4 set aside on the straddle. The pin is renamed
`a_ray_beside_a_shallow_spiric_edge_is_read_by_the_span_walk` and
asserts none set aside on a crossing.

This answers the straddle section above only. The uniform rays' crossings
in band of a face's own boundary (the first sections), the cone frustum's
aims at `Kε`, and a spiric lower bound that is the distance from the
curve are not touched, so the issue stays open.
