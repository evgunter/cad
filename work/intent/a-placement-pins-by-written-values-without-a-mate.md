---
id: a-placement-pins-by-written-values-without-a-mate
kind: issue
title: Stage 3 D′, the placement pivot (PR 5494): a placement names its target Space, mates read only geometry and values chart on the target's axes, four operators and one product slot; built as three PRs
status: parked
opened: 2026-10-18
priority: P0
cost: H
refs: [transform-retires-into-a-placement]
blocked_on: [assemble-is-an-operator-and-the-product-is-a-slot]
---

**Spec:** `docs/INTENT-STAGE3-SPEC.md` §5′, restated on PR 5494's rulings against main at `4a4bb23608`; where this body and §5′ differ, §5′ governs. The design is ruled: PR 5494 (fork-log rows 189–191; Ev 2026-10-22, "it looks good!!") answered §5′'s Q1, Q2, Q3 and Q5, and the orchestrator's helpers ruling (2026-10-20, `work/intent/log.md`) answered Q4. This row is D′'s umbrella and closes when its three PRs have merged:

1. **D′1** `a-placement-names-its-target-space`: `Space` a minted kind (the world, `new_space()`, `a.space`); `Place` and `InstantiatePart` read a `target`; every body's space derived from the recipe and `SpaceMismatch` decided statically (the edit door, the builder, Python; a re-target leaves reads unresolved); every bundle pinned, so the loose copy retires.
2. **D′2** `a-mate-reads-only-geometry-and-a-value-charts-on-the-target-space`: `WorldRead` retires into values; a value names no mate and charts on the target space's axes; a point mate's spin chain; the helpers become one pure constructor; inline is one rule, with the flatten edit.
3. **D′3** `assemble-is-an-operator-and-the-product-is-a-slot`: `Node::Assemble` (`⊔`) with the census at its door; one product slot; one source per placement; an instance reads its part's product; picks, so `RoleSeg::Placed`/`InPart` and `CopyKey` retire.

Each absorbed row is `parent:` the PR that closes it (spec §5′).

Waiting on D′: E's core and the carry (`a-placement-carries-its-sources-at-rest-result`) on D′3, `flag-and-pivot-pose-kinds` on D′2, G on D′3 and E (spec §0, §12 "Sequencing").

Findings recorded on this row before the split, with the PR that ends each:
- **A copy at the identity places again** (2026-10-18). `mate::pin::copy_of` tells a placed copy by `value.motion.is_some()`, so a copy at the bit-exact identity is taken as unplaced. D′2 retires `copy_of`'s motion test with the helpers, and the D rows that relied on it (in `msolve1_transform_aware`, `msolve13_read_at_operand` and `mate1_member_vocab`) are restated there.
- **Deleted in D, rebuilt over picks in D′3:** `msolve1_transform_aware::a3b_a_placed_copy_of_a_pattern_seats_and_gates` (the at-rest gate reported `StaleContactDeclaration` for copies 1 and 2 of a placed 3-copy pattern) and `msolve13_read_at_operand::a1b_a_placement_above_the_operand_mints_nothing` (a mate read at an unplaced pattern copy minted on a placement of that copy standing elsewhere).
- **The helpers' turns read off geometry** (D's exact-turn lane, 2026-10-20): `place_at`'s plane-path `Spin` (`Chart::spin_at`) and axis-path world read (`axis_in_world` → `moved_by`'s turns); `place_against`'s plane-path `Spin`, tilted-path frame (`frame_at_in`) and axis-path target frame (`axis_placed`); `frame_at`'s picked frame moved by `moved_pose`'s turns; `pin_at`'s `world_at` → `moved_by`. D′2 retires all of them; the callers that meant an exact frame write it (`moved_frame`, `corner_frame`, or a mateless bundle).
- 2026-10-21, D's review (frozen head `a59da0cecc`), findings D′2's pure constructor retires: `pin.rs` `written()` classifying a value by `f64 == 0.0`; `place_at_in`'s fallback chain; `m10_5_clearance_interval::slid_by` reverse-engineering `place_against`'s output; the gimbal's `√ε` threshold; `refactor::reads_a_number`.
- 2026-10-21, PR #5547's review: `pin.rs` `axis_placed` seeds the copy's own frame about its axis with world `x`, or world `y` where `|dir.x| ≥ 0.5`, `orthonormal_basis`'s shape without the name. The spin it picks reaches no solved pose (both targets read the composed frame through its `z` only). D′2 retires `axis_placed` with `place_at`/`place_against`.
