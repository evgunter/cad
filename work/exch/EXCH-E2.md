---
id: EXCH-E2
kind: unit
title: the step-import chart-coherence consumer — findings reach the importer's diagnostics
status: closed
opened: 2026-09-17
branch: exch/e2-coherence-consumer
parent: coherence-findings-have-no-step-import-consumer
closed: 2026-09-18
pr: 2837
---


Executes `coherence-findings-have-no-step-import-consumer` (FIX's
re-homed half, refs 2408/1585): step-import consumes
`topo::coherence::examine_chart_coherence(&Body<f64>, tol)` so chart
coherence findings reach the importer's diagnostics, inheriting PR
2408's skipped/unexamined distinction. The row's pre-answered
questions bind: establish early whether step-import's path is
monomorphic at `f64` (if so, no capability trait — markedly cheaper
than 2408's `ChartCoherenceLane`); the "fires on every cylinder"
hazard (`Unexaminable::NonIsoCarrier`) is priced before wiring; the
missing `Display` on the kernel coherence types is raised toward
TESS (`topo/src/coherence.rs` is theirs), never hand-copied into a
second vocabulary. E build, single style review, no A/B row.
