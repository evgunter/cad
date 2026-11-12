---
id: a-lily-wall-tangent-sector-order2-margin-of-8-microns-escalates-at-1e-6
kind: issue
title: k-lint (dev-probe): tangent_sector_order2 reads 8.33e-6 on demo/lily_walls, in the band at 1e-6, where decide_order2 escalates
status: open
opened: 2026-11-10
priority: P3
cost: M
refs: [k-lint-dev-probe-large-k-gate-fails-on-51730-margins-once-the-sweep-runs]
---


Found by INSTR's large-K lint lane (`instr/klint-large-k`), reading the
uncapped split of
`work/instr/k-lint-dev-probe-large-k-gate-fails-on-51730-margins-once-the-sweep-runs.md`.
Measured with `scripts/k_probe_sweep.sh`'s dump half on main `9fe94da35`
(2026-11-10), linted as the nightly's `large-K lint` step lints it.

## What

`tangent_sector_order2` is the second-order sector reading
(`crates/geom-brep/src/enters.rs`, `decide_order2`, ~642), which
`crates/topo/src/boolean/sectors.rs` reads. On `demo/lily_walls` at 1e-6
it lands in the band 8 times, all at 8.3336e-6. At 1e-9 the same 8 rows
are definite at 8.3336e-6, under the 4e-5 floor. The margin does not
move with ε. `decide_order2` maps an in-band reading to
`LeverEscalation`, so unlike the walks this site refuses. The scene
still builds at 1e-6, so a caller takes another road there, and which
road that is has not been read.

## The family it belongs to

It is one of three small rule-1 families on the lily at 1e-6, 26
samples in all: `bool_ray_torus_disc` (14, `inside`),
`tangent_sector_order2` (8, `splice`) and `bool_conic_quadric_sub_clear`
(4, `germ`). Each margin sits a few microns from zero. At 1e-6 that is
inside the band `(1e-6, 1e-5)`, and at 1e-9 it is definite. This is the
case `docs/K-REPORT.md`'s M7-F1 note describes ("ε = 1e-6 has thin
FEATURE headroom at millimetre scale"), now landing under rule 1.
K-REPORT's trigger protocol (M7 addendum, 2026-08-05, caveat 1) puts two
checks before the landing is read as ε-versus-scale evidence:

1. that the margin's dimension matches the predicate's comparand;
2. what the deciding site does with an in-band reading.

Both checks belong to the owner of the deciding site. Re-deriving
k-lint's thresholds cannot touch rule 1, which reads only the
sample's `outcome`. The lily's scenes are not recorded at 1e-12 (the stem's
pinned stop), so that row says nothing about these sites.

## Owed

Check the dimension and the disposition. If both are sound, the
in-band reading is true K evidence for the lily at 1e-6: say so on the
INSTR row named above. Otherwise fix the site. Reproduce:
`CAD_TOLERANCE_EPS=1e-6 cargo run --manifest-path demos/tour/Cargo.toml --features probe -- k-probe /tmp/k.csv`,
then grep the CSV for the row name and `indeterminate`.

