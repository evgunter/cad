---
id: tangent-arc-ahead-in-band-builds-a-straddling-arc-at-interval
kind: issue
title: paths: a tangent arc whose target is collinear ahead only to within the band builds an arc from a straddling across at Interval, and the next junction reads NaN
status: open
opened: 2026-10-19
priority: P2
cost: M
---


Found while writing `arc_onto`'s degeneracy rows (`crates/profile/tests/arc_onto.rs`).

`tangent_arc_geom` (`crates/profile/src/path.rs`, the free fn both `tangent_arc_to` and `arc_onto` call) decides `path_collinear_target` on `across` and, when it is ZERO and `along` positive, proceeds to `tangent_arc`, which builds a line only where its quarter-tangent `across / (|d| + along)` is EXACTLY zero (`crate::is_exact_zero`). The decision and the build read two different facts. At f64 an axis-aligned or exactly symmetric director makes `across` exactly zero, so the two agree; at `Interval` a director such as `.toward(1, 1)` normalizes to an enclosure, `across` straddles zero, the gate decides it collinear, and `tangent_arc` builds an arc with `rho = |d|² / (2·across)` from the straddling enclosure.

Reproduction (passes at f64, refuses at Interval): `At(0,0), LineTo(2,0), Toward(1,1), TangentArcTo(Point(3,1)), LineTo(0,1), LineTo(Start)` through `profile::replay` at `Interval` refuses at step 4: "whether the path turns at this junction is undecided: margin is invalid (NaN or a refused enclosure)". `arc_onto((3,1)).toward(1,1)` from the same leg end refuses the same way, through the same kernel.

The likely shape of the fix: when the gate DECIDES the target collinear ahead, emit the straight segment the decision names rather than re-reading exact zero. At f64 that would turn today's in-band near-collinear tiny-bulge arcs into lines, so it moves stored bits and wants its own rows.
