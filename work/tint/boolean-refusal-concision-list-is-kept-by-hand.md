---
id: boolean-refusal-concision-list-is-kept-by-hand
kind: issue
title: refusal_concision's BooleanError list renders a hand-picked subset of the enum's arms, and no roster ties it
status: open
opened: 2026-10-20
priority: P4
cost: E
---


Found by `wire-census2`'s sweep for the class of
`kernel-payload-lists-in-the-concision-census-are-kept-by-hand`, whose
PR welds the feature-tree census's kernel lists to their enums
(`every_forwarded_arm_has_a_row`, `crates/editor-core/tests/refusal_concision_chains.rs:1389`).

`rendered_boolean_refusals`
(`crates/editor-core/tests/refusal_concision.rs:159`) is the census's home
for `topo::BooleanError`: the feature-tree census samples only its
`Join` arm (the `Boolean/Join/` rows, twinned with `Split/Join/`), and
its `forward` (`crates/editor-core/tests/refusal_concision_chains.rs:1185`, `UNWELDED_BOOLEAN`) cites this row for `NodeErrorKind::Boolean`. The list
renders "every arm whose prose the concision pass wrote", by hand: a
scripted comparison of its constructors against the enum finds 37 of
76 arms, so 39 never render through the budget there — `Band`,
`CrossingAtConeApex`, `NormalAtConeApex`, `UnjoinedOperand`,
`PoisonedCarrierDatum`, `DeclarationContradicted`,
`ContactContradicted`, `ContinuationContradicted`,
`UnsupportedDeclarationClass`, `InvalidInput`, `PairingMismatch`,
`ClassificationInvariant`, `CrossingInsertion`, `Euler`, `Pcurves`,
`Join`, `JoinUndecided`, `JoinCarrierUnsupported`, `RestPieceUnread`,
`CarryRefused`, `JoinDesync`, `SplitLineage`, `LineageRow`,
`EdgeUndescribed`, `MixedSeam`, `TornComponent`, `SeamOrientation`,
`ZipCorrespondence`, `Merge`, `ResultInvalid`,
`ResultVolumeImplausible`, `VolumeUnmeasured`, `VolumeCorrupt`,
`VolumeUndecided`, `UnrepresentableResult`, `GraftRecertify`,
`Pieces`, `Revert`, `LineWedges`. Its `PointInSolidError` rows (through
`BooleanError::Containment`) skip `InsideOut`, which the chains
census's `separation_reasons` (`crates/editor-core/tests/refusal_concision_chains.rs:6247`) now renders.

Some of these may render elsewhere (`Join` through the chains census's
`Boolean/Join/` rows). Each wants a row or a reason in-file.

`NormalAtSplineCrease` joined the set on 2026-10-26 (PR 5802, the
sector walk's spline arm). It renders through no row either.

## Repair shape

An `f6_variants!` roster for `BooleanError` and a set difference over
the list's rows, as `every_forwarded_arm_has_a_row` does; then a row
per arm the roster turns up, or a named reason. When it lands, move
`NodeErrorKind::Boolean` in the chains census's `forward` off
`Unwelded(UNWELDED_BOOLEAN)`.

## The volume backstop left the operation path (2026-10-27)

PR 5842, under `[ev]` PR 5780 (the boolean's volume relations are a test oracle, `topo::test_support::volume_relations`), deleted the backstop: `BooleanError::{ResultVolumeImplausible, VolumeUnmeasured, VolumeCorrupt, VolumeUndecided}`, `BooleanDecision::VolumeBackstop`, `AtRestPolicy::gate_volume_backstop` and the `volume_backstop*` predicates are gone. Four of the arms it lists are gone with the enum's variants; the hand-kept list is still this row's.
