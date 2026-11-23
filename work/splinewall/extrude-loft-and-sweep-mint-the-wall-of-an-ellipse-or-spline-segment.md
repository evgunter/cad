---
id: extrude-loft-and-sweep-mint-the-wall-of-an-ellipse-or-spline-segment
kind: unit
title: Extrude, loft and sweep mint the wall of an ellipse or spline segment: the ruled NURBS wall, or an elliptic-cylinder kind for the extruded ellipse
status: parked
opened: 2026-11-22
priority: P1
cost: H
design: true
parent: an-ovoid-cannot-be-authored
blocked_on: [measure-what-an-exact-nurbs-wall-of-revolution-does-today, a-profile-segment-is-an-ellipse-arc]
---



Part 3's other sweeps (`an-ovoid-cannot-be-authored`, Ev PR 7289):
extrude, loft and sweep of an ellipse or spline segment mint its exact
wall.

## The open question (why `design: true`)

Ev left one choice to the build (PR 7289: "no real opinion about the
lean in 3"): whether an **extruded ellipse arc** is an elliptic-cylinder
surface kind or part 3's ruled NURBS wall. Both designers lean kind,
unsure. Note what main's `[surfaces]` now says: "A swept locus earns a
kind where it is a quadric or one of the torus's sheets, so that a line
meets it in a closed form and a plane cuts it in a curve kind that
exists", and the elliptic cylinder is a quadric whose plane sections are
ellipses and lines. Read as ratified, that sentence makes it a kind; the
ruled NURBS answer would need `[surfaces]` re-worded. Weigh it before a
lane builds this unit (designers first, per
`memories/orchestration-model.md`; it adds a kind or re-words a
ratified sentence, so the answer goes to Ev).

## What is true when it is done

- Extrude of an ellipse segment mints the chosen wall; extrude of a
  spline segment mints the ruled NURBS wall; loft and sweep sections
  holding an ellipse or spline arc mint their NURBS walls. Each builds
  and is at rest at every lane scalar; until then each refuses typed
  (row 2) naming this unit.
- If the elliptic cylinder is a kind, it lands as OVOID's spheroid did:
  the variant with every arm refusing first, then its readers.

## Rows and ground

- Crosses LATHE and STRUT (`extrude.rs`), CARVEREST (`loft`). Announce
  on their logs.
- Review: **single (FULL)** if NURBS; **dual** if a kind. Difficulty
  **L** (NURBS) or **H** (a kind).
