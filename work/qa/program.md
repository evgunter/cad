---
id: qa
kind: program
title: S-QA — gates that lie
status: closed
opened: 2026-08-29
closed: 2026-08-31
area: infra
prefix: qa/
tag: (S-QA orchestrator)
ab_band: 800-899
paths: [.github/workflows/*, local-scripts/*, scripts/doc-gate.sh, scripts/gates/gate-roster.sh, scripts/gates/probe-suite-census.sh, scripts/*.py, Cargo.toml]
keep_out: [scripts/gates beyond the two J-named scripts and tools/ and docs/K-REPORT.md are Track K's, crates/test-utils and crates/*/tests mechanisms are Track W's, the bounds-allowlist rows stay contested, k-lint distribution semantics are K-telemetry ground, a scheduled full run on main is declined (Ev twice) and is not re-proposed]
---

Test and CI infrastructure that reports green without looking: silent
passes, matrix under-reporting, test-integrity races, operability, and SMELL
track J claimed whole. Closed: the exit walk `docs/S-QA-EXIT-WALK.md` is
ratified (Ev, in-session, 2026-08-31) and is the done-state of record; Track
J is empty. Standing residue lives where the walk names it — Track W carries
the tests-leg measured-claim sweep with issue 651 as class home, issue 1317
registers the doc-gate's two remaining axes, and issues 470 and 466 stay
parked with their recorded reasons. Charter, rulings and the unit list:
`work/qa/plan.md`; narrative in `work/qa/log.md`.
