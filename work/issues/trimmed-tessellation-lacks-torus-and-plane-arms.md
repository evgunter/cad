---
id: trimmed-tessellation-lacks-torus-and-plane-arms
kind: issue
title: mesh tessellate_trimmed has no torus or plane arm - a spiric-bounded wall cannot mesh once the spiric carrier lands
status: open
opened: 2026-09-13
refs: [spiric-carrier-ruling, 1858]
priority: P0
cost: H
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

The plane × cone split (`work/reach/plane-cone-elliptic-section-split-refusal.md`)
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
