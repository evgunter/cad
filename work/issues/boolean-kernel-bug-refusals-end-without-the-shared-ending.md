---
id: boolean-kernel-bug-refusals-end-without-the-shared-ending
kind: issue
title: topo: the Boolean's and mass properties' kernel-bug refusals end in a tag or a hand-spelled report, not the shared kernel-defect ending
status: open
opened: 2026-09-28
---


(ENCL implementer, from the sweep of
`work/encl/kernel-defect-endings-and-repair-labels-have-no-shared-home.md`;
the files below are on no program's ground.)

## What

The kernel-defect ending has one home now,
`geom_core::KERNEL_DEFECT_RECOURSE` ("There is no way through: this is
a kernel defect; report it"), with `KERNEL_OR_FILE_DEFECT_RECOURSE`
for a body that may have been read. That unit folded
`BooleanError::ScaffoldingOperand` and `ResultVolumeImplausible`,
whose "please report it" endings were the same sentence. These
refusals end instead in a "(kernel bug)" tag or a hand-spelled report,
which carries no `There is no way through` marker (zero to
`test_utils::refusal::recourse_markers`):

- `topo::BooleanError::ClassificationInvariant`, `JoinDesync`, `TornComponent`,
  `SeamOrientation`, `ZipCorrespondence`, `ResultInvalid`
  (`crates/topo/src/boolean/mod.rs`): a "(kernel bug)" tag, a bare
  "invariant violated: {what}", or
  "— kernel bug, no invalid body is returned".
- `topo::props::MassPropsError::RingOnCurvedFace`
  (`crates/topo/src/props.rs`): "report this rather than repairing a
  body", behind a `mass properties:` prefix.
- `topo::PcurveMintError::LoopWraps` (`crates/topo/src/pcurves.rs`), ending "so
  report the body that reached it".

Not this class: panic and `debug_assert!` messages that carry the same
"(kernel bug)" tag (`euler.rs`, `revert.rs`, `merge_faces.rs`,
`surgery.rs`, `body.rs`, `boolean/combine.rs`) are for a developer, not
refusals a viewer shows; and a `Recourse:` whose second branch is a
report (`validate`'s `LaminaWedge`, `MeterError::NormalFloor`,
`PcurveMintError::MissingCache`) is a labelled repair, not a dead end.

## Repair shape

End each in the shared constant; the Boolean arms that render keys or
`Debug` payloads keep them (a kernel finding's report needs them).
