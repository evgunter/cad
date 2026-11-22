---
id: main-red-a-ball-r-0-1-across-the-cylinder-rim-refuses-b-minus-a-at-edge-in-face
kind: issue
title: Main red: cylinder_sphere_frame's ball r 0.1 across the rim refuses B ∖ A at EdgeInFace (CarrierUnread)
status: closed
opened: 2026-11-20
priority: P0
cost: M
closed: 2026-11-22
pr: 7392
branch: green/rim-ball
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

## 2026-11-22: the verdict, and the pin (U2)

**The refusal is correct and conservative under the slit's contract,
but it comes from a certificate too weak for this edge.** The row's
expectation stays right and the band stays P0. A print at the slit's
unread arm on the row's own pose (CI run 44997911372, main `1d14b92ca6`,
throwaway branch) shows that face 4 is the drum's cut plane and edge 16
is the sphere × cylinder section edge, a clamped spline view with both
ends on the rim (`Names { ends: true }`).
- Its 33 sampled heights over the plane are `3.5e-13` and `5.1e-13` at
  the two ends and `0.010 … 0.126` m between them.
- All its control points but the two end ones are beyond the band on
  that side (`1.35e-4 … 0.126`).

`holding` takes a spline carrier that no certificate clears as lying in
the face, and `CarrierUnread` says only that the reading is missing. The
certificate, `clears_plane`, needs every control point decided beyond
the band (`RIDER_HULL_OFF`). A clamped carrier's end control points are
its endpoints, so it can never clear a carrier that ends on the plane.
The body still exists at the oracle's volume. Six neighbouring poses
(±0.01 in `c.x` and `c.z`, ±0.001 in r) refuse the same way at all three
ε, so this is not a knife edge.

The fix is not in PR 7052's hunk (the reading is `slit.rs`'s), so the
row is pinned at every ε to its one stop
(`test_utils::pinned_red::row`: the panic must be the whole one-element
list, B ∖ A at `EdgeInFace { edge: EdgeKey(16v1), face: FaceKey(4v5),
gap: CarrierUnread }`, so a second off run, another edge or face, or any
other stop is red). Fix row, P0:
`work/sectarms/a-rim-ball-refuses-b-minus-a-at-the-slits-unread-carrier-since-the-periodic-loop.md`.
SECTARMS was told on its log.

**The other eight pose rows and the sibling list.** No other pose row
stops at the slit. On the 11-21 nightly the other eight pose rows pass at
every ε, except `wider_than_the_wall` at 1e-6. That row and
`every_segment_joins_neighbouring_sites_along_its_loop` are red at 1e-6
only, at `GermSectionUnbuilt { .. TubeStraddles }` on the sliver pose
r 0.699. They went red in the same window, and the cause is the lost pose
PR 7052 filed itself
(`work/sectarms/a-sliver-loop-that-carried-as-a-clamped-fit-refuses-its-tube-as-a-periodic-one-at-a-coarse-band.md`),
not this one. Another lane holds them. Of the other seven rows on the EMBED sibling list
(`work/embed/the-sliver-concession-reads-inside-the-band-against-the-finished-body-clause.md`,
"1. Baseline"), none stops at `CarrierUnread`. In the 11-21 nightly
(44870651337), `germ_tilted_rod::other_spins_and_steeper_tilts_refuse_at_their_germs`
is red at every ε, but on its own stop, `PeriodicView`. That is also
7052's, and it has its own row
(`work/sectarms/main-red-the-tilted-rods-other-spins-row-asks-a-periodic-view-past-its-window.md`).
The seeded fuzzer `square_search::the_square_wall_arm_agrees_with_the_traced_section`
is red at the default ε and 1e-12 on strip mismatches, which is not this
cause. The other five pass at every ε. Related but not the same case:
`work/splice/an-unread-rider-related-only-by-its-own-vertices-refuses.md`
(an exemption by lineage, for a face and edge of one operand). Here the
edge is a section edge whose ends lie on the other operand's rim, so that
exemption would not cover it.
