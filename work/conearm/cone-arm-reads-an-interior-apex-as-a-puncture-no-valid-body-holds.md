---
id: cone-arm-reads-an-interior-apex-as-a-puncture-no-valid-body-holds
kind: issue
title: The cone closed form reads an apex inside a face as a legal puncture, a state tier 3 now refuses
status: open
opened: 2026-10-29
priority: P3
cost: E
---

Filed by RESTCURVE's `validity-refuses-an-interior-chart-singularity` (branch `restcurve/chart-singularity-is-a-vertex`), on CONEARM's slate because `crates/geom-brep/src/props/curved.rs`'s cone arm is CONEARM's.

## What

Tier 3 now refuses a cone face whose apex lies in its interior or inside one of its edges (`topo::ValidationError::ChartSingularityNotAVertex`, read by `geom_brep::chart_singularity::locate`). The cone closed form still reads that face as legal: `cone_face_read` (`crates/geom-brep/src/props/curved.rs`, near `:1897`) treats the apex as "the chart's puncture: a face whose loops wind it on balance holds it, and it winds back" (`with_apex.push([-total])`), and `cone_boundary`'s apex fold (`cone_apex_level`, `require_rim_only_closed`) gives a rim-only apex cap an extent. Through a valid body neither is reachable any more; the Euler doors still state the face (`crates/topo/tests/cone_apex_cap_body.rs`, `crates/mesh/tests/meridian_free_face.rs`, `a_cone_apex_cap_and_a_one_rim_cylinder_face_refuse_by_kind`).

## What a fix owes

Decide whether the closed form keeps measuring the face (it is what a raw door such as `measure_unvalidated` reaches) or refuses it by the same reading, citing `chart_singularity::locate` rather than re-deriving the apex's place; then retire the puncture arm and the apex fold, or say why they stay. `cone_apex_cap_body.rs` pins the fold's side reading today.
