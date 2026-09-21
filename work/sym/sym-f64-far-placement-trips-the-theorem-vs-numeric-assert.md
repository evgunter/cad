---
id: sym-f64-far-placement-trips-the-theorem-vs-numeric-assert
kind: issue
title: Sym<f64>/Sym<Probe> at a far placement panic in Decide's theorem-vs-numeric debug_assert: the point channel is not a proof
status: open
opened: 2026-09-14
priority: P0
cost: H
---


**Found by SYM-6's r2 review, off the door and pre-existing** (reproduced at
merge-base `c02e1b1e4` and at head `2621bc0a9`).

`impl<T: Decide> Decide for Sym<T>::sign_within` (`crates/geom-core/src/sym.rs`,
the `debug_assert!` "the numeric channel proved this margin nonzero and the
form says it is identically zero") fires at `Sym<f64>` and `Sym<Probe>` on an
ordinary document placed far from the origin: a stadium extruded / the M10-9
washer revolved on a sketch plane at `(d, d, d)` with `d = 1e9` at ε = 1e-9,
and already at `d = 1e6` at ε = 1e-12 (`crates/sweep/tests/sym6_r2_e2e.rs`,
the `Sym<f64>` and `Sym<Probe>` rows, which catch it and print it). The form is
a genuine theorem (the placement cancels exactly); the `f64` channel's rounding
at that magnitude exceeds the band and it answers a definite sign. The
assertion's comment says "the enclosure proves it" — true at `Interval`, where
the numeric channel is a certified bracket, and not at `f64`/`Probe`, where it
is a point comparison. At bare `f64` the same margin is refused typed by the
residual gates (`ResidualExceeded { Surface2Residual }`); at `Sym<f64>` it is a
panic in every profile with debug assertions on.

No shipped lane replays at `Sym<f64>` (the driver replays at `Sym<Interval>`),
so this is a limit of the unit-test lane rather than a shipped defect; it
stands in the way of any fixture-scale row that wants to drive `Sym<f64>` far
from the origin (e.g. one that would count `Disputed` on the M10-10 documents).
