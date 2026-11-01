---
id: mm-scale-sphere-rim-probe-no-longer-refuses
kind: issue
title: rim_dim_scale_twins' mm-scale sphere rim row is red under --features probe: the interior rim is no longer refused, so the probe sweep's pre-pass stops at geom-brep
status: closed
pr: 4696
opened: 2026-10-11
priority: P1
cost: E
closed: 2026-10-15
---


## What

`geom-brep::rim_dim_scale_twins::mm_scale_sphere_rim_level_margin_is_the_direction_chord`
(`crates/geom-brep/tests/rim_dim_scale_twins.rs`, ~388, a
`feature = "probe"` suite) panics on its first assertion, `the interior
rim must be refused`: `rim_level_margins` over `sphere_interior_rim(1e-3,
0.5)` reports no refusal. Red at default ε, 1e-6, 1e-9 and 1e-12, at
`origin/main` 2026-10-11 (after `bf05e073d`, PR 4573's sphere arm,
merged; not bisected).

Found by INTENT's `intent/probe-battery-places-its-fixture` lane while
running `scripts/k_probe_sweep.sh`'s plain pre-pass past
`assemble_one_local_battery`: the pre-pass runs the roster in order, so
this row is the next stop. `RUN_FLOOR` rosters
`plain:geom-brep:rim_dim_scale_twins`, so nightly `k-lint (dev-probe)`
still lints nothing while it is red (with
`silent-fixed-predicates-probe-reds-under-all-features-on-main`, the
topo row after it).

## Owed

Decide whether the probe's premise (an interior rim on a 1 mm sphere is
refused) still holds: if the kernel now rightly accepts it, re-baseline
the row to what it decides; if not, the refusal regressed. Then re-run
the pre-pass (`run_plain` in `scripts/k_probe_sweep.sh`).

**Also reproduced** by the INTENT orchestrator, on main at 6e3985cb76:
`cargo nextest run --workspace --all-features -E 'test(mm_scale_sphere_rim_level_margin_is_the_direction_chord)'`
fails at `rim_dim_scale_twins.rs:388` ("the interior rim must be refused"). The row's doc cites the near-polar accepting-direction hazard (retired audit note N7), so treat the lost refusal as a defect until shown otherwise.

## Fixed by PR 4696 (FLUX cone-vector-area)

The row read `props_rim_level` off `curved_face`; since the sphere's flux
lane measures by Gauss–Bonnet (PR 4573) it records no such verdict and
measures the L. PR 4696 points the suite's `rim_level_margins` at the
predicate's one remaining home, the shape door
(`geom_brep::require_iso_rectangle`), which it had to do for its cone
rows too; the row's subject — the margin is the direction chord, linear
in scale — is unchanged. (FLUX cone-vector-area lane)

## Closed (2026-10-15)

#4696 merged on 2026-10-11. The row sat at `review` after its fix landed. (CIW orchestrator)
