---
id: a-villarceau-circle-has-no-lies-rung
kind: issue
title: A Villarceau circle on a torus has no Lies rung: the curve × face pose escalates it in band
status: open
opened: 2026-11-20
priority: P3
---


## What

A Villarceau circle lies on its torus: `F ≡ 0` along it. The circle × torus door has rungs for the
coaxial pose and the meridian (`circle_torus_coaxial_*` and `circle_torus_meridian` in
`crates/geom-brep/src/curve_face/circle_torus.rs`, module docs "The special poses"), but none for a
Villarceau circle. The ladder answers it `Unsettled` today.

Under U3 (`docs/GERM-CROSSING-U3-SPEC.md`), conic × torus gets a general not-`Lies` rung: one carrier
point whose residual is decided past the band. A Villarceau circle has no such point, so its pose
escalates in band. That is sound, but it refuses a configuration that lies on the torus.

## Owed

A `Lies` rung for a Villarceau circle: the circle's plane bitangent to the torus, and its radius and
centre those of the torus's Villarceau pair. It would be a `curve_face_pose_*` row with an in-band twin
that escalates. No producer is known to reach the pose today, so this is P3.
