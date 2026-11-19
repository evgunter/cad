---
id: step-import-cuts-faces-and-splits-edges-three-ways
kind: issue
title: step-import cuts a face over SolidSpec three ways and splits an edge two ways, each with its own helpers
status: dispatched
opened: 2026-11-17
priority: P1
cost: M
---

(Found by the review of PR 7084, `importgate/knot-place`; filed by its implementer.)

## What

Three passes over `SolidSpec` each cut a face into pieces with their own loop surgery:

- `crates/step-import/src/knot_split.rs`, `split_face_once`'s `trace` (the C⁰-knot split);
- `crates/step-import/src/normalize/singularity.rs`, `cut` (`singularity.rs:1166`), with `split_at_point` (`:591`);
- `crates/step-import/src/normalize.rs`, `mint_band` (`normalize.rs:336`).

There are two edge splitters, each with its own use rewriting:

- `knot_split.rs` `split_edge_at` with `expand_uses`;
- `normalize.rs` `split_at_param` (~`normalize.rs:156`) with `expand_split_uses` (~`normalize.rs:114`).

Their helpers are duplicated too: `use_start` (`knot_split.rs` and `normalize.rs:314`), and `boundary_census` (`knot_split.rs`) beside `face_boundary_census` (`normalize.rs:321`).

Each copy decides the same facts (which uses a split edge's halves replace, which vertex a use starts at, a face's census) with its own code, so a fix to one leaves the others.

## Repair shape

One edge splitter with its use rewriting, and one census and `use_start`, in one module the passes share. Then one face-cutting step, which the three passes call with their own cut lines. Behaviour moves nowhere: the C⁰-knot, band and singularity rows pin each pass.
