---
id: near-pole-read-tries-only-the-images-own-orbit-point
kind: issue
title: The near-pole read tries only the image's own orbit point, so a whole-turn gap at a band-sized lever refuses
status: open
opened: 2026-10-13
priority: P3
cost: E
---


Found on branch `pcert/cone-singular-gate-lever`, writing
the narrow-cone joint's rows.

`topo::pcurves::near_pole_gap_closes` (`crates/topo/src/pcurves.rs:4365`)
tries the orbit points `m = 0` and, with a sphere twin, `m = ±1` half
steps (`:4372`). It never tries a whole period off the image. So a
joint whose gap is a whole turn (`τ`) at a band-sized lever `ℓ` refuses
whenever `τ·ℓ` is in the band. The marks read Zero or in band, and the
image's own point is `τ·ℓ` metres off in the chart metric, though
the image one period over meets the joint exactly.

Row `pcurves::stretch_meter::a_narrow_cone_joint_clear_of_the_apex_takes_its_own_image`
shows it: at `α = 0.01`, `d = 2e-8` (lever `2e-10`), a gap of `τ`
reads `Discontinuity`. The row admits a refusal there and requires a
lift only where the marks decide.

Both lifts name the one point, so taking `m = ±1` (and `±2` half steps
with a twin) where its gap decides within ε would be sound by the same
argument as `m = 0`. Loud today (a refusal), never a wrong answer.
Probe (six crates, default ε): no suite joint reaches it. The 31 cone
joints whose marks missed at a lever with room were all gaps of exactly
`±π` at levers of 0.5–1 m, in `site_rows`' walk, cleared by its
designed fallback.
