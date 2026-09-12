---
id: fit-delta-probe-can-exceed-the-picture-it-sizes
kind: issue
title: fit_delta's probe is sized off the requested delta, so it can tessellate more than the picture it is sizing, on the UI thread
status: closed
opened: 2026-09-10
closed: 2026-09-12
pr: 2464
---

## The finding

Measured by the PERF GUI lane (`perf/explore-gui`; release, 4-vCPU
box). `scene::fit_delta` (`crates/viewer/src/scene.rs:947`) sizes the
display budget by tessellating once at `PROBE_FACTOR` (8) × the
**requested** δ and solving `triangles ≈ C/δ`. Its own docs price that
at "about an eighth of a full tessellation", which holds only when the
budget does not move δ. Whenever the budget has to coarsen by more than
8×, the probe tessellates `budget × δ_final / (8 × δ_requested)`
triangles — more than the picture it is sizing:

| document | probe triangles | probe wall (frozen UI) |
|---|---:|---:|
| `gallery_ring` | — | 376 ms |
| `die_composed_tour` | — | 219 ms |
| `tube_ring` | 2.1 M | 2744 ms |
| `hollow_tube_ring` | 3.3 M | 4080 ms |

It runs inside `ui()` (`app.rs`'s fit block) before the index seam is
submitted, once per document that arrives, so `Open` stops repainting
for it. `work/view/ui-thread-work-after-the-index-seam.md` hit (1) is
this cost with the wrong size on it.

## What a fix is

"Stop doing this": the probe must never be larger than the picture.
Probe coarse and solve, committing only a δ whose predicted count the
probe has already bounded (a probe at a δ coarser than any admissible
answer costs at most `budget / 8` triangles by the same 1/δ law); or
move the probe onto the index worker, which already holds the body.
Low clarity cost; `scene.rs` last moved 2026-09-06. VIEW territory
(`crates/viewer/*`), announced there.
