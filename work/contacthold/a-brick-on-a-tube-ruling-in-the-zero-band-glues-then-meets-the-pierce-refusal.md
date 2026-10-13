---
id: a-brick-on-a-tube-ruling-in-the-zero-band-glues-then-meets-the-pierce-refusal
kind: issue
title: A brick on a tube's ruling within the zero band glues, then every op refuses CurvedPierceUnsupported
status: open
opened: 2026-10-13
priority: P2
---



## What

A brick spanning a tube's tangent ruling, with its top `0.5·ε` below the
wall's carrier, so the gap is in the zero band. The tube is `r ∈ [0.3, 1]`
about `y`, `y ∈ [0, 1]`; the brick is `x ∈ [−0.5, 0.5]`,
`y ∈ [0.2, 0.8]`, top at `z = −1 − 0.5·ε`. The glue door reads the pair
tangent and glues it, and then every op in both orders refuses
`CurvedPierceUnsupported`. C8 (`crates/topo/README.md`) says a pair the
witness lane has a locus for never meets that refusal. The same happens on
PR 4918's base.

Measured on PR 4918 after merging main (2026-10-13, ε = 10⁻⁶, 10⁻⁹ and
10⁻¹²): ∪, ∖ and ∩, both orders, all `CurvedPierceUnsupported`. ∪ used to refuse
`TangentSlitArmUnbuilt`; PR 4921 retired that arm, and ∪ now meets the
pierce refusal like the other two. With the gap at 1.5·ε to 8·ε the door
escalates `Coincidence(TangentLocus)` on `pc_parallel_gap`, and at 20·ε
every op builds.

A sibling of
`two-tubes-in-the-zero-band-answer-by-operand-order.md`: two poses where
the door glues in the zero band and a later stage cannot carry the glued
pair. Pose: `r2_brick_on_the_ruling_at_multiples` in
`crates/sweep/tests/review2_glue_door_probes.rs` on branch
`contacthold/glue-door-escalates-review-2`.
