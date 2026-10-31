---
id: split-and-join-kernel-defect-endings-are-bug-only-typed-refusals
kind: issue
title: topo: twenty-six Boolean, split, join and piece-sort refusals name the kernel alone; two are reachable through a public door, and one may be a construction's limit
status: open
opened: 2026-10-20
parent: a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids
priority: P3
cost: M
---

A child of the census in `work/encl/a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids.md` (as of main 5df6e6c854); why a typed `KERNEL_DEFECT_ENDING` is wrong in either D2 class is that row's "What" and "Shape of the work".

## Sites

Doors: the split doors, and the Boolean doors through the join, the
containment reads and the piece sort.

- `crates/topo/src/chord_join.rs` `SplitJoinError` `Display`,
  `LineageRow` (~668). **Class 2, likely**: the join appends these rows
  itself, and `lineage::RowRefusal` is refused at append.
- `crates/topo/src/splitting/mod.rs` `SplitError` `Display`,
  `LineageRow` (~792). **Class 2, likely**, for the same reason.
- `crates/topo/src/splitting/containment.rs`
  `PointInLoopError::render_as`, `CorruptLoop` (~492). **Class 2,
  likely**: a torn body, and tier 1 holds at every public door
  (`[failure]`'s closure property).
- `crates/topo/src/splitting/finish.rs` `FinishError` `Display`,
  `NestingContradiction` (~839). **Class 2, likely**: the variant's own
  doc says outlines decided disjoint cannot each enclose the other.
- `finish.rs` `FinishError` `Display`, `SlitUnplaced { in_hole: false }`
  (~848), and `SlitUnplaced { in_hole: true }` (~858). **Class 2,
  likely**: the variant's doc says a section region of the side covers
  the slit's edges.
- `crates/topo/src/splitting/section.rs` `SectionError` `Display`,
  `NestingContradiction` (~250). **Class 2, likely**, as `finish.rs`'s.
- `section.rs` `SectionError` `Display`, `Corrupt` (~255). **Class 2,
  likely**: a traversal over a tier-1 body does not fail.
- `crates/topo/src/pieces.rs` `PieceSortError` `Display`, `Move`
  (~169). **Class 2, likely**: the sort is moving a piece it has just
  found into its own solid. `pieces.rs` is in no program's territory,
  and HONE's charter is the nearest.

`work/honeword/kernel-bug-refusals-end-without-the-shared-ending.md` covers
the same files' "(kernel bug)" tags. Its repair, which appends this
constant, is the shape this row retires; see the note there.

## Repair shape

- **Class 2**: `unreachable!` or `debug_assert` per `[failure.taxonomy]` rows 4–5, answering row 0 first; the tests that pin the text move to `#[should_panic]`.

Re-check each `likely` or `unsure` argument before acting on it; one that fails moves the site to class 1.

## Second-pass sites

(Census second pass, as of main 69ab780194: `## Census, second pass` in the parent row.)

Doors: the Boolean doors, and the split doors through the finish and the join.

- `crates/topo/src/boolean/refusal_routes.rs` `BooleanDecision` ending, `Neighbours(PlaneRung::Norm)` (~1548). **Class 2, unsure**: a poisoned norm of a stored unit normal. No check was found that `set_face_surface` refuses a plane whose normal is not finite and non-zero before a caller's body reaches the Boolean.
- `refusal_routes.rs`, `PierceOnFace | SplitPointOnCircle` (~1570). **Class 2, unsure**: the pierce point is computed in f64, so an in-band residual at a scale near ε may be that computation's limit (class 3).
- `refusal_routes.rs`, `ArcSpan` (~1600). **Class 2, likely**: the edge's certification decided the same margin at the same band.
- `refusal_routes.rs`, `VolumeBackstop` (~1601). **Class 1, likely**: it is the escalated sibling of `BooleanError::ResultVolumeImplausible`, which the public `AtRestPolicy::gate_volume_backstop` reaches with a caller's result (the first pass's CLEAVE line). Only the Boolean's own call is a postcondition.
- `refusal_routes.rs`, `SelfCheck(_)` (~1701). **Class 2, likely**: a broken invariant, as the check's definite refusal says.
- `crates/topo/src/boolean/edge_join.rs` `JoinRefusal::render`, `Kernel` (~782). **Class 3, unsure**: the kill, re-description or re-mint it wraps runs Euler certification, and user geometry reaches certification's exact rows at a kernel build (SELECT's `edge-certification-exact-residuals-end-as-a-kernel-defect-at-the-attach-doors`, citing `work/germ/conic-section-just-past-its-band-fails-certification-downstream.md`, `work/lathe/the-guided-derived-frame-extrude-escalates-as-a-kernel-defect.md`, `work/cleave/a-boolean-corner-on-a-curved-wall-is-read-against-the-wall-not-its-section.md` and `work/cleave/rejoining-a-tangent-seam-cut-refuses-the-union.md`). **Fixed by PR 5528**: at a build it ends in `KERNEL_LIMIT_RECOURSE`; at rest and at import it is unchanged.
- `edge_join.rs` `JoinRefusal::render`, `ChartClass(Image)` (~765). **Class 2, unsure**: every caller joins a body it built, but the image is read off edges that the same certification limits reach. Left by PR 5528: the join re-derives an image the mint already certified, and `the-join-lever-names-a-pole-for-a-chart-class-reading` owns its verdict.
- `crates/topo/src/boolean/mod.rs` `BooleanError` `Display`, `JoinDesync` (~4814, "kernel bug or corrupt reduction"). **Class 1, sure**: the public `topo::graft_disjoint` (`instance.rs` ~105) returns it for a source that is not exactly one solid (its doc: "a caller error at THIS door"), `graft_disjoint_all_keyed` (~262) for an empty source, and the test `a_source_that_is_not_a_single_solid_refuses_typed` reaches it. Inside the Boolean it is the lockstep invariant.
- `boolean/mod.rs` `BooleanError` `Display`, `TornComponent` (~4867), `SeamOrientation` (~4923), `ZipCorrespondence` (~4926). **Class 2, likely**, each: the components, the seams and the zip are the op's own, built from operands that the operand gate holds to tier 2.
- `crates/topo/src/splitting/mod.rs` `SplitReduceError` `Display`, `ConsecutiveOnSectors` (~756). **Class 2, likely**: the coplanar gate rules it out.
- `crates/topo/src/splitting/finish.rs` `SplitFinishError` `Display`, `DegenerateSide` (~891), `TornComponent` (~896) and `Corrupt` (~903). **Class 2, likely**, each: the finish reads pieces it cut from a tier-1 body.
- `crates/topo/src/chord_join.rs` `SplitJoinError` `Display`, `CutInvariant` (~659) and `Corrupt` (~662). **Class 2, likely**, each: as above.

