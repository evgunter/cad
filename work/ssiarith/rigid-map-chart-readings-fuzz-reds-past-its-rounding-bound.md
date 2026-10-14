---
id: rigid-map-chart-readings-fuzz-reds-past-its-rounding-bound
kind: issue
title: ssi/enclose: the rigid-map chart-readings fuzzer reds past its 256-ulp rounding bound at a recorded seed
status: open
priority: P2
cost: E
opened: 2026-10-14
---

(Filed by the ENCL implementer of `param-span-reversed-at-a-build-tells-two-stories`. The row went red on PR 5023's hosted CI at the 1e-6 row, run 38502955863. That branch leaves `crates/geom-brep/src/ssi/` unchanged against main.)

## What

`ssi::enclose::tests::a_rigidly_mapped_wall_reads_the_seated_chart_speeds_floors_and_tube` (`crates/geom-brep/src/ssi/enclose.rs`) is a counterexample-search fuzzer with a fresh seed per run. It asserts each chart reading of a rigidly mapped wall lies within `within_rounding` of the seated one: `256 · f64::EPSILON · (1 + reach) · scale · max(|a|, |b|)`.

A local hunt at `CAD_FUZZ_EFFORT=8` found a counterexample within three runs. It reproduces at the default ε, 1e-6 and 1e-12 alike, so it does not depend on ε:

- Reproduce with `CAD_FUZZ_SEED=0x8371abdecebb95f9 CAD_FUZZ_EFFORT=8`.
- Case: "two-span patch", a rigid map about `(0.8892767327884497, -0.3111456490169011, 0.529547310454898)` through `(-0.7856759962338078, 0.5412358116783427, 1.931254384335574)` by 3.30562505731778.
- The transverse margin moved from 0.002672789803881874 to 0.0026727898038860737. That is about 4.2e-15 absolute, or 1.6e-12 relative, roughly 7000 ulps, against the 256-ulp budget scaled by reach and the window's `1 / width`.
- The CI failure's own seed is lost: the job log the GitHub tool returns holds only its last 5000 lines.

## Question

Is the margin reading's rounding under a rigid map larger than the in-file derivation claims ("a few hundred ulps of the coordinates' reach, relative, scaled up on a window by the cut's `1 / width`")? Or is the reading itself less rigid-invariant than the derivation assumes? Either way, re-derive the bound or the reading, and pin this seed's case as a fixed row.

