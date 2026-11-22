---
id: a-fitted-rows-first-projected-certificate-costs-a-quarter-of-a-ball-cylinder-boolean
kind: issue
title: A fitted chord's first projected pcurve certificate (check 4 at the mint) costs about a quarter of a ball × cylinder boolean, and no row owns its cost
status: open
opened: 2026-11-21
priority: P1
cost: H
---


Filed by SECT's orchestrator, 2026-11-21, from the re-profile on
`a-ball-and-cylinder-boolean-costs-seconds-per-op-in-an-optimized-build`
(its 2026-11-21 section, PR 7285): ball ∪ rod at the default ε, opt-level
2, main `afa8184d`. After PR 7193 each kept row's projected certificate is
derived once, at `pcurves::mint_pcurves` → `pcurve_cache::certify_projected`
→ `run_projected_checks`: 8 calls, 0.48–0.53 s, 23 % of the op. Most of it
is check 4 on a fitted row. Banded P1 as a blocker of that P1 row (its
~0.5 s target is not reachable without this or another unowned stage).

What closing it takes: measure check 4's own split (hull terms, the
fitted lane's per-turn reading, `twin_span_hulls`), then make a fitted
row's first certificate cheaper without weakening it — the same kind of
work PR 7173 did for the section's tube. Measure first.
