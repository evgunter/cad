---
id: a-cone-stated-image-at-a-fuzz-seed-reads-in-band-at-eps-1e-12
kind: issue
title: A correct cone stated image reads in band at ε 1e-12 at one fuzz seed, so main's kernel tests go red by seed
status: closed
opened: 2026-11-12
closed: 2026-11-12
branch: pcert/cone-image-fine-band
priority: P1
cost: M
refs: [a-correct-stated-image-far-from-the-origin-reads-in-band-at-a-fine-band]
---

Filed by the BAND orchestrator (2026-11-12) from PR 6514's hosted CI, which went red on a commit touching only a dual-review row file.

## What

`crates/geom-brep/src/pcurve_cache/analytic_image_rows.rs` `a_stated_image_on_its_carrier_certifies` (the panic at :561) is a fuzz row. At ε 1e-12, with `CAD_FUZZ_SEED=0x3f4f8ff23555b906 CAD_FUZZ_EFFORT=1`, a correct cone image (a `General` degree-3 NURBS on a constant-u iso, over [0.0968, 0.6776]) fails at Interval with `Escalated { check: StatedImage, sample: 5, cause: Indeterminate { margin: Enclosure [0, 1.03e-12], band: { zero: 1e-12, escalate: 1e-11 }, predicate: "pcurve_map_residual" } }`. The test asserts that a correct image certifies.

Reproduced on main at `63d935ad38`:

    CAD_TOLERANCE_EPS=1e-12 CAD_FUZZ_SEED=0x3f4f8ff23555b906 CAD_FUZZ_EFFORT=1 \
      cargo nextest run -p geom-brep --lib -E 'test(a_stated_image_on_its_carrier_certifies)'

## Why it matters

The seed is drawn per run, so any PR whose diff puts geom-brep in the ε 1e-6/1e-12 step can go red here by chance. Main's kernel job is red at this seed. Nothing false is certified: the read ends in its lever, as in the far-image row this refs. Whether the fix is the row (at ε 1e-12 a correct image may end in its lever, as the far-image row's sibling accepts) or the bound (the map residual's enclosure at Interval on a cone) is PCERT's call.

Evidence: PR 6514, run 43143064970, job 130935003715, head `d3a7afb287`.

## Closed

Closed by `pcert/cone-image-fine-band`. The read's width was the schedule's
station and not the map residual: at `Interval`, the station
`t0 + (t1 − t0)·5/8` was 3 ulps wide. The image and the carrier were each
evaluated at that box, and each de Boor level carried its width, so the two
points were each about 5e-13 wide and their distance was [0, 1.03e-12]. The
stated route now reads its schedule at held stations
(`pcurve_cache.rs` `Stations::Held`, `certify.rs` `held_station`): its
ends as held, every interior station the `f64` schedule entered as a point.
The same read is now 2.0e-13, the cone's evaluation at 13 m.

The seed is pinned as `analytic_image_rows.rs`
`a_cone_image_at_a_steep_sample_certifies`. Its width assertion goes red at
every ε under `PCERT_RADIAL_PAIR_MUTANT=stated-station-computed`.

Sweep of `a_stated_image_on_its_carrier_certifies`:

- At ε 1e-12, 333 of 3000 seeds refused a correct image before the fix and
  0 after.
- At ε 1e-9, 0 of 1000 refused either way.

Filed from the class sweep:

- `a-lanes-schedule-read-at-a-computed-station-widens-by-the-stations-arithmetic`
  (P3);
- `select/the-edge-certificates-check-4-reads-at-a-computed-station` (P3);
- `a-closed-form-image-at-a-fuzz-seed-reads-its-coefficient-bound-in-band-at-eps-1e-12`
  (P1), another door's main red by seed.
