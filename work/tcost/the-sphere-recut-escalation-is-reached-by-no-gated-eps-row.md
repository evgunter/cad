---
id: the-sphere-recut-escalation-is-reached-by-no-gated-eps-row
kind: issue
title: the sphere recut's mapped-source escalation arm (RECUT_MAPPED_ENCLOSURE_HI) is reached by no gated eps row
status: open
opened: 2026-09-25
---


Found by PATHS' `geom-brep-sketch-segment-full-turn` (#3254). The file
is shared with TINT (`territory` names tcost and tint); filed here.

## What stands

`sweep/tests/m5_s12_curved_ops_interval.rs`'s
`interval_sphere_subtract_decides_definitely_after_the_recut` and
`review_arceval_r1_probes.rs`'s `e2_recut_escalation_hi_is_pinned_to_the_measured_constant`
each have an arm for `ε < RECUT_MAPPED_ENCLOSURE_HI`: the plate − ball
recut escalating on `carrier_matches_mapped_source`, pinned bit-exactly.

Since #3254, `SketchSegment::restrict` keeps the parent arc's carrier
instead of re-deriving the sub-arc's centre from its chord. The chain's
enclosure then fits inside ε at every gated row (1e-6, 1e-9, 1e-12), and
the recut certifies at all three. Measured with `CAD_TOLERANCE_EPS`:
- it decides at 2e-13 and above;
- at 1e-13 it escalates with `hi = 1.0679358714628548e-13`;
- at 5e-14 it escalates with `hi = 5.2405466889108475e-14`;
- at 1e-14 it refuses with `ClassificationInvariant`.

The constant is re-stated at the 1e-13 measurement, and the arm still
exists, but **no gated ε row reaches it**. It runs only when someone
sets `CAD_TOLERANCE_EPS=1e-13` by hand. The measured `hi` also tracks
ε, so the constant's old claim that it was "ε-INDEPENDENT" is corrected
at the constant.

## What is owed

A decision: either give the arm a row that reaches it (a per-process
`Tolerance::init` at 1e-13, which holds under nextest's one process
per test but not under `cargo test`), or accept that it is documentation
of a sub-matrix ε and say so there.
