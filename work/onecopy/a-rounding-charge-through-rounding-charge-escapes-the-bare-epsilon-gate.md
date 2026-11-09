---
id: a-rounding-charge-through-rounding-charge-escapes-the-bare-epsilon-gate
kind: issue
title: A conditioning charge routed through rounding_charge escapes the bare-EPSILON gate
status: closed
opened: 2026-11-04
closed: 2026-11-09
pr: 6620
---


Filed by `onecopy/p0-needle` (PR 6525), under item 7(c) of the decision on that PR.

## What

`[simple.conditioned]` says an ill-conditioned form is re-posed, not charged. The gate that looks for charges finds bare `f64::EPSILON` arithmetic. A charge routed through `geom_core::rounding_charge` (`running.rs`) carries the same conditioning while the gate cannot see it.

Two sites charged a globally posed harmonic for its conditioning:
- `pose/window.rs` `Window::read`. The level was charged `rounding_charge(terms)` with `terms ≈ ((|w| + R)/s)²`. The height was charged `Window.noise`, which was `rounding_charge(|o_a| + |o_b| + 2·lever)`. About 7.5e-13 at coordinates of 180, it refused `sweep`'s leaning prisms at 1e-12. **Re-posed in PR 6525**: both are read locally.
- `pose.rs` `coplanar_ellipses`, through `coplanar_level`'s `terms`. **Open**, see `coplanar-ellipses-reads-the-level-from-its-global-harmonics`.
- `pose.rs` `transverse`'s `Range` (found by the delta review of PR 6525). It still charges the same globally posed height `H` with the pose's `noise`: `Range::first_harmonic(.., noise)` feeds `band_arc` (`e = escalate + noise`) and `extreme_sign(row::EXTREME, …, r.noise)`. **Open.**

## Owed

Sweep `rounding_charge` callers whose `terms` grow with a ratio or a distance from the origin, rather than with the magnitudes of the terms actually summed. Then extend the gate, or the `[simple.conditioned]` sweep, to see a charge routed through the door.

## Resolved

- **The gate.** `bare-rounding-allowlist.sh` derives its charge doors from `geom_core::running`'s `pub` items, so every door is counted the day it is declared there. That covers `direction_rounding`, `exact_vec`, `unit_defect`, `dot`/`cross` and the two ULP constants, which the hand-spelled list missed. It counts a door through a path, an alias, a module alias, a glob or a scoped import. A charge carried in a variable, field or helper is still one site at its making, and the gate names that as a known gap. The second-pass sweep found every production helper of that shape in a file the list already schedules.
- **The sites.** `Window::read` was re-posed in PR 6525. `coplanar_ellipses` is `coplanar-ellipses-reads-the-level-from-its-global-harmonics`, closed by PR 6565. `transverse`'s `Range`, together with the PARALLEL screen and `coplanar_circles`, which read the same mint, is `transverse-range-charges-the-global-height-with-world-origin-noise`.
- **The sweep** of terms that grow with distance from the origin is recorded on `work/oneround/site-rounding-charges-owed-a-re-posing.md`.
