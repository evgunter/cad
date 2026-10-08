---
id: all-tier-blanks-seeds-so-a-slow-set-edit-runs-no-slow-row
kind: issue
title: SEEDS is empty on the all tier, so a crate diff that edits the slow-set list runs none of its slow set
status: closed
opened: 2026-10-01
closed: 2026-10-01
priority: P3
cost: E
branch: ciw/seeds-on-the-all-tier
---

`scripts/ci-filter.py` `_all_tier` returned `SEEDS=""` on every `all`
verdict, and `classify` reaches `all` by raising `Bail` at the first
workspace-level path, discarding the seeds it had collected. ci.yml's
`test (the slow set of the crates this diff seeds)` step is gated on
`needs.filter.outputs.seeds != ''`, so a diff touching crate sources and
any workspace-level file ran no slow row of the crates it changed — while
the same crate diff alone (`closure`) ran them. `.config/nextest.toml`,
where the slow set itself is listed, is workspace-level, so editing the
slow-set list switched off the step that runs it. `gate ok` stayed green
over the skipped step.

Witness: PR #2468 (crates under editor-core/geom-core/sweep plus
`.config/nextest.toml`), runs 36818290656 and 36830873431 — `TIER=all`,
slow-set step skipped.

## Closed

SEEDS means the members whose own non-docs files changed on every code
tier: `_all_tier` takes the file list and derives it with `_seeds`, which
shares `_own_member` with `classify`. Empty on `Cargo.toml` alone, on
docs/aux, and on `--force-all`. The `decorate` axes keyed on seeds
(`RUN_VIEWER_TOOLKIT`, `RUN_PNCAD_PY`) and ci.yml's `flag`/`eps_extra`
branch on the tier first, so they are unchanged. Selftest rows pin the
crate + `.config/nextest.toml` case and the workspace-file-first ordering.
