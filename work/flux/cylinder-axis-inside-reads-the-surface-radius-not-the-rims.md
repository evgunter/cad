---
id: cylinder-axis-inside-reads-the-surface-radius-not-the-rims
kind: issue
title: The cylinder decides props_chart_axis_inside at its surface radius, escalating a legal face whose rims the axis class admits
status: open
opened: 2026-10-19
priority: P3
cost: E
---

Found by the FLUX cone-closure unit (`flux/cone-closure-and-per-term-whole`,
PR 5333), and corrected in its review: the unit first filed this as "the
decide is subsumed by the parse's axis class; delete it", which holds
for the torus and not for the cylinder.

`closure::require_axis_inside` (`crates/geom-brep/src/props/closure.rs`)
decides a closed form's circle radius definite at `Kε` before
`curved::about_axis` carries a rim's stored turn to the axis. `Kε` is
also the least radius `props_circle_axis_class` admits a rim at, so a
call that reads the rim's own radius never refuses an admitted face:

- **the torus**, which decides it at each parallel's own radius
  (`torus_chart_loop`, the `TorusStep::Rim` arm, `curved.rs:4490`):
  dropping that call reds no row in geom-brep's closure, cone,
  iso-parse, s58 or rim-dimension suites at ε = 1e-9. That call is the
  subsumed one.
- **the cylinder** decides it once, at the **surface** radius
  (`cylinder_windings`, `crates/geom-brep/src/props/curved.rs:1342`),
  which a rim's may exceed by up to `ε`. A cylinder narrower than `Kε`
  whose rims the axis class admits then escalates a legal face. Probe
  (`review_5333_cylinder_axis_inside_reads_the_surface_radius`, on
  `review/5333-01A5XHA3`, `crates/geom-brep/tests/review_5333_probes.rs`):
  the band between two whole rims of radius `r_c` on the cylinder of
  radius `R`:

  | R | r_c | f64 | interval |
  |---|---|---|---|
  | 9.6ε | 10.4ε | escalates, `props_chart_axis_inside` | escalates |
  | 9.95ε | 10.5ε | escalates | escalates |
  | 10.5ε | 10.5ε | measures | measures |
  | 10.2ε | 10.9ε | measures | measures |

  at ε = 1e-6, 1e-9 and 1e-12 alike. It predates PR 5333 (its base reads
  the same).

The repair: the cylinder reads each rim's own radius (or, once it does,
drops the call if the axis class then subsumes it, as the torus's is),
with a row on the first two probe lines that measures their closed form
`2π·r_c·h`. Not fixed in PR 5333.
