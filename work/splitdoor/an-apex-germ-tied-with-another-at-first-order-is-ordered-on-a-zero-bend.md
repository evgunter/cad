---
id: an-apex-germ-tied-with-another-at-first-order-is-ordered-on-a-zero-bend
kind: issue
title: an apex germ tied at first order with another germ is ordered on a section bend of zero, unconstructed
status: open
opened: 2026-10-29
priority: P2
cost: M
---


## What

PR 5956 reads a cone apex's section bend as 0 in `insert::section_chords`
(`section_bend`), because a ruling is straight. Two cases follow, and
neither has been constructed or measured:

- Two apex germs that tie along one tangent bend alike (both 0), so they
  read `SectionGermsUnordered { Coincident }`.
- An apex germ tied with a curved face's germ is ordered on the curved
  germ's bend against 0. That is right only where the cone's section
  really is the ruling on both sides of the vertex.

## Construction recipe

Two cones sharing an apex vertex, with their rulings tangent along one
line: for example, a cone and a coaxial cone of a different half-angle,
unioned so they share the apex. Then a plane through the apex holding
the line both cones meet along.

For the curved-germ case: a cone whose apex vertex is also a corner of a
cylinder or sphere face, tangent to one of its rulings there, split by
the plane holding that ruling.

Measure first. Then pin the refusal or the closed form.
