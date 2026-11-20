---
id: a-small-toruss-aims-land-on-its-faces-shared-seam-where-the-quartic-reads-in-band
kind: issue
title: A torus split on its inner equator aims both faces at the seam, where the ray's quartic ladder reads in band
status: open
opened: 2026-10-19
priority: P3
cost: M
---


Filed by the edge-meeting ray unit
(`a-ray-meets-an-edge-in-band-as-half-a-crossing-for-each-face`).

## Measured

`crates/sweep/tests/shell7_r2_probes.rs` `p3b_probe_the_tori_aimed_rays`
(ignored), at ε 1e-12: the revolve-door torus R 1e-10, r 2e-11 (its tube
4Kε across) is two faces split on its inner equator, and
`side_reader::aim_at` aims both at the torus chart's fixed point
(π, π), `(−8e-11, 0, 0)`, which is that equator: the faces' shared seam.
Each ray is set aside in band of the ray's quartic ladder:
`line_quadric_torus_split` (8.0e-12) on main, `line_quadric_torus_odd`
(7.2e-12) with the near-normal aims of the edge-meeting ray. The volume
reads the torus, which validates at rest either way. The tori at R 2e-8
and 2e-9 read by ray.

## What to find

Whether a torus face's aim should be its own window's middle, off the
seam, as a cylinder's and a cone's are, and whether the quartic at a
tube a few Kε across reads at any aim. The seam's own reading (one root,
one comparison against the seam's parameter) is
`a-ray-meets-a-curved-faces-edge-in-band-by-each-faces-own-trim`.

Moved from FLUSHTAIL to HONEJOIN on 2026-11-20, by `git mv`, when FLUSHTAIL merged into HONEJOIN: FLUSHTAIL held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one HONEJOIN carries.
