---
id: cylinder-sphere-ssi-is-a-test-vehicle-beside-the-ruling-sweep
kind: issue
title: cylinder_sphere_ssi is a public door no production path reads, beside the section value's ruling sweep: retire it or move it under test support
status: open
opened: 2026-11-04
priority: P3
cost: M
refs: [cylinder-sphere-reads-its-pose-on-the-ruling-sweep]
---



Filed by the cylinder × sphere ruling-sweep lane (branch
`sectarms/cylinder-sphere-ruling-sweep`, 2026-11-04).

## What

The section value now counts cylinder × sphere on its ruling sweep and
marches from the count's witnesses (`ssi::march_cylinder_sphere`, crate
private), so C5's ruled-pair rule holds on every production path. The
public door `geom_brep::ssi::cylinder_sphere_ssi` (`ssi.rs`) still
seeds and accounts a caller's slab by the 3-D cell sweeps
(`implicit_pair_r3` → `exhaust::seed_r3`, `exhaust::account_r3`). As of
this branch's merge base nothing outside tests calls it: its callers are
`crates/geom-brep/tests/{m5_pr7_ssi,review_m5_pr7_adversarial,
ssi_limb3_one_arc,sphere_torus_ssi,pair_section,intersect_table}.rs`
and `crates/topo/tests/{fixture/mod,m5_pr7_split_meter}.rs`. The grep
covered `crates/*/src`, `demos`, `tools`, `benches` and the Python
bindings.

It is the generic ℝ³ implicit-pair door's test vehicle. Its slab can
cut the section, which the value's ball never does, so it exercises
`implicit_pair_r3`'s open-branch and accounting paths. Its doc now says
this.

## What the change is

Either retire it, moving its rows onto a door that still reads the 3-D
sweeps (sphere × torus, or torus × torus), or move it under
`test-support`, so a public door no longer reads this pair's
exhaustiveness a second way beside the value (`[one-way.readings]`).
