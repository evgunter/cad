---
id: spline-skin-and-loft-refusals-state-no-standing
kind: issue
title: SplineError, SkinError and LoftError state no standing, so an unbuilt carrier is classed only by its sentence
status: open
opened: 2026-11-22
priority: P3
cost: E
refs: [f64-built-data-is-built-at-t]
---


Filed by the `f64-built-data-is-built-at-t` lane, 2026-11-22.

DESIGN `[failure.taxonomy]` row 2: a state reachable by input that is valid but unbuilt is a typed error with standing `Refusal::Unbuilt`, and "each refusal type a valid input can reach states every variant's class in one exhaustive method". `geom_core::spline::SplineError` (`crates/geom-core/src/spline/knots.rs`), `sweep::SkinError` (`crates/sweep/src/skin.rs`) and `sweep::LoftError` (`crates/sweep/src/loft.rs`) have no such method: `grep -rn "Standing for" crates/*/src` finds no impl for any of the three.

`SplineError::CarrierKnotNotAPoint` and `CarrierWeightNotAPoint` are row 2: a carrier at an enclosure scalar may hold an enclosure knot or weight (C6), and its readers do not take one yet (`work/inlane/an-enclosure-carrier-holds-point-weights-and-knots.md`). PR 7292 re-worded their `Display` to say so and to end on `NOT_YET_ENDING` (it had told the caller to supply the carrier "built at a point scalar (`f64`) and lifted"). The class is still carried only by that sentence. At the loft's door the same refusal ends `Recourse: evaluate the loft at a point scalar` (`skin.rs` `POINT_SCALAR_LEVER`, through `scalar_limit`), which is a lever that works and is a different class again.

Owed: a standing on the three types, with these two variants `Unbuilt`, and the loft's ending for them decided against it.
