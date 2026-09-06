---
id: debug-only-gate-step-name-understates-its-subjects
kind: issue
title: ci.yml's 'bit-identity debug-only guard (topo/source.rs)' step title names one subject of a gate that now scans a list
status: open
opened: 2026-09-06
---


## Finding

Filed by the GATES orchestrator from PR 2030. `.github/workflows/ci.yml:1366`
titles the step `bit-identity debug-only guard (topo/source.rs)`; since
PR 2030 the gate scans a (subject, symbol) list — `topo/src/source.rs`'s
bit channel and `editor-core/src/product.rs`'s gather counter, with
more candidates filed on `work/gates/` — so the title understates it.
One line, CIW's file; the gate's own header no longer cites the step
title, so nothing else rots when it is renamed. Sequenced after PR
2030 lands.
