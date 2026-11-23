---
id: a-path-sweeps-station-turn-at-interval-widens-its-walls-past-the-band-at-the-tightest-eps
kind: issue
title: A path sweep's station turn, evaluated at Interval, widens its walls' pcurve residual past the band at eps 1e-12 (a wide-start-frame sweep refuses where main's f64-turned one certified)
status: closed
opened: 2026-10-19
priority: P2
cost: M
pr: 6138
closed: 2026-10-31
---


Found merging main into #5068 (`the-sketch-plane-is-its-frame`).

`sweep/tests/a_loft_computes_its_walls_in_t.rs`,
`a_sweep_from_a_wide_start_frame_encloses_its_family_at_interval` (main's
#5233): at `CAD_TOLERANCE_EPS=1e-12` the `Interval` sweep refuses at the
skin's chart-residual check, `pcurve_map_residual` indeterminate over
`[0, 1.08e-12]` against the `1e-12` band (sample 8). The start frame's box
is `ε/256 ≈ 4e-15` wide, so the width is not the box's. It passes at the
default ε and at 1e-6, and on main at all three.

What differs from main: `sweep_places` decides each station's tangent and the
turn at the lane scalar (`UnitVec3::new` under `sweep_path_tangent`, then
`RigidMotion::turned_between`, the quaternion matrix), where main computed the
axis and angle in `f64` and entered them as points. The few-ulp enclosure of
the normalised tangents and the turn's entries is what the wall certification
amplifies to about the band at the tightest ε. Anchoring the turn at the world
origin and setting the station origin exactly (so the start origin enters
once) was tried and did not change the margin.

Owed: find which factor the certification amplifies (the turn's entries, or
the section points they carry) and narrow it, for example evaluating the
turn's matrix from the decided tangents' value channel once the decision is
made at `T`, which keeps the decision at the lane and the entries points. The
row pins the refusal at `ε ≤ 1e-12` by name until then.

## 2026-10-21: the margin moved, the amplification did not

With the sweep's caps charted along the start frame's own `x` axis
(`swept::cap_plane`, PR #5547) the row encloses at all three gated ε, so
its 1e-12 pin is gone. The refusal is still there one notch tighter: at
`CAD_TOLERANCE_EPS=5e-13` the same row refuses at the same check,
`pcurve_map_residual` indeterminate over `[0, 5.22e-13]` against the
`5e-13` band (sample 8), a residual about 1.04× the band where the 1e-12
one was about 1.08×. The station turn's amplification is still owed; no
gated row pins it now, so the reproduction is the 5e-13 run above.

## Re-homed at PATHS's close (2026-10-21)

Moved from `work/paths/` to INLANE (`work/inlane/`) when PATHS closed (`docs/doc-ledger/paths-leaves-the-tracker.md`), not to PATHS's successor PROFILE. The site is `crates/sweep/src/skin.rs`'s `sweep_places`, which is INLANE's territory. The question is what a station turn evaluated at the lane scalar costs the walls' certification, and that is INLANE's charter: every carrier's values in the lane scalar. The item's id, content and history are unchanged.

## Closed by CARVE's PR 6138 (2026-10-31)

PR 6138 (`work/carve/sweep-frame-is-a-minimal-rotation-from-the-start-tangent.md`)
replaces the station turn with a carry computed once in `f64` from the
path and the station count (`sweep::skin::PathCarry`; `crates/sweep/README.md`
`[sweep.frame]`). Every station's rotation now enters the lane scalar as
the point its `f64` entries are (`RigidMotion::computed_at`), and no
station turn is evaluated at `Interval`. That is the narrowing this row
owed: the decisions are taken once, and the entries are points. The
reproduction, `a_sweep_from_a_wide_start_frame_encloses_its_family_at_interval`
at `CAD_TOLERANCE_EPS=5e-13`, fails on main (PR 6138's reviewer, at
`a7fdbf5db8`) and passes at the PR's head (re-run 2026-10-31 in a debug
build).
