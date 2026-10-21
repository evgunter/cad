---
id: a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids
kind: issue
title: workspace: 56 sites append KERNEL_DEFECT_ENDING to a typed refusal, and about 40 more end the same way through a helper or in their own words; each is either reachable by input (and the ending's claim is false) or reachable only by a kernel bug (and D2 says it must panic)
status: open
opened: 2026-10-19
priority: P3
cost: M
---


(Filed by the ENCL orchestrator, 2026-10-19. Held until `[ev]` PR 5134 settled what a contradiction past the import door is. It merged 2026-10-19.)

## What

`geom_core::predicate::KERNEL_DEFECT_ENDING` ("There is no way through: this is a kernel defect; report it") is the ending of "a refusal that only a kernel defect reaches". `rg -c KERNEL_DEFECT_ENDING crates` counts 113 mentions on main 74c567c570. The census below (main 5df6e6c854) finds 56 of them appending the ending to a typed refusal, plus a second-pass scope it lists unclassified: about 25 sites reaching the constant through a helper, and 18 typed arms that call themselves a kernel bug in their own words. The mention counts, by file, when the row was filed:

| Count | File |
|---|---|
| 9 | geom-brep `ssi.rs` |
| 8 | geom-brep `certify.rs` |
| 7 | topo `shell.rs` |
| 7 | topo `boolean/mod.rs` |
| 6 | geom-brep `offset_fit.rs` |
| 5 | editor-core `eval/mod.rs` |
| 4 | topo `boolean/ops.rs` |
| 4 | editor-core `names/emit.rs` |
| 4 | editor-core `eval/anchor.rs` |

`[failure]` (D2) splits states into two classes, and every site lies in one of them:

- **Input reaches it.** Then it is a typed refusal, but the ending's claim ("this is a kernel defect") is false and withholds the lever.
- **Only a kernel bug reaches it.** Then D2 says it MUST panic, and "downgrading one … to a typed error launders a bug into a supported outcome".

A typed error carrying this ending is therefore wrong whichever class the site is in. The exceptions, if any, are where the site cannot tell: a kernel approximation limit that `[tolerance.recourse]` gives a last-resort recourse. Those take that recourse, not this ending.

`[tolerance.recourse]` (iv), as revised by PR 5134, applies the same split to the sized decisions. The proven-zero implementation (`a-proven-zero-is-definite-yet-ends-as-no-size-at-rest`) does it for the `Unminted` side. This row is the rest of the tree.

## Shape of the work

- **Census.** One line per site: its error, its door, and which class it is in, with the reachability argument.
- **Class 2** becomes `unreachable!` / `debug_assert` per `[failure]`'s taxonomy, with its test rows moved to `#[should_panic]`.
- **Class 1** takes its decision's lever.
- **The import door** keeps the file's typed ending, as `KERNEL_OR_FILE_DEFECT_ENDING`'s at-rest sites do, until EXCH's import row closes.

The census probably splits by owner. Rows in another program's territory go to that program's slate with this row as parent. The constant itself (geom-core, SCALAR's territory) retires once no site needs it.

Related: `kernel-defect-endings-and-repair-labels-have-no-shared-home`.

## Census

(ENCL implementer, 2026-10-20.) **As of main 5df6e6c854, and not
guarded**: nothing checks it, so a site that appends the constant after
that commit is invisible to it until someone re-sweeps. One line per site
that appends `KERNEL_DEFECT_ENDING` (or `kernel_defect_ending!`) to a
typed refusal, grouped by owning program. Format: file `fn`/variant
(~line) — door(s) — class, confidence — the reachability argument.
Classes: **1** input reaches it; **2** only a kernel bug reaches it;
**3** a kernel approximation limit (`[tolerance.recourse]` last
resort); **4** the import door. **HELD** marks D10 territory: classified,
no child row. Each owning program's non-held sites are carried by the
child row named in its heading.

**Counts.** 56 appending sites: class 1 — 7; class 2 — 47 (5 of them
HELD); class 3 — 2; class 4 — 0. No site was left unclassified:
9 are `sure`, 39 `likely`, 8 `unsure`.

### ENCL — `crates/geom-brep` offset fit and patch bound (row `offset-fit-and-patch-bound-kernel-defect-endings-split-by-class`)

- `offset_fit.rs` `OffsetFitError` `Display`, `Fit(_)` (~764) — `fit_offset`, `fit_offset_at`, `approx_offset_surface[_at]`, and the shell/offset doors through `ReplaceFaceError::Fit` — **2**, unsure: the interpolation's samples and parameters are the refinement schedule's own over a face whose non-finite points refuse first (`NonFiniteSample`). Unsure for two live risks: the parameters' collapse in `normalized` under rounding, and a solve whose pivot is not certified.
- `offset_fit.rs` `OffsetFitError` `Display`, `Structure(_)` (~769) — same doors — **2**, unsure, possibly 1: `KnotVector::on_domain(lo, hi)` (geom-core `spline/knot_values.rs`) refuses breakpoints that rounding collapses, and a legal face on a domain far from the origin (u in [1e15, 1e15 + 1]) may collapse the averaged knots — `DerivedKnots`' family. Class 2 only once rounding is proven unable to collapse them.
- `offset_fit.rs` `OffsetFitError` `Display`, `MintLimb` (~875) — same doors — **2**, unsure: the certification re-derives the bound the loop accepted on the same fit, so a sample above it is a contradiction; the other arm, a round's residual coming out NaN, may be overflow on finite input (then 1).
- `patch_bound.rs` `PatchBoundError::note`, `RefinementFailed` (~201) — `pub fn patch_cells_refined(n, splits)` — **1**, sure: the test `colliding_split_points_do_not_zero_the_second_partial` records it on a valid face, which falsifies the variant doc's "insertion into a valid clamped vector is total" (~150).
- `patch_bound.rs` `PatchBoundError::note`, `QuotientRefused` (~211) — `patch_cells`, `patch_cells_refined` (rational arm), `integral_area_floor` (~938) — **2**, likely: every `None` of `cell_second_quotients` and `polynomial_area_pieces` is structural (geom-core `spline/derivative.rs`), and the gates before it exclude each.
- Every `PatchBoundError` arm above is also reached through mesh `nurbs_cert.rs` `face_err` (~151), which imported NURBS faces reach, and through `geom-brep/src/offset_meters.rs` (`MeterError::PatchBound`, ~807).
- `patch_bound.rs` `PatchBoundError::note`, `DerivedKnots` (~216) — `patch_cells_refined`, `offset_fit`'s integral lane (~2204, ~2215) — **1**, sure: the variant's own doc says a valid face with a span a few ulps wide reaches it.

### OFFSET — `crates/geom-brep/src/offset.rs` (row `offset-distance-kinds-differ-ends-as-a-kernel-defect`)

- `offset.rs` `OffsetDistanceError` `Display`, `KindsDiffer` (~539) — `pub fn geom_brep::offset_distance(from, onto)` — **1**, sure: any caller hands it two surfaces whose kinds do not pair (`tests/approx_surface.rs` and `tests/offset_mint.rs` do); only the shell op's own call (`topo/src/shell.rs` ~3831) holds the pairing.

### SHELF — the shell op and the offset doors (row `shell-and-replace-face-kernel-defect-endings-split-by-class`)

- `topo/src/shell.rs` `ShellError` `Display`, `Insert` (~736) — `topo::shell` — **2**, likely: the shell op certifies its own cavities before the insert door reads them.
- `shell.rs` `ShellError` `Display`, `Pcurve` (~756) — `topo::shell` — **3**, unsure: no gate before the closing mint certifies that a fitted offset carrier admits pcurves, so the pcurve fit's limit is not excluded. `PcurveMintError` also wraps `UnsupportedChart`/`IsoUnsupported` (`pcurve_cache.rs`), which may be D2 row 2 (not yet built, `NOT_YET_ENDING`) rather than a limit; decide per arm.
- `shell.rs` `ShellError` `Display`, `NotValid` (~767) — `topo::shell` — **2**, likely: a postcondition on the op's own result (`[failure]`: a check failing on the kernel's own result reports a bug). Probably reached today through the bug in `shelf/replace-face-transports-a-nurbs-edge-as-a-ruling`: a panic before that fix ships a reachable panic.
- `shell.rs` `AsShelled` `Display`, the six "asked" arms (`StaleFace`, `EmptyGroup`, `GroupChartsDiffer`/`TogetherChartMixed`, `SharedSurfaceKey`, `TogetherFaceRepeated`, `TogetherPartialSet`) (~808) — `topo::shell` — **2**, sure: the shell op composes every offset-door call itself.
- `shell.rs` `AsShelled` `Display`, `Fit { InvalidRequest }` (~894) — `topo::shell` — **2**, sure: the thickness gate passes only a finite positive wall and the tolerance is the run's witness.
- `shell.rs` `AsShelled` `Display`, `Pcurve` (~899) — `topo::shell` — **3**, unsure: as `ShellError::Pcurve`, including its per-arm row-2 question.
- `topo/src/replace_face.rs` `ReplaceFaceError` `Display`, `Structure` (~843) — the offset/replace-face doors — **2**, likely: moving a valid spline's control points keeps its knots and degree.
- `replace_face.rs` `ReplaceFaceError` `Display`, `ResultNotClosed` (~983) — the offset/replace-face doors — **2**, likely: a postcondition on the door's own result. It has a known input trigger through a kernel bug, `shelf/replace-face-transports-a-nurbs-edge-as-a-ruling`; converting it to a panic before that fix ships a reachable panic.

### TOPO — `crates/topo/src/merge_faces.rs` (row `merge-faces-undescribed-ending-is-a-typed-kernel-defect`)

- `merge_faces.rs` `undescribed_ending` (~799, ~800; one site, two arms) — `merge_coplanar_faces`, and the Boolean's `EdgeUndescribed` — **2**, likely: the kernel constructed an exact carrier, or its walk broke; the fitted arm takes the certification table's ending where it has one.

### RESTFRONT — `crates/topo/src/validate.rs` (row `result-not-at-rest-renders-an-empty-findings-list`)

- `validate.rs` `write_result_not_at_rest`, the `[]` arm (~5943) — the Boolean's `ResultInvalid` and the split's `FinishError::ResultNotAtRest` (`finish.rs` ~879) — **2**, sure: every caller passes the gate's findings, which are non-empty when it refuses; row 0 applies (a non-empty findings type).

### CLEAVE — `crates/topo/src/boolean/mod.rs` (evidence on `classification-invariant-family-types-bug-only-states-against-d9-row-4`)

- `BooleanError` `Display`, `SplitLineage` (~4819) — `topo::boolean::{union, subtract, intersect}` — **2**, likely: the split records are the Boolean's own.
- `BooleanError` `Display`, `LineageRow` (~4823) — same — **2**, likely: `lineage::RowRefusal` is refused at append, on rows the op itself writes.
- `BooleanError` `Display`, `ResultVolumeImplausible` (~4937) — the Boolean, and the public `topo::AtRestPolicy::gate_volume_backstop(op, a, b, result, band, tol)` (`props.rs`, re-exported from `lib.rs`), which takes a caller's `result` — **1**, sure: `props.rs`'s test (~5355) plants one and gets this refusal. Only the Boolean's own call (`ops.rs` ~1005) is a kernel postcondition, so the repair splits the public door's input case from it.
- `BooleanError` `Display`, `VolumeCorrupt { operand: None }` (~4953) — the same two doors — **1**, likely: through `gate_volume_backstop` the "result" is the caller's body, whose mass properties may be corrupt; the Boolean's own call is the class-2 postcondition. The `Some` arm already names the file.
- `BooleanError` `Display`, `EdgeInFace { Crossed | TwinIncident }` (~5001) — same — **2**, likely: a postcondition on the result's edges.
- `BooleanError` `Display`, `ContactContradicted` with a `Tangent` declaration (~4585) — same, plus the glue door — **2**, unsure (the variant's comment: a glued tangency is the Boolean's own reading) — **HELD** (contact records).

### HONE — the split, the join and the piece sort (row `split-and-join-kernel-defect-endings-are-bug-only-typed-refusals`)

- `topo/src/chord_join.rs` `SplitJoinError` `Display`, `LineageRow` (~668) — the split and Boolean doors through the join — **2**, likely: rows the join itself appends.
- `topo/src/splitting/mod.rs` `SplitError` `Display`, `LineageRow` (~792) — the split doors — **2**, likely: as above.
- `topo/src/splitting/containment.rs` `PointInLoopError::render_as`, `CorruptLoop` (~492) — the split and Boolean containment reads — **2**, likely: a torn body, and tier 1 holds at every public door (`[failure]`'s closure property).
- `topo/src/splitting/finish.rs` `FinishError` `Display`, `NestingContradiction` (~839) — the split doors — **2**, likely: the variant's doc — outlines decided disjoint cannot each enclose the other.
- `finish.rs` `FinishError` `Display`, `SlitUnplaced { in_hole: false }` (~848) — the split doors — **2**, likely: the variant's doc — a section region of the side covers the slit's edges.
- `finish.rs` `FinishError` `Display`, `SlitUnplaced { in_hole: true }` (~858) — the split doors — **2**, likely: as above.
- `topo/src/splitting/section.rs` `SectionError` `Display`, `NestingContradiction` (~250) — the section door — **2**, likely: as `finish.rs`'s.
- `section.rs` `SectionError` `Display`, `Corrupt` (~255) — the section door — **2**, likely: a traversal over a tier-1 body does not fail.
- `topo/src/pieces.rs` `PieceSortError` `Display`, `Move` (~169) — the Boolean's and the split's piece sort (`BooleanError::Pieces`) — **2**, likely: moving a piece the sort just found into its own solid. (`pieces.rs` is in no program's territory; HONE's charter is the nearest.)

### CARVEREST — `crates/sweep/src/swept.rs` (row `cap-plane-refusals-end-as-typed-kernel-defects`)

- `swept.rs` `CapPlaneError` `Display`, `Orientation` (~795) — the extrude/revolve/sweep/loft verbs' `CapPlane` — **2**, likely: the type's doc — unreachable from a validated profile, whose winding is certified on the same plane.
- `swept.rs` `CapPlaneError` `Display`, `EdgeOn` (~801) — same — **2**, likely: as above.

### PATHS — `crates/profile/src/validate.rs` (row `constructed-tangent-joint-refusals-are-typed-kernel-defects`)

- `validate.rs` `ProfileError` `Display`, `TangentJointOutOfRange` (~1394) — `Profile::validate` over a constructed loop — **2**, likely: the lattice never mints such a joint; only `ConstructedLoop::fixture` (`path/program.rs`, behind `cfg(any(test, feature = "test-support"))`, absent from shipped builds) builds one.
- `validate.rs` `ProfileError` `Display`, `TangentJointOnFullTurn` (~1400) — same — **2**, likely: as above.

### LIB — `crates/pncad/src/workspace.rs` (row `workspace-pin-refusal-ends-as-a-typed-kernel-defect`)

- `workspace.rs` `resolve_recourse`, `WorkspaceError::Pin` (~717) — the workspace's open and resolution scans — **2**, likely: a pin over a document that has just loaded fails only on a serializer defect.

### DOCTAIL — the document model (row `part-and-refactor-kernel-defect-endings-split-by-class`)

- `editor-core/src/eval/parts.rs` `PartFault` `Display`, `PartProduct` where `product_recourse` says `KernelDefect` (~426) — evaluation of an instantiate node — **2**, likely, for `EvaluationOfAnotherDocument`, `UnknownNode`, `RootFailed`, `RootPoisoned`; its `ContactLineage` kind is **HELD** (contact records).
- `parts.rs` `PartFault` `Display`, `ReferenceCycle` (~445) — evaluation with a caller's `PartResolver` (a `pub trait` the `EvalOptions` carry) — **1**, sure (editor-core `tests/part_depth_bound.rs` ~266 builds the unpinned resolver): the evaluator does not check that the resolved document hashes to its pin (`parts.rs` `.resolve(doc_ref, tol)`, ~1031), so a resolver that returns a document referring back closes a loop; the "a store that checks pins cannot hold a loop" premise holds only for the pncad store.
- `parts.rs` `PartFault` `Display`, `NotEntered` (~457) — same — **2**, unsure: the census pass enters every part before the run reads it, but the `enter_census` doc (~632) says a cancel leaves rows that refuse `NotEntered`; a conversion must prove nothing reads after a cancel.
- `editor-core/src/refactor.rs` `RefactorError` `Display`, `Pin` (~1203) — the split-part refactor — **2**, likely: the part replayed through the edit doors and the save validator agrees.
- `refactor.rs` `ReplayTail` `Display`, the `defect` closure (~1746; one site, the arms listed under it) — the split-part and inline refactors — **2**, likely: each replayed edit re-writes what a validated document already holds.
- `editor-core/src/edit.rs` `step_ids_recourse`, `NotMinted` (~3254) — `EditError::StepIdsRefused` — **2**, likely: no edit door raises it (`InsertNode` raises `Preminted`, `SetProgram` the shape and keep arms; the load door renders `SnapshotError::StepIds` itself); row 0 applies.
- `edit.rs` `EditError` `Display`, `InvalidDistribution { NonFinite }` (~4186) — the variable edit doors — **2**, likely: the edit door routes a non-finite offset to `NonFiniteVar` — **HELD** (parameters).
- `editor-core/src/range.rs` `RangeError` `Display`, `Derivation` (~542) — the range certification door — **2**, likely: each derived edit re-writes a held value or declares one over a validated seed — **HELD** (parameters).

### WIRE — the evaluation seat (row `eval-wiring-kernel-defect-endings-are-bug-only-typed-refusals`)

- `editor-core/src/eval/mod.rs` `NodeErrorKind` `Display`, `ProfileAnchor` (~2280) — evaluation of a profile node — **2**, likely: validate's canonical form is an exact reindexing of its input.
- `eval/mod.rs` `NodeErrorKind` `Display`, `ProfileRecordMissing` (~2289) — the guided lift over a parameter box — **2**, likely — **HELD** (parameters). Holding it is conservative: the record is a replay record of WIRE's profile lift, not a variable.
- `eval/mod.rs` `NodeErrorKind` `Display`, `MissingSlot` (~2480) — every verb node's slot reads (`eval/wire.rs` `need_scalar` and kin) — **2**, likely: `Node::slots` and the wire agree.
- `eval/mod.rs` `NodeErrorKind` `Display`, `VerbArity` (~2496) — the run doors — **2**, likely: the per-verb correspondences and the run doors agree.
- `editor-core/src/eval/anchor.rs` `PiecesFault` `Display`, `NoRecord` (~190), `NoIds` (~195), `Length` (~205), `NoStepId` (~211) — evaluation of a profile node (`NodeErrorKind::ProfilePieces`) — **2**, likely, each: the naming anchor, the replay record and the minted step ids are built by one evaluation of one program.

### REFS — names and reads (row `read-body-index-and-emission-kernel-defect-endings-split-by-class`)

- `editor-core/src/names/interrogate.rs` `InterrogateError` `Display`, `NoSuchBody` (~183) — `pub fn Evaluation::read_body(var, index)` (`eval/mod.rs` ~255) and the interrogation doors — **1**, sure: the index is the caller's, and any index past the node's outputs reaches it.
- `editor-core/src/names/emit.rs` `Category::ending`, `Emission` (~659; one site, nine variants: `Duplicate`, `EntityNamed`, `KindMismatch`, `ShortTie`, `Unnamed`, `MissingUpstream`, `Emission`, `SplitLineage`, `Settle`) — every node's name emission — **2**, unsure: the type's own category 1, "an emission BUG"; about 151 raisers were not re-derived here.

### MSOLVE — placement (no row: HELD)

- `editor-core/src/mate/reach.rs` `Display`, `MalformedBody` (~118) — the mate solve's reach — **2**, likely: a face whose surface resolves to nothing is a tier-1 break — **HELD** (placement).

### Mentions that append nothing

- `geom-core/src/predicate.rs` `KERNEL_DEFECT_ENDING` (~2245) and `kernel_defect_ending!` — the definition.
- `geom-core/src/predicate.rs` `Culprit::ending`, `Culprit::Kernel` (~1457) — the helper channel's root (second pass below).
- `geom-core/src/lib.rs` (~58) — the re-export.
- `geom-core/README.md` (~258) — prose.
- `topo/src/lib.rs` `test_support::offer_key` (~632) — reads the ending to sort refusals; appends nothing.
- doc comments and `use` lines in `predicate.rs`, `offset.rs`, `offset_fit.rs`, `shell.rs`, `boolean/mod.rs`, `splitting/containment.rs`.
- tests that assert the text: `geom-brep/src/ssi.rs` (9), `certify.rs` (9), `ssi/march.rs` (2), `offset_fit.rs` (2), `patch_bound.rs` (2); `topo/src/boolean/ops.rs` (3), `boolean/mod.rs` (1), `refusal_routes.rs` (2), `insert.rs`, `join.rs`, `vtxfac.rs`, `lineage.rs`, `validate.rs` (2), `pieces.rs`, `chart_region.rs`, `chord_join/section_end_rows.rs`; `editor-core/src/names/emit.rs` (3); `editor-core/tests/refusal_concision_chains.rs` (3), `tests/display_contract.rs`; `step-import/tests/tier_gate.rs`.

### Second pass: the channels the constant's name cannot match

Not classified here; they are this row's remaining scope.

- **The helper channel**, which reaches the constant through `geom_brep::recourse::defect_ending(Reading::Build)`, `Unsized::Defect.recourse(_, Reading::Build)` or `Culprit::Kernel`: `geom-brep/src/ssi.rs` `SsiError::ending` (~1298, ~1341, ~1368, ~1376, ~1469, ~1472), `TraceDecision::ending` (~2265, ~2273), the step cap (~2021); `ssi/exhaust.rs` (~456); `props/mod.rs` (~643); `pcurve_cache.rs` (~2419); `certify.rs` `ending` (~1350, ~1351, ~1354); `topo/src/merge_faces.rs` (~1108, ~1114); `topo/src/props.rs` (~3644); `topo/src/validate.rs` `PLANAR_CORNER` (~2682); `topo/src/chart_region.rs` (~435); `topo/src/boolean/refusal_routes.rs` (~1548, ~1570, ~1600, ~1601, ~1701). About 25 sites. At a build each ends exactly as an appended constant does. Added by PR 5464 (2026-10-20): `geom-brep/src/edge_nurbs.rs` `PlaneNurbsRefusal::ending`, `PcurveFit` (the plane × NURBS lane's chart-image interpolation through the schedule's foot points), which at a build now ends in `KERNEL_DEFECT_ENDING` through `recourse::defect_ending(at)`. Its doc gives two causes: a non-finite foot, which is bug-only (class 2), and a collocation system the interpolation cannot solve, which is the interpolation's limit (class 3, the last resort). The same PR's second fix pass made three more types' `Display` read a build through `recourse::defect_ending`, and each arm below now ends in `KERNEL_DEFECT_ENDING` there, not `KERNEL_OR_FILE_DEFECT_ENDING`. First, `topo/src/boolean/edge_join.rs` `JoinRefusal`, `Kernel` and `ChartClass(Image)`: the shell, merge-faces, offset and split doors, each joining a body it built; class 2, likely. Second, `topo/src/transform.rs` `TransformError`, `NullScaffold` and `Corrupt`: the transform door; class 2, likely, since tier 1 and tier 2 hold at every public door.
- **The prose channel**: typed `Display` arms that call themselves a kernel bug in their own words, without the constant: `profile/src/path.rs` `UnderdeterminedLeg`, `OverdeterminedJunction`; `geom-brep/src/intersect.rs` `WrongLane`; `topo/src/splitting/mod.rs` `ConsecutiveOnSectors`; `splitting/finish.rs` `DegenerateSide`, the both-sides component arm, `Corrupt`; `boolean/mod.rs` `JoinDesync`, `TornComponent`, `SeamOrientation`, `ZipCorrespondence`; `chord_join.rs` `CutInvariant`, `Corrupt`; `transform.rs` `Corrupt`; `sweep/src/blend/mod.rs` `SurgeryInvariant`; `sweep/src/loft.rs` `SeamStructure`, `SectionStructure`; `editor-core/src/resolve/hit.rs` `UnnamedEntity`. The same pattern's other hits are panic messages (`debug_assert!`/`unreachable!` postconditions in `surgery.rs`, `revert.rs`, `euler.rs`, `extrude.rs`, `revolve/`, `loft.rs`, `blend/surgery.rs`, `combine.rs`, `instance.rs`), which are already what `[failure]` asks for.
- **What neither pass can see**: a refusal whose text names no defect at all yet is reachable only by a bug, and a sentence assembled at run time from parts.
