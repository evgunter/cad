---
id: edit-refusals-short-of-the-shape-guard
kind: issue
title: edit: refusals the viewer draws that state no recourse, by the shape guard's census
status: open
pr: 3492
branch: edit/part-refusal-recourse
opened: 2026-09-29
priority: P2
cost: M
---

(CHROME `refusal-residue`, from the shape guard's zero-recourse check.)

## What

`test_utils::refusal::problems` now flags a refusal that states no
recourse: no `Recourse:`, no "There is no way through" in either case,
and none of the shared unlabelled repairs (`BARE_RECOURSES`). The
standard (`work/chrome/error-and-check-text-overflows-its-region.md`,
"The standard a refusal is rewritten to") says the recourse is the
part never to drop, and where there is no way through the sentence
says so.

These rows still render with none. Each is admitted by exact id, under
the comment naming this file:

- `crates/editor-core/tests/refusal_concision_chains.rs`, `FILED_NO_RECOURSE`:
  6 feature-tree rows, a part's failure (`crates/editor-core/src/eval/parts.rs`):
  `Part/DepthExceeded`, `Part/NoResolver`, `Part/ReferenceCycle` and
  `Part/Unresolved` over its three faults (`Part/PartProduct` left the
  list when its sentence gained a recourse,
  `part-product-refusal-draws-the-gathers-stage-prefix`).
- `crates/viewer/tests/refusal_concision_edits.rs`, `FILED_NO_RECOURSE`:
  7 status-line `EditError` placement arms, held for the placement unit
  (`## Built` names them). Every other status-line arm states its
  recourse (PR 3490).

## A part's wrapper

The `instantiating <document id>:` label this row first recorded is
gone: EDIT's carried-refusal unit reworded the part wrapper, and the
hex ids it still prints are admitted span by span (`ADMISSIONS`).

## Repair shape

Rewrite each arm at its source to the standard: add the recourse the
raise site supports, or say "There is no way through" where none
exists (`geom_core::KERNEL_DEFECT_ENDING` and its siblings for a
kernel defect). Read the raise sites first: a recourse is a claim.
Then drop the row's entry. The lists carry a must-fire check
(`every_admission_admits_a_row_it_is_needed_for`, and the same check
in the edit and at-rest suites), so an entry left behind after the fix
goes red.

## A predicate's name on screen (CHROME fix pass, PR 3457)

A predicate's static name is routing: `geom_core::IndeterminatePayload`
does not render it, and every escalation says in words what was being
decided (`test_utils::refusal::subjectless_escalations`). The three flip
reports in `crates/editor-core/src/resolve/mod.rs` say it too
(`FlipSubject`), and leave the name to `Debug`.

## Ruled and spec'd (2026-09-29, EDIT orchestrator) — the status-line half, single review

**Tier:** single review (one Opus FULL). A recourse is a claim about what
the user can do, so every added one is checked against its raise site.

**Scope: the `EditError` status-line rows only.** That is
`crates/viewer/tests/refusal_concision_edits.rs`'s `FILED_NO_RECOURSE`,
raised through `crates/editor-core/src/edit.rs`. Two groups are out of
scope:
- **The eight feature-tree rows** (`refusal_concision_chains.rs`, the
  `Part` family). They wait for PR #3482 (part/product refusals), which
  is live on those rows.
- **Arms the placement unit reshapes or deletes.** The placement design
  on `[ev]` #3441 removes maintenance and moves placement onto gauges.
  These arms are skipped, not rewritten:
  - `Edit/MaintenanceRefused`, `Edit/MaintenanceUnrecorded`;
  - `Edit/ImproperPlacement`, `Edit/NonFinitePlacement`,
    `Edit/PlacementOnNonInstance`, `Edit/PlacementAxis`,
    `Edit/PlacementRuleMismatch`, `Edit/EmptyPlacementList`;
  - `Edit/MateRefused`, whose carried text MSOLVE owns.

  Leave their entries with a comment naming the placement row.

**The rule, per the row's repair shape:**
1. Read each arm's raise site or sites. Then either add the recourse
   the site supports, or end with "There is no way through" where none
   exists, using the shared endings. Never invent a recourse the edit
   door cannot honour.
2. Drop the arm's `FILED_NO_RECOURSE` entry. The must-fire check reds a
   stale entry.
3. Stay within the 75-word budget, and pass `problems` in full.
4. Keep every Python tag word frozen, because tags do not change. Any
   Display text a Python or viewer test pins is re-baselined and named
   in the PR body.
5. The predicate-name sites in `resolve/mod.rs` (the three flip reports)
   are in scope: say the decision in words, and leave the name to
   `Debug`.

**Rows:** the edits roster, green with the list shortened. Add one
planted mutant per family class (a recourse removed) that reds, and
record it.

**When done:** the feature-tree half and the placement arms remain on
this row, so it stays open, and the unit sets the `## Built` section.

## Built (2026-09-29, PR 3490) — the status-line half

- **56 `EditError` arms state a recourse.** Each was read against its raise site first, and each is labelled `Recourse:`. The two count arms (`DocParamCountHasNoUnit`, `DocParamCountHasNoDistribution`) name redeclaring the parameter continuous, which the create-or-replace door accepts. Where a slot reads the count as a count, that redeclaration refuses with `SlotDocParamDimension`'s own recourse, which also gets through (`edit_refusal_recourse.rs` follows both).
- **A recourse is written once per phrase** (`HELD_NODE`, `NAME_A_HELD_ENTITY`, `OR_A_DECLARED_PARAM`, `CLEAR_ONE_FIRST`, `COUNT_REDECLARED`), and each says what to do rather than restating the rule (`SlotDimensionMismatch`, `InvalidTolerance`, `NominalOutsideSupport`).
- **Forwarded faults keep their own sentence.** Where the fault is shared with the load door (`RootFault`, `DistributionFault`, `SlotDimensionFault`, `MeasureNodeFault`, `DimensionError`, `MetaVersionError`), the recourse sits in the `EditError` wrapper. `Roots` and `InvalidDistribution` match on the fault. No edit door raises `InvalidDistribution` over `NonFinite` (`distribution_fault_error` routes it to `NonFiniteDocParam`), so that arm ends in the kernel-defect ending and has no roster row.
- **The roster derives its fault rows** from witness chains whose matches are exhaustive, so a new `RootFault` or `DistributionFault` arm does not compile without a place in the chain or a stated reason for none.
- **A door that forwards an edit nobody authored states its own recourse.** `EditError::problem` renders the refusal without its recourse. `SplitError::PartEdit`/`RemainderEdit` and `InlineError::Edit` render that problem, then their own ending, arm by arm (`refactor.rs`, `ReplayTail`): rebind a forward reference to an earlier node, clear what this document sets on the instance's name, or give the call a resolver; every other arm is the kernel-defect ending. The secondary doors do the same: Python's `Doc.node_kind`, the viewer's `ReplayError` (a damaged file or a defect), `RangeRefusal::Derivation` (a defect), and `DeclareError::Edit` (stale findings, or a defect).
- **Flip reports read one lookup** (`crates/editor-core/src/decision.rs`) over each owner's words: `names::decision_words`, `eval::decision_words`, `topo::decision_words` and `profile::decision_subject`. They name the margin whose sign flipped ("the margin deciding … flipped from negative to positive"). `edit_refusal_recourse::every_predicate_a_subtract_logs_has_words_or_a_reason` holds every predicate a plain subtract logs to words or a reasoned `WORDLESS` entry; the unworded ones are filed as `flip-reports-name-no-decision-for-most-predicates`.
- **The status-line `FILED_NO_RECOURSE` entries are gone** except the seven placement arms.

**What remains on this row:**
- The six feature-tree rows (`refusal_concision_chains.rs`, the `Part` family).
- The seven placement arms held for `placement-is-spelled-three-ways-node-registry-and-rule`: `MaintenanceUnrecorded`, `ImproperPlacement`, `NonFinitePlacement`, `PlacementOnNonInstance`, `PlacementAxis`, `PlacementRuleMismatch`, `EmptyPlacementList`.

**Corrections to this row:**
- Its status-line count was 65. The file held 63 under this row's comment, because the bare `MaintenanceRefused` and `MateRefused` sit under MSOLVE's comment.
- The roster misses three `EditError` arms. They are filed as `edit-refusal-roster-renders-no-program-or-step-id-arm`.
- The viewer's own `Refusal` arms are held to no shape guard. That gap is filed on CHROME as `viewer-own-refusals-are-held-to-no-shape-guard`.

## Ruled and spec'd (2026-09-29, EDIT orchestrator) — the feature-tree half, single review

**Tier:** single review (one Opus FULL).

**Scope.**
- **The six `Part` rows still in `refusal_concision_chains.rs`'s `FILED_NO_RECOURSE`:** `Part/DepthExceeded`, `Part/NoResolver`, `Part/ReferenceCycle`, and `Part/Unresolved` over `EpsilonSeam`, `PinMismatch` and `Unresolved`. These are `PartFault`'s arms in `eval/parts.rs`.
- **Rides with it:** `part-unresolved-refusal-draws-the-workspaces-stage-prefix`. Real-text rows for `Part/Unresolved`, and the store's stage word (`workspace:`) not drawn inside the part's sentence. This is the same move #3482 made for `PartProduct` with `ProductError::sentence()`.
- **Also rides with it:** `edit-refusal-roster-renders-no-program-or-step-id-arm`. Add the missing status-line rows (`SetProgramOnNonProfile`, `StepIdsRefused` over each `StepIdFault` arm through an exhaustive witness, `NameStepNeverMinted`), and give each the recourse its raise site supports. The `StepIdFault` recourse goes in the `StepIdsRefused` wrapper, matched on the fault, as `Roots` does.
- **Not in scope:**
  - the seven placement arms, which stay for the placement unit;
  - the hex ids `ReferenceCycle` prints (`part-refusals-name-documents-by-hex-id`, held until the viewer has an id-to-file mapping). Its admitted spans stay.

**Rule.** Same as the status-line half, PR 3490:
- read every raise site, including secondary doors;
- write only a recourse the user can take from that door, or the shared ending where there is none;
- use `InThePart` (from #3482) for the part-side recourse;
- keep Python tag words frozen;
- re-baseline and name any pinned text.

**Rows.**
- The chains roster is green with the six entries gone.
- Real-text `Part/Unresolved` rows are built through a real `Workspace` / `DirResolver`.
- The three new status-line rows exist.
- One planted mutant per class reds.

## Built (2026-09-29, PR 3492) — the feature-tree half

- **Every `PartFault` arm a resolution raises states a recourse, or says there is no way through, and each is worded per door.** `NoResolver` is raised only at an API door, and says to pass a resolver over the part's store (`PASS_A_RESOLVER`, shared with the split's own resolver). The viewer never raises it: a session with no file resolves through `docio::NoFile`, which says to save the document beside its parts, and `instance_authoring.rs` follows it. `EpsilonSeam` says to open the part at its own tolerance, record the tolerance edit, save it over its file and accept its updated version, which keeps the part's id however it was minted. `DepthExceeded` says to flatten the nesting. `ReferenceCycle` ends in the kernel-defect ending, because a store that checks pins cannot hold a loop.
- **The resolver states what its store knows, by when it takes its scan.** `WorkspaceError::resolve_failure(Scan)` carries the store's sentence with no stage word, plus a recourse for the arms a resolution meets (`resolve_recourse`). A `Workspace` holds the scan it was opened with (`Scan::AtOpen`, Python's `resolver=` too), so an unknown id says to put the file back and open the store again; the viewer's `DirResolver` scans at every resolution (`Scan::PerResolution`) and says only the first. A part file missing since the scan says to put it back at that path; an unreadable one, to make it readable. `PinMismatch` states its own recourse, labelled. Each is followed word for word: `crates/pncad/tests/all.rs` (`workspace_resolve_door_refusals_meet_the_standard_and_their_recourses_get_through`), the Python row in `test_assembly_eval.py`, and the viewer rows.
- **One rendering helper.** `editor_core::sentence` holds `Staged` (a refusal's stage word and its stripped sentence), used by `ProductError`, `PersistError` and `WorkspaceError`, and `Recourse`, the one spelling of the label.
- **The split's recourse moved.** "Pass a resolver" now lives in the split's own resolver (`WithPart`). `ReplayTail` forwards a mate refusal whole, so no recourse is stated twice. `WithPart`'s pin arm is reached when a kept instance shares the split's part id, and says to split under an id the document does not reference.
- **The chains roster's six `Part` entries are gone.** `Part/Unresolved(*)` is rendered from real text in `crates/viewer/tests/instance_authoring.rs` (`every_unresolved_part_badge_meets_the_refusal_standard`).

**What remains on this row:** the seven placement arms held for `placement-is-spelled-three-ways-node-registry-and-rule`.

**Filed:** `part-refusal-over-a-store-that-will-not-scan-or-load-states-no-recourse` (the scan and load arms of a real store that no row renders), `part-nesting-segfaults-before-the-depth-bound` (`DepthExceeded` is unreachable: the stack runs out first), and `no-file-part-recourse-names-a-save-the-browser-build-lacks` on CHROME.

## Built (2026-09-30, PR 3497's fix pass) — two of the seven placement arms

- **`ImproperPlacement` and `NonFinitePlacement` state a recourse and name their frame.** Each carries a `FrameSite` (`crates/editor-core/src/placement.rs`): the instance's placement frame (`SetPlacement`), an explicit rule's listed placement, or a transform's literal step, by index. The sentence is the site's subject, the frame rule's clause (`FrameFault`'s `Display`) and `FrameFault::recourse`, which every raising door honours: each refuses the edit that carried the frame, so making it again with the frame repaired gets through.
- **A third arm, `NonRigidPlacement`, meets the standard from birth**: a proper frame that is not definitely rigid at tolerance, refused at the same three doors by `topo::check_rigid`, the predicate the evaluation's `NotRigid` applies.
- The load door's `SnapshotError::PlacementNonFinite`/`PlacementImproper`/`PlacementNonRigid` carry the site too, and end in the kernel-or-file ending (the edit doors never admitted a non-finite or mirrored frame) or, for a non-rigid one earlier builds admitted, the regenerate recourse.
- Their `FILED_NO_RECOURSE` entries in `crates/viewer/tests/refusal_concision_edits.rs` are gone.

