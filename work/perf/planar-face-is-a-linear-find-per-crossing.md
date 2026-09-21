---
id: planar-face-is-a-linear-find-per-crossing
kind: issue
title: planar_face is a linear find per declared pair per crossing, and confirm_declarations scans faces per record - the arena-scan shape the census grep could not see
status: open
opened: 2026-09-13
priority: P4
cost: D
---


## The finding

`census::planar_face` (`crates/topo/src/census.rs:779`) is
`geo.faces.iter().find(|f| f.key == key)` — a linear scan of the planar
snapshot — and it is called per declared pair per crossing from
`ee_cross_backed` (`:1876`), and `confirm_declarations` scans the faces
per record the same way (`:3285`). This is the arena-scan shape of
`work/perf/plan.md` §2.1 that PERF-12's grep
(`vertex_faces\.iter()\|\.iter()\.filter(|`) could not see — a `find`,
not a `filter`. The backstop's arm 2 also walks the whole face arena
twice per census (`:3113`, `:3146`), once per box kind; linear, but
unswept. On the corpus the declared-pair and record counts are small,
so none of these shows in the measurements; they scale with
declarations × crossings.

## What a fix is

A `BTreeMap<FaceKey, usize>` (or the `FaceGeo` index) built once in
`snapshot`, read by `planar_face` and the confirm pass; D9-neutral (an
inverse map changes no order).
