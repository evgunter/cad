---
id: a-sphere-cut-in-circle-across-a-cut-of-its-face-refuses-in-the-voice-of-a-kernel-bug
kind: issue
title: A sphere cut-in's section circle across a cut of its face refuses ClassificationInvariant, the voice of a kernel bug, where no reading parts it
status: open
opened: 2026-11-01
priority: P3
cost: M
refs: [a-fate-cut-end-on-an-edge-of-its-faces-division-lies-in-no-piece, a-fate-cut-placement-limit-refuses-in-the-voice-of-a-kernel-bug, cut-in-refusals-no-probe-reaches]
---


Carried from
`a-fate-cut-end-on-an-edge-of-its-faces-division-lies-in-no-piece`,
whose `## Owed` noted it and whose fix (branch
`cleave/fate-cut-end-on-a-division-edge`) does not reach it: a
different reading, of a sphere face's pieces against a section
circle's crossings. Unmeasured.

## What

`boolean::ops`'s sphere cut-in places the section circle in the piece
of a cut sphere face that holds both its crossings (`sphere_face_region`
read at each crossing). Where no piece holds both, and no piece reads
both on its boundary (the circle across an earlier cut's meridian), or
where one piece holds them while another reads them on its boundary, it
refuses `corrupt("cut-in: the section circle lies in no one piece of
its face")` (`ops.rs`, the cut-in's `holder` / `across_a_cut` match,
~7397): the voice of a kernel bug, for a placement the reading does not
part.

## Owed

Whether any input reaches it (a probe, as
`work/gauge/cut-in-refusals-no-probe-reaches.md` asks of its four
siblings). If one does, read the placement or refuse in a typed voice
that names it; if none can, say why and make it the invariant it
claims to be.
