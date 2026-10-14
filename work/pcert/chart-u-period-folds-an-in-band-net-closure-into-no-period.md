---
id: chart-u-period-folds-an-in-band-net-closure-into-no-period
kind: issue
title: chart_u_period folds an in-band seam closure into 'no period', so a spline wall's wrap joint misses as decided
status: open
priority: P3
cost: M
refs: [an-in-band-near-pole-read-is-dropped-so-a-half-turn-joint-misses-as-decided]
opened: 2026-10-14
---

Found by PR 5071's second review (MINOR-2); filed in that PR. The same
class as its parent: an undecided reading folded into a decided answer.

**The fold.** `topo::pcurves::chart_u_period`
(`crates/topo/src/pcurves.rs`, near `:1844`) decides whether a spline
chart's net closes in `u` by `pcurve_chart_u_closed` on each seam pair
of control points, and takes `Ok(Sign::Zero) => {}`, `_ => return None`.
An in-band seam distance is folded into "no period", the same answer as
a seam decided open.

**The reach.** A spline wall whose seam rows are apart by more than
the band's zero and less than its escalate edge. Its chart is read as
aperiodic, so `decide_joint`'s azimuth rule is `Aperiodic` and a wrap
joint (a seam crossed whole) keeps its gap of a whole `u` range.
`spline_gap_closes` then decides that gap a definite
`PinMiss::Discontinuity`, i.e. `PcurveMintError::LoopDiscontinuity`,
where the honest answer is an escalation: the net may close.

**The fix shape.** `chart_u_period` answers a tri-state (closes, decided
open, in band with its cause). Its callers (`chart_boundary`,
`decide_joint`'s callers, the split and merge walks, about six sites)
surface the in-band case as an escalation.

**Same shape elsewhere, not judged:**
- `editor-core::expr::refuse_non_finite`
  (`crates/editor-core/src/expr.rs:1908`, `expr_non_finite`):
  `Ok(Sign::Zero) => Ok(value), _ => Err(NonFiniteResult)`.
- `editor-core::clearance` (`crates/editor-core/src/clearance.rs:3370`):
  `Ok(Sign::Zero) => .., _ => false` on the bound's margin.

**Sweep owed with this item** (PR 5071 review, R1 S4): the
`.ok()` / `.unwrap_or(..)` drop of an `Err` on a `Result<Sign, _>`, a
shape PR 5071's sweep named as its blind spot and did not walk. It is
common in this codebase; walk it here and dispose of each hit.
