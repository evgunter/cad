---
id: ssi-a-chart-rung-can-be-narrower-than-eps-in-metres
kind: issue
title: ssi: a chart tube rung's metre width is the rung times local over sup speed, so the tube can be narrower than eps in metres
status: open
opened: 2026-10-06
priority: P3
---


(Filed by the transversality-lever lane, 2026-10-06, from the fork on
`work/ssi/ssi-transversality-at-a-point-is-spelled-three-ways.md`.)

## What

Limb 3's chart tube pads each axis by the rung over that axis's sup
chart speed (`ChartSpeeds::pad`, `crates/geom-brep/src/ssi/enclose.rs`
~527; read in `certify_branch`'s chart arm,
`crates/geom-brep/src/ssi/certify.rs` ~1442). Where the chart runs
slower than its sup, the pad's width in metres is `rung · local/sup`
speed, less than the rung. The ladder's floor is a multiple of ε in
rung units (`tube_ladder`), so on a chart whose speed varies by more
than that multiple the narrowest tube can be narrower than ε in
metres, where its chain no longer separates anything the run can
resolve.

## Repair shape

State the floor in metres at the slowest speed over the windows (a
certified lower speed bound), or refuse a rung whose metre width falls
below the floor.
