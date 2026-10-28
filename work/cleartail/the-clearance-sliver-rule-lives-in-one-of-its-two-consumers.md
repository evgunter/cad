---
id: the-clearance-sliver-rule-lives-in-one-of-its-two-consumers
kind: issue
title: topo::clearance::sliver is the drive's terminal-sliver rule hosted in one of its two consumers; both read only Indeterminate fields
status: open
opened: 2026-10-28
priority: P4
cost: E
refs: [SHELL-3]
---


## Finding

From the full review of PR #5909 (SHELL-3, the move into
`crates/topo/src/clearance.rs`), style lane Q1, reported as likely.
The shape predates the move, which put it in one file. The review's
report is summarised in `work/clear/log.md` (2026-10-28).
Filed by the CLEAR orchestrator.
