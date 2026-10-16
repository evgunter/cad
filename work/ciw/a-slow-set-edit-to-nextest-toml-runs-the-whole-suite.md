---
id: a-slow-set-edit-to-nextest-toml-runs-the-whole-suite
kind: issue
title: an edit to .config/nextest.toml that only adds or drops slow-set names forces TIER=all, about a quarter of all PRs' full-suite runs
status: open
opened: 2026-10-16
priority: P3
cost: M
---


## Measured (2026-10-16, CIW orchestrator)

I replayed 257 of the 263 first-parent merges to main from 2026-10-09 to 2026-10-16 through main's `scripts/ci-filter.py`; 6 were cut off by a time limit.
- 108 were docs, 82 closure, 3 aux, and 64 all.
- Of the 64 all-tier merges, **36 forced TIER=all because of `.config/nextest.toml`**. `docs/predicate-dimension-audit.md` is a separate row; 2 came from `ci.yml`, and one each from a script, `interval-transcendentals/` and `benches/`.

`docs/prompts/implementer-discipline.md` §2 tells a lane to add any test costing ≥ 1 s to the `ci` profile's slow-set filter in the PR that adds the test. So the commonest way for a PR to land in the slowest tier is to follow that instruction. TIER=all is a cold workspace build plus the whole suite at three ε rows: about 30 min of the `test` job before any slow set runs.

## Wanted

When a `.config/nextest.toml` diff changes only exact-name entries of the `ci` profile's `default-filter`, classify it by the crates those names belong to, through their `binary_id(<pkg>::…)`. Any other nextest.toml change still forces TIER=all. ci-filter.py is S-TCOST's ground, so announce the seam.
