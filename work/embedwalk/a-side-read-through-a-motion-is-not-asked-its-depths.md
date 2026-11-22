---
id: a-side-read-through-a-motion-is-not-asked-its-depths
kind: issue
title: A clearance side read through a motion is not asked the depth gap, so a placed pair and the same pair built in place can read different verdicts, a budget refusal against a Holds at the root
status: open
opened: 2026-11-21
priority: P1
cost: M
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
