---
id: boolean-kernel-bug-refusals-end-without-the-shared-ending
kind: issue
title: topo: the Boolean's, mass properties' and pcurves' kernel-bug refusals end in a tag or a hand-spelled report, not the shared kernel-defect ending
status: open
opened: 2026-09-28
---


(ENCL implementer, from the sweep of
`work/encl/kernel-defect-endings-and-repair-labels-have-no-shared-home.md`;
the files below are on no program's ground.)

## What

The kernel-defect ending has one home now,
`geom_core::KERNEL_DEFECT_ENDING` ("There is no way through: this is
a kernel defect; report it"), with `KERNEL_OR_FILE_DEFECT_ENDING`
for a body that may have been read. That unit folded
`BooleanError::ResultVolumeImplausible`, whose "Please report it"
ending was the same sentence. These refusals end instead in a
"(kernel bug)" tag or a hand-spelled report, which carries no `There
is no way through` marker (zero to `test_utils::refusal::recourse_markers`):

- `topo::BooleanError` (`crates/topo/src/boolean/mod.rs`):
  `ClassificationInvariant` (`:1752`, a bare "classification invariant
  violated: {what}"), `JoinDesync` (`:1785`, "(kernel bug or corrupt
  reduction)"), `TornComponent` (`:1791`), `SeamOrientation`
  (`:1801`), `ZipCorrespondence` (`:1804`), each a "(kernel bug)" tag,
  and `ResultInvalid` (`:1810`, "— kernel bug, no invalid body is
  returned").
- `topo::BooleanError::ScaffoldingOperand` (`boolean/mod.rs:1564`):
  "This is a bug in whatever produced that body; please report it".
  Not folded onto `KERNEL_DEFECT_ENDING`, because "kernel defect" is
  not established: `Body::mev_null` is public, so an API caller can
  hand the Boolean a body left mid-surgery. The ending still carries no
  marker. Its repair has to decide whose defect it names (a dead end
  that does not claim the kernel, or a repair for the caller who
  produced the body).
- `topo::props::MassPropsError::RingOnCurvedFace`
  (`crates/topo/src/props.rs:234`): "report this rather than repairing
  a body", behind a `mass properties:` prefix.
- `topo::PcurveMintError::LoopWraps` (`crates/topo/src/pcurves.rs:429`),
  ending "so report the body that reached it".

Not this class: panic and `debug_assert!` messages that carry the same
"(kernel bug)" tag (`euler.rs`, `revert.rs`, `merge_faces.rs`,
`surgery.rs`, `body.rs`, `boolean/combine.rs`) are for a developer, not
refusals a viewer shows; and a `Recourse:` whose second branch is a
report (`validate`'s `LaminaWedge`, `MeterError::NormalFloor`,
`PcurveMintError::MissingCache`) is a labelled repair, not a dead end.

## Repair shape

End each in the shared constant; the Boolean arms that render keys or
`Debug` payloads keep them (a kernel finding's report needs them).
