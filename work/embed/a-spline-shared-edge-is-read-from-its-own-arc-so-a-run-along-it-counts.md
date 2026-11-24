---
id: a-spline-shared-edge-is-read-from-its-own-arc-so-a-run-along-it-counts
kind: issue
title: A spline shared edge is read from its own arc, so a face's run along it counts as its lever
status: open
opened: 2026-11-24
priority: P1
cost: M
---


## What is read

The embedding question levers a pair's parting at how far the narrower
face runs from what the two share, measured across it. For a line or a
circle the shared locus is the whole carrier, so a face's run along the
edge is no distance from it; an ellipse and a spiric oval are read over
their whole turn. A spline (`Curve3::Nurbs`, `Curve3::Periodic`) is
read over the edge's own span (`geom_brep::extent`'s `Locus::of`,
`crates/geom-brep/src/extent.rs`): the locus is the edge's arc, and a
boundary station past the arc's end, along the curve's continuation,
reads its distance to the arc's end as run. That is the over-read
`work/honeone/the-wedge-reach-counts-a-faces-run-along-its-edge.md`
names for the wedge's reach, retired for lines and circles by
`the-embedding-question-reads-tier-3s-class-and-the-pairs-extent-from-the-verdict`
and left for splines.

**It is permissive**: a longer lever certifies a pair a shorter one
would refuse.

## What is measured

On `embed/verdict-read` before its second merge (PR 7666), counted by
an instrumented build: 19 rows of
`crates/sweep/tests/self_intersection_proves_injectivity.rs` and the
tour's walk read spline loci (924 and 520 readings). Not measured:
whether any of those edges' spans stop short of their carrier's domain
(where the span is the whole domain the arc is the whole curve and
nothing is over-read), and whether any verdict or receipt moves when
the whole carrier is read. A probe that prints each spline locus's
span beside its carrier's domain is written
(`Locus::of`, one `eprintln!` in the spline arm) and was not run.

## What would close it

Read a spline's locus over its carrier's whole domain where that is
bounded and covered, or bar a station past the arc's ends from the
run; then the count above, and the suite and the walk for any verdict
or receipt that moves.
