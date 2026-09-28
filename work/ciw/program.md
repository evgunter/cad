---
id: ciw
kind: program
title: CIW — hosted CI, workflows and scripts
status: ready
opened: 2026-09-03
area: infra
prefix: ciw/
tag: (CIW orchestrator)
ab_band: 1500-1599
paths: [.github/workflows/*, local-scripts/*, .claude/hooks/*, scripts/apt-install.sh, demos/*.sh, demos/*.py, scripts/check-*.py, scripts/criterion-emit.py, scripts/doc-gate.sh, scripts/check_admesh.sh, scripts/check_step.sh, scripts/k_probe_sweep.sh, scripts/tess_budget_cut.sh, scripts/tess_budget_sweep.sh]
keep_out: [scripts/gates/* is GUARD's (work/guard/program.md), tools/* is INSTR's (work/instr/program.md), scripts/ci-filter.py is S-TCOST's, scripts/work.py is the tracker's own and changes only with work/README.md, CI build knobs (profile/cache) stay S-TCOST's rule — measured in-unit or not at all, what a main push re-gates is an [ev] ruling]
priority: P4
---

**The workflow half**: `.github/workflows`, the jobs that gate a PR and
the nightly that holds the rest, and what a reader of a red run can
tell from it.

The per-PR gate is sized for latency (`ci.yml`); everything else runs in
`nightly.yml`. `work/ciw/latency-cut.md` carries the selection and its
evidence, and a change that adds work to the per-PR gate argues against
it.

Ev put this band low by name — *"it is low priority to improve tooling
in order to cause ci to fail less / main to be red less often"* — and
most of the slate is that. The rows that are not about redness are about
a PR that is silently UNGATED rather than red (a head with no merge ref
gets no run), and a red inherited from main that is not attributed to
the merge that caused it.

Charter and unit order: `work/ciw/plan.md`; narrative in `work/ciw/log.md`.
