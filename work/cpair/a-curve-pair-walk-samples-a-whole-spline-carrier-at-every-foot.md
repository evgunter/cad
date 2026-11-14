---
id: a-curve-pair-walk-samples-a-whole-spline-carrier-at-every-foot
kind: issue
title: The curve-pair walk samples a spline arc's whole carrier at every foot it polishes, so a meeting on a 6365-span carrier costs its leaves times its spans
status: open
opened: 2026-11-11
priority: P2
cost: M
refs: [cylinder-sphere-frame-rows-take-twenty-minutes-a-boolean-at-1e-12]
---


Found while measuring
`work/sectarms/cylinder-sphere-frame-rows-take-twenty-minutes-a-boolean-at-1e-12.md`.

## What is seen

`Curve3::foot` on a spline (`crates/geom/src/curves/on_carrier.rs`,
`foot`) seeds its polish from samples of **every knot span of the
carrier** (`SPLINE_SAMPLES` per span), then picks starts from them
(`spline_starts`: each local minimum, then `LEAST_SAMPLES` passes over
all the samples, each comparison a `k_stats::choose`). The cost of one
foot is the carrier's span count, whatever span of it the caller reads.

`curve_pair::walk::meet` (`crates/geom-brep/src/curve_pair/walk.rs`)
asks for a foot at each span end (`polished_feet`), at each witness, and
twice at every settled leaf of its bisection (`rb.curve().foot(…)`,
`ra.curve().foot(…)`). topo's chord join reads its path parity through
it (`ring_path::spline_crossings` → `curve_pair::meet`), where the spline
side is an arc of the section's carried loop: at ε 1e-12 that carrier
has 6365 spans, and the arc is a part of it.

## Measured

`cylinder_sphere_frame::a_ball_across_a_cylinder_wall_in_two_loops`,
its first run (`A ∪ B`) only, ε 1e-12, debug, after
`sectarms/cylsph-1e-12-cost` (PR body): 16 meetings asked 2435 spline
feet, which sampled 7.95 million span windows between them. `gdb`
samples (10 s apart) put 14 of 40 in `spline_crossings` →
`curve_pair::meet`, most of them in `spline_starts`' `smaller` and in
the samples' `NurbsCurve3::eval`, about a third of the run's 455 s.

## What closing it takes

A foot whose cost follows the span the caller reads, not the carrier:
for instance the walk's foot reading the arc's own span (and a margin
past it) rather than the whole carrier, or `spline_starts` choosing its
least samples in one pass. The first changes what a foot returns (the
nearest point on the arc rather than on the carrier), so it needs the
reader's argument that no caller rests on the far branch; the second
changes how many `curve_foot_polish` choices reach the K sink. Either
is this program's call (`on_carrier.rs` is ONECOPY's ground, the callers
are this program's).
