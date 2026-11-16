---
id: a-lily-wall-conic-quadric-sub-clearance-of-3-microns-is-in-band-at-1e-6
kind: issue
title: k-lint (dev-probe): bool_conic_quadric_sub_clear reads 3.06e-6 and 3.09e-6 on demo/lily_walls, in the band at 1e-6 and under the floor at 1e-9
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

`bool_conic_quadric_sub_clear` is the clear row of the conic × quadric
ladder's verifying subdivision (`geom_brep::rows::CONIC_QUADRIC_LADDER`,
read by `crates/topo/src/boolean/conic_quadric/mod.rs` through
`LADDER_ROWS`, ~206). On `demo/lily_walls` at 1e-6 it lands in the band
4 times, at 3.0555e-6 and 3.0912e-6. At 1e-9 the 3.0555e-6 reading
recurs twice, definite and under the 4e-5 floor. It is bit-identical
at both rows, so it does not move with ε: it is geometry, and it is the
smallest ε-independent definite margin in the whole 1e-9 file. The
row's smallest definite margin at 1e-6 is 5.6e-5. The open row `work/germ/coaxial-unions-with-a-torus-or-a-sphere-and-a-tangent-cone-refuse.md`
names the same row name for another reason.

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

