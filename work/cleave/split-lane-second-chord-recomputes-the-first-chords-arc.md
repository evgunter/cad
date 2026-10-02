---
id: split-lane-second-chord-recomputes-the-first-chords-arc
kind: issue
title: The split lane's second chord recomputes its segment's section arc from another run, and from no run at all after a cross-loop join
status: open
opened: 2026-10-02
priority: P1
cost: M
refs: [blind-d-pocket-subtract-refuses-with-join-internal-words, pierce-ring-has-no-join-arm]
---


## What

`crates/topo/src/chord_join.rs`, `ChordJoiner::join`, under
`Chords::Split` (the plane split's lane, `splitting::join`): each of a
segment's two chords asks `chord_spec` for its section arc again, from
the run that chord co-bounds. The first chord reads the run
`[h1 .. h2]` (or, across two loops, the `mekr` target's whole cycle);
the second reads `run2` — the edge between the halves when they are
adjacent, else the first chord's run. The two chords are one segment
(the null edges are zero-length, so the second is the first run back),
so its arc is decided twice, from two windows.

After a cross-loop join the second chord's run is empty: `run_halves`
is only filled when `l1 == l2`, so `run2` is `[]` and a curved face
refuses `SectionArcWindow { NoChartedRun }` whatever the first chord
found.

## Why it matters

That was the boolean's wall pierce-ring door: on the boolean lanes the
`mekr` merging a pierce ring into a wall face's outer loop minted its
first chord, and the second refused `NoChartedRun` on its empty run.
JOIN-3 gave a boolean match its chord curve once (`SegmentCurve`), and
the groove, the spun snowman and the diagonal bars at `c = 0.9` build
since (`work/tang/pierce-ring-has-no-join-arm`, `## Measured (JOIN-3)`).
The split lane keeps the per-chord computation. Unmeasured on a split:
no split fixture is known to reach a ring on a curved face, and the
two windows agree wherever both runs bound a region of the face that
does not wrap its chart.

## The shape of a fix

The split's `join` computes the segment's curve once, as the boolean's
does (`ChordJoiner::segment_curve` with `JoinLane::Split`), and both
chords read it. The one thing to settle: the split mints its aux plane
lazily at the first conic chord, so a curve computed before the
adjacency skip would mint it for a join whose chords are both skipped
(an orphan surface tier 1 refuses).
