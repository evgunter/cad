---
id: no-row-guards-that-an-f31-renamings-readers-take-only-its-sine-and-cosine
kind: issue
title: No row guards that a reader of an F31 renaming takes the angle only through its sine and cosine (or as a copy), so a box consumer reading the spin's value would pass
status: open
opened: 2026-11-20
priority: P3
cost: M
refs: [frame-lanes-still-pick-their-atan2-formula-across-a-landmark-cut, 7165]
---


Found in review of CHARTTAIL PR 7165 (`charttail/solve-atan2`).

## What

F31 (`docs/predicate-dimension-audit.md`) keeps a cut-side or copy choice off the log only because no outcome of it is a reading. Both copies name one angle, so a reader that takes the angle only through its sine and cosine, or only as a copy it renames again, cannot tell the choice was made. Nothing checks that premise.

At `editor_core::mate::solve::box_atan2` (`mate_spin_cut_side`) it is a prose invariant. Its home is `box_atan2`'s doc: the spin is read at box and symbolic scalars only through its sine and cosine, by `turn_to` in `crates/editor-core/src/mate/value.rs`. A box or `Sym` consumer that starts reading the spin's value would pass every row while the choice silently became a verdict. Examples: a spin quoted in a payload, compared against a written angle, or differenced without a sine and cosine.

At `f64` such a reader already exists, the gesture read-back (`Coord::Spin(None) => chart.spin(..)` over `Chart::<f64>`). It is outside F31 only because `f64` reads the platform `atan2` and asks no cut side.

## The class

The same premise carries `geom::Azimuth::near` (`azimuth_near_cut_side`) and `geom_core::Period`'s `near`, `above` and `run_from` marks. Their readers are listed in F31 by prose alone.

## What would close it

A row that goes red when a box or `Sym` consumer reads the angle's value rather than its sine and cosine, or a copy it renames again. One shape: run the mate corpus at `Interval` with the cut side forced each way (both formulas) and assert the placed poses are equal enclosures. That shape fits each F31 site.
