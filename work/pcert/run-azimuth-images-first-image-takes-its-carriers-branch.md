---
id: run-azimuth-images-first-image-takes-its-carriers-branch
kind: issue
title: chord_join's window walk takes its first image's branch from the carrier's parameter, so a run whose first carrier is stated whole periods off reads its window that far off; the pin is the placed image (B)
status: open
opened: 2026-10-15
priority: P3
cost: E
---


Found by PR 5126's reviews; deferred from PR 5126 (unit A of the
`[ev]` PR 5142 answer) to unit B.

## What

`topo::chord_join::run_azimuth_images` (`crates/topo/src/chord_join.rs:3096`)
pins each image to the previous one's exit. The first takes `base`
(`chord_join.rs:3133`), the carrier's own branch. So a run or ring whose
first carrier is stated N periods off the azimuth it covers reads its
whole window N periods off, and so do `face_azimuth_window` and
`face_loop_azimuth_windows`. Measured on PR 5126's fixtures: the
half-turn cylinder patch 5 periods below, walked from its far-stated
arc, read the window `(−31.4, −28.3)` where its twin reads `(0, π)`.
Today it takes a loop whose `first` is the far-stated carrier.

## Why A did not pin it

PR 5126 pinned the first image to the chart's azimuth at the
carrier's midpoint (`stable_azimuth`). That moved a valid body:
`editor-core`'s
`refusal_concision::the_cone_plane_union_refusal_names_the_conic_and_ends_on_its_recourse`.
A revolved cone face can start its walk on the seam meridian at
`u = τ`, whose midpoint's azimuth reads −2.4·10⁻¹⁶. The full-turn
window moved from `[0, τ]` to `[−τ, 0]`, and the union refused at an
edge–face touch instead of naming its hyperbola. A seam edge sits on
the window's cut by rounding, so no anchor read off one edge's own
azimuth is safe.

## Fix

C4's anchor: the image placed over the edge's span in the chart's
principal window through a stated reference (unit B's `near`). Once B
places images, the first image is the placed row and needs no pin of
its own. The row to carry: start `run_azimuth_images` at every member
of PR 5126's `cylinder_patch_below(5.0)` and `(10.0)` and of
`slit_cap_below(3.0)` and `(10.0)`, and read each start's window
against the principal twin's from the same member.
