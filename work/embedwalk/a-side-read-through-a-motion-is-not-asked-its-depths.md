---
id: a-side-read-through-a-motion-is-not-asked-its-depths
kind: issue
title: A clearance side read through a motion is not asked the depth gap, so a placed pair and the same pair built in place can read different verdicts, a budget refusal against a Holds at the root
status: dispatched
opened: 2026-11-21
priority: P1
cost: M
branch: embedwalk/placed-depth
---


Found by lane `embed/nested-tori` (2026-11-21), merging main into PR
7119.

## Evidence

`Side::windows_at` reads a side through a rigid motion: the window's
carrier is mapped into the question's frame, and the face is read in
its body's own coordinates (`Window::own`). The depth gap
(`clearance`'s `parted_in_depth`) reads each face's cover off its body
(`topo::face_cover`), so for a placed window the two covers are in two
frames, and a gap between them says nothing: two coaxial walls 0.001
apart in their own frames cross when one is shifted 0.5, and would
still read 0.001 apart.

So `parted_in_depth` answers `false` where either window is placed
(`clearance::depth_rows::walls_read_through_a_motion_are_not_parted_in_depth`),
and a placed pair is decided by its cells.

**That is a difference of verdict, not only of cost.** The same two
faces built in place are parted at their root and hold; read through a
motion they are refined, and a pair the cells cannot part within the
budget refuses: the nested tori 0.025 apart spend 65,536 pairs and
refuse `Budget(Pairs)` where the in-place pair holds in none. Both
answers are sound; one geometry reads two of them by how it was
posed. P1 for that: one case handled two ways.

## What is needed

A cover read through a motion (`Cover` has no map by a rigid motion),
or the motion carried on the window so the depth gap can read both
faces in one frame. Assemblies place instances, so a thin gap between
two placed parts (a pin in a bore) is the case this leaves.

## Measured (lane `embedwalk-placed-depth`, 2026-11-25)

The hollow torus of `sweep`'s
`two_nested_tori_a_thin_gap_apart_hold_at_their_roots` (`R = 2`, tube
`0.45`, wall `0.025`), its outer two faces against its inner two, the
strict bound at 65,536 pairs, both sides read in place and both read
through one skew motion (a turn of 0.9 about `(1, −2, 0.5)`, then a
shift `(7, −3, 4)`):

| | in place | through the motion, before | through the motion, after |
|---|---|---|---|
| `f64` | Holds, 0 splits | `Budget(Pairs)`, 65,532 splits, 14.3 s | Holds, 0 splits |
| `Interval` | Holds, 0 splits | `Budget(Pairs)`, 65,532 splits, 17.2 s | Holds, 0 splits |

Two torus patches (`topo`'s
`clearance::depth_rows::nested_tori_read_through_a_motion_hold_at_their_root_as_built_in_place`,
chart window `(0.1, 3.0) × (−3.0, 3.0)`), at 65,536 pairs: in place
Holds with no split; before, the inner placed beside an outer built
where the motion puts it held after 36,998 splits, and both placed
after 36,706; after, both hold with no split. The row asks at 4,096
pairs, where before each placed pair refused `Budget(Pairs)`.
