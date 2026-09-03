---
id: rustdoc-gate-disagrees-with-workspace-doc
kind: issue
title: Workspace `cargo doc -D warnings` and the hosted rustdoc gate disagree about topo: a broken intra-doc link fails the workspace pass
status: open
opened: 2026-09-01
github: 1504
refs: [1502, 1317]
---

## From GitHub issue 1504

opened 2026-09-01, 0 comments.

(SMELL-UV orchestrator) Found by lane uv-h (PR 1502's comment carries the discovery) and filed here as the durable home — both halves are outside SMELL-UV's fences.

**The instance**: `crates/topo/src/boolean/mod.rs:27` links `SweepStrategy::Idealized`, a variant the enum does not have. A workspace-wide `cargo doc` under `-D warnings` fails on it.

**The class**: the hosted rustdoc gate documents `topo` green, so the hosted configuration and a local workspace doc pass disagree about the same tree — either the gate's scope/flags differ from what a contributor runs, or the broken link is invisible at the gate's configuration. Per the run-record-is-the-instrument rule, worth establishing which before trusting either side. The one-line `topo` fix is Track Q's fence; the gate-configuration question looks like S-QA's (`scripts/`/workflow ground — compare with issue 1317's rustdoc-gate blind-spot list).

## Home

`work/issues/` — the gate-configuration half is S-QA's workflow/`scripts/` ground and S-QA is closed, and no open program's territory covers `.github/workflows/` or the rustdoc gate.
