---
id: a-full-turn-bead-reads-its-volume-uncomputable-at-eps-1e-12
kind: issue
title: The full-turn bead fixture's revolve returns a body whose volume reads Indeterminate on props_chart_loop_whole at eps 1e-12, so full_turn_wall's height-window row is red on main
status: open
opened: 2026-11-12
priority: P1
cost: M
parent: cylinder-and-torus-edge-misses-are-read-back-about-the-axis
---


Found by BAND while bringing PR 6520 up to date with main. CI on its head
`2fe187ca58` (job `test (kernel)`) went red here, and the test fails
identically on origin/main `9ee1c046ef` and `6c70512339`, so it is not
6520's.

`CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p sweep --test all -E
'test(both_doors_answer_the_bead_by_its_height_window)'` panics at
`crates/sweep/tests/common/bead.rs:39` (the fixture's
`revolve(..).unwrap()`, test at `crates/sweep/tests/full_turn_wall.rs:93`):

```
NotFinished { findings: [VolumeUncomputable { solid: SolidKey(1v1),
  source: Face { face: FaceKey(3v1), source: Escalated { cause:
  Indeterminate { margin: Enclosure { lo: 0.0, hi: 6.84e-12 }, band:
  Band { zero: 1e-12, escalate: 1e-11 },
  predicate: Some("props_chart_loop_whole") }, check: Inventory } } }] }
```

The bead's face area encloses `[0, 6.8e-12]` at a 1e-12 band: the
measurement does not separate the area from zero at this tolerance. This is
either a fixture too thin for 1e-12 (scale it or skip the eps) or the area
enclosure being looser than it needs to be; deciding which is this row.

## Claimed by BOXFINISH (2026-11-14): measured, and it is the edge-miss row

Moved here from LATHE by `git mv`, id unchanged; carried by
`cylinder-and-torus-edge-misses-are-read-back-about-the-axis`.

Measured on main `b363f5fa`: still red. The refusing body is the
fixture's fourth (tilted, scale 100: sphere R 100, bore r 50, origin
(30, −110, 200)), and FaceKey(3v1) is the bore's CYLINDER, not the
sphere zone. The refusal is raised at `closed_loop_turns`, fed by
`cylinder_windings` (`crates/geom-brep/src/props/curved.rs`), at
`Interval`, in the role walk's re-derivation. It is neither a fixture
too thin nor an area: it is the whole-turn residual of the bore's loop
(rim, seam ruling, rim, seam ruling).

- `Interval` lane, lever 50 on every edge: per-edge miss ±5.6e-14,
  ±4.4e-14, ±5.2e-14, ±4.4e-14 rad; Σ.hi 9.84e-12 m; less the 3ε slack,
  `[0, 6.84e-12]`.
- `f64` lane, same loop: Σ = 3.1e-15 m, `Zero`.

A local mirror of the cone's constructors (rulings exact, rims carried),
unreviewed and not pushed, made this row pass at 1e-12 with Σ.hi
3.6e-13 m. That sizes the fix; it is not the fix.
