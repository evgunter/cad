---
id: shell-corrupt-refusal-ends-in-a-kernel-bug-tag
kind: issue
title: topo: ShellError::Corrupt and ReplaceFaceError::Corrupt end in a '(kernel bug)' tag, not the shared kernel-defect ending
status: open
opened: 2026-09-28
---


(ENCL implementer, from the sweep of
`work/encl/kernel-defect-endings-and-repair-labels-have-no-shared-home.md`.)

## What

`topo::ShellError::Corrupt`'s `Display` (`crates/topo/src/shell.rs:656`)
reads "{key:?} stopped resolving mid-construction (kernel bug)", and
`topo::ReplaceFaceError::Corrupt` (`crates/topo/src/replace_face.rs:560`)
reads "replace_face_offset: the body's referential coherence broke
mid-plan (kernel bug)": no `There is no way through` marker, so
`test_utils::refusal::recourse_markers` counts zero, and no report.
The ending has one home now, `geom_core::KERNEL_DEFECT_ENDING`
("There is no way through: this is a kernel defect; report it").

## Repair shape

Replace each tag with ". {KERNEL_DEFECT_ENDING}" (the stage prefix is
`replace-face-refusals-open-with-a-stage-prefix-and-name-keys`'s).
