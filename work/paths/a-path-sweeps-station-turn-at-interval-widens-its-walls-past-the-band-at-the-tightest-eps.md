---
id: a-path-sweeps-station-turn-at-interval-widens-its-walls-past-the-band-at-the-tightest-eps
kind: issue
title: A path sweep's station turn, evaluated at Interval, widens its walls' pcurve residual past the band at eps 1e-12 (a wide-start-frame sweep refuses where main's f64-turned one certified)
status: open
opened: 2026-10-19
priority: P2
cost: M
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
