---
id: tilted-torus-pairs-read-no-ball-separation
kind: issue
title: A torus against a tilted torus, an oblique cylinder or a cone off its axis refuses on reach even where the two carriers' bounding balls are apart
status: open
opened: 2026-10-27
priority: P3
cost: E
---


Found by SECT's lily lane (`sect/lily-parallel-tori-certificate`) in its
sweep of the section certificate's `Intractable` arms, 2026-10-27.

## Measured

`boolean/section_cert.rs` `torus_pair` answers `Section::Intractable` for
a torus against a torus tilted to it (`Pose::Other`), a cylinder neither
parallel nor square to its axis (`square_pose` → `None`), and a cone off
its axis (`Pose::Parallel | Pose::Other`). A pair reaches the certificate
when its faces' boxes overlap, which does not need the carriers to come
near: a torus lies in the ball of radius `R + r` about its centre, so a
partner whose own bound (a torus's ball, a cylinder's distance from the
centre to its axis less `rc`) stands past it, by a decided margin, meets
it nowhere (W0), whatever the tilt.

## What the taker owes

That W0 row ahead of the pose reading on each of the three arms, decided
across nothing (it reads stored carriers, no turned image), and rows on
both sides of it.
