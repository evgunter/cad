---
id: the-projected-pcurve-checks-method-depends-on-the-asking-door
kind: issue
title: A projected pcurve row's check 4 is read one way at the site mint (no fitted lane) and another at the mint and the gate, so one row's certificate is computed two ways
status: open
opened: 2026-11-20
priority: P2
cost: E
---


Found by a SECT designer's probe (2026-11-20, ball ∪ rod on PR 7173's
head). `projected::run_projected_checks` takes its per-turn first reading
only where a fitted lane is passed (`first = … && lane.is_some()`); the
site mint reaches it through `certify_walked` → `PcurveCache::certify`
with `None`, while `mint_pcurves` and the gate pass a lane. One row, two
methods depending on who asks: one reading per question is broken. Fix:
one method at every door (or the lane-free door does not certify).
