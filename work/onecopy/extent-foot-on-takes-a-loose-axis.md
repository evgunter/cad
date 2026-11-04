---
id: extent-foot-on-takes-a-loose-axis
kind: issue
title: ExtentBall::foot_on and the extent's foot_on take a loose (origin, axis) instead of a geom::Line
status: open
opened: 2026-11-03
priority: P2
cost: E
---


Found by the frame-readings unit's axis-line sweep. `geom_brep::ExtentBall::foot_on(origin, axis)` (`crates/geom-brep/src/extent.rs`) and the extent's own `foot_on(origin, axis)` take an axis as two loose vectors and build `geom::Line { origin, dir: axis }` inside, so every caller takes its axis line apart to call them: `topo/src/boolean/carrier_eq.rs` (twice), `geom-brep/src/intersect.rs` (three times), `geom-brep/src/pair_section/torus.rs`, `torus_arms.rs`, `cylinders.rs` (twice) and `extent.rs` itself. It is the placement decision's cause 2 (a home's door taking loose vectors).

Fix: both take `&geom::Line` (a surface's `Frame::axis_line`).

Not done in that unit because the extent rows stayed on ONEHOME when ONECOPY was cut (`work/onecopy/program.md`'s keep-out); move this row there if that owner wants it.
