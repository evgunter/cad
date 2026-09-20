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
paths: [.github/workflows/*, local-scripts/*, .claude/hooks/*, scripts/apt-install.sh, demos/*.sh, demos/*.py, scripts/check-*.py, scripts/criterion-emit.py, scripts/opt-level-calibrate.py, scripts/interval-only-selection.py, scripts/nightly-only-selection.py, scripts/pr-added-tests.py, scripts/doc-gate.sh, scripts/rundump-guard-selftest.sh, scripts/check_admesh.sh, scripts/check_step.sh, scripts/k_probe_sweep.sh, scripts/tess_budget_cut.sh, scripts/tess_budget_sweep.sh]
keep_out: [scripts/gates/* is code-quality Track K's, tools/* is INSTR's (work/instr/program.md, opened 2026-09-08), scripts/ci-filter.py and slowest-tests.py and base-test-listing.sh are S-TCOST's, scripts/work.py is the tracker's own and changes only with work/README.md, CI build knobs (profile/cache/sharding) stay S-TCOST's rule — measured in-unit or not at all, the one-line viewer bin rename in crates/viewer/Cargo.toml is announced to CHROME, what a main push re-gates is an [ev] ruling before any change to the F3 trim]
priority: P4
---

**The workflow half, after the 2026-09-20 cut**: `.github/workflows`,
the gating jobs, the critical path and what a reader of a red run can
tell from it.

Ev put this band low by name — *"it is low priority to improve tooling
in order to cause ci to fail less / main to be red less often"* — and
that is most of what is here: billed-minute arguments, prose digits,
cache shapes, job names, retries on third-party fetches. Fifty of its
rows are class `E`.

Three are not about redness but about a PR that is silently UNGATED
rather than red: a PR whose merge ref cannot be computed gets zero
gating jobs, a PR that goes `mergeable_state` dirty against a moved
main gets no Actions run on its next push, and a red inherited from
main is not attributed to the merge that caused it.

CIW measured **160 budget points unpriced and 67 once its rows carried
a cost**. It was cut into BLIND (instruments that cannot see what they
name; Ev's medium band) and MIRROR (the `scripts/` checkers) and this
remainder. CIW keeps its band 1500-1599.

Charter and unit order: `work/ciw/plan.md`; narrative in `work/ciw/log.md`.
