---
id: cover-app-readers
kind: unit
title: sweep's blend reach and surgery and editor-core's mate and measure reaches read topo's exported covers
status: parked
opened: 2026-11-02
priority: P1
cost: M
blocked_on: [cover-topo-faces]
---


Fifth unit of the cover build (`cover-geom-readings` has the order and the ratified text).

`sweep::blend::reach::sheet_clip` reads `topo`'s exported covers:
- its upper ends from `support` and `distance_range`;
- its lower `r` bound from `distance_range`'s near end, or kept as a clearance question if that is what it is.

That also retires `bandreach/blend-sheet-clip-picks-its-coaxial-arm-on-a-bracket`; coordinate with BANDREACH. editor-core's `mate::reach::face_reach` and `eval::measure::{reach_of, curve_reach}` read the covers too.

- **`topo::boolean::trace::carrier_extent`** still matches on kind to choose a cover or a literal and returns `None` for splines, which have a cover (PR 6383's review). Its last caller, `editor-core::trace::reach_of`, moves onto the cover here, and the kind match goes.
