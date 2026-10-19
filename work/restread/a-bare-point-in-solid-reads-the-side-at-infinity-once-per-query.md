---
id: a-bare-point-in-solid-reads-the-side-at-infinity-once-per-query
kind: issue
title: A bare point_in_solid(body, q) reads the side at infinity once per query: no body-level cache
status: open
opened: 2026-10-18
priority: P3
cost: M
---


## What

The side at infinity of a selection whose rays all miss is read off its
volume's sign. Where the closed form has no value for a face, that reading
goes through the quadrature (`boolean::solid_contain::missed_side`, step 3).
The reading is cached per selection (`SolidFaces`' `VolumeReads`). The
census arm, the boolean's witness ladder (`operand_faces`) and any caller
holding `SolidFaces::of_body` pay for it once. A bare
`point_in_solid(body, q)` builds a fresh selection on every call and pays
on every query that falls through to step 3.

Measured (PR 5311, debug build, best of 3): 48 queries on the spiric box ∩
the one-face torus, of which 24 have every ray miss:

- 641 ms through the bare door;
- 83 ms on main, which refused those 24;
- 30 ms over a held `of_body`.

No kernel caller needs more today: every in-crate loop holds a
selection. External callers of the bare door, such as test helpers and
API users probing many points, pay the per-query cost.

## The fork

Choose one:

- a body-level cache of the at-infinity reading, which needs
  invalidation on every mutation door of `Body` (no interior cache
  exists there today);
- documenting the held selection as the door for many points, which
  `point_in_solid`'s doc now does, and leaving it at that.

The first is a design change to `Body` and goes to Ev as a design fork.
