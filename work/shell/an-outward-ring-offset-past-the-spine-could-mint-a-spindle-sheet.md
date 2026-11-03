---
id: an-outward-ring-offset-past-the-spine-could-mint-a-spindle-sheet
kind: issue
title: offset: an outward ring-torus offset past R − r refuses TorusRing where it could mint a Surface::Spindle apple, and a spindle sheet's own offset refuses SpindleSheet
status: open
opened: 2026-11-03
priority: P3
cost: M
---

(SPIRICARM, stage 1 of `horn-and-spindle-tori-have-no-representation`, the consumer rows its ruling names.)

## What

`geom_brep::offset::offset_surface` (`crates/geom-brep/src/offset.rs`):

- the torus arm refuses `OffsetError::TorusRing` when `geom::ring_torus(R, r + d)` decides `Zero | Negative`. An outward offset of a fat ring past `R − r` is an apple sheet (`0 < R < r + d`), and `R = r + d` a horn: both are now `Surface::Spindle`, so the refusal is a kind change the door could mint (`Torus → Spindle`);
- the new `Surface::Spindle` arm refuses `OffsetError::SpindleSheet` outright; the sheet's offset by `d` is the sheet of minor radius `r + d` while `−(r + d) < R ≤ r + d`, a ring once `r + d < R`;
- `offset_distance` refuses a spindle `from` with the same error, and `offset_between` has no `Torus → Spindle` row.

The consumers that move edges with an offset (`topo::replace_face`'s transport, `offset_axial`'s `surface_residual`) carry `Surface::Spindle` arms that answer "no edge is moved with one", which this row would change.

## Done when

The offset door mints the spindle sheet an offset reaches, or the row records why it should not.

