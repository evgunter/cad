---
id: a-villarceau-image-escalates-at-the-interval-door-at-eps-1e-12
kind: issue
title: A Villarceau circle's correct image escalates at the interval stated door at ε 1e-12
status: open
opened: 2026-11-12
priority: P3
cost: C
---

Filed by `pcert/derived-sphere-image-accuracy` (PR 6870), from its FULL
review's probes. This was there before that PR, whose change does not
reach the Villarceau arm (`pcurve_cache.rs` `torus_oblique_circle`).

## What

A Villarceau circle on a torus of minor/major ratio `s = 0.9`, stated by
its derived `FocalSection` image, escalates at the `Interval` stated
door at ε 1e-12. Its true miss is 3.2e-14. At ε 1e-9 it certifies for
`s` from 0.9 down to 1e-7, and the f64 door certifies it.

It is an over-wide interval enclosure of a correct image, so nothing
false is certified; the lever ends it.
