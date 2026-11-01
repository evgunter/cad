---
id: run-azimuth-images-first-image-takes-its-carriers-branch
kind: issue
title: chord_join's window walk takes its first image's branch from the carrier's parameter, so a run whose first carrier is stated whole periods off reads its window that far off; the pin is the placed image (B)
status: closed
opened: 2026-10-15
priority: P3
cost: E
closed: 2026-10-15
branch: pcert/azimuth-placement
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

## A row B must flip

`topo::pcurves::pole_slit_tests::a_body_stated_periods_off_reads_downstream_as_its_principal_twin`
pins `chart_region_overlap`'s `Err(TouchingBoundary)` on the half-turn
cylinder patch's second face at 6 and 10 periods below (PR 5126's
second review, S8). That is a refusal of a valid body: the corner reads
`π` plus 4 ulp because the image runs on the carrier's far-stated
parameter. Once B places images over their edges' spans, that row is
expected to read `PositiveArea` like its twin. B re-reads it and moves
the pin.

## Closed

By unit B1 (branch `pcert/azimuth-placement`): every derived image is
placed over its edge's span in the chart's principal window
(`geom_brep::ANGLE_WINDOW_START`), so `run_azimuth_images`' first image,
derived over its own edge's span, is placed like every other, and no
pin of its own is needed. The row this item asked for is back:
`topo::chord_join::first_image_rows::the_window_walk_reads_a_far_stated_face_in_its_twins_window_from_any_start`.
`refusal_concision`'s cone union reads as before, with the window's
cut at `−1`, at `0` (on the cone's seam, `u ≈ −2.4e-16`) and at `0.5`
(the seam placed at `τ`), so no reader on that union path tells the
window's copies apart. What turned it red under unit A's pin, traced by
putting the pin back: the pin read the first rim's midpoint azimuth off
the physical radial direction (`π`), but the cone face lies on the
`v < 0` nappe, whose chart azimuth is the physical one plus `π`, and the
image read `u ≈ 0` there. The half-period gap refused in
`nearest_branch` (`SectionInvariant`, "half a period from the previous
edge's exit"), the cone face's trim lost its window
(`contain::curved_face_placement_decided` read `Trim(None)`), and the
union fell to `CurvedPierceUnsupported`. The placement reads the chart's
own azimuth through the derivation, so the nappe is right.

The `TouchingBoundary` pin on the half-turn patch's second face at 6 and
10 periods does **not** flip: placement names the image's values in the
window, but the patch's corner is the image evaluated at the carrier's
far-stated end parameter, `α + (π − 6τ)` with `α ≈ 6τ`, which still
rounds 4 ulp off `π`. The body is valid; the refusal is the overlap's
exact-collinearity reading of a near-flush seat on rounded input, as at
any near-flush seat. The pin stays and says so.
