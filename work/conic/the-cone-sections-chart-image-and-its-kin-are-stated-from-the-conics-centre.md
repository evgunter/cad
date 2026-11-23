---
id: the-cone-sections-chart-image-and-its-kin-are-stated-from-the-conics-centre
kind: issue
title: The cone section's chart image and its kin are stated from the conic's centre
status: parked
opened: 2026-11-23
priority: P1
cost: H
parent: one-stored-form-waits-on-the-readers-of-a-conics-centre
refs: [7510, 7258]
blocked_on: [the-certified-lanes-harmonics-of-a-conic-are-formed-about-its-centre]
---


Cut from `one-stored-form-waits-on-the-readers-of-a-conics-centre` on 2026-11-23, when its census came in at 323 production reads against the row's 90 (PR 7510, the first part: directions, cover, extent and the gate). The functions below are pinned by that PR's gate (`clippy.toml` `disallowed-methods`, an `#[expect]` on each function, `scripts/gates/conic-centre-reads.counts`) as of `bd07a867e7`. **The sorting is the implementer's, from the census line of each read and not from reading each function through**, and a `°` marks a function that reads only `Circle::frame`. **What these readers should ask the conic is a design question in front of three designers** (`analysis/design-fork/conic-reader-interface`); two of the three already say this family is not one family, so the list is where to look, not the unit's shape.

## The functions (geom-brep)

- `pcurve_cache.rs`: `derive_class_zero` (the cone's focal-section image; `work/pcert/the-cone-sections-image-is-built-from-its-derived-centre.md`), `carrier_harmonic`, `anchored`, `run_iso_arc_checks`°, `torus_oblique_circle`°
- `pcurve.rs`: `ellipse_pcurve_on_plane`
- `certify.rs`: `arc_of_circle`° (check 4's `pcurve_map_residual` path)
- `props/curved.rs`: `require_cone_ellipse`, `cone_loop_winding` (the cone's ellipse read in its chart)

The unit's row asked that the cone section's chart image be shown stated from the stored curve; PR 7510 does not do it, and the pcert row stays open on this one.

## Decided (Ev, PR 7523, 2026-11-23), and parked on the polynomial

The chart image and its kin are stated from the conic's position through the polynomial along a conic (`the-certified-lanes-harmonics-of-a-conic-are-formed-about-its-centre`, which builds it); **parked on that row**. Run on main by the suspects lane (`analysis/oneround/conic-suspects/task1.md`): `pcurve_cache::anchored` and `carrier_harmonic` refuse and never certify wrongly, and their `ResidualExceeded` for an ellipse that lies in the plane to 1e-16 is a false refusal; `require_cone_ellipse` refuses `VolumeUncomputable` near a semi-axis of 1e6 m at ε 1e-9, which is the refusal `split` gives there.

## Moved to CONIC (2026-11-23)

Moved from ONEROUND by `git mv` when that track was split on Ev's answer to PR 7523 (`work/conic/log.md`); the id and the body above are unchanged.
