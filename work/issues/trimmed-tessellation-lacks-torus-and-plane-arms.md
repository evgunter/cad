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
