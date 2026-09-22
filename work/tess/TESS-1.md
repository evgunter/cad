---
id: TESS-1
kind: unit
title: a meridian-free curved face refuses typed, where today it meshes as a hole
status: closed
opened: 2026-09-18
refs: [rim-only-sphere-cap-panics-at-census]
pr: 2852
branch: tess/1-meridian-free-refusal
priority: P0
cost: D
closed: 2026-09-22
---


Spec: `docs/TESS-1-SPEC.md`. Full v6 dual (a kernel unit: it moves what
`tessellate` answers). Block TESS-B1, record branch-side on
`tess/b1-block`.

## Closed (2026-09-22, PR 2852 merged at `5a83b580b`)

`TessellateError::MeridianFreeCurvedFace { face, surface }`, raised by
`walk::require_a_meridian` the moment the loop's traversals are
classified and none is a meridian — D2 row 1 (invalid input, per
ruling (N)); no float decides it. Rows: the sphere cap at both poles
and several latitudes, the two-cap sphere, MESH-12's fixture at
Δv = 0 (and Δv = 1.5ε, now `UnsupportedCurvedShape{Escalated}` at
PROPS' door), the cone apex cap, the one-rim cylinder, the STEP
fixtures; the seamed twins as positive control; the zero-height
polygon's admission by `require_swept_rectangle` executed again;
the reviewer probes lifted (`loops_the_meridian_guard_admits.rs`).
Census message states its count and does not claim its cause list is
complete. Filed from the unit: `rim-free-loop-on-a-poleless-chart-
meshes-as-a-hole`, `trimmed-and-planar-lanes-answer-ok-on-an-empty-
patch`, `geom-brep-readme-c12-misstates-the-mesh-lanes` (TESS); the
poleguard prose (TINT); the guide and `.pyi` lag (LIB); evidence on
ATREST's validity row (one structural fact, three spellings) and
EXCH's normalization row. Spec deleted (`docs/DOC-LEDGER.md`).
