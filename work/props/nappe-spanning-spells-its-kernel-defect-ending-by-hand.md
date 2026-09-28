---
id: nappe-spanning-spells-its-kernel-defect-ending-by-hand
kind: issue
title: geom-brep: PropsError::NappeSpanning spells its kernel-defect ending by hand, without the marker
status: open
opened: 2026-09-28
---


(ENCL implementer, from the sweep of
`work/encl/kernel-defect-endings-and-repair-labels-have-no-shared-home.md`.)

## What

`geom_brep::props::PropsError::NappeSpanning`'s `Display`
(`crates/geom-brep/src/props/mod.rs:561`) reads "no construction here
produces such a face, so report it rather than repairing a body: a cone
face stays on one nappe": a hand-spelled kernel-defect ending with no
`There is no way through` marker (zero to
`test_utils::refusal::recourse_markers`), behind an `integral
properties:` stage prefix. The ending has one home now,
`geom_core::KERNEL_DEFECT_ENDING` (or `KERNEL_OR_FILE_DEFECT_ENDING`
if the face may have been read from a file). Its sibling in `topo`,
`MassPropsError::RingOnCurvedFace`, is filed on the unowned slate
(`work/issues/boolean-kernel-bug-refusals-end-without-the-shared-ending.md`).

## Repair shape

End the sentence in the shared constant; the prefix and the rest are
`props-refusal-prose-outgrows-the-viewer`'s.
