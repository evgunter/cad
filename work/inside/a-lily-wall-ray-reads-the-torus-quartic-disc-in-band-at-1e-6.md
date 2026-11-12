---
id: a-lily-wall-ray-reads-the-torus-quartic-disc-in-band-at-1e-6
kind: issue
title: k-lint (dev-probe): bool_ray_torus_disc lands in the band 14 times on the lily at 1e-6, at |m| 6.0e-6 and 7.8e-6
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

`bool_ray_torus_disc` is the disc row of `topo`'s ray × torus quartic
(`geom_brep::rows::RAY_TORUS`, read by
`crates/topo/src/boolean/solid_contain.rs` through `RAY_TORUS_ROWS`,
~2880). At 1e-6 it lands in the band 14 times: 8 on `demo/lily_walls`
and 4 on `demo/lily` at −5.966e-6, and 2 on `demo/lily_walls` at
−7.774e-6. At 1e-9 the same scenes record it as definite and under the
4e-5 floor: 10 rows on `demo/lily_walls` from 3.3e-8 to 7.8e-6, and 4
on `demo/lily` at 5.9e-7. So it moves with ε, though not in proportion
to it, which suggests the rays themselves differ from one ε row to the
next. Whether a quartic's discriminant is metred as a length is the
first question to ask.

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

