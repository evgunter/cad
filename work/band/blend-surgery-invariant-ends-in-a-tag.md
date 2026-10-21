---
id: blend-surgery-invariant-ends-in-a-tag
kind: issue
title: sweep: BlendError::SurgeryInvariant calls itself a kernel bug in a typed refusal; a bug-only state panics
status: open
opened: 2026-09-28
parent: a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids
priority: P4
cost: E
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

**Superseded in direction (ENCL, 2026-10-21): read the evidence below before acting on this section.**

End the sentence in the shared constant.

## Evidence: the repair above is the shape D2 retires (ENCL, 2026-10-21)

This arm is a site of the parent census's second pass (`work/encl/a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids.md`, `## Census, second pass`, main 69ab780194). **Class 2, likely**: the blend surgery contradicting its own earlier steps is reachable only by a bug, and `[failure]` says such a state panics. Ending it in `KERNEL_DEFECT_ENDING` instead moves it from one wrong shape to another. The repair is `unreachable!` with row 4's message (what the surgery re-derived, and why a correct one always passes), after answering row 0; the tests that pin the text move to `#[should_panic]`.
