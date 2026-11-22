---
id: the-carrier-frame-fuzz-bodies-go-red-under-no-mutant
kind: issue
title: The two projected fuzz bodies go red under no frame-cost mutant; only their pinned companions guard the cost
status: open
opened: 2026-11-21
priority: P2
cost: E
---


## What

The carrier-frame cost (PR 7131) is guarded at ε 1e-12 by mutants A, F1X and F1F. The two fuzz bodies count an escalation and let a draw build, so no mutant turns them red:
- `crates/geom-brep/src/pcurve_cache/envelope_lemma_fuzz/projected.rs:281` `a_carrier_on_its_chart_images_exactly_at_f64_and_interval`;
- `projected.rs:1504` `a_line_near_a_ruling_certifies_and_its_envelope_dominates`.

Only their pinned-seed companions (`a_carrier_on_its_chart_pinned_where_the_frame_cost_refuses`, `a_line_near_a_ruling_pinned_where_the_frame_cost_refuses`) go red under A and F1X. Give each body an assertion a mutant can reach, or record why the companions are the whole guard.

Filed from the second delta review of PR 7131.
