---
id: a-fate-cut-touch-end-on-an-edge-of-its-faces-division-is-unplaced
kind: issue
title: A fate cut's touching end on an edge of its dropped face's division, at no vertex of it, has no placement
status: open
opened: 2026-11-01
priority: P2
cost: M
refs: [a-fate-cut-end-on-an-edge-of-its-faces-division-lies-in-no-piece, a-fate-cut-touch-end-at-a-division-vertex-lies-in-no-piece, a-fate-cut-placement-limit-refuses-in-the-voice-of-a-kernel-bug]
---


Split from `a-fate-cut-end-on-an-edge-of-its-faces-division-lies-in-no-piece`
in PR 6185's fix pass: the same reading at the other caller of
`fate::lies`, a separate unit.

## What

`fate::cut`'s touch placement (`End::Touch`) reads where the kept
vertex's point lies among the dropped face's pieces (`fate::lies`):
inside one piece it mints a lone vertex, at vertices of the division it
ends the chord there (PR 6185). On an edge of the division, at no vertex
of it, it refuses `ClassificationInvariant` "a fate cut's end lies on an
edge of its face's division" (`fate::Lies::OnEdge` at the touch
placement). No witness reaches it:
PR 6185's grids (two and three lumps touching along a segment or an
edge, flush partners, all poses, three ε) reach the pierced corner's
on-edge case only.

## Owed

A witness: a kept vertex resting on a dropped face, no record naming a
partner of it there, at a point inside an edge of the face's division
(another lump's imprint edge through it). Then end the chord on that
edge, split at the point (`semv` on the division's edge, not a lone
vertex, which `ring_vertex_in` cannot mint on an edge), or refuse as a
`fate::Limit`, the vocabulary the sibling row's fix uses, not as
`ClassificationInvariant`.
