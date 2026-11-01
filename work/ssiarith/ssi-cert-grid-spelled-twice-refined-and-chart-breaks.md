---
id: ssi-cert-grid-spelled-twice-refined-and-chart-breaks
kind: issue
title: The SSI_CERT_SPANS grid is spelled twice (refined clears the carrier's knots, chart_breaks both curves'), reconciled only by prose
status: open
opened: 2026-10-09
priority: P4
cost: E
refs: [4438]
---

Filed by NURBS (`nurbs/refine-dir-hairline`, PR 4438). The finding is
the review's style item S6, and it predates that PR.

`ssi::certify` builds the `SSI_CERT_SPANS` uniform grid at two sites:

* `refined`: `domain_grid_points(carrier.knots(), SSI_CERT_SPANS)`, for
  the box chain and the analytic limb's composite. It clears only the
  CARRIER's interior knots, and it is skipped altogether once the
  carrier has `SSI_CERT_SPANS + degree` control points.
* `chart_breaks`: `range_grid_points(lo, hi, SSI_CERT_SPANS, carrier ∪
  pcurve knots)`, for the NURBS limb's tensor composite. It clears
  both curves' knots, and it has no cut-off.

The two agree only through the analytic limb's comment ("the same
structure choice `refined` makes for the box chain … expressed as
breaks instead of a refit"). PR 4438 put both on one clearance rule
(`GRID_CLEARANCE` of the spacing), but they are still two calls
against two mandatory sets with two cut-off policies. A change to one
(the count, the cut-off, which knots it defers to) does not reach the
other.

The fix shape is one function naming the SSI certificate grid, which
takes the knots it must defer to. Each caller passes its own set, so
the difference between the two sites is visible at the call.
