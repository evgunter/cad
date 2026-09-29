---
id: ci-filter-narrates-deleted-machinery-as-current
kind: issue
title: ci-filter.py's header narrates sampling, lanes, shards and the read reach as the present, and keeps modes and outputs no workflow reads
status: open
opened: 2026-09-28
priority: P4
cost: M
---


Found by the 2026-09-28 tracker sweep of `work/ciw/`, `work/tcost/` and
`work/tint/` after the CI-latency cut (`work/ciw/latency-cut.md`). The
sweep deleted the tracker rows about sampling, the lane axis, shards,
the archive hand-off and the read reach; `scripts/ci-filter.py` still
describes all of them, much of it in the present tense, and an agent
reading it would take them for live machinery.

## What is stale

- The module docstring (`scripts/ci-filter.py:1-355`): TIER=closure is
  described as *"the DEPENDENT CLOSURE PLUS THE READ REACH"* (`:19-26`),
  although `decorate` states at `:1241` that there is no read reach; the
  CONFIGURATION COVERAGE section (`:122-305`) argues sampling, lanes,
  shards, job minutes, `--config`, and a `the configuration this run
  gates` step `ci.yml` no longer has; `:335-343` cites
  `docs/CI-MINUTES-2026-08.md` and says the nightly's `--gated-set` row
  bounds a skipped suite's latency, where `nightly.yml`'s `full-suite`
  runs everything with no filter.
- Outputs no workflow reads: `REACHED` (`:74`), `CONFIG_SOURCE`
  (`:102`, `:2470`), and `RUN_PNCAD_PY`
  (`run-pncad-py-is-computed-and-gates-nothing` owns that one).
- Modes no workflow invokes: `--gated-set` (`gated_set`, `:2080`) and
  `--config` (`:3723`). `ci.yml`'s `workflow_dispatch` input is `scope`
  (`changed` / `all`), which maps to `--base` or `--force-all`.
- Billed-minute prose at `:138`, `:457`, `:1465`.

## The work

Rewrite the header to what the script does today (tiers, the dependent
closure plus test-utils, SEEDS, the per-file gate, the oracle and
viewer signals), in the present tense, and delete the dead modes and
outputs with their selftest cases, per
`docs/prompts/implementer-discipline.md`'s rule on a mode no workflow
invokes. `--gated-check` stays: `scripts/gates/gated-suite-paths.sh`
calls it.
