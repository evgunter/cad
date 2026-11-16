---
id: an-escalation-returned-by-question-mark-ahead-of-a-later-answer
kind: issue
title: A reading's escalation returned by ? ahead of a later reading that could still answer: pieced_arc::contact's ends, CarrierLoop::place_off
status: closed
opened: 2026-11-06
priority: P2
cost: M
pr: 6574
closed: 2026-11-06
---

Found by `kept-refusal-loops-the-ladder-row-never-listed`'s second pass
(`onecopy/refusal-standing`). The class: a reading's first escalation
returned by `?`, so a later reading that would answer definitely is
never read (`geom_brep::ladder`'s module docs: "a ladder that returned on
its first undecided reading would escalate where a reading it had not
yet read already answers").

## Done (PR for `onecopy/refusal-standing`, tranche 4)

- `splitting::pieced_arc::PiecedArc::contact` climbs its two ends as one
  ladder (`Any`: either end at the point is `End`).
- `splitting::pieced_arc::PiecedArc::contact`'s ends are the walk's
  premise (PR 6553's review): an end in band blocks `On`, which says the
  point is clear of both ends. So only the other end answers past it.
- `splitting::containment::CarrierLoop::place_off` **keeps its early
  return, by its consumers** (PR 6553's review). The answering edge's key
  is the face door's `FaceContainment::OnEdge`
  (`boolean::contain::boundary_pre_pass`, the `OnEdge::On` arm). An edge
  read in band may be the true holder, so no later edge answers past it.
  The site says so.
- **The gate:** `scripts/gates/escalation-question-marks.sh` registers
  every production `decide*(…)?` inside a loop body, per file, with each
  file's disposition in `escalation-question-marks.counts`. Its blind
  spots are stated in its header. The first is a `?` on a callee that
  decides inside it, which is how `place_off` hid.

## What remains: `boolean::contain::boundary_pre_pass`'s two caller `?`s

**Read these first under `place_off`'s ruling.** The face door's answer
names the cell that holds the point (`OnVertex(v)`, `OnEdge(e)`). A cell
read in band may then be the true holder, and both `?`s may be right as
they stand. A port is owed only where a later cell answers something
the earlier cell in band could not also be.

`boundary_pre_pass` (the face containment door's boundary pass) is the
same class one level up, in two places:

1. **A vertex's escalation**, `one_vertex_at(…)?` in the vertex pass,
   returns ahead of every edge. A point within the band of a vertex can
   lie definitely on an adjacent edge, which would answer
   `FaceContainment::OnEdge`.
2. **Each loop's `place_off(…)?`** returns ahead of every later loop. An
   escalation on the outer loop refuses where a hole's boundary would
   place the point.

**The constraint a port must keep: the vertex-then-edge order.** The
`OnEdge::End` arm's agreement argument rests on it. The vertex pass
has placed the point definitely clear of both of the edge's vertices
before an edge's carrier end is read in band of the point, so the two
passes disagree only by as much as a carrier's end sits off its stored
vertex (`bool_contact_arc_end_vertex`, `ArcEndGaps`). A ladder over the
vertices and the loops' edges therefore cannot reorder them. An edge's
`End` reading after a vertex kept in band, rather than decided clear,
needs its own argument before it can answer.

So the port is one ladder over the vertices then each loop's edges
(`First`, which cell holds the point), with `PrePass` restructured to
carry the kept readings. It moves outcomes wherever a vertex or an outer
loop is read in band ahead of a definite placement. Measure that and pin
each move.

## Closed (2026-11-06, PR 6574)

`boundary_pre_pass`'s two `?`s are right under `place_off`'s ruling.
Consumers read which cell holds the point (census's `OnEdge(e)` and
`OnVertex(v)` arms, and `reduce`'s `OnEdge(ey)`). So a vertex or loop
read in band may be the true holder, and no later cell answers past it.
The function's doc says so. The vertex-then-edge order, which the
`OnEdge::End` agreement argument rests on, is unchanged.
