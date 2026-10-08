---
id: the-dual-review-streams-third-readout-is-owed
kind: ruling
title: The dual review stream reached its third readout point at DR-104, the first M-tier miss (a holdout pair's tallied MAJOR whose other review raised none); Ev rules on what the stream does next
status: open
opened: 2026-10-07
priority: P4
needs_ev: true
pr: 4283
---


Rule 9 of `docs/DUAL-REVIEW-PROTOCOL.md` owes a readout at the first M-tier miss: a tallied finding in a holdout pair whose other review raised no MAJOR. Taken first, the sequential arm would have shipped that finding.

DR-104 (JOIN, PR 4274) is that pair:
- **The unit:** an M unit. Its arm byte was 81, which gives HOLDOUT.
- **R1 raised M1:** a legal pose that main refused typed now reaches `ClassificationInvariant` on 84 probe lines. Coded UNILATERAL, a code defect, demonstrated by execution, and tallied.
- **R2 raised no MAJOR.**

A separate agent writes the readout blind, off-file, on the branch `analysis/dual-review/readout-3`. Duals continue until Ev rules.
