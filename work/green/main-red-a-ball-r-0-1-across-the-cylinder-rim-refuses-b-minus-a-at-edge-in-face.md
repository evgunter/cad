---
id: main-red-a-ball-r-0-1-across-the-cylinder-rim-refuses-b-minus-a-at-edge-in-face
kind: issue
title: Main red: cylinder_sphere_frame's ball r 0.1 across the rim refuses B ∖ A at EdgeInFace (CarrierUnread)
status: open
opened: 2026-11-20
priority: P0
cost: M
---


Found by SECT's `sect/ball-cylinder-cost` lane (2026-11-20), running the
slow sphere × cylinder rows its diff touches.

## Measured

`cargo nextest run -p sweep --test all -E 'test(=cylinder_sphere_frame::a_ball_across_a_cylinder_wall_r_0_1_across_the_rim)'`
at opt-level 2 (debug assertions on), default ε, on main `cd469f2d`
(the lane's branch with `crates/geom-brep/src/intersect/section_bounds.rs`
put back to main's, so main's kernel): FAIL in 10.9 s, at
`builds_at_its_volume_or_refuses_at_its_door`'s
`assert!(off.is_empty())` (`crates/sweep/tests/cylinder_sphere_frame.rs`):

```
"the row's ball r 0.1 across the rim, B ∖ A: refused at None: EdgeInFace { edge: EdgeKey(16v1), face: FaceKey(4v5), gap: CarrierUnread }",
```

The row wants a body at the oracle's volume (`Ends::Builds`). The
refusal is the slit's `EdgeInFaceGap::CarrierUnread`
(`topo::boolean::slit`): an edge whose carrier is not certified off the
face's surface over its whole length. The same failure on the lane's
branch. Not bisected; the other eight `cylinder_sphere_frame` pose rows
pass on both.
`work/embed/the-sliver-concession-reads-inside-the-band-against-the-finished-body-clause.md`
lists this row among those red in a mutation build "whether they are
red on a clean main was not checked": it is.

## 2026-11-21: the nightlies' junit (U2, `green/rim-ball`)

Read from each scheduled nightly's `full-suite-junit-default` artifact:

| nightly | main | this row |
|---|---|---|
| 43561237293 (11-14) | `ff2b5f5d54` | pass, 24.4 s |
| 43680833019 (11-15) | `dbb21b4a12` | pass, 24.8 s |
| 44023697569 (11-17) | `cdee746161` | pass, 13.9 s |
| 44243918484, 44464826302 (11-18, 11-19) | | no full-suite leg ran (the build-once job failed) |
| 44680731754 (11-20) | `8039ba062e` | FAIL, the stop above, 19.5 s |
| 44870651337 (11-21) | `f5ef8f2ba0` | FAIL, the same stop, at every ε |

So it went red between `cdee746161` and `8039ba062e`: 172 first-parent
commits, 49 merges touching some `crates/*/src/`. The test file itself
last changed in PR 6864 (11-12), so a kernel merge moved it.

## 2026-11-22: bisected to PR 7052 (U2)

PR 7052 (`sectarms/periodic-loop-carrier`, merge `6c2430e888`, a closed
fitted section loop carried as a periodic cubic) turns the row red. Each
side was run on hosted CI by a throwaway branch (the commit plus a
comment line naming the row, so the gate's slow-set step selects it),
default ε, the gate's own opt-level:

- `fa4e9ba921` (7052's first parent): PASS, 13.9 s (run 44997693000);
- `6c2430e888` (7052's merge): FAIL, 14.0 s, at the stop above,
  `EdgeInFace { edge: EdgeKey(16v1), face: FaceKey(4v5), gap: CarrierUnread }`
  (run 44997688528).

The two 1e-6 siblings red from the same window
(`a_ball_across_a_cylinder_wall_wider_than_the_wall`'s `TubeStraddles`,
`every_segment_joins_neighbouring_sites_along_its_loop`) are the lost
pose PR 7052's body names and files
(`work/sectarms/a-sliver-loop-that-carried-as-a-clamped-fit-refuses-its-tube-as-a-periodic-one-at-a-coarse-band.md`);
another lane holds them.
