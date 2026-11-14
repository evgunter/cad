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

## The merge, and three more rows (GREEN, 2026-11-14)

A first-parent bisect of main at 1e-12 names PR 6505
(`onedoor/one-surface-record`, the face-anchored measure, merge
`9e1484043`); its first parent `e996c0049` passes the bead. Three more
rows of nightly 43371638371 refuse with the same signature
(`VolumeUncomputable`, `props_chart_loop_whole`, check `Inventory`,
margin `[0, H]`), each on a cylinder wall by its fixture's construction:

- `sweep` `near_circular_sections::a_turned_near_square_rod_minus_a_box_builds`
  (H 3.65e-12) and
  `near_circular_sections::a_turned_near_square_split_of_a_rod_validates_its_halves`
  (H 3.35e-12). Both refuse in the rows' `moved_body`, the f64 lane, on
  a plain rod turned rigidly, before any boolean. Both pass at
  `e996c0049` and fail at `9e1484043`.
- `sweep` `verbs_germarms2_interval::the_meeting_axes_pose_builds_at_the_certified_scalar`
  (operand B, H 1.11e-12). This one was already red at `e996c0049` on
  GERM's later stop
  (`work/germ/main-red-the-meeting-axes-pose-refuses-its-endpoint-at-1e-12.md`);
  PR 6505 moved its first stop earlier, to operand B's volume. Fixing
  this row exposes GERM's stop, so its pin cites both rows.

**A plain turned rod is among this row's instances**: a rigidly turned
cylinder rod does not finish at ε 1e-12 in the `f64` lane. Its band
depends on whether ε 1e-12 counts as a normal setting; that is the
owner's call.

The pins that cite this row: the bead (`full_turn_wall::both_doors_answer_the_bead_by_its_height_window`), the two near-square rows, and the meeting-axes pose. Each pin goes red the day its stop moves. For the first three, that is this row's done-when. For the meeting-axes pose it is the move to GERM's stop.