The prose arms are also listed by `kernel-bug-refusals-end-without-the-shared-ending` on this slate and by `work/issues/boolean-kernel-bug-refusals-end-without-the-shared-ending.md`. Those rows' repair, appending the constant, is the shape this row retires.

**Class 1** (`VolumeBackstop`): the volume decision's lever at the public door, shared with CLEAVE's repair of `ResultVolumeImplausible` (`classification-invariant-family-types-bug-only-states-against-d9-row-4`). The Boolean's own call panics.

**Class 1** (`JoinDesync`): split the graft doors' caller case into a variant of its own that names the caller's lever (pass a source holding exactly one solid, or a non-empty one); the Boolean's lockstep case stays class 2.

**Class 3** (`JoinRefusal::Kernel`): the last-resort recourse where certification's limit refused, as SELECT's row decides it. Do not panic.

**`JoinRefusal::Kernel` wraps more than certification** (ENCL, PR 5528's review). `JoinRefusal::of` (`boolean/edge_join.rs`) turns every `BooleanError` that is not `JoinUndecided` or `JoinCarrierUnsupported` into `Kernel`. Besides Euler certification, that covers the bug-only `JoinDesync` arms (`edge_join.rs` ~952, ~1096), `MixedSeam` from the pair rule over the joined span (~1017), and `ops::seam_refusal`'s lever rung (`BooleanError::of_lever_rung` on `LeverArm::Seam`) and `Escalated { SeamJet }` from the `OnPair` re-ask (~1015). The wrapper hides the seam lever those carry. PR 5528 keeps the last resort at a build as the one ending honest across the mix. Repair: split `Kernel` by cause, so the seam refusals surface their own lever, the bug-only arms take their own reading, and the certification limit keeps the last resort.

## The volume backstop left the operation path (2026-10-27)

PR 5842, under `[ev]` PR 5780 (the boolean's volume relations are a test oracle, `topo::test_support::volume_relations`), deleted the backstop: `BooleanError::{ResultVolumeImplausible, VolumeUnmeasured, VolumeCorrupt, VolumeUndecided}`, `BooleanDecision::VolumeBackstop`, `AtRestPolicy::gate_volume_backstop` and the `volume_backstop*` predicates are gone. Its `refusal_routes.rs` `VolumeBackstop` item is gone; the rest stand.
