---
id: path-sweeps-exhaust-memory-at-sym-interval
kind: issue
title: five path sweeps exhaust a 12 GB cap at Sym<Interval>, on main and with station frames minted at the scalar alike
status: open
opened: 2026-11-14
priority: P2
---



Filed by the skin-turn lane (PR 6949; measured while minting each path-sweep station's frame at the lane scalar, `crates/sweep/src/skin.rs` `PathCarry::minted`).

**What runs out.** `sweep::test_support::sweep_body_construction::<Sym<Interval>>` inside `geom_core::sym::with_session` (budget 4096 terms, degree 128; each body on its own thread, the process under a 12 GB address-space cap). It aborts with `memory allocation of … bytes failed` after 72–118 s on five of the path sweeps `crates/sweep/tests/self_intersection_proves_injectivity.rs` builds at `f64` and `Interval`:

| Row's path | Stations | Main (frames entered as opaque atoms) | Frames minted at the scalar |
|---|---|---|---|
| closed spine, radius 3 | 9 | OOM at 1e-9 and 1e-12 | OOM at 1e-9 and 1e-12 |
| S duct | 13 | OOM at 1e-9 and 1e-12 | OOM at 1e-9; at 1e-12 finishes, refusing `pcurve_map_residual` |
| half-turn helix | 33 | OOM at 1e-9 and 1e-12 | OOM at 1e-9 and 1e-12 |
| 370° ribbon | 48 | OOM at 1e-9 and 1e-12 | OOM at 1e-9 and 1e-12 |
| two-turn helix | 65 | OOM at 1e-9 and 1e-12 | OOM at 1e-9 and 1e-12 |

So the frames are not the cause: main's twelve opaque atoms per turn exhaust memory as well. Each path is a cubic interpolant (`NurbsCurve3::interpolate`) with 9 to 65 points.

At 256 terms and degree 32 the ribbon and the helix finish, in 32 s and 44 s. Both refuse at the end cap's `newell_plane_residual`: 132 of 13 640 and 132 of 18 519 decisions are symbolic, and 18 381 and 25 747 forms freeze. The closed spine still runs out at that budget. The three-station elbow (`sweep::test_support::elbow_path`) finishes at 4096 terms in 26–51 s.

**Owed.** Find what grows: form expansion inside the budget, the DAG, or a per-decision cache. Then make a session's memory bounded by the budget, so a body that cannot be certified symbolically refuses or freezes rather than aborting the process.

The probe that measured this is `crates/sweep/tests/skin_turn_probe.rs` at commit `c9ea79bf` on `onehome/skin-turn`. It is an ignored test: `PROBE_ROW=<name> CAD_TOLERANCE_EPS=<eps>` selects a row, and `PROBE_TERMS` sets the budget.
