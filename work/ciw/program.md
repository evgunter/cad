---
id: ciw
kind: program
title: CIW — hosted CI, workflows and scripts
status: open
opened: 2026-09-03
area: infra
prefix: ciw/
tag: (CIW orchestrator)
ab_band: 1500-1599
paths: [.github/workflows/*, local-scripts/*, demos/*.sh, demos/*.py, scripts/check-*.py, scripts/criterion-emit.py, scripts/opt-level-calibrate.py, scripts/interval-only-selection.py, scripts/nightly-only-selection.py, scripts/pr-added-tests.py, scripts/doc-gate.sh, scripts/rundump-guard-selftest.sh, scripts/check_admesh.sh, scripts/check_step.sh, scripts/k_probe_sweep.sh, scripts/tess_budget_cut.sh, scripts/tess_budget_sweep.sh]
keep_out: [scripts/gates/* and tools/* are code-quality Track K's, scripts/ci-filter.py and slowest-tests.py and base-test-listing.sh are S-TCOST's, scripts/work.py is the tracker's own and changes only with work/README.md, CI build knobs (profile/cache/sharding) stay S-TCOST's rule — measured in-unit or not at all, the one-line viewer bin rename in crates/viewer/Cargo.toml is announced to CHROME, what a main push re-gates is an [ev] ruling before any change to the F3 trim]
---

The S-QA ground, unowned since 2026-08-31: workflow files, the render
lanes, the parity checkers, the perf emitters and the demo shell and
Python. Every item is E — the fix is written in the item — with two
rulings split out as `[ev]` PRs. Review posture: batched style review,
no A/B row (infra-only units, the S-TCOST precedent). Charter and unit
order: `work/ciw/plan.md`; narrative in `work/ciw/log.md`.
