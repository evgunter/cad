---
id: meter
kind: program
title: METER — the budget and K instruments
status: closed
opened: 2026-09-06
area: infra
prefix: meter/
tag: (METER orchestrator)
ab_band: 3200-3299
paths: [tools/*, docs/K-REPORT.md, docs/TESS-BUDGET.md, docs/tess-budget-data/*, docs/k-report-data/*]
keep_out: [code-quality Track K is claimed whole with GATES — this program takes tools/* and the two instrument documents with their data and GATES takes scripts/gates/*, scripts/tess_budget_cut.sh and tess_budget_sweep.sh and k_probe_sweep.sh are CIW's — the cut-prefix pin's direction is drawn with CIW in the PR that lands it, the tess-budget re-baseline (the committed cut) is PROPS' by its keep_out — D206's re-cut is announced there and lands with PROPS' agreement, crates/geom-brep/src/props/*.rs is PROPS' — k-lint's roster pin reads its names and edits nothing there, crates/topo/src/entity.rs is unowned and demos/ is Track X — D201's stable face identity is a design question drawn as a fence before a lane, crates/mesh/* is S-MESH's — tess-lint's mesh citations are read only and the sizing POLICY (S29) is S-MESH's design PR, memories/tessellation-budget.md and perf-measurement-lane.md govern what an instrument may claim]
closed: 2026-09-08
---

Code-quality Track K's instrument half: `tess-meter`, `tess-lint` and
`k-lint` under `tools/`, and the two documents they feed
(`docs/TESS-BUDGET.md`, `docs/K-REPORT.md`) with their committed data.
The slate is the face-identity join the budget gate cannot make
(consumer and producer halves), the split-scan resolution that exceeds
the gate's own tolerance, three prose claims the instruments no longer
describe, and three rosters and prefixes pinned in no direction.
Class E with one design question first (`D201`). Charter and order:
`work/meter/plan.md`; narrative in `work/meter/log.md`.
