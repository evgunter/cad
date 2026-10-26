---
id: unread
kind: program
title: UNREAD — measurements, populations and pins that nothing reads
status: ready
opened: 2026-10-26
area: infra
prefix: unread/
tag: (UNREAD orchestrator)
ab_band: 15700-15799
paths: [crates/pncad/src/prelude.rs, crates/topo/src/review_m1_pr5_internal.rs, crates/viewer/src/vocab.rs]
keep_out: [opened 2026-10-26 by GUARD's split - the rows moved by git mv with ids and bodies unchanged, a new gate is a new file under scripts/gates/* which is GUARD's directory - announce it on GUARD's log and build it on GUARD's harness (lib.sh) rather than a private reader, .github/workflows/* is CIW's - wiring a new gate or a probe-feature run into a job lands by announced seam, tools/* and docs/K-REPORT.md and docs/TESS-BUDGET.md are INSTR's, crates/*/tests/* and crates/test-utils/* are S-TINT's and S-TCOST's, the three paths claimed here are where three rows' guards land and are shared with whichever programs own those crates - a row here edits them only for the guard it adds, run scripts/work.py territory on your branch and announce the seam in the PR]
---

The guards that do not exist yet. Every row is something the tree
states — a measured number, a census's membership, a compile-fail proof,
a table's reasons, a probe-gated pin, a formatting promise — that no
gate, test or CI job reads, so it goes stale without anything going red.
The question each row answers is *"what goes red if this stops being
true?"*, and *"nothing, and here is why"* is a complete answer for one.
Cut out of GUARD on 2026-10-26 when GUARD measured 43 points against 30.
Charter and order: `work/unread/plan.md`; narrative in
`work/unread/log.md`.
