---
id: a-bundle-folds-in-the-joined-space
kind: issue
title: A bundle's constraints all target one copy or all the world: folding a bundle whose mates read two targets needs the fold in the joined space
status: open
opened: 2026-10-11
priority: P1
refs: [a-mate-relates-two-poses]
parent: a-placement-pins-by-written-values-without-a-mate
---


Stage 3 B's named interim (`docs/INTENT-STAGE3-SPEC.md` §3; the B2 ruling): every constraint of one bundle targets one other copy, or every one targets the world. A bundle mixing targets refuses `MateFault::TargetsDiffer { placement, constraints }` (`crates/editor-core/src/mate/solve.rs:1561`, in `read_bundle`), naming the bundle's constraints in order.

What is owed is the arithmetic: a bundle whose mates read two copies (or a copy and the world) relates its copy to a joined space, and its fold has to intersect cosets expressed in that space — each target's pose carried into one frame before `fold_pair` — rather than in one target's frame, which is all `fold_bundle` (`mate/solve.rs:1590`) builds today. Lifting it retires `TargetsDiffer`; the rows that pin the refusal (`intent_s3_b_placements`) then pin the fold.
