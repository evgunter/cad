---
id: split-lane-second-chord-recomputes-the-first-chords-arc
kind: issue
title: The split lane's second chord recomputes its segment's section arc from another run, and from no run at all after a cross-loop join
status: closed
opened: 2026-10-02
priority: P1
cost: M
refs: [JOIN-3, pierce-ring-has-no-join-arm]
branch: cleave/split-segment-curve
closed: 2026-10-06
pr: 4081
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
chords read it. The split mints its aux plane lazily at the first conic
chord; `segment_curve` already computes the curve only for a chord the
joiner's plans (`first_chord`, `second_chord`) say is minted, so no aux
surface is minted for a join whose chords are both skipped — the split's
skip test is `between_edge_is_section`'s in-plane question rather than
the boolean's locus edge.

## Built (branch cleave/split-segment-curve)

The split's `join` now computes the segment's curve once, through the
same `ChordJoiner::segment_curve` the boolean calls, and both chords
read it (`SegmentCurve::running_from`). The plan is decided once too:
`JoinPlan::of` plans both chords before any surgery, with the adjacency
skip `SegmentEdge` asks — `Locus` (the boolean's edge) or `InPlane`
(the split's section, `between_edge_is_section`). `segment_curve` and
`join` both read that one plan, and so does the boolean's role order
(`segment_chord_sites` is gone). `segment_curve` curves only a chord
the plan mints, so no aux plane is minted for a join whose chords are
both skipped. It answers `None` there, which the split takes as "mint
nothing" (as before) and the boolean refuses (as before). `chord_spec`'s
one non-test caller is `segment_curve`.

The window premise above had already gone by the time this landed
(`5ec92edc58`, the chord takes the pairing's arc): the second chord no
longer read a run, and so no longer refused `NoChartedRun`. What it
still did was decide the arc again, from the other end's departure.
Measured on a split that reaches a ring on a curved face — the drum
pocketed through its wall by a square bar (the mouth is a ring of the
wall band), cut by a tilted plane, so the join `mekr`s the wall's outer
loop to the ring: main builds it, but two of the section's three wall
arcs on the below half are not the above half's arc run back — one off
by an ulp at each end, one a whole period away (`[2.678, 5.820]`
against `[0.464, 3.605]` on the reversed carrier) — and the halves'
total misses the closed form by `2.8e-11`. The branch mints every pair
as the one curve run back, bit for bit, and on this pose the total
meets the closed form to `1e-14`. That is one pose, not a property:
halves whose quadrature refines differently still miss within their
pads
(`work/cleave/split-halves-volumes-sum-to-the-whole-only-within-their-pads`). Pinned by
`split_section_rings::a_split_across_a_ringed_wall_mints_each_segment_on_one_curve`
(red on main). Probed alongside, all building on both: the pocket and a
through-bar, under ∖ and ∪, cut at `y = 0.1`, `y = 0`, `x = 0.1`
(rulings), the tilt and `z = 0.95`.

## Closed (PR 4081, 2026-10-06)

Review tier: single FULL. Verdict APPROVE-WITH-FIXES, no MAJOR; all seven claims held. The fix pass
planned a join's chords once (`JoinPlan`), made the ringed-wall row assert that it reaches the ring,
and corrected the PR body's numbers. The conservation residual is its own row,
`split-halves-volumes-sum-to-the-whole-only-within-their-pads`.
