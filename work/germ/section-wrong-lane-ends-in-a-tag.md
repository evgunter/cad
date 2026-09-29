---
id: section-wrong-lane-ends-in-a-tag
kind: issue
title: geom-brep: SectionError::WrongLane ends in '(a kernel bug)', not the shared kernel-defect ending
status: open
opened: 2026-09-28
---


(ENCL implementer, from the sweep of
`work/encl/kernel-defect-endings-and-repair-labels-have-no-shared-home.md`.)

`crates/geom-brep/src/intersect.rs` is GERM's, REACH's and TANG's by
territory.

## What

`geom_brep::intersect::SectionError::WrongLane`
(`crates/geom-brep/src/intersect.rs:708`) reads "the section was
dispatched to the arm for {expected}, not this pair's (a kernel bug)":
no `There is no way through` marker (zero to
`test_utils::refusal::recourse_markers`) and no report. The ending has
one home now, `geom_core::KERNEL_DEFECT_ENDING`.

## Repair shape

Replace the tag with ". {KERNEL_DEFECT_ENDING}".
