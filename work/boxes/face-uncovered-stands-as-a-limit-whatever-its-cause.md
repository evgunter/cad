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

## A second reader stands it as a limit (2026-11-20, `oneround/box-region`)

The session box now reads every face's cover (`topo::session_box`), and a face with none refuses as `ValidationError::SessionBoxUncovered`, a limit, whatever `FaceUncovered` holds. Measured across the workspace's fast and slow sets at ε = 1e-9, 1e-6 and 1e-12, every finished-body ask that met such a face met a stored-data fault: a placeholder or seed NURBS net (`geometric_cube::without_the_top_cap_tier3_rejects_the_nurbs_seed`, `review_m3_pr4::a_placeholder_nurbs_wall_is_refused_at_rest`, `verbs_cylsph_opening::the_join_dispatchs_refusal_says_what_it_actually_wires`), a net carrying poison (`tier3_tests::a_described_net_carrying_poison_is_named_by_the_surface_check`), or a torus datum describing no locus (`tier3_tests::check_1_names_the_analytic_datum_that_describes_no_locus`). So the gate and tier 3′ hand that finding back and refuse with it only where tier 3's checks find nothing (`session_box::asked_before_the_checks`); check 1 names the fault first. When `FaceUncovered` gains a standing by cause, `SessionBoxUncovered` should read it rather than stand as a limit by fiat.
