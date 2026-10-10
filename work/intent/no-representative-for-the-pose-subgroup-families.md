---
id: no-representative-for-the-pose-subgroup-families
kind: issue
title: The fold has no representative for the four pose subgroup families; it refuses NoRepresentative
status: open
opened: 2026-10-10
---

## What

INTENT stage 3 A made `PoseSymmetry` answer one `Subgroup` for poses and
mates (`crates/editor-core/src/mate/coset.rs`), adding four families a
mate never produced: `Spherical` (a point's), `Parallel` (a direction's),
`Translation` and `PlaneTranslation`. The fold's closure table carries
their rows, but `intersect` (`coset.rs:738`) has no representative
construction for them: a fold reaching one refuses
`FoldStop::NoRepresentative` (`coset.rs:1308`, gated by
`SubgroupFamily::folded_by_mates`, `coset.rs:266`), and the mate solve
asserts it never does (`crates/editor-core/src/mate/solve.rs:1890`),
which holds while a mate pins only a frame, an axis or a plane.

## Fix shape

Stage 3 C (a mate relates two poses) is the first caller that can fold a
point or a direction: build the representatives there (a point's fixed
point, a direction's line through the held representative), retire
`folded_by_mates`, and turn the solve's `unreachable!` into the ordinary
arm.
