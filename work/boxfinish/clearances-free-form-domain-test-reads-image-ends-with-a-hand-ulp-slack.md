---
id: clearances-free-form-domain-test-reads-image-ends-with-a-hand-ulp-slack
kind: issue
title: Clearance's free-form domain test reads its loop's image ends and chains them with a hand ulp slack, a second rectangle reading beside the lift's runs
status: open
opened: 2026-11-15
priority: P3
cost: E
---


Found by the sweep of `boxfinish/rectangle-runs` (check 7's rectangle
certificate moved onto the lift's held runs, `topo::pcurves::Lift::runs`
and `Lift::joint_point`).

**What is there.** `topo::clearance::iso_bounded_domain`
(`crates/topo/src/clearance.rs`, the free-form face's admission to the
subdivision) reads the same shape check 7 now reads off the lift, by a
second route:
- it evaluates every image at both ends of its span (`row.pcurve.eval(t0)`,
  `eval(t1)`) and asks both ends to sit on one side of the knot domain,
  each coordinate compared exactly with the domain's ends (`on_side`);
- it chains the ends into a closed curve where consecutive enclosures
  overlap within `injective::few_ulps` of their magnitude (`meet`), a
  hand rounding slack of the kind `[simple.conditioned]` names, the
  shape check 7's `end_slack` had before this branch deleted it;
- it reads the winding off an interval shoelace over those ends.

**Why it is not a defect today.** Each test is conservative at
`Interval`: a cut's or a re-minted corner's end is an enclosure that
overlaps its neighbour, and the side test reads the held coordinate,
which a minted image holds exactly. A sub-rectangle face refuses
(`NOT_COVERING`), which is this test's design: it admits only the whole
patch.

**What the finding is.** One loop shape, "four runs of iso images on
four lines that meet", now has two readers that state it differently:
the lift's runs (no end read, joints at meets) and this one (ends read,
joined by a slack). The lift's `runs` and `joint_point` answer what
`iso_bounded_domain` asks, with the domain test reduced to comparing
four held values with the knot domain's ends.

**Not measured:** whether any face reaches `iso_bounded_domain` with a
loop whose ends meet only within the slack at `f64` (the case the slack
exists for).
