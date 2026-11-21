---
id: a-side-read-through-a-motion-is-not-asked-its-depths
kind: issue
title: A clearance side read through a motion is not asked the depth gap, so two placed instances a thin gap apart are still parted cell by cell
status: open
opened: 2026-11-21
priority: P2
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
(`clearance::depth_rows::walls_read_through_a_motion_are_not_parted_in_depth`).
A placed pair is then decided by its cells, as every pair was before
PR 7119: sound, and as dear as it was.

## What is needed

A cover read through a motion (`Cover` has no map by a rigid motion),
or the motion carried on the window so the depth gap can read both
faces in one frame. Assemblies place instances, so a thin gap between
two placed parts (a pin in a bore) is the case this leaves.
