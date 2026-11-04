---
id: a-rounding-charge-through-rounding-charge-escapes-the-bare-epsilon-gate
kind: issue
title: A conditioning charge routed through rounding_charge escapes the bare-EPSILON gate
status: open
opened: 2026-11-04
---


Filed by `onecopy/p0-needle` (PR 6525), under item 7(c) of the decision on that PR.

## What

`[simple.conditioned]` says an ill-conditioned form is re-posed, not charged. The gate that looks for charges finds bare `f64::EPSILON` arithmetic. A charge routed through `geom_core::rounding_charge` (`running.rs`) carries the same conditioning while the gate cannot see it.

Two sites charged a globally posed harmonic for its conditioning:
- `pose/window.rs` `Window::read`. The level was charged `rounding_charge(terms)` with `terms ≈ ((|w| + R)/s)²`. The height was charged `Window.noise`, which was `rounding_charge(|o_a| + |o_b| + 2·lever)`. About 7.5e-13 at coordinates of 180, it refused `sweep`'s leaning prisms at 1e-12. **Re-posed in PR 6525**: both are read locally.
- `pose.rs` `coplanar_ellipses`, through `coplanar_level`'s `terms`. **Open**, see `coplanar-ellipses-reads-the-level-from-its-global-harmonics`.

## Owed

Sweep `rounding_charge` callers whose `terms` grow with a ratio or a distance from the origin, rather than with the magnitudes of the terms actually summed. Then extend the gate, or the `[simple.conditioned]` sweep, to see a charge routed through the door.
