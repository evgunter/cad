---
id: main-red-aimed-ray-across-a-pair-in-band-at-1e-6
kind: issue
title: Main red: offer_rows' aimed_ray_across_a_pair_in_band panics at ε 1e-6
status: open
opened: 2026-11-02
priority: P3
cost: E
---

Found by SPLITDOOR (`splitdoor/cone-apex-boolean`, PR 6266) running its
edited rows at three ε. Red identically on main `3a672b604`.

## What

`CAD_TOLERANCE_EPS=1e-6 cargo nextest run -p topo -E
'test(aimed_ray_across_a_pair_in_band)'` panics at
`crates/topo/src/boolean/offer_rows.rs` (`aimed_pair`'s fallthrough,
"the aimed ray reads the pair"). At the default ε it passes. The fixture
places the second brick a fixed `gap` above the first, so at 1e-6 the
side reader answers neither `Ok` nor an in-band pair.

## Where to look

Whether the offer row's `gap` should scale with the run's ε, as the
other offer rows' poses do, or the reader's answer at 1e-6 is a defect.
