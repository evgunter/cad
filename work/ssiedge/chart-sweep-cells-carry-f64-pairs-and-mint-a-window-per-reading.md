---
id: chart-sweep-cells-carry-f64-pairs-and-mint-a-window-per-reading
kind: issue
title: ssi: UvRect carries f64 pairs, so every NurbsBoxes reading over a chart cell mints its own UvWindow
status: open
opened: 2026-10-09
---


Filed by the NURBS span-locator unit's fix pass (PR 4442), which made
every `NurbsBoxes` reading take a `UvWindow` (two `ParamRange`s) and
retired the readers' own `f64` doors.

## The finding

`crates/geom-brep/src/ssi/exhaust.rs`'s `UvRect` holds its sides as
`(f64, f64)` pairs, so a chart cell can be NaN or inverted, and every
reading over one mints its window at the call site: `UvRect::window()`
then a refusal arm.
- `ssi/boundary.rs`: `partials`, the corner `speed` closure,
  `side_reading`'s cut piece, `beyond_reach` and the strip pieces.
- `ssi/one_arc.rs`: `phi_over`, `exit_slope`, `crossing_near`.
- `ssi/exhaust.rs`: the sweep cell's `rect_box`.

Each site spells the refusal its reading already gave for a refused
window (`refused_box`, `NaN`, `false`), which is correct but repeated.

## Why it was not done in the unit

`UvRect` is the sweep's own cell type: about 27 constructions and 31
side reads across `exhaust.rs`, `boundary.rs` and `one_arc.rs`, most of
them splitting, meeting and padding cells in the SSI programs' ground.
Making it hold a `UvWindow` is a refactor of those sweeps rather than
of the locator, so it was left out of a locator PR.

## Repair

Give `UvRect` `UvWindow` sides, minted where a cell is made (the split
and pad arithmetic already produces ordered numbers there, and a `meet`
that comes out empty already answers `None`). Then a cell is a window
by construction, and the per-reading `window()` arms go away.
