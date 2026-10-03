---
id: nappe-spanning-spells-its-kernel-defect-ending-by-hand
kind: issue
title: geom-brep: PropsError::NappeSpanning spells its kernel-defect ending by hand, without the marker
status: review
opened: 2026-09-28
branch: props/recourse-grammar
pr: 3942
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

## Resolved (props/recourse-grammar)

`PropsError::NappeSpanning`'s `Display` ends in
`geom_core::KERNEL_OR_FILE_DEFECT_ENDING`, not
`KERNEL_DEFECT_ENDING`: the variant is read over a body at rest (tier
3's check 7 and the Boolean's volume backstop), which a parsed STEP cone
face reaches as surely as a defective construction, and one rendering
serves both readers. `recourse_markers` now counts 1 on it, pinned by
`every_props_error_arm_names_a_recourse`.

The `integral properties:` prefix is gone with it
(`props-refusal-prose-outgrows-the-viewer`, which owned it), and
`every_props_error_arm_fits_where_it_is_shown` holds every arm of the
enum to no stage prefix and the 75-word budget.
