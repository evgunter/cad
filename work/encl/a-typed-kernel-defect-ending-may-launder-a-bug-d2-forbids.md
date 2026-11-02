---
id: a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids
kind: issue
title: workspace: 56 sites append KERNEL_DEFECT_ENDING to a typed refusal, and 51 more end the same way through a helper or in their own words; each is either reachable by input (and the ending's claim is false) or reachable only by a kernel bug (and D2 says it must panic)
status: open
opened: 2026-10-19
priority: P3
cost: M
---


(Filed by the ENCL orchestrator, 2026-10-19. Held until `[ev]` PR 5134 settled what a contradiction past the import door is. It merged 2026-10-19.)

## What

`geom_core::predicate::KERNEL_DEFECT_ENDING` ("There is no way through: this is a kernel defect; report it") is the ending of "a refusal that only a kernel defect reaches". `rg -c KERNEL_DEFECT_ENDING crates` counts 113 mentions on main 74c567c570. The census below (main 5df6e6c854) finds 56 of them appending the ending to a typed refusal. The second pass (main 69ab780194) finds 51 more typed refusals that end by naming the kernel alone without appending it: 29 reach the constant through a helper (`recourse::defect_ending(Reading::Build)`, `Unsized::Defect`, `Culprit::Kernel`), and 22 say so in their own words. The mention counts, by file, when the row was filed:

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

- `editor-core/src/eval/parts.rs` `PartFault` `Display`, `PartProduct` where `product_recourse` says `KernelDefect` (~426) — evaluation of an instantiate node — **2**, likely, for `EvaluationOfAnotherDocument`, `UnknownNode`, `ReadFailed`, `ReadPoisoned`; its `ContactLineage` kind is **HELD** (contact records).
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

The scope the first pass named. `## Census, second pass` below classifies it, and what it adds.

- **The helper channel**, which reaches the constant through `geom_brep::recourse::defect_ending(Reading::Build)`, `Unsized::Defect.recourse(_, Reading::Build)` or `Culprit::Kernel`: `geom-brep/src/ssi.rs` `SsiError::ending` (~1298, ~1341, ~1368, ~1376, ~1469, ~1472), `TraceDecision::ending` (~2265, ~2273), the step cap (~2021); `ssi/exhaust.rs` (~456); `props/mod.rs` (~643); `pcurve_cache.rs` (~2419); `certify.rs` `ending` (~1350, ~1351, ~1354); `topo/src/merge_faces.rs` (~1108, ~1114); `topo/src/props.rs` (~3644); `topo/src/validate.rs` `PLANAR_CORNER` (~2682); `topo/src/chart_region.rs` (~435); `topo/src/boolean/refusal_routes.rs` (~1548, ~1570, ~1600, ~1601, ~1701). About 25 sites. At a build each ends exactly as an appended constant does. Added by PR 5464 (2026-10-20): `geom-brep/src/edge_nurbs.rs` `PlaneNurbsRefusal::ending`, `PcurveFit` (the plane × NURBS lane's chart-image interpolation through the schedule's foot points), which at a build now ends in `KERNEL_DEFECT_ENDING` through `recourse::defect_ending(at)`. Its doc gives two causes: a non-finite foot, which is bug-only (class 2), and a collocation system the interpolation cannot solve, which is the interpolation's limit (class 3, the last resort). The same PR's second fix pass made three more types' `Display` read a build through `recourse::defect_ending`, and each arm below now ends in `KERNEL_DEFECT_ENDING` there, not `KERNEL_OR_FILE_DEFECT_ENDING`. First, `topo/src/boolean/edge_join.rs` `JoinRefusal`, `Kernel` and `ChartClass(Image)`: the shell, merge-faces, offset and split doors, each joining a body it built; class 2, likely. Second, `topo/src/transform.rs` `TransformError`, `NullScaffold` and `Corrupt`: the transform door; class 2, likely, since tier 1 and tier 2 hold at every public door.
- **The prose channel**: typed `Display` arms that call themselves a kernel bug in their own words, without the constant: `profile/src/path.rs` `UnderdeterminedLeg`, `OverdeterminedJunction`; `geom-brep/src/intersect.rs` `WrongLane`; `topo/src/splitting/mod.rs` `ConsecutiveOnSectors`; `splitting/finish.rs` `DegenerateSide`, the both-sides component arm, `Corrupt`; `boolean/mod.rs` `JoinDesync`, `TornComponent`, `SeamOrientation`, `ZipCorrespondence`; `chord_join.rs` `CutInvariant`, `Corrupt`; `transform.rs` `Corrupt`; `sweep/src/blend/mod.rs` `SurgeryInvariant`; `sweep/src/loft.rs` `SeamStructure`, `SectionStructure`; `editor-core/src/resolve/hit.rs` `UnnamedEntity`. The same pattern's other hits are panic messages (`debug_assert!`/`unreachable!` postconditions in `surgery.rs`, `revert.rs`, `euler.rs`, `extrude.rs`, `revolve/`, `loft.rs`, `blend/surgery.rs`, `combine.rs`, `instance.rs`), which are already what `[failure]` asks for.
- **What neither pass can see**: a refusal whose text names no defect at all yet is reachable only by a bug, and a sentence assembled at run time from parts.

## Census, second pass

(ENCL implementer, 2026-10-21.) **As of main 69ab780194, and not
guarded**, as the first pass is. One line per typed refusal that ends by
naming the kernel alone without appending the constant: through a
helper that picks `KERNEL_DEFECT_ENDING` at a build
(`recourse::defect_ending(Reading::Build)`, `Unsized::Defect`, or
`Culprit::Kernel` through `Reading::culprit`), or in its own words
("(kernel bug)", "this is a kernel bug", "report the body"). A helper
site counts only where it renders at a build; one read at rest ends in
`KERNEL_OR_FILE_DEFECT_ENDING` and is out of scope (`validate.rs`
`PLANAR_CORNER` and `geom-brep/src/props/mod.rs` `PropsCheck::Exact`,
whose `Display` reads `Reading::AtRest`). Format and classes as the
first pass, plus **5**: valid but not yet built (`[failure.taxonomy]`
row 2, `NOT_YET_ENDING`). Grouped by owning program; each heading names
the row that carries its non-held sites.

**Counts.** 51 sites (29 helper, 22 prose): class 1 — 11; class 2 — 37
(3 of them HELD); class 3 — 2; class 5 — 1. Confidence: 3 `sure`,
32 `likely`, 16 `unsure`. Each class-2 line names its invariant and
who establishes it; where no public door could be ruled out, the line
says `unsure`.

### SSIMARCH — `crates/geom-brep/src/ssi.rs`, `ssi/exhaust.rs` (row `ssi-kernel-defect-endings-split-by-class`)

These render only through the public `SsiError::render` / `ending`
(a caller of `cylinder_sphere_ssi`, `plane_nurbs_ssi`,
`trace_plane_nurbs_uncertified` or `idealized_trace_r3`); no shipped
caller renders them (`topo/src/offset_derive.rs` prints the payload,
`edge_nurbs.rs` re-types the arms it keeps).

- `ssi.rs` `SsiError::ending`, `MarchTolMismatch` (~1298) — **2**, likely: `seam_tol` is private and its one caller passes the `MarchTol` that `MarchTol::from_band` minted from the run band.
- `SsiError::ending`, `StepUnusable { SpeedUnusable }` (~1341) — **2**, likely: the chart mint refuses a degenerate chart before any march.
- `SsiError::ending`, `Fit(TooFewPoints)` (~1368) — **2**, likely: refinement hands the fit its samples or refuses, and a non-finite sample refuses as `NonFinitePoint` first.
- `SsiError::ending`, `Fit(InvalidTolerance | KnotAlgebra | RaggedRows | BudgetExhausted)` (~1376) — **2**, likely: the interpolation the march calls runs none of them.
- `SsiError::ending`, `TubeProbeSilent { refused: 0 }` and `TubeDegenerate(PcurveTangentUnusable)` (~1469) — **2**, unsure: a fitted cubic's tangent that vanishes may be the fit's limit (3), not a bug.
- `SsiError::ending`, `ChartRow` (~1472) — **2**, likely: a NURBS wall that validated re-wraps every boundary row; the spline layer refuses a weight that is not strictly positive when the net is built (geom-core `spline/knots.rs`).
- `step_budget_recourse`, `StepBound::Cap(StepCap::Diagonal)` (~2045; reached by `StepBudget` and `RefinementExhausted`) — **2**, unsure: the diagonal ends an open branch in one step, but a closed branch held at that cap is not argued.
- `TraceDecision::ending`, a poisoned margin on `StepProgress | ClosureTangent | BranchOpenEnd` (~2288) — **2**, unsure: operands are refused only when not finite, so finite operands at extreme magnitude may overflow a margin. If they can, this is 1 and takes the range lever.
- `TraceDecision::ending`, `BoundarySection` (~2296) — **2**, unsure: the same overflow question.
- `ssi/exhaust.rs` `FloorRefusal::ending`, `DomainUnreadable` (~456) — **1**, likely: `SsiDomain::check` tests each field for finiteness, but `Box3::around(center, half_extent)` can overflow. A transversal `cylinder_sphere_ssi` with `center.x = 1e308` and `half_extent = 1e308` passes `check`, and `SweepFloor::r3` then reads an infinite reach. The variant's doc ("every SSI door checks the caller's domain first") misses the sum. The lever is `DomainUnusable`'s.
- A second laundering channel, not counted as a site: `edge_nurbs.rs` `of_tube` (~1609) and `refusal` (~2077) re-type `TubeProbeSilent { refused: 0 }` and a catch-all `_` into `Unsupported`, the not-yet-built row. Those arms are repaired with the SSIMARCH sites above.

### SELECT — edge certification's ending table (row `edge-certification-exact-residuals-end-as-a-kernel-defect-at-the-attach-doors`)

- `certify.rs` `CertCheck` ending, `Ending::Residual(Unsized::Defect)` for `EndpointStart | EndpointEnd | WitnessSurfaceResidual | WitnessMidpoint | TangentParallel | TangentGap | MappedSource | ChartResidual` (~1347; one site, eight checks) — every certification read at a build, including the Euler attach doors, whose `EulerOpError` `Display` reads `Reading::Build` (`topo/src/euler.rs` ~1808) — **1**, sure: `topo/tests/geometric_cube.rs` `wrong_cache_is_rejected_at_attachment` hands `Body::mev` a carrier 100ε off its endpoint and gets `ResidualExceeded { EndpointStart }`, which ends "this is a kernel defect". User geometry reaches the same rows at the kernel's own builds too, so no door makes them class 2. `germ/conic-section-just-past-its-band-fails-certification-downstream`, `carve/the-guided-derived-frame-extrude-escalates-as-a-kernel-defect`, `cleave/a-boolean-corner-on-a-curved-wall-is-read-against-the-wall-not-its-section` and `cleave/rejoining-a-tangent-seam-cut-refuses-the-union` measure a split, an extrude and Booleans refusing `EndpointStart`/`EndpointEnd` (80 runs at 1e-12 in the germ row). There the construction's error outgrows the band, which is class 3 (the germ row's argument) or class 1 where a lever moves it. The repair splits by origin (`topo::policy_lane::Origin { Callers, Kernels }`), and neither side panics.
- `certify.rs`, `ChartImage` → `Unsized::Defect` (~1348) — same doors — **1**, likely: a caller's `EdgeCurveSpec` carries its own description, so its chart image is the caller's at the attach doors. At a kernel build it is unsure, for the reason the exact rows above give.
- `certify.rs`, `PlaneNurbsReportedTransversality` → `Unsized::Defect` (~1351) — the plane × NURBS lane at a build — **2**, likely: a fold over samples that each decided transverse; the per-sample transversality decision, earlier in the same lane, establishes it.
- `edge_nurbs.rs` `PlaneNurbsRefusal::ending`, `PcurveFit` (~376) — the plane × NURBS lane at a build — **3**, likely, with a class-2 arm. The variant gives two causes: a collocation the interpolation cannot solve (its limit, 3) and a non-finite foot (2). The repair splits the variant.

### PCTAIL — `crates/geom-brep/src/pcurve_cache.rs` (row `pcurve-winding-incidence-and-sector-refusals-end-as-a-kernel-defect`)

- `PcurveCheck::routing`, `ChartWinding | ChartIncidence | Sector` → `Unsized::Defect` (~2439), rendered at a build by `PcurveCertifyError`'s `Display` (~2310). It covers `PlacementTurnsExceeded` and the escalations of all three checks. Doors: every pcurve mint at a build, including `EulerOpError::PcurveMint` over a caller's carrier at `mev`/`mef`. **1**, unsure: these checks read a carrier the caller may have supplied, and nothing excludes an in-band incidence or sector read for a carrier that edge certification passed.

### TOPO — `crates/topo/src/merge_faces.rs` (row `merge-faces-undescribed-ending-is-a-typed-kernel-defect`)

- `MergeDecision::ending`, `DeclaredPlanes(Parallel | Norm)` and `DeclaredOffset` (~1108) — `merge_coplanar_faces` — **2**, likely: the declared rung bridges in-band parallelism, so only an unreadable norm escalates — **HELD** (declared pairs).
- `MergeDecision::ending`, `PairReach` (~1114) — `merge_coplanar_faces` — **2**, unsure: the merge reads a caller's body under a build reading. An extent that does not read over stored geometry is the file's at rest, so the reading may be mislabelled rather than the state bug-only.

### FLUX — `crates/topo/src/props.rs` (row `shell-classification-volume-refusals-end-as-a-kernel-defect`)

- `ShellClassifyError::recourse`, `Escalated | ZeroVolume | Straddles` → `Unsized::Defect` (~3718), rendered at a build by `Display` through `ending()` (~3727). Doors: the public `classify_shells`, `classify_shells_structural` and `classify_shells_of` over a caller's body, plus the shell op, the piece sort and `stands`. **1**, likely: those doors take any tier-1 body. That includes a flat shell built through the Euler doors, or a sliver void a subtract leaves, whose certified volume enclosure holds zero. The geometry is the user's, so the lever is to move it. The arm's "(D10)" premise (`props.rs` ~3678) is stale.

### CHART — `crates/topo/src/chart_region.rs` (no row: HELD)

- `ChartRegionError` `Display`, `Escalated` on a poisoned margin (~435) — the face–face contact record's overlap read — **2**, unsure (overflow on finite trims) — **HELD** (contact records).
- `ChartRegionError` `Display`, `Corrupt` (~460, prose: "a kernel invariant … report this") — same — **2**, likely: tier 1 holds at every door — **HELD** (contact records).

### HONE — the Boolean, split and join (row `split-and-join-kernel-defect-endings-are-bug-only-typed-refusals`)

- `boolean/refusal_routes.rs` `BooleanDecision` ending, `Neighbours(PlaneRung::Norm)` (~1548) — the Boolean doors and `merge_coplanar_faces` — **2**, unsure: a poisoned norm of a stored unit normal; no check was found that `set_face_surface` refuses a plane whose normal is not finite and non-zero before a caller's body reaches these doors.
- `refusal_routes.rs`, `PierceOnFace | SplitPointOnCircle` (~1570) — the Boolean doors — **2**, unsure: the pierce point is computed in f64, so an in-band residual at a scale near ε may be that computation's limit (3).
- `refusal_routes.rs`, `ArcSpan` (~1600) — the Boolean doors — **2**, likely: the edge's certification decided the same margin at the same band.
- `refusal_routes.rs`, `VolumeBackstop` (~1601) — the Boolean, and the public `AtRestPolicy::gate_volume_backstop` over a caller's result — **1**, likely: the escalated sibling of the first pass's `ResultVolumeImplausible` (CLEAVE, class 1 at the public door). Only the Boolean's own call is the class-2 postcondition.
- `refusal_routes.rs`, `SelfCheck(_)` (~1701) — the Boolean doors — **2**, likely: a broken invariant, as the check's definite refusal says.
- `boolean/edge_join.rs` `JoinRefusal::render`, `Kernel` (~782) — the shell, merge-faces, offset and split doors — **3**, unsure: the kill, re-description or re-mint it wraps runs Euler certification, which the SELECT line above shows user geometry reaching at a kernel build. **Fixed by PR 5528**: at a build it ends in `KERNEL_LIMIT_RECOURSE`; at rest and at import it is unchanged.
- `edge_join.rs` `JoinRefusal::render`, `ChartClass(Image)` (~765) — same doors — **2**, unsure: every caller joins a body it built, but the chart image is read off edges that the same certification limits reach.
- `boolean/mod.rs` `BooleanError` `Display`, `JoinDesync` (~4814, "kernel bug or corrupt reduction") — the Boolean doors, and the public `topo::graft_disjoint` (`instance.rs` ~105) and `graft_disjoint_all_keyed` (~262) — **1**, sure: `graft_disjoint` returns it for a source that is not exactly one solid (its doc: "a caller error at THIS door"), `graft_disjoint_all_keyed` for an empty source, and the test `a_source_that_is_not_a_single_solid_refuses_typed` reaches it. The repair splits the caller's case from the Boolean's lockstep invariant.
- `BooleanError` `Display`, `TornComponent` (~4867), `SeamOrientation` (~4923), `ZipCorrespondence` (~4926) — the Boolean doors — **2**, likely, each: the components, seams and zip are the op's own, built from operands the operand gate holds to tier 2.
- `splitting/mod.rs` `SplitReduceError` `Display`, `ConsecutiveOnSectors` (~756) — the split doors — **2**, likely: the coplanar gate rules it out.
- `splitting/finish.rs` `SplitFinishError` `Display`, `DegenerateSide` (~891), `TornComponent` (~896), `Corrupt` (~903) — the split doors — **2**, likely, each: the finish reads the pieces it cut, over a tier-1 body.
- `chord_join.rs` `SplitJoinError` `Display`, `CutInvariant` (~659) and `Corrupt` (~662) — the split and Boolean doors — **2**, likely, each: as above.

### SHELF — `crates/topo/src/transform.rs` (row `shell-and-replace-face-kernel-defect-endings-split-by-class`)

- `TransformError::render`, `NullScaffold` (~202) — `topo::transform_rigid` — **1**, likely: the public `mvfs` → `set_face_surface(plane)` → `Body::mev_null(MevSite::Lone)` leaves a null-scaffold edge, and the tree treats scaffolding as the caller's unfinished edit elsewhere (`Unfinished::SCAFFOLDING_REFUSAL`, blend's breach arm, mesh `NullScaffoldEdge`). Its lever is to finish the edit first. PR 5464 made its build-side `Display` end in `KERNEL_DEFECT_ENDING`. **Fixed by PR 5528**: at a build it ends in `Unfinished::SCAFFOLDING_RECOURSE`; at rest and at import it keeps the door's defect ending.
- `TransformError::render`, `Corrupt` (~227) — same — **2**, likely: a dangling key is a tier-1 break, and tier 1 holds at every public door (`[failure]`'s closure property).

### CARVEREST — `crates/sweep/src/loft.rs` (row `cap-plane-refusals-end-as-typed-kernel-defects`)

- `LoftError` `Display`, `SeamStructure` (~295) — the loft verbs — **2**, likely: a wall that validated re-wraps.
- `LoftError` `Display`, `SectionStructure` (~300) — the loft verbs — **2**, likely: `validate_loft` refuses counts and mismatched sections first (`SkinError::SectionShapeMismatch`), and a loopless section never leaves profile validation (`stacking_fold`'s doc).

### BAND — `crates/sweep/src/blend/mod.rs` (row `blend-surgery-invariant-ends-in-a-tag`)

- `BlendError` `Display`, `SurgeryInvariant` (~2089) — the fillet and chamfer verbs — **2**, likely: the surgery contradicts its own earlier steps.

### PATHS — `crates/profile/src/path.rs` (row `constructed-tangent-joint-refusals-are-typed-kernel-defects`)

- `PathError` `Display`, `UnderdeterminedLeg` (~2170) — `Profile` path elaboration — **2**, unsure: the elaborator's backstop for bindings the typed surface guarantees. It has 14 raisers, in `path/family.rs`, `path/onto.rs` (~161, ~210) and `path/arc_fillet.rs`. `arc_fillet.rs` `map_refusal` (~459) also raises it on `LegDegenerate`, whose escalated sibling has a user recourse (`FILLET_LEG_EXTENT_RECOURSE`), so its definite arm should tell the same story. A probe is owed before any panic.
- `PathError` `Display`, `OverdeterminedJunction` (~2175) — same — **2**, likely: the elaborator's backstop; the typed authoring surface establishes that a junction is bound once.

### GERMTAIL — `crates/geom-brep/src/intersect.rs` (row `section-wrong-lane-ends-in-a-tag`)

- `SectionError` `Display`, `WrongLane` (~712, "(a kernel bug)") — the public `geom_brep::plane_sphere_section(plane, sphere, band)` and its sibling section arms (`sphere_sphere_section`, `cylinder_cylinder_section`, …) — **1**, sure: any caller that hands an arm a pair of other kinds reaches it (`plane_sphere_section` ~1199). SSI's own `WrongLane` names the lever (`WRONG_LANE_RECOURSE`).

### DOCTAIL — the document model (row `part-and-refactor-kernel-defect-endings-split-by-class`)

- `editor-core/src/resolve/hit.rs` `UnnamedEntity` `Say` (~65) — the public `editor_core::entity_name` / `face_name` (~306, ~320), pick resolution, and pncad-py's `unnamed` — **1**, likely: those doors take a caller's `EntityRef { body: u32, key }`, and `body: 999` passes unchecked to the name table's miss. This is the shape of the first pass's REFS `NoSuchBody`.
- `editor-core/src/eval/parts.rs` `PartFault` `Display`, `PartProductFailureUnrecorded` (~410) — evaluation of an instantiate node — **2**, unsure: the product and the evaluation should be one evaluation's, but the first pass found the caller's `PartResolver` reaching `ReferenceCycle`, and a resolver whose product disagrees with its evaluation is not excluded.

### TESS — `crates/mesh/src/types.rs` (row `tessellation-ring-and-step-refusals-name-the-kernel`)

- `TessellateError` `Display`, `RingOnCurvedFace` (~548; raised in `curved.rs` and `trimmed.rs`) — `mesh::tessellate` over any body at rest — **5**, likely: the variant says no construction produces a curved face with a ring. But `topo::props`' `MassPropsError::RingOnCurvedFace` treats the same state as reachable and not yet measured ("move the cut so it crosses the face's edge"), and the Euler doors and a file reach it. The variant's "no construction produces one" (`types.rs` ~548) is a stale premise.
- `TessellateError` `Display`, `StepNotPositive` (~574) — `mesh::tessellate` — **2**, unsure: `ceil_count`'s every caller derives its step (`sqrt` of a non-negative ratio, or `cap_angular`), but representability (`half_angle > 0`) is checked only at tier 3, not at `set_face_surface`, so a negative stored half-angle may reach it.

### ENCL — this row's own ground (carried by this row)

- `geom-brep/src/recourse.rs` `Unsized::recourse`, `Fit | Bound` on an unreadable arm (~163) — every `Unsized::Fit`/`Bound` decision read at a build — **2**, unsure: an unreadable margin at a build ends in the defect ending on an approximating decision too. Whether finite input at extreme magnitude can poison such a margin is the same overflow question as SSIMARCH's.
- `topo/src/pcurves.rs` `PcurveMintError` `Display`, `LoopWraps` (~704, "report the body that reached it") — every pcurve mint — **1**, unsure: the variant's doc records that its "no constructor" claim was measured false once, and a seamless loop around a cylinder face built through the public Euler doors is not ruled out.

### What the second pass swept, and could not see

- **Helper channel**: `defect_ending(`, `Culprit::Kernel`, `Unsized::Defect`, `.culprit()` over `crates/` (non-test), then each match's reading traced to where it renders. A `Reading::Build` passed through a variable is followed only one call deep. An `Unsized::Fit`/`Bound` decision that never reads an unreadable arm is counted once, as the family above.
- **Prose channel**: `kernel (bug|defect)` case-insensitive over non-test, non-comment lines, without `assert`/`panic!`/`unreachable!`/`expect(`; the same phrase wrapped across a line break; `(bug)`, `a bug in`, `is a bug`, `internal error`, `cannot happen`; `report (it|this|the)`; `invariant` and `contradicted its own` in `write!`/`format!`. Hedged notes ("may indicate a kernel bug worth reporting") ride a lever or the last resort and are out of scope. So are panic messages (`surgery.rs`, `revert.rs`, `euler.rs`, `instance.rs`, `combine.rs`, `blend/surgery.rs`, `revolve/`, `extrude.rs`, `loft.rs` postconditions, `mesh/src/walk.rs`, `nurbs_cert.rs`) and in-file tests.
- **Not matched, and not classified here**: refusals that name no culprit at all. These include the "(corrupt body)" family (`splitting/section.rs` and `finish.rs` `Corrupt`, `chord_join.rs` `Corrupt`; `corrupt(…)` constructors in `props.rs`, `coherence.rs`, `props/quad_lane.rs`, `boolean/ops.rs` cut-in, `torus_region.rs`, `sphere_region.rs`, `solid_contain.rs`; `step-export` `Corrupt`) and bare invariant arms (`BooleanError::ClassificationInvariant`, `SplitJoinError::SectionInvariant`). They are `[failure.taxonomy]` row-4 candidates whose text claims nothing false, and `work/honeword/kernel-bug-refusals-end-without-the-shared-ending.md` and CLEAVE's `classification-invariant-family-types-bug-only-states-against-d9-row-4` already list most of them. Also not matched: a sentence assembled at run time from parts.

## The volume backstop left the operation path (2026-10-27)

PR 5842, under `[ev]` PR 5780 (the boolean's volume relations are a test oracle, `topo::test_support::volume_relations`), deleted the backstop: `BooleanError::{ResultVolumeImplausible, VolumeUnmeasured, VolumeCorrupt, VolumeUndecided}`, `BooleanDecision::VolumeBackstop`, `AtRestPolicy::gate_volume_backstop` and the `volume_backstop*` predicates are gone. Its `ResultVolumeImplausible`, `VolumeCorrupt` and `refusal_routes.rs` `VolumeBackstop` sites are gone; the rest stand.
