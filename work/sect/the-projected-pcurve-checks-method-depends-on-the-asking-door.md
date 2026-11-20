---
id: the-projected-pcurve-checks-method-depends-on-the-asking-door
kind: issue
title: A projected pcurve row's check 4 is read one way at the site mint (no fitted lane) and another at the mint and the gate, so one row's certificate is computed two ways
status: closed
opened: 2026-11-20
priority: P2
cost: E
closed: 2026-11-20
---


Found by a SECT designer's probe (2026-11-20, ball ∪ rod on PR 7173's
head). `projected::run_projected_checks` takes its per-turn first reading
only where a fitted lane is passed (`first = … && lane.is_some()`); the
site mint reaches it through `certify_walked` → `PcurveCache::certify`
with `None`, while `mint_pcurves` and the gate pass a lane. One row, two
methods depending on who asks: one reading per question is broken. Fix:
one method at every door (or the lane-free door does not certify).

## Closed (2026-11-20)

By `a-certificate-is-read-by-its-support`. Every site-mint call on a
projected net's row refused (`FittedLaneUnsupported`, the site holding
no fitted door) after checks 1–3; the lane-free door now certifies no
row only the fitted door certifies (`Pcurve::needs_fitted_door`), so
check 4 is read one way, with the door held. The gate and `carry_rows`
read a row's certificate where its support compares the same. Ball ∪
rod: 32 projected checks → 8, one per kept row, at `mint_pcurves`.
