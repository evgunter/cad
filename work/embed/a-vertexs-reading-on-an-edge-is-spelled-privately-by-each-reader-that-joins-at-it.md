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
