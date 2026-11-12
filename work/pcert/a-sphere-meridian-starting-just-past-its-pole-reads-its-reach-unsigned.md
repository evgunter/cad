---
id: a-sphere-meridian-starting-just-past-its-pole-reads-its-reach-unsigned
kind: issue
title: A sphere meridian starting just past its pole reads its reach unsigned, images mirrored and escalates
status: open
opened: 2026-11-12
priority: P3
cost: C
---

Filed by `pcert/derived-sphere-image-accuracy` (PR 6870), from its FULL
review's probes. This was there before that PR, which leaves it as it
was.

## What

`pcurve_cache.rs` `chart_image`, sphere meridian arm: the latitude
phase is read as `Azimuth::of(aa, reach)` with `reach = a_r.norm()`
(negated on the far half). At a pole start with `‖a_r‖` in band,
`d̂` is read off `b_r` (the `pole_frame` gate). `reach` is still
unsigned, so a start just past the pole on the other side of `d̂` reads
its `δ` on the near side and the image is mirrored there.

The miss is `2‖a_r‖`. In the review's probe, at `r = 0.01` and a start
`−1e-7` rad past the pole, it is `2e-9`. Check 4 reads that miss, so the
image escalates. It never certifies.

## Direction

At a pole start, read `reach` signed along the `d̂` the derivation chose
(`a_r · d̂`), so `δ` lands on the start's own side.
