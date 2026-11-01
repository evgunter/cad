---
id: a-fate-cut-touch-end-at-a-division-vertex-lies-in-no-piece
kind: issue
title: A fate cut's touching end at a vertex of the dropped face's division lies inside no one piece of its face
status: dispatched
opened: 2026-10-27
priority: P1
cost: M
refs: [a-fate-cut-end-between-two-touching-lumps-lies-in-no-piece]
branch: cleave/fate-touch-end-at-a-division-vertex
---

Disclosed by PR 5812 (`cleave/flush-partners-at-a-slit-build`), the
blind spot of its sweep.

## What

PR 5812 places a fate corner's pierced end that lies at a vertex of
the kept face's division by its direction (`boolean/fate.rs`
`piece_holding`, read through `lies`). `cut`'s touch placement, an
edge's end where the kept vertex touches the dropped face with no
surgery (`End::Touch`, `fate.rs:1528`), still reads `fragment_at`
(`:1560`), which needs the point strictly inside one piece: at a vertex
of the dropped face's division it refuses `ClassificationInvariant` "a
fate cut's end lies inside no one piece of its face", a claimed kernel
defect, and it would mint a lone vertex where the face already holds
one.

No witness reaches it: the two-lump family of PR 5812 places its
touches inside one piece. A fate edge whose end is the lumps' shared
point, with the partner's face divided there by the other lump's
imprint, would.

## Owed

A witness; then place the end at the division's vertex (the chord
ends there, so no lone vertex is minted), or refuse typed.

