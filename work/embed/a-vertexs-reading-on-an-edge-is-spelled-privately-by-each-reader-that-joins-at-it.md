---
id: a-vertexs-reading-on-an-edge-is-spelled-privately-by-each-reader-that-joins-at-it
kind: issue
title: Where topology says two stored values are one point, each reader that joins there spells the by-key reading for itself: at least seven sites in injective.rs
status: open
opened: 2026-11-22
priority: P1
cost: M
---


Filed by the EMBED orchestrator, 2026-11-22. Owed since the third
by-key ruling (log, 2026-11-21) and given its list by PR 7322's review.

## What

Four times in one week the certificate was found comparing two stored
evaluations of what topology says is one point, and each time the
comparison decided rounding:

- a pole's or apex's collapsed side against the edge that ends there
  (`embed/pole`, PR 7115: `ends_on_collapse`, key AND the band's
  incidence `parallel_at`);
- two collinear edges of a chart line at their shared vertex
  (`embed/collinear-cover`, PR 7322: `joints`, key alone), and one step
  earlier, which rows are one line at all (that PR's review, MAJOR-1:
  two joined rows holding `v = 0.0` and `v = 4.0e-16` read as two
  lines);
- a run's switch from a curve to the next edge against a partner's
  side that ends at the same vertex (`embed/tangent-corner`, PR 7320);
- whether a zero side lies on its edges, decided by a rounded bound
  against 1 (`embed/strip`, PR 7334: `hi <= 1.0`).

The ruling each time: identity is read by key, never by a numeric
compare or a slack. Each lane wrote its own reading. PR 7322's reviewer
counted the private spellings of "vertex v's reading on edge e" in
`crates/topo/src/injective.rs` on its head: `line_ends` (1666),
`corners` (1829), `present_line_corner` (about 4268,
`if e.ends[0] == c.vertex { i0 } else { i1 }`), `through_of` (about
3123), `continues`, and, on their branches, `embed/pole`'s
`ends_on_collapse` and `embed/tangent-corner`'s run end.

## What is not known

Whether one form fits them. The reviewer: "pole needs a band check,
this needs none"; `corners` is a third kind. So this row is first a
reading of the seven once they are on main together: what each asks
(which end of which edge is this vertex; what the row holds there; what
is conceded), and which are one question.

## Beside it, the cause

`boundary_of` lifts each row's end as its own `pcurve.eval(t)`, a point
at f64, so one vertex has as many readings as rows that end at it
(`the-lift-reads-a-rows-ends-as-points-at-f64-so-a-joints-two-rows-read-a-rounding-apart`,
P3, filed by PR 7322; its review notes a row's held value `iso.at` is
the same per-row reading). A lift that gave a vertex one reading on a
face's chart would remove the question at its source; the by-key
readers are what the certificate does meanwhile. Which of the two this
row ends in is its design question.

## When

After PRs 7115, 7322, 7320 and 7334 have landed, so the readings exist
side by side.

## What a by-key reading concedes (PR 7322's reviewer, 2026-11-22)

A by-key join compares nothing, and it still rests on a tolerance: the
range it bridges is whatever lies between a vertex's two stored
readings. On a body at rest that is bounded by two certified checks of
tier 3 (each row's `pcurve_envelope` within ε of its carrier; each
carrier end pinned within ε of its vertex), so the two chart ends of a
joint are within 4ε of each other in space (`pcurves/decision.rs`,
1335-1345, as an inference; no one check decides that distance). In
the chart the bridged range is about 2πε over the lever: near a pole or
apex it can be a large azimuth and still a few ε of boundary.

So each by-key reading is sound to ε in metres, the class of the
module's other concessions, and is one: a face that truly came through
a gap that narrow would go unread by that reader (the distance side of
the question reads pieces of the boundary that close to a vertex). The
orchestrator's three rulings said "identity by key, never a compare or
a slack" and did not say this. What follows:

- each reader lists what it bridges, bounded by which checks, in the
  module docs' list of what the certificate concedes (asked of PR 7322
  in its fix pass; owed by the others);
- each reader says which gate has run before it (the bound is a fact of
  a body at rest);
- the unified form, when it is written, carries that bound in one
  place.

Whether this class wants Ev's word, as the sliver did, is part of this
row's design question: it declares a boundary continuous at a vertex
topology says it is continuous at, where the sliver declared a face
absent from a cell it was in.

## The eighth spelling (`embed/tangent-corner`, PR 7320, 2026-11-23)

Beside the run's own end (the seventh, `RunEnd`, read from the
corner's vertex key), PR 7320 has a second reader: **a partner's zero
side cut at the run's vertex**, `Along::stops` recorded in
`present_chart` and read by `Along::ending`.

- **What it asks.** Does this side's one edge end at the vertex the
  graph leaves its own edge at? By key: a line of the other axis
  whose own edge ends at that vertex clips the rectangle (`corners`).
- **What it bridges.** The strip between the edge's stored end and
  the other line's outer bound: two rows' readings of one vertex's
  coordinate, times the chart's speed along the side. `Along::ending`
  compares no length; a keyed side is cut however far past the vertex
  it reaches (`a_keyed_side_is_cut_at_the_vertex_however_far_past_it_reaches`,
  to `0.1`).
- **Which gate has run before it.** `sides_are_the_faces` (every
  side its edges' own run on to the other line's far bound, every
  corner convex): a stop is recorded only on a rectangle that passed
  it, and on no whole patch
  (`a_chart_side_stops_at_the_corners_it_makes_and_at_no_other_end`).
  The bound is that test's and the body's, not the reader's.
