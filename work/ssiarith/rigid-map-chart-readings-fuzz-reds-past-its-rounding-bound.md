---
id: rigid-map-chart-readings-fuzz-reds-past-its-rounding-bound
kind: issue
title: ssi/enclose: the rigid-map chart-readings fuzzer reds past its 256-ulp rounding bound at a recorded seed
status: closed
branch: nurbs/rigid-map-chart-allowance
closed: 2026-10-14
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


## A second seed (nurbs lane `pair-walk-domain-side-end`, 2026-10-14)

Met at the default effort in a local 1e-12 run. It reproduces on main at `9f97c7b` in a fresh target directory:

- Reproduce with `CAD_FUZZ_SEED=0x1d0a3b646ab0444b CAD_FUZZ_EFFORT=1`.
- Case: "two-span patch" again, a rigid map about `(-0.1568, 0.9182, 0.3246)` through `(0.4462, 0.6121, -0.4581)` by 2.9171.
- The same transverse margin moved, from 0.002672789803881874 to 0.0026727898038855606: about 3.7e-15 absolute.
- So effort 1 meets it too, and the per-PR gate can turn red on it.

## Resolution (NURBS build A, 2026-10-14)

**The fuzzer's assertion was stricter than the statement; no bound is
wrong.** Both items are one defect: every recorded seed fails on the
same reading, the two-span patch's second-window transverse margin
(0.0027 seated).

That margin is `mig(n·d_k) / stretch`, where each `d_k` is no longer
than the stretch. So the coefficients it reads, and their rounding,
sit at the stretch's scale, and a small margin is a cancellation among
them. Its rounding is relative to one (the most a margin can read),
not to its own value. `within_rounding` read every reading relative to
itself, which for a margin of 0.0027 asks for about 370 times less
than its arithmetic carries.

Measured over 4000 random maps on every wall of the row, in units of
`ε·(1 + reach)·scale`:

| reading | relative to | worst move |
|---|---|---|
| speeds and pads | own value | 78 |
| margins | own value | 257, the two-span margin |
| margins | one | 8.1 (9.44 worst measured, at `CAD_FUZZ_EFFORT=64`) |

The allowance was 256 throughout. After review it is 256 for speeds and
pads and 32 for margins: about three times the worst measured margin
move, and about eight times more sensitive than 256.

**Fix** (`crates/geom-brep/src/ssi/enclose.rs`, test module only):
`within_rounding` takes a `Magnitude`: `Own` for speeds and pads (at 256
ulps) and `Unit` for margins (at 32). The test and helper docs say why.

**Pinned:** `the_recorded_rigid_maps_move_a_small_margin_by_its_stretchs_rounding`
holds all four recorded maps as fixed rows:

- `0x8371abdecebb95f9` (effort 8);
- `0x166d62361818555e`;
- `0xdfa19917eaceb89e` (1e-6);
- `0x1d0a3b646ab0444b` (1e-12).

Each row checks that every speed and pad stays within its rounding at
its own value, every margin within its rounding at one, and every floor
decides as seated. Reading the margins at `Own` instead reds it.

**Checked:**

- All four seeds pass the fuzzer.
- 40 fresh seeds at effort 8 (about 10 000 maps × 7 walls) pass.
- A production mutant making the margin frame-dependent by `1e-10`
  relative reds both the fuzzer and the pinned row.
