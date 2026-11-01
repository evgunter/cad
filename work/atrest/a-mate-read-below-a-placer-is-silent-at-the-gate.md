---
id: a-mate-read-below-a-placer-is-silent-at-the-gate
kind: issue
title: A mate read below a placed consumer mints nothing, and the gate's unattributed findings do not point at it
status: open
priority: P3
cost: M
opened: 2026-10-10
---

Found by review R2 of INTENT stage 2 F (PR 4566, minor m2).

## What

Before F, a mate read below a placer (at a transform a pattern or a placed union consumes) refused at the gate as `RefusedRef::MovedAbove { at, by }`, naming the operand and the placer: a recourse ("read the side at the placer"). Under F a mate side is a read, and the gate mints on the world copy of the body it reads (spec §7, Q8). No placement reads that transform, so the mate mints nothing (`assembly::mint`, `side_placed`). The seat it would have declared reaches the gate as an unattributed contact, with nothing pointing at the mate that meant to declare it. `crates/editor-core/tests/msolve13_read_at_operand.rs` A1(a) and `msolve5_operand_refusals.rs` pin the behaviour.

## What it wants

A non-gating pointer: an unattributed finding whose two faces are the world copies of faces some mate reads below a placer could name that mate and the placement reading its consumer ("re-read the side at the placement's body"). It is a hint, not a refusal: the mate states nothing about the product, as ruled.

## Note (2026-10-26, ATREST orchestrator)

Stage 5 C retires the gate (`docs/INTENT-STAGE5-SPEC.md` §4). Once it lands, the hint belongs on the `unproven-coincidence` lint's finding for the seat, not on a gate refusal. Re-read this row against C before building it.