**What remains on this row:** five placement arms held for `placement-is-spelled-three-ways-node-registry-and-rule`'s P2: `MaintenanceUnrecorded`, `PlacementOnNonInstance`, `PlacementAxis`, `PlacementRuleMismatch`, `EmptyPlacementList`.

## Built (2026-10-01, the placement unit's P2) — the placement arms

Ruling 10 of `docs/EDIT-PLACEMENT-SPEC.md`'s P2, against the five arms still on the list:
- **Deleted with what they refused:** `MaintenanceUnrecorded` (and, under MSOLVE's comment, `MaintenanceRefused` with its eleven forwarded rows). No edit records a frame, so nothing is maintained and nothing is unrecorded.
- **Re-shaped, and stating a recourse:** `PlacementOnNonInstance` is `OffsetOnNonInstance` (`DocEdit::SetOffset` on a node that instantiates no part), ending "aim the offset at a node that instantiates a part". `PlacementAxis` — now raised by an instance's offset and a gauge's placement as well as a transform's — ends "give the rotation axis a direction of nonzero length".
- **New, stating a recourse from birth:** `GaugeOnNonPlaced`, `GaugeNotLive`, `NotAGauge`, `GaugeCycle` (the `SetGauge` door), and `MateFault::OffsetDisagrees` / `OffsetUnchecked` under `MateRefused` (`OFFSET_RECOURSE`; repair the node whose placement did not evaluate, or clear the offset).
- **Left, untouched by P2:** `EmptyPlacementList` and `PlacementRuleMismatch`, the explicit placement rule's.

`crates/viewer/tests/refusal_concision_edits.rs` renders every new arm, its `FILED_NO_RECOURSE` holds the two rule arms under this row's comment, and its `MaintenanceRefused` admissions are gone. `FrameSite::Registry` went with the registry; a literal step of an offset or a gauge is `FrameSite::Step`.

**What remains on this row:** the two placement-rule arms, `EmptyPlacementList` and `PlacementRuleMismatch`.
