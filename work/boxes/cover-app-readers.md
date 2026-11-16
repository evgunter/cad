---
id: cover-app-readers
kind: unit
title: sweep's blend reach and surgery and editor-core's mate and measure reaches read topo's exported covers
status: closed
opened: 2026-11-02
priority: P1
cost: M
branch: boxes/cover-app-readers
closed: 2026-11-09
pr: 6588
---


Fifth unit of the cover build (`cover-geom-readings` has the order and the ratified text).

`sweep::blend::reach::sheet_clip` reads `topo`'s exported covers:
- its upper ends from `support` and `distance_range`;
- its lower `r` bound from `distance_range`'s near end, or kept as a clearance question if that is what it is.

That also retires `bandreach/blend-sheet-clip-picks-its-coaxial-arm-on-a-bracket`; coordinate with BANDREACH. editor-core's `mate::reach::face_reach` and `eval::measure::{reach_of, curve_reach}` read the covers too.

- **`topo::boolean::trace::carrier_extent`** still matches on kind to choose a cover or a literal and returns `None` for splines, which have a cover (PR 6383's review). Its last caller, `editor-core::trace::reach_of`, moves onto the cover here, and the kind match goes.

## Closed (PR 6588, 2026-11-09)

sweep's `sheet_clip` and editor-core's measure, mate and trace reaches read topo's face cover. `carrier_extent` is deleted, and `at_rest::beyond`'s plane/cylinder gate is gone. The line-site range is posed as distances (the rotating and counter-rotating parts). One full review and one fix pass. The sheet-clip aim went: no face today's doors build lets the pole cut bite. (BOXES orchestrator)

What PR 6588's readers cost the sym walks (the plate's ledger, and freezes across the sym11 corpus) is filed as `work/boxes/cover-readers-moved-the-plates-sym-ledger-undisclosed.md`.
