---
id: a-quarter-arc-swept-annulus-exceeds-its-triangle-certificate
kind: issue
title: a swept circle_split annulus refuses tessellation CertificateExceeded at delta 1e-2 on a station-count x skin-degree lottery, up to 53x over at 16 arcs
status: open
opened: 2026-10-02
priority: P2
cost: H
---

Found by SHOW's `klein-scene-should-adopt-the-one-body-loop-sweep` unit
(2026-10-02) and mapped by its review. The Klein bottle's loop ships at
a setting inside the passing neighbourhood (`demos/tour/src/klein.rs`,
`STATIONS` / `V_DEGREE`); this row is the lottery around it.

## The strongest clue first: a 53× sliver

The annulus with each wall as `circle_split(.., 16, ..)`, 17 stations,
v-degree 3, refuses

```
CertificateExceeded { face: FaceKey(19v1), bound: 0.5314705929049474, requested: 0.01 }
```

— 53 times over δ, while the same section at 9, 17, 33 and 65 stations
at v-degree 2, and at 33 and 65 at v-degree 3, meshes. A triangle bound
that far over on a smooth wall reads as a Delaunay sliver against a
malign band, the configuration `rim-chords-exceed-snapped-column-count`
recorded as reachable-by-construction and never observed. Untraced;
that row is the first place to look.

## The body

Annulus `R ± WALL/2` = 0.275 / 0.225 m, each wall `circle_split(centre,
r, n, 0.0)`, swept by `sweep::sweep_body` along a degree-3 interpolant
through 49 exact points of two tangent arcs of radius 1.2 m (270° then
90°, world xz-plane), from `path_start_frame` at (0, 0, 3). The columns
are `mesh::tessellate(&body, 1e-2, Tol::witness())`; every cell passes
tier 3 (`validate_geometric`) at ε = 1e-9 except 4 arcs at 9 stations,
v-degree 3, which refuses `QuadratureBudget` (the quad row's ground,
`a-swept-circle-section-loop-decides-its-volume-sign-only-at-the-origin`).

| n | stations | v-deg 2 | v-deg 3 |
|---|---|---|---|
| 4 | 9 | meshes (95,194 tris) | meshes (101,762) — but tier 3 refuses `QuadratureBudget` |
| 4 | 11 | meshes (100,450) | — |
| 4 | 13 | meshes (84,500) | meshes (83,268) |
| 4 | 15 | meshes (90,280) | — |
| 4 | 17 | meshes (96,204) | meshes (100,732) |
| 4 | 19 | meshes (100,912) | — |
| 4 | 21 | **0.01226** (face 7v1) | meshes (118,748) |
| 4 | 25 | **0.01261** (face 7v1) | — |
| 4 | 33 | **0.01139** (5v1) | **0.01243** (7v1) |
| 4 | 65 | **0.01093** (5v1) | **0.01082** (5v1) |
| 8 | 9 | **0.0276** | **0.0208** |
| 8 | 17 | **0.0340** | **0.0281** |
| 8 | 33 | meshes (154,668) | **0.0302** |
| 8 | 65 | **0.0309** | meshes (302,266) |
| 16 | 9 | meshes (74,906) | **0.0958** |
| 16 | 17 | meshes (79,662) | **0.531** |
| 16 | 33 | meshes (147,026) | meshes (143,896) |
| 16 | 65 | meshes (288,572) | meshes (280,832) |

Bold is the refused bound against `requested: 0.01`. The n = 4 cells at
9–25 stations (v-degree 2) and 13, 17, 21 (v-degree 3) were re-measured
at ε = 1e-6 and 1e-12 with the same mesh outcome and triangle count in
every cell. A `circle` section (n = 2) never meshes: its semicircle walls
carry the C0 crease `lofted-circle-sections-are-unmeshable-and-say-so-three-steps-late`
records.

## Why it matters

Nothing about the geometry changes across a row: more stations or a
higher skin degree is a finer approximation of the same tube, and the
mesher's answer flips with it, both ways, with no monotone trend in
either direction. A user who picks a station count picks a lottery
ticket. `TessellateError::CertificateExceeded`'s doc reads the refusal
as a kernel-side defect or degenerate geometry, and these walls are
neither thin nor degenerate.

## Done when

The table above meshes in every cell at δ = 1e-2 — or each remaining
refusal is shown to be correct (a geometry the certificate genuinely
cannot bound) with its mechanism named. Moving the Klein loop's
`STATIONS` does not touch this row: the loop already ships inside the
passing neighbourhood, and the lottery is around it.
