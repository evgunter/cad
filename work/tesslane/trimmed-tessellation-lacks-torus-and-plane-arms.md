---
id: trimmed-tessellation-lacks-torus-and-plane-arms
kind: issue
title: the trimmed lane has no torus or cone arm - a torus wall bounded by a spiric and a cone face cut by an ellipse refuse at the chart roster
status: open
opened: 2026-09-13
refs: [1858]
priority: P0
cost: H
design: true
---


## What

Found by the CURVED-SPIRIC spec survey (2026-09-13). The trimmed
tessellation lane (`crates/mesh/src/trimmed.rs`, `tessellate_trimmed`)
dispatches on the chart kind and carries no torus or plane arm for a
trim region bounded by a non-circle, non-line image. When the spiric
rim carrier lands (`docs/CURVED-SPIRIC-SPEC.md` PR-1a), the klein
elbow's hollowed torus wall and its meridian caps carry spiric-bounded
faces, and `tessellate` refuses typed at that lane. A typed frontier,
recorded by PR-1a's census row; not a STOP.

## Fix shape

A torus arm and a plane arm in the trimmed lane taking the face's
certified pcurve images (PR-1b's `Pcurve::Spiric`, or the `chart_bound`
description) as the trim polygon's source; the plane cap's oval and the
wall's `(u(v), v)` image are both closed-form samplers. E–M, after
PR-1b.

## Home

Unowned at filing — `crates/mesh/*` is S-MESH's; filed by the CURVED
orchestrator.

## The cone chart too (REACH, plane × cone split fix pass, 2026-10-01)

The plane × cone split (`plane-cone-elliptic-section-split-refusal` (REACH, closed by PR 3688))
mints cone faces trimmed by a tilted section `Ellipse`, with the exact
`Pcurve::ConeSection` image stored on each. Measured: the half above a
frustum (radius 1 → 1/2, revolved about `y`) cut through `(0, 0.5, 0)`
with normal `(sin 0.3, cos 0.3, 0)` refuses `tessellate` with
`UnsupportedCurve` at the trimmed router (`trimmed.rs`, the "conic/B-
spline/spiric trim on a plane/cone/sphere/torus chart" note), on the
cone face's ellipse edge. The cylinder twin of the same cut tessellates
(145 positions), so the planar section face is not the blocker. The
cone arm's sampler is the stored image: slant harmonic, azimuth
`u0 + σ(t + 2·atan2(β sin t, 1 − β cos t))`, cut-free. Owner by
territory: TESS (`crates/mesh/src/trimmed.rs`).

Moved from `work/issues/` to `work/tess/` on 2026-11-03, when the `work/issues/` bucket was retired (Ev's direction, in chat); id and body unchanged.

- 2026-11-14 — moved from TESS to TESSLANE by `git mv` when TESS and TESSSIZE merged into their successor TESSLANE (`docs/doc-ledger/tess-and-tesssize-leave-the-tracker.md`), id and body unchanged.

## Measured on main (2026-11-14, TESSLANE; probes on `tesslane/measure-reach`)

Run at `d6e1eb1f`, through the public doors, at three δ each; every result was the same at each δ.

- **Torus wall bounded by a spiric: refuses, as filed.** The sectioned vessel quarter and the klein elbow, each through `topo::finish` and `topo::shell`, build, and `tessellate` answers `UnsupportedCurve` on the spiric edge, raised by `trimmed::trim_frontier` from `tessellate_trimmed`'s chart roster. The half-edges store `Pcurve::Spiric`.
- **Plane cap bounded by a spiric: not a refusal.** `tessellate::lane_of` sends every plane to the planar lane, and both spiric-bounded caps return `Ok` from `planar::tessellate_planar`. The body refuses only for its torus wall, so no whole body pins the caps through `check_mesh` yet. This row's "plane arm" is not owed.
- **Cone face cut by a section ellipse: refuses, as filed.** Both frusta split through `(0, 0.5, 0)` with normal `(sin 0.3, cos 0.3, 0)`; all four halves build and refuse at `trim_frontier` on the `Ellipse` edge. The stored image is `Pcurve::FocalSection` (the row's `ConeSection`). The same refusal stops all six booleans of a widening frustum against a brick tilted 10° (`cone_join_lane` C1), which is the cone half of `cone-and-sphere-faces-with-conic-or-spline-trims-have-no-trimmed-arm`, now carried here.

**What each pair has and lacks**, by function:

| pair | present | absent |
|---|---|---|
| cone × ellipse | the image (`Pcurve::FocalSection`, which `trim_polygon` accepts); a 3-D chord bound (`sizing::ellipse_step`); a certificate (`cert::cert_cone`); the curved lane's step (`sizing::cone_grid_step`) | a UV speed bound for the image; a cone arm in `trimmed::Lane` and its lattice steps; an apex rule for the polygon |
| torus × spiric | the image (`Pcurve::Spiric`); a 3-D chord bound (`sizing::spiric_step`); certificates (`cert::cert_torus`, `cert_spindle`); the curved lane's steps (`sizing::torus_grid_steps`) | a UV speed bound for the image; a torus arm and its lattice steps; a seam rule for a polygon crossing either period |

**This row is one cell of a larger gap**, with `valid-ringed-torus-faces-build-but-cannot-be-meshed`, `a-notched-or-ringed-cylinder-wall-does-not-tessellate` and the sphere-pole rows: `lane_of` routes on the kinds of the OUTER loop's carriers, the trimmed lane constructs three arms (cylinder, sphere, NURBS), and both curved lanes return at a ring before reading any loop. What a non-rectangular curved face is meshed from is with designers before a unit is cut (`design: true`).
