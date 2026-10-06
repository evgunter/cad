---
id: sweep-body-makes-every-caller-derive-its-start-frame
kind: issue
title: sweep_body takes a start placement every caller derives the same way, from path_start_frame at the path's start
status: open
opened: 2026-10-03
---


Found by SHOW's `long-turn-helix-has-no-demo` (PR 3918), the third
tour scene to spell it.

## What

`sweep::sweep_body(profile, place, path, stations, v_degree, tol)`
takes `place`, the profile's placement at the path's start. Every
demo caller derives it the same way: read the path's domain start, take
`ders1` there, and hand point and tangent to
`geom_core::linalg::frame::path_start_frame`.

- `demos/tour/src/projectbox.rs`, `spring` (the coil);
- `demos/tour/src/klein.rs`, `sweep_loop` (the loop);
- `demos/tour/src/skinned.rs`, the arc duct, the S duct and the
  twisted duct (three times in one file).

The M8-14 tests do it too, through their own `normal_start_place`
helper (`crates/sweep/tests/common`). Five spellings of one
derivation is a door the library does not have: a sweep that places the
profile in the plane normal to the path's start tangent by itself, or a
`place` argument that can say "normal to the path".

## Done when

A caller can sweep a profile normal to its path without deriving the
start frame by hand, and the tour's sweeps use that door. The explicit
`place` argument stays for a profile that is not normal to its path.
