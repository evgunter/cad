---
id: the-integral-meters-piece-refinement-is-not-monotone
kind: issue
title: Refining the integral speed meter's pieces is not monotone: about 5% of nets read lower at 32 pieces than at 16
status: deferred
opened: 2026-10-12
priority: P4
cost: M
---

## What

`NurbsCurve3::piece_assembly` (`crates/geom/src/curves/nurbs.rs`) bounds
each piece by projecting its Bernstein coefficients on that piece's own
direction (their sum). Halving a piece gives each half its own
direction, and a half's coefficients need not dominate the parent's in
its new direction, so a finer schedule can read lower. PR 4824's FULL
review measured it moving `INTEGRAL_METER_SPLITS` from 16 to 32: about
5% of degree 2–10 random integral nets read lower at 32, the worst by
24% (its probe case 2349, degree 10: 1.3019 → 0.9953;
`piece_coverage_meter.rs` pins it). The swaying family, which the count
was raised for, gains throughout (degree 3, `a = 4`: 0.128 → 0.552 of
the ceiling).

## Shape

Make refinement monotone: a sub-piece also tries its parent's direction
(one more dot per coefficient), or the meter takes the max of the 16-
and 32-piece readings. Cost reference (`Interval` loft, 40 sections
degree 3, median of 20): 11.1 ms at 16 pieces, 16.2 ms at 32; the max of
both is about their sum of piece costs, near 21 ms by that table.

## Deferred (2026-10-12, the NURBS orchestrator's ruling)

Measured on `nurbs/monotone-piece-refinement` (main `966bf8c220`): on
6000 random integral nets of degree 2–10 (free, near-straight and
swaying; seed `0x4824` on `test_utils::fuzz::Rng`) the 32-piece reading
falls below the 16-piece one on 362 (6.0%), at worst by 44%. No
decision moves: on those nets plus 3000 more at degree 11–13 (whose
schedule is 16 pieces either way), **0** have a positive 16-piece
reading under a non-positive meter, and the least `span·meter` among
the losers is `0.07`, far above any tolerance band the consumers
(`Margin::metered` in `certify`, `topo::split`, `pcurve_cache`) decide
against.

Both fixes were measured. **(a)** — each piece also tries its parent's
direction, taking the larger — is monotone by construction (a child's
Bernstein coefficients are convex combinations of its parent's, so in
the parent's direction it reads at least the parent's bound): 0 of
6000 nets below the 16-piece reading, never below (b), up to +77% over
plain 32. **(b)** — the max of both schedules — is monotone too. Cost on
PR 4803's loft harness (`Interval`, swaying ±0.25, min of 20; 40
sections degree 3 / 24 sections degree 5): plain 32 12.6 / 12.0 ms,
(a) 19.5 / 18.5 ms, (b) 17.2 / 16.9 ms.

The ruling: reach that moves no decision is not worth +55%, so the
behaviour stays, disclosed, and **(a) is the recorded fix** if a
consumer ever needs it. `piece_coverage_meter.rs`'s
`a_finer_schedule_can_read_lower_on_a_degree_ten_net` keeps recording
case 2349.
