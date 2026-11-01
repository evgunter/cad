# DOORS — log

## 2026-10-02 — opened at AUTHOR's exit, blocked

This is the blocked half of AUTHOR's cut: 5 rows, every one waiting
on a kernel row on RECIPE's or CLEAR's slate, so each moved as `parked`
on its `blocked_on` (they were `open` under AUTHOR). Unparked, they
weigh 15 budget points against 30. The unblocked half is AUTHTAIL. Per `work/README.md`, a track
with nothing to dispatch is `blocked` with no orchestrator; seam notes
on RECIPE's and CLEAR's logs say which of their rows has a consumer
here.

AUTHOR's narrative is recoverable from
`docs/doc-ledger/author-leaves-the-tracker.md`.
- 2026-10-03 — Seam note from RECIPE: `declared-pairs-are-a-booleans-own-payload` closed (PR #3902: `DocEdit::SetDeclare` sets a live Boolean's or Union's pairs; `declare` appends one finding, `declare_all` replaces). `the-boolean-door-evaluates-its-boolean-twice` and `a-union-that-becomes-flush-later-can-only-be-deleted-and-re-added` set `open`, DOORS `ready`, as AUTHOR's exit note asked. (RECIPE orchestrator)
- 2026-10-03 — Seam note from RECIPE: `extrude-distance-is-a-depth-and-a-side` closed (PR #3912: `Node::Extrude { profile, distance, side }`, `DocEdit::SetExtrudeSide`, a negative depth refuses `NegativeDepth` quoting its value with a recourse naming the other side). `the-create-pane-has-no-extrude-side` set `open`; the viewer's create pane inserts `side: Along` today. (RECIPE orchestrator)
- 2026-10-06 — Seam note from RECIPE: PR 3886 merged (names in words per Ev's #3906 and #4069). `face-pick-cannot-name-which-face` is opened: the pick readout can speak through `Speaker::within` on the landed evaluation (its composer is noted in that row). (RECIPE orchestrator)
- 2026-10-12 — Seam note from ENCL (PR 4811, merged at `abff76bb25`): editor-core's undecided sentences render through `Indeterminate::undecided`. Evaluation, naming and selection are ended by one closed `decision::Decision` table with a kind column; evaluation constructors take a `Decision`, and an unknown name ends loud (`MissingRecourse`). D10 ground keeps main's text through held variants: `NodeErrorKind::MeasureEscalated` (measure distance/gap parallelism), `PlacementAxisEscalated` (`EditError::PlacementAxis`) and `PoseDirectionEscalated` (datum directions and frames, pose `degenerate`). All three classify as Escalated, and pncad-py `tags.rs` and viewer `tree.rs` gained arms. Left for after D10, on the ENCL row: the placement route compares the role word, the naming door's `&str` feed, `placement.rs`'s stale `# Errors` doc, and `datum_unit_norm` vs `eval_direction_norm` ending two ways. (ENCL orchestrator)
- 2026-10-26 — (tracker sweep) **AUTHTAIL folded in** (Ev, in chat, 2026-10-26). Both programs were AUTHOR's successors on the viewer authoring files, so they are one track again. AUTHTAIL's eleven live rows moved here by `git mv`, ids and bodies unchanged (two open, nine parked on INTENT's PR 5636), each with a `## Moved (2026-10-26)` note. Of its nine closed rows, `the-viewer-cannot-author-a-mirror` is kept here: `the-mirror-keeps-two-pieces-…-after-dprime3` is the residue of its mirror arrival fork and points at its `## Closed` for the decision. The other eight are deleted; nothing live leans on their content. AUTHTAIL's directory is deleted (`docs/doc-ledger/authtail-leaves-the-tracker.md`). `paths` is the union of both programs'; the title, charter and `plan.md` cover both halves, and the plan carries AUTHTAIL's D′3 order and lessons. Path citations re-aimed at `work/doors/`: `crates/viewer/src/pane/viewport.rs`, `crates/viewer/src/sketch.rs` and PROFILETAIL's close-from-a-tip row. Load 13.5 of 30, top band P1 (dispatchable), with two P0 rows parked on D′3.

- 2026-10-26 — seam from ATREST (stage 5 C, `the-at-rest-census-is-a-check`, branch `atrest/the-at-rest-census-is-a-check`): viewer `session.rs`: the at-rest badge reads the checks report; no door changed.
- **2026-10-30, crossing announced by (INTENT orchestrator):** INTENT stage 6 E (`work/intent/the-sketch-tool-authors-a-tangent-continuation.md`, spec `docs/INTENT-STAGE6-SPEC.md` §6) edits `crates/viewer/src/sketch.rs` and `crates/viewer/src/pane/profile.rs`. The sketch tool draws a picked face's trace and writes C's `OnTrace`/`AlongTrace` step, and `sketch::is_committed` is fixed for a profile drawn on a frame. Branch `intent/s6-e-sketch-tool`. It takes none of this program's rows.
- 2026-10-30 (INTENT tail retriage) Received from INTENT by `git mv`, ids and bodies unchanged, priority and cost set (PR body has each row's reason): `a-takeover-could-re-seat-a-pick-its-result-keeps` (P3), `a-takeovers-rebind-cannot-reach-part-spelled-names` (P3). INTENT's charter is D10 and these are follow-ups found while building it.
- 2026-11-01 — (CHROME orchestrator) `a-mate-on-a-taken-over-copys-face-goes-stale` set `open`: its blocker, PR 5636 (D′3), merged at `29110056`. A PR number in `blocked_on` is not checked by lint, so the row read as parked on a fired trigger. CHROME's lane traced Ev's stale held face (bracket walk, 2026-10-31) to this row's takeover and added the evidence under `## Evidence` (PR 6204). CHROME's `a-findings-face-reference-reads-as-stale` (P1) is now parked on this row. The row stays here because plan item 5/6's one generalised takeover rule is DOORS' to build once. It is a P0 with its design decided, and DOORS has no orchestrator.
- 2026-11-01 — (DOORS orchestrator) **picked up; triage and cut.** D′3 (PR 5636) merged at `29110056`, so eight rows parked on it had fired. Lint does not check an int blocker. Dispositions:
  - `a-combine-deletes-the-placement-it-just-repointed` is closed as absorbed. `Place` now has one source, so the operands' placement lists are disjoint.
  - Four held-pick and chrome rows are unparked.
  - The split, the fillet operand, the mirror's two pieces and the generalised-takeover row are re-parked on SPACES' `[ev]` PR 6209. That PR retires §5′.3 (b) and, on Ev's answer, hands (a) to SPACES. Its drawn-bodies rule may also dissolve the split and the fillet-operand rows for unplaced bodies.
  - The P0 `a-mate-on-a-taken-over-copys-face-goes-stale` is not parked on 6209. Its rebind follows any re-point, whichever readers that re-point reaches.
  - `face-pick-cannot-name-which-face` is re-priced from legacy D to M. Its names-layer blocker is in (`names::words::leaf_role`).

  The track then weighed about 52 against 30, so it was **cut along its seam**. HELD (`work/held/`) takes the nine rows on what the viewer holds, marks and reads back: held picks, committed-node marks, authored picks, read-back frames, `moves_of` and the sketch preview. That is 23 points, top P2, `ready`. DOORS keeps the takeover and the rulings' doors, at 29 points. Re-aimed three citations (`sketch.rs`, `pane/viewport.rs`, PROFILETAIL's close-from-a-tip row). Plan rewritten. Sitting: the P0 rebind, the extrude side and the face words as three lanes, and designers on the clearance consumer.
- 2026-11-01 — (DOORS orchestrator) Dispatched:
  - `doors/take-over-rebinds`, the P0 rebind. Tier: style review plus a correctness reviewer.
  - `doors/extrude-side` (P1). Tier: style review.
  - `doors/face-words` (P1). Tier: style review.
  - The designers on `the-gui-has-no-clearance-consumer`: a three-designer arm, A, B and C, drawn per `docs/DESIGN-FORK-PROTOCOL.md`; the bytes are on `analysis/design-fork/doors-clearance-consumer`.

  All three lanes run on this box. That is 4 vCPU and about 28 GB of disk, under the one build slot, with hosted CI as the record.
- 2026-11-01 — (DOORS orchestrator) `the-create-pane-has-no-extrude-side` closed as superseded by D10: FORK-S3P moved the side to the mates, and INTENT D removed `side` from `Node::Extrude`. PR 6260 (merged) filed the stale negative-depth recourse on LATHE. **Class: a row specified before a later ratified redesign read as live.** The dispatch brief and this plan were built from the row's pre-D10 text, and the lane caught it, not the orchestrator. Before writing a brief from a row older than the ratified decisions it touches, check those decisions. The clearance designers' first reports are in. All three corrected the row's premise (the measure's door is topo's `min_separation`, and `editor_core::clearance` is E7's toleranced question) and differ on four points, so round 1 is running.
- 2026-11-01 — (DOORS orchestrator) **Clearance consumer fork to Ev: `[ev]` PR 6261, fork row 231.** Three designers converged in two rounds on:
  - a kernel door on `pncad` giving certified readings at nominal, on `Interval`, with numbers across the seam;
  - topo keeping the closest pair found in the measure payload;
  - a viewer certify seam shaped like the coincidence door;
  - outward-rounded brackets.

  Residue left for Ev: whether the faces that floor `lo` go in the value, and whether to reword E3. The row is rewritten to the recommended design. **Class: the row's premise named the wrong engine** (`editor_core::clearance` is E7's toleranced door, and the measure is topo's `min_separation`); all three designers caught it. Off-question findings filed on CLEARTAIL:
  - `the-clearance-leaf-doors-ask-an-assertions-question-a-second-way` (P3);
  - `needs-enclosures-recourse-sends-the-reader-to-a-parameter-box-and-a-type-path` (P4).

  CLEARTAIL's `measure-clearance-refused-states-no-recourse-…` (P4) words the refusals the consumer will draw. It is re-banded when the consumer's unit is specified and parks on it.
- 2026-11-01 — (DOORS orchestrator) Ev on PR 6261: no face set. Ev's other two points:
  - An assertion `min_clearance ≥ c` should hand `c` to the search and cut short. Today `wire_assertion` compares the measure node's full `min_separation` bracket against `c`. Meanwhile topo's `Sweep`, the predicate search that discharges and stops at the first verified violation, is reached only by the test-only `editor_core::clearance`.
  - The witness is "a violating pair", shown only on a violation, never "the closest pair found".

  **Class: all three designers framed the bound engine as the toleranced question and missed that the document's own assertion is the predicate it answers.** The question to Ev now is whether a bare clearance measure shows its bracket. The row and the PR are re-cut to that design once Ev confirms.
- 2026-11-01 — (DOORS orchestrator) `face-pick-cannot-name-which-face`: PR 6262 is in, green, with a style review dispatched (tier as planned). The spec was **stale again, a second instance of the class logged above**: the declare offer (`c6e9e3d95b`), the add-datum form and face-frame datums (`ac1c2847ad`) and `tree::node_number` (`295c2e786e`) were all gone on main. The lane routed the sites that remain through one composer, `tree::PickedName`. It filed `a-selections-kept-labels-miss-the-nodes-its-face-words-name` (P3). **Friction:** the shared disk hit 3.7 GB free with two 11 GB viewer targets. The finished implementer's target was handed to its reviewer by rename, and the extrude lane's clone was reclaimed.
- 2026-11-01 — (DOORS orchestrator) PR 6262 style review: mergeable after fixes. The composer re-derives no words.
  - **MAJOR:** a mirror drop names a deleted copy by its bare tag.
  - **Minors:** a debug print; the panel and the refusal speak two documents; `, on M` repeats a node that a join already named.
  - **Q1:** the body scope now has three spellings.

  **Classes, recorded here as their durable home:**
  - (a) Labels kept at pick time while the sentence is recomputed at drop time against a later document (`on_body`, `seat_of` or `body_of` in a `reconcile` or `PickLost`). Look in `seats.rs` and `blend.rs`.
  - (b) A test re-plumbed to a speaker that production does not use, so it no longer tests what the user sees.
  - (c) "Which node do the words already name", re-stated outside the names layer's walk.

  The fix pass, sent to the same lane, takes everything and sweeps (a).
- 2026-11-01 — (DOORS orchestrator) PR 6264 reviews:
  - **Style:** duplicates the unit minted. There is now a third rebind loop off the `name_carriers` enumeration, and the carrier rule restates `merged.rs`. The `Place`-copies-taken predicate exists three times, and the Face/Edge twin was minted again. The held-pick re-seat has two homes, and the `SegRewrite` census is stale. Prose rot is in `refactor.rs`'s header and `create`.
  - **Correctness:** soundness holds on the fragment/merge boundaries, and rebinds cannot chain.
    - **MAJOR:** a part-spelled takeover now refuses the whole gesture (`NameStepNeverMinted`).
    - **Minors:** `reseated`'s read-order dependence is unguarded and silent, and the door evaluates on the UI thread at every stage preview, which the background run then repeats (379 ms on the die union, debug build).

  **Class, recorded here as its home:** name-step and liveness checks that walk embedded names are exempted only on the outer node (`Rebind`, appearance, insert liveness, `InterfaceRecord` outers). The fix pass, sent to the same lane, takes everything and sweeps that class.
- 2026-11-01 — (DOORS orchestrator) PR 6262 re-verify (head `6552f7d174`): no MAJOR, not mergeable. **Class repeated: the fix pass minted new two-way spellings while closing old ones.** The new pairs:
  - `DrawnBody::of` and `in_doc`;
  - `Seats::said` and `said_by`;
  - two `feature()` methods;
  - three shapes of "a pick plus its kept labels".

  **Unilateral ruling:** the viewer's chrome speaks the landed document everywhere: panels, refusals, tree, header and notices. `shown_speaker` is removed. It moved the panel/refusal split rather than closing it, and its join wording could describe a carry the landed run did not do. Fix pass 2 was sent to the same lane with a self-check for new duplicates.
- 2026-11-01 — (DOORS orchestrator) Two fix passes are in, and each is re-verifying.
  - **PR 6264** (head `bab0326e0a`):
    - The part-spelled refusal is fixed as a class. The name-step exemption now applies per embedded level, at the insert, appearance, payload, `Rebind` and load doors.
    - `SegRewrite::from` makes `Reseat` stateless.
    - The door skips evaluation when nothing reads the copy, and seeds the landing run when something does. The die union drops from 363 to 23 ms when nothing reads the copy, and the background run from 342 to 41 ms when one does.
    - Held picks re-seat at landing.
    - The takeover moved out of `refactor` into `editor_core::take_over`. There is one Rebind emitter, `rebinds_at_sites`.
    - Rows closed: `a-takeovers-rebind-cannot-reach-part-spelled-names` and `an-open-tools-picks-on-a-taken-over-copy-go-stale`.
  - **PR 6262** (head `53bb5a6012`), pass 2 after the landed-everywhere ruling:
    - One `HeldPick` and one `DrawnBody`; `BlendTarget` is deleted.
    - One `feature()`.
    - The Checks test now runs through production.
    - The part file is left out of pick sentences. The lane argued that a `Speaker` cannot reach `PartFiles`, and that tags already tell instances apart. Accepted, pending the re-verify.
- 2026-11-01 — (DOORS orchestrator) PR 6264 re-verify (head `bab0326e0a`): not mergeable.
  - **MAJOR:** the pass moved held-pick re-seating from commit to landing. A gesture committed from a held pick inside that window writes a stale reference. Undo also cleared every pending takeover.
  - **Minors:** a copy of a copy and the fused pattern are not seen as readers, and `successor.rs` asserts a false premise about them. The load door is now looser than the edit door for payload names. "Is this a part's name" has three predicates.

  **Unilateral ruling:** re-seat at commit. Held picks count as readers, so the door still skips evaluation when nothing reads the copy. The pending queues are deleted. Fix pass 2 is sent to the same lane.

  **Class, again:** a fix pass bought its cost win with a deferral that opened a correctness window. Re-verify caught it.
