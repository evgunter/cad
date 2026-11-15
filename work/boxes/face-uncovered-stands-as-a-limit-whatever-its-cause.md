---
id: face-uncovered-stands-as-a-limit-whatever-its-cause
kind: issue
title: ChartRegionError::FaceUncovered stands as a limit whatever its cause, where EdgeUncovered stands by its cause
status: open
opened: 2026-11-15
priority: P3
cost: E
---


`ChartRegionError::refusal` (`crates/topo/src/chart_region.rs`, the `FaceUncovered(_) => Ok(Refusal::Limit)` arm near :435) gives a face with no cover row 2, a limit, whatever caused it. A face cover refuses for an unbounded carrier (row 2) and also for a poisoned net, a non-finite datum or a boundary edge with no cover (stored-data faults, row 4). `crate::cover::EdgeUncovered::refusal` (`crates/topo/src/cover.rs`, PR 6976) derives an edge's standing from its cause: a limit only where every unknown is an unbounded locus, an abort otherwise, and an abort for a mixed refusal until it is split. `FaceUncovered` should stand the same way, read from its carrier refusal and its `BoundaryUnknown` entries, in one place beside the edge's.
