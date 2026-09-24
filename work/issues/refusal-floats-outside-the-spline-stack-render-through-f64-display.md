---
id: refusal-floats-outside-the-spline-stack-render-through-f64-display
kind: issue
title: Refusals outside the spline stack still interpolate f64 through bare Display, so a value past 1e16 or under 1e-4 prints positionally
status: open
opened: 2026-09-24
priority: P3
cost: E
---

Filed by PORT's `port/wrap-a` lane, which closed
`refusal-messages-render-floats-through-f64-display` for the spline
stack (`geom-core`'s `spline/` and `linalg/lsq.rs`, and `geom`'s
curves and surfaces). That unit added the one rendering the rest of
the tree can use: `geom_core::Readable` (`crates/geom-core/src/readable.rs`)
— positional on `1e-4 ≤ |x| < 1e16` and at zero, exactly as the bare
`{}` reads there, and scientific outside, so `1e308` reads `1e308`
rather than 309 digits and `1e-300` reads `1e-300` rather than three
hundred zeros.

## The population left

A refusal `Display` that interpolates an `f64` with a bare `{}`,
outside the spline stack. Swept with a heuristic scan: an `f64` (or
`(f64, f64)`) field declared in the same file and named bare inside an
`impl … fmt::Display for` block, test modules cut. The hits, by crate
(the owner is `python3 scripts/work.py territory` on each path):

- `crates/editor-core/src/analysis.rs`: `AnalysisPolicyError` (`{mass}`),
  `ParamBoxError` (`[{lo}, {hi}]`).
- `crates/editor-core/src/distribution.rs`: `DistributionFault`
  (`{sigma}`, `{lo}`, `{hi}`).
- `crates/editor-core/src/edit.rs`: `EditError`'s improper placement
  frame (`{determinant}`); `crates/editor-core/src/placement.rs`:
  `FrameFault` (`{determinant}`).
- `crates/editor-core/src/mate.rs`: `MateFault` (`{metres}`,
  `{radians}`, `{arm}`, `{value}`).
- `crates/editor-core/src/range.rs`: `RangeRefusal` (`[{lo}, {hi}]`).
- `crates/geom-brep/src/offset_fit.rs`: `OffsetFitError` (`{d}`,
  `{tolerance}`, `{achieved}`, `{b}`, `{bound}`, the `(u, v)` pair).
- `crates/geom-brep/src/offset_meters.rs`: `MeterError` (`{floor}`,
  `{speed_lever}`, `{thinness}`, `{reach}`, `{headroom}`, the
  curvature pair).
- `crates/geom-brep/src/ssi.rs`: `SsiError`'s foot point (`t = {t}`).
- `crates/profile/src/validate.rs`: `FilletLegCarrier` (`{radius}`,
  `{angular_margin}`).
- `crates/step-export/src/lib.rs`: `StepExportError` (`{volume}` twice,
  the uncertainty override `{value}`).
- `crates/step-import/src/error.rs`: `StepImportError`'s ε_in override
  (`{value}`).
- `crates/sweep/src/blend/mod.rs`: `BlendError` (`{radius}`,
  `{margin}`, `{size}`); `crates/sweep/src/revolve/tube.rs`:
  `TubeError` (`{eps}`).
- `crates/topo/src/query.rs`: `RimError` (`{gap}`).
- `crates/viewer/src/camera.rs`: `CameraError` / `CameraOpError` /
  `CameraOp` (`{radius}`, `{fov_y}`, `{aspect}`, `{required}`,
  `{max_distance}`, `{factor}`, `{yaw}`, `{pitch}`, `{right}`, `{up}`);
  `crates/viewer/src/display.rs`: `DisplayFault` (`{determinant}`);
  `crates/viewer/src/scene.rs`: `SceneError::InvalidDisplayTolerance`
  (`{delta}`). The viewer already switched two arms to `{:e}` by hand
  (`SceneError::DisplayToleranceOverflowsMillimetres`,
  `CameraError::SceneRadiusOverflowsZoomBand`), which is the class
  answered per arm.

Not hits: sites whose value is non-finite by construction
(`quantity`'s `FmtQuantityError`, `step-export`'s "no Part 21 real
representation", the viewer's "not a finite number" arms) — `Readable`
renders those exactly as `{}` does.

## What the scan cannot see

A field declared in another file than its `Display`; a value reached
through a method or an expression rather than a named field; an `f64`
inside a `Vec`, an array or a generic `T: Real`; and prose assembled
with `format!` outside a `Display` impl. The per-crate owner's pass
should look there too.

## What the fix looks like

Each site through `geom_core::Readable` (every crate above depends on
`geom-core`), with any message-pinning test moving with it; or, where a
site's value is provably inside `[1e-4, 1e16)`, a line saying so.
