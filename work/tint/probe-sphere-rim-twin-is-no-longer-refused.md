---
id: probe-sphere-rim-twin-is-no-longer-refused
kind: issue
title: geom-brep's probe-gated rim_dim_scale_twins::mm_scale_sphere_rim_level_margin_is_the_direction_chord reds on main: the interior sphere rim is no longer refused
status: open
opened: 2026-10-11
priority: P1
---


INTENT's nurbs u2 lane found this; the INTENT orchestrator reproduced it on main at 6e3985cb76:

```
cargo nextest run --workspace --all-features \
  -E 'test(mm_scale_sphere_rim_level_margin_is_the_direction_chord)'
```

It panics at `crates/geom-brep/tests/rim_dim_scale_twins.rs:388`
with `the interior rim must be refused`. The file is
`#![cfg(feature = "probe")]`, so `-p geom-brep` without the feature runs
0 tests. No per-PR CI row runs it, the same blind spot as
`probe-sweep-prepass-reds-on-assemble-one-local-battery`.

Owed:
- find which change stopped `rim_level_margins` refusing the
  sphere's interior rim at 1e-3 scale;
- decide whether the refusal or the row is what is wrong. The row's
  doc cites the near-polar accepting-direction hazard (retired audit
  note N7), so treat a lost refusal as a defect until shown otherwise;
- check that the probe-suite census (`scripts/gates/probe-suite-census.sh`)
  rosters this suite as executed somewhere.
