---
id: at-rest-trim-containment-checks-against-the-stored-hull
kind: issue
title: tier 3's trim containment (check 5) measures each stored row against the hull of the stored rows, so it cannot fail at rest
status: closed
opened: 2026-10-02
closed: 2026-10-03
---

Found by the PCERT delta review of PR 3759 (item 6, NOTE).

`validate_pcurves` (`crates/topo/src/pcurves.rs`) re-certifies every
stored row against the window the face's stored rows hull out to
(`stored_rows`' `window`, folded by `hull_of`), and certification's
check 5 (`PcurveCertifyError::TrimEscape`) asks whether the row's
`chart_box` stays inside that window. Each row's own box is one of the
boxes the hull folds, so the check cannot fail at rest. The mint is
vacuous the same way, deliberately (`certify_walked`'s docs). Before
PR 3759 a complete face was already measured against its stored hull
(log, review of PR 3610), so 3759 extends the vacuity to half-minted
faces rather than creating it. Still, at rest no row is held to a trim
region it did not help make.

What a check that can fail would need is a window that does not come
from the rows themselves. Two candidates:

- **The chart's own parameter domain.** That is the polar range
  `[0, π]` on a sphere, a NURBS chart's knot domain, the apex-free side
  of a cone, and one period of azimuth span on a periodic chart. This
  is independent of the rows and never refuses a legitimate face. It
  catches a row stated off the chart's domain, which today only the
  per-chart lanes refuse.
- **A trim region built from the loops' vertices.** This is rejected at
  the mint (`certify_walked`): a boundary arc may bulge past its
  endpoints, so the region would have to be the vertex box widened by
  each edge's own certified extent. That brings back the rows'
  contribution.

The first is the cheaper and the honest one. Until either lands, check
5 at rest is documented as vacuous where it runs (`validate_pcurves`'
docs).

## Closed

Closed by retirement, in the PR that implements
`pcurve-loop-decisions-state-a-3d-identity-plus-a-branch-margin`:
check 5 (trim containment against a caller's `ChartWindow`) is gone
from the row certificate, with `PcurveCheck::TrimContainment` and
`PcurveCertifyError::TrimEscape`, and every window lookup with it
(`certify_walked`, `split_cache`, `carry_rows`, tier 3's stored hull).
Ev ratified C4's re-wording on [ev] PR 3919: a row certifies against
its carrier and chart alone, and the face's loops are the loop walk's.

The chart-domain candidate above stays the check to add **if a producer
ever needs it**: the chart's own parameter domain (a sphere's polar
range, a knot box, the apex-free side of a cone) is independent of the
rows and can fail. Both designers leaned against building it now: the
derivation already refuses an off-domain image, fidelity pins a stored
row to the derivation, and its content belongs to each lane's check 1,
not to a window the rows supply.
