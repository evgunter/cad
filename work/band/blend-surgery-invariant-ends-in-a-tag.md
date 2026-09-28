---
id: blend-surgery-invariant-ends-in-a-tag
kind: issue
title: sweep: BlendError::SurgeryInvariant ends in '(a kernel bug)', not the shared kernel-defect ending
status: open
opened: 2026-09-28
---


(ENCL implementer, from the sweep of
`work/encl/kernel-defect-endings-and-repair-labels-have-no-shared-home.md`.)

`crates/sweep/src/blend/mod.rs` is BAND's and CARVE's by territory.

## What

`sweep::BlendError::SurgeryInvariant` (`crates/sweep/src/blend/mod.rs:1453`)
reads "{detail} — at {at}: the blend surgery contradicted its own
earlier steps (a kernel bug); nothing about the body needs changing".
That is a hand-spelled kernel-defect ending with no `There is no way
through` marker (zero to `test_utils::refusal::recourse_markers`) and
no report. The ending has one home now, `geom_core::KERNEL_DEFECT_ENDING`.

## Repair shape

End the sentence in the shared constant.
