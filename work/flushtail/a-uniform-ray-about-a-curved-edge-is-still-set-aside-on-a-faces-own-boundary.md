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
