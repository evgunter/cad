---
id: a-torus-face-aims-at-a-fixed-chart-point-that-a-symmetric-pose-holds
kind: issue
title: A torus face aims at the chart point (π, π), which a symmetric pose holds
status: open
opened: 2026-10-27
priority: P2
cost: M
refs: [the-side-readers-aims-decide-exact-ties-at-a-documents-nominal]
---

`crates/topo/src/side_reader.rs` `aim_at` aims a torus face, whole or
trimmed, at the chart point `(π, π)`: the inner equator at the azimuth
opposite the chart's reference. That is a symmetric point of the chart.
A torus whose reference azimuth lies in a mirror plane of the body puts
the aim on that plane, and a ray from it may then take questions that
tie, as the planar and sphere aims did before `flush/aims-ties` moved
them off their symmetric points.

It is unmeasured. No corpus document casts a torus aim: `tube_ring`,
`hollow_tube_ring` and their kin read zero reader decisions. The
normal there is read by the face's own reader
(`face_outward_normal_at`, which refuses a horn or spindle torus).

A trimmed torus could aim as the trimmed sphere does, at a point of a
boundary arc stepped into the face by evaluation alone. A whole torus
has no boundary to step from.
