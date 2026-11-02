---
id: boolean-kernel-bug-refusals-end-without-the-shared-ending
kind: issue
title: topo: the Boolean's, mass properties' and pcurves' kernel-bug refusals end in a tag or a hand-spelled report, not the shared kernel-defect ending
status: open
opened: 2026-09-28
parent: a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids
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
- `topo::BooleanError::ScaffoldingOperand`: decided by CLEAVE's
  `a-strut-bearing-operand-passes-the-boolean-gates-and-refuses-at-the-join`.
  The operand gate now refuses on tier 2's own verdict (a strut, an
  empty loop or a null edge left by a public Euler op is the caller's
  unfinished edit), so the text ends in a caller recourse ("finish that
  edit first"), not a report. Off this list.
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

**Superseded in direction (ENCL, 2026-10-21): read the evidence below before acting on this section.**

End each in the shared constant; the Boolean arms that render keys or
`Debug` payloads keep them (a kernel finding's report needs them).

## Evidence: the repair is the shape D2 retires (ENCL, 2026-10-21)

`work/encl/a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids.md` (`## Census, second pass`, main 69ab780194) classifies `JoinDesync` as class 1, sure: the public `graft_disjoint` reaches it with a caller's source. It classifies `TornComponent`, `SeamOrientation` and `ZipCorrespondence` as class 2, likely. HONE's child row `split-and-join-kernel-defect-endings-are-bug-only-typed-refusals` carries them. A bug-only state panics, so appending the constant moves them from one wrong shape to another. `pcurves.rs` `LoopWraps` is classified there too: class 1, unsure. The parent row carries it.

## The volume backstop left the operation path (2026-10-27)

PR 5842, under `[ev]` PR 5780 (the boolean's volume relations are a test oracle, `topo::test_support::volume_relations`), deleted the backstop: `BooleanError::{ResultVolumeImplausible, VolumeUnmeasured, VolumeCorrupt, VolumeUndecided}`, `BooleanDecision::VolumeBackstop`, `AtRestPolicy::gate_volume_backstop` and the `volume_backstop*` predicates are gone. `ResultVolumeImplausible`, the variant it cites as folded, is gone; the refusals it lists stand.
