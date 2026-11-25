# FORMWORDS — the log

## 2026-11-02 — opened

Cut out of FORMS, which was carrying 52 budget points against a
30-point ceiling once Ev's create/edit reports landed on it. The seam
is by question. FORMS keeps the rows Ev reported, which ask whether
creating a thing and editing it are one implementation. The rows here
ask how many vocabularies the forms answer their shared questions in,
and the seat gate the forms consult. The eleven rows arrived by
`git mv` with their ids and bodies unchanged. The past entries of
FORMS' log stay there. Nothing dispatched.

Signed (FORMS orchestrator).
- 2026-11-20 — (tracker sweep) **FORMS merged in.** It held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size). Its 6 live rows moved here by `git mv`, ids and bodies unchanged, each with a dated Moved note; its charter and plan are the `## From FORMS` sections of `program.md` and `plan.md` (`docs/doc-ledger/forms-merges-into-formwords.md`). (tracker sweep)

## 2026-11-22 — sitting opened

(FORMWORDS orchestrator) Took the track: ten P1 rows. Ev's standing
instructions for this sitting (in chat, 2026-11-22): every lane runs
on Opus; where a protocol calls for two reviewers or designers, two
Opus; no experiment rows are recorded (`docs/MODEL-COMPARE-LOG.md`,
`docs/DESIGN-FORK-LOG.md`), so no model draw is made either.

Dispatched at once, six implementer lanes:
- `formwords/refused-entry` — `a-refused-entry-sticks-in-its-field-after-a-number-replaces-it`
  (Ev's report today). Full review: undo and the driven-slot guard are
  where a confident wrong answer would hide.
- `formwords/held-pick` — `a-creation-forms-held-pick-survives-a-document-swap`.
  Ruled shape 1, made structural: everything document-bound in viewer
  app state is reset wholesale where a document is replaced, so a new
  field cannot miss the reset; the lane reports back if shape 2 is
  plainly better. Full review (a live wrong answer).
- `formwords/denotes-body` — `denotes-body-enumerates-its-gaps-against-the-operand-door-and-misses-one`.
  Value-wanting callers get a value answer; the enumerating doc goes.
  Full review (a gate admitting what the evaluator refuses).
- `formwords/seat-vocab` — `a-fifth-spelling-of-this-seat-is-empty` and
  `four-pick-state-vocabularies-in-one-create-module`, with
  `a-per-kind-sentence-lives-in-the-widget-not-on-the-choice` riding if
  it lands on the same ground. Style review (vocabulary consolidation;
  correctness not meaningfully at risk).
- `formwords/dimension-ladders` — `dimension-to-unit-ladders-have-six-homes-in-the-viewer`,
  `drag-tick-has-three-homes` and `probe-seed-respells-props-authored-in`:
  one question (what a field of a dimension looks like). Style review.
- `formwords/mirror-plane` — the mirror half of
  `a-plane-other-than-a-face-cannot-be-authored-in-the-viewer`: the
  mirror takes a Plane or Frame variable as the split already does.
  The row stays open for the general plane door. Full review.

Two design forks weighed concurrently, two Opus designers each: the
general plane/axis door (`a-plane-other-than-a-face-…`, with the
adjacent P2 `no-viewer-row-re-points-a-live-nodes-read`), and
`a-projected-split-is-unreachable-from-the-viewport`, as the plan asks.

Not taken: the circle-split pair and the other P2–P4 rows (sitting
rule: P0 and P1 only).

## 2026-11-22 — two forks weighed

(FORMWORDS orchestrator)
- **Plane door.** The two designers agreed: a plane or axis is a pose
  variable with one definition editor, anonymous at its reader and
  named to share, with `VarDecl` gaining a pose arm. One factual split
  between them: one designer said `VarDecl` already takes a pose. It
  does not (`crates/editor-core/src/var.rs`), and the other designer's
  reading stands. The design is put to Ev on `[ev]` 7367. The build
  waits for Ev's answer on the approach, and it stacks on the
  mirror-plane lane, since both reshape `SessionOp::AddMirror`'s plane.
- **Projected split.** Both designers found the premise retired, so the
  row is closed, not put to Ev (its Closed section has the evidence).
  The residue is two existing rows on DOORS and CHROME; this log
  records the order they should land in.

## 2026-11-22 — plane door re-weighed on Ev's question

(FORMWORDS orchestrator) Ev asked on `[ev]` 7367 why planes are not
placed the way bodies are. Neither designer had weighed that option;
both had wrongly read `[placement.world]` as putting a space's planes
out of reach, when placement values already chart a space's axes. Three
more rounds followed. Both designers moved to a **datum**, a node that
places a pose, and both kept Slide and Flip with no gap mate (an offset
is a slid far side, per `[placement.mates]`). Their positions on
frame-only versus per-kind crossed twice, and both named the same
question underneath (is a datum placed like a body, or declared like a
pose), so it goes to Ev as a stated split. The PR is updated in place,
and its DESIGN.md diff writes the frame-only reading.

A class finding with no home: the placement editor never writes a slid
far side. Filed as P0 on DOORS:
`the-placement-editor-cannot-author-an-offset-from-a-face`.

- 2026-11-22 — (FORMWORDS orchestrator) PR 7374 (seat-vocab) merged after a style review with correctness claims and a fix pass. It closes `a-fifth-spelling-of-this-seat-is-empty`, `four-pick-state-vocabularies-in-one-create-module` and the ride-along `a-per-kind-sentence-lives-in-the-widget-not-on-the-choice`. Merged on the fix pass's green CI and the orchestrator's read, without a delta review.
- 2026-11-22 — (FORMWORDS orchestrator) PR 7376 (held-pick) merged after a full review and a fix pass, on the fix pass's green jobs and the orchestrator's read. It closes `a-creation-forms-held-pick-survives-a-document-swap` and the claimed `the-drafts-keep-held-slot-ids-across-a-document-replacement`.
## 2026-11-22 — mirror-plane reviewed

(FORMWORDS orchestrator) PR 7373's review found it mergeable. It
raised two MINORs: a variable picked before a face was silently
dropped, and a hover sentence pointed at a pick the GUI cannot make. A
fix pass is running on both. A class to watch: seats that decline a
pick **silently** although `Tools::feed`'s doc promises a notice. The
fix pass closes it for the plane seats; a datum or seat unit that adds
a pick route checks its own seats against that doc.

## 2026-11-22 — friction: disk and runners

(FORMWORDS orchestrator)
- **Disk.** Eight concurrent viewer lanes, each with its own target,
  filled this container's ~60 GB allowance twice. Two lanes lost a
  build to it and deleted their own targets. The rule applied since:
  `CARGO_INCREMENTAL=0`, `-p <crate>` only, and the orchestrator
  reclaims a review lane's clone and target the moment its report is
  in. More than five or six concurrent lanes belong in their own cloud
  sessions (`memories/orchestration-model.md`).
- **Runners.** A docs-only PR waited about 17 minutes for its first
  job, and one lane's `gate ok` sat queued for over an hour after its
  jobs were green. The time queued is a finding of its own; nothing in
  this track's diffs causes it.
- **Review tiers dispatched:** held-pick full, seat-vocab style with
  correctness claims (its commit-withholding is a behaviour change),
  and refused-entry, denotes-body and mirror-plane full.

## 2026-11-22 — reviews in; two P0s found

(FORMWORDS orchestrator)
- **PR 7379 (denotes-body)** was not mergeable: its merged head did
  not compile, and its review found an evaluator panic.
  `split_port` hits `unreachable!` on a split of a family that a
  placement reads, which an ordinary split gesture reaches. Claimed
  `a-split-of-a-family-has-no-halves-a-read-reaches` from REFS at P0
  and dispatched `formwords/split-of-family` (dual review). The same
  review found that the subtract tool seat cuts once per member when
  handed a pattern whole (N plates, one hole each). Filed
  `a-subtract-tool-seat-takes-a-whole-family-and-cuts-once-per-member`
  (P0, design), and two designers are weighing it.
- **PRs 7376, 7374 and 7378** are mergeable, each with a fix pass
  running.
- **Class: the slow-set rule is judged at CI speed.** Lanes timed rows
  locally at opt-0 and over-applied the 0.5 s bar. CI's
  `check-fast-set-times` is the instrument, and briefs now say so.
- **Class: a test whose premise excludes the failing mode.** Two
  reviews (7374, 7379) found rows that a mutation of the very gate
  they name leaves green. Each review's mutations were the instrument
  that caught it; future briefs ask reviewers for one mutation per new
  gate row.

- 2026-11-22 — (FORMWORDS orchestrator) PR 7378 (refused-entry, Ev's report) merged after a full review and a fix pass, on green CI. A class finding with no home yet: app-feature test rows can be neither in the slow set (`check-slow-set-names.py` reads them as dead) nor timed (`check-fast-set-times` times only the default-eps step). So the app-feature job has no time bar, and a 12 s row runs on every viewer PR.
- 2026-11-22 — (FORMWORDS orchestrator) PR 7372 (dimension-ladders) merged after a style review with correctness claims and a fix pass, on green CI. It closes the three ladder rows. The fix pass added API to editor-core, quantity and geom-core (`Dimension::quantity`, `Notation::unit_of`, `UnitQuantity::units`), and its PR names their owners.
- 2026-11-22 — (FORMWORDS orchestrator) PR 7379 (denotes-body) merged after a full review and a fix pass. Its one red, test (kernel), is a sweep row at 1e-12 inherited from main: reproduced on main, and owned by TESSLIE row `main-red-a-cylinder-wall-through-a-prism-corner-union-is-volume-uncomputable-at-1e-12`, where the reproduction is now noted. Annotated on the PR.
## 2026-11-22 — subtract-by-family weighed

(FORMWORDS orchestrator) Both designers agree, independently: the
evaluator is right by D10's letter, and the defect is one level up, in
the subtract's tool being a `Body` seat. Their recommendation is to
make it `Bodies`, which reverses agent text approved in passing on
FORK-DM4, in D10 and in REFERENCES DM4. Put to Ev on `[ev]` 7386. That
branch also carries this branch's tracker state, so this branch will
need no PR of its own if 7386 lands first. Off the question, both name
the `ValuePayload::Instances` doc (`crates/editor-core/src/eval/mod.rs`)
as stale: it says a body seat meeting a family whole refuses
`WrongOperand`, while reach makes it map. It belongs with the build
after Ev's answer.

- 2026-11-22 — (FORMWORDS orchestrator) PR 7373 (mirror-plane) merged after a full review and a fix pass, on green CI and the orchestrator's read. The fix pass added the shared `PlaneTool<E>` and did not get a delta review. `a-plane-other-than-a-face-cannot-be-authored-in-the-viewer` stays open for the datum build (`formwords/datum`).
- 2026-11-22 — (FORMWORDS orchestrator) `[ev]` 7386 approved and merged: a subtract's tool is a `Bodies` seat. Ev then questioned implicit mapping itself ("we shouldn't be implicitly mapping over lists!"). Filed `a-family-read-at-a-body-seat-maps-implicitly` (P1, design), with two designers weighing it. The split-of-family lane is told to keep its fix minimal meanwhile.
- 2026-11-22 — (FORMWORDS orchestrator) Every lane stopped at the weekly usage limit and resumed when usage came back.
- 2026-11-22 — **From VSEAM:** `SessionOp::SketchOnFace` retires in PR 7466 (`vseam/sketch-on-face-retires`). The viewer's on-face act is now `SessionOp::DrawOnFace`, followed by the ordinary extrude and union. Rows here that cite `SketchOnFace` as evidence keep their history, but its code citation goes stale at that merge. The affected rows are `doors/a-held-face-stays-marked-while-its-form-is-collapsed`, `doors/a-seated-tools-held-node-is-drawn-nowhere`, `formwords/a-plane-other-than-a-face-cannot-be-authored-in-the-viewer` and `vgeom/a-profile-drawn-on-a-frame-is-drawn-at-the-worlds-xy`. (VSEAM orchestrator)

- 2026-11-22 — (FORMWORDS orchestrator) Implicit-map fork: both designers agree on an explicit binder with nothing mapping implicitly. They crossed twice on whether a binder's extent is stored per definition or derived from reads (with written yields and containment), so it goes to Ev as a stated split on `[ev]` 7409, with the DM4 list-splice question. The datum lane hit a conflict with doctail PR 7151 (FaceChart's carriage lever), so the datum lands without the FaceChart fold and the fold becomes its own P3 row. That is a sequencing call made on the lane's recommendation.

- 2026-11-22 — (FORMWORDS orchestrator) Friction: four reviewed PRs (7376, 7378, 7379, 7380) went CONFLICTING again each time main moved, because both sides append to `work/formwords/log.md` and GitHub ignores the union driver. A conflicting PR gets no CI run, silently. They were combined into PR 7461 for one CI run. A class finding: closure log lines on unit branches make every unit re-conflict at each merge. Write closure log lines on the orchestrator branch instead, and keep only the item flip on the unit branch.

- 2026-11-22 — (FORMWORDS orchestrator) PR 7416 (binder rewording of D10 `[repetition]`, `[placement]`, `[assertions]`, DM3, DM4, VR3/VR4) merged as a faithful elaboration of Ev's ruling on 7409, on my read. I settled its open points as elaborations, recorded here for Ev's retroactive review:
  - an orphan k-reader is reported as unused, not refused;
  - nested binders are sugar for one multi-index binder;
  - a free `index(N)` retires;
  - an assertion inside a binder is its own yield;
  - no rule for i = j, since the domain states it;
  - a bare `[xs, a]` refuses;
  - `Union([…xs])` names like `Union(xs)`.
  The binder build is dispatched to the cloud session `formwords/binders`, in three PRs (kernel, Python, viewer), each with a dual review.

- 2026-11-22 — (FORMWORDS orchestrator) Datum PR 7382 dual-reviewed (two cloud-session reviewers). A: mergeable. B: two MAJORs, flatten strands a datum (refactor.rs) and Python cannot read a datum's bundle. Both trace to one class, also behind a coincidence MINOR: "placement" has two extents in the code. `Doc::placements()` excludes datums while `bundle_of` includes them, and about 15 sites hand-match `Place | InstantiatePart | Datum`. The fix pass makes `placements()` mean every bundle holder, as `[placement.datums]` says, and names the copy-making subset. It also adds "its frame" to `[placement.datums]`'s near-side parenthetical, since the code admits a frame-to-frame mate. That is an elaboration of an approved design, recorded here for Ev.

- 2026-11-23 — (FORMWORDS orchestrator) PR 7426 (subtract Bodies) dual-reviewed: both reviewers found the same MAJOR independently. `fold_cuts` is a drifted copy of the union's `fold_members` without the `name_union` end pass, so names depend on the order of the tools. A class finding: the third near-copy of a fold in `eval/wire.rs` (`fold_members`, `fold_cuts`, and `cut_tools` copying `wire_combine`'s `whole`). Every structural-fix lane's review has found this shape. The fix pass makes the union and the subtract share one fold core. Also found: five pinned name digests moved without re-baselining. Merged 7380 (face-offset) on its own green run. The combined PR 7461 hit the document-swap destructure's intended compile error on face-offset's new `mate_offset_edits`, fixed in the combined branch (dropped at the door).

- 2026-11-23 — (FORMWORDS orchestrator) Friction, runners. At 04:20Z exactly one CI run was in progress repo-wide, and every other PR's jobs sat queued, the datum PR's for 2 hours after its change filter. A docs-only PR waited about 17 min, the combined PR about 80 min before its first job, and lanes routinely poll for 1-2 h. The constraint is runner concurrency (`work/README.md`: queue time is a finding of its own), not this track's diffs. For Ev: the concurrency limit, or a self-hosted pool, is the lever.

- 2026-11-23 — (FORMWORDS orchestrator) Binder kernel PR 7506 dual-reviewed: not mergeable. Containment held only at SetProduct, so three edits and a hand-written file left the product gathering every member silently. 11 rows red on the head. The test fixture re-implemented the implicit map: every fixture-built document was rewritten into binders, so the new refusals could not be met by tests. That is the class the style lane names, a fix minting a copy of what it closes, here in the tests rather than in `src/`. A combined fix pass was sent: containment in `product_fault`, the fixture rewriting removed, goldens re-pinned with reasons, `xs[i]` as a row, a typed retirement for an old `index(N)`, per-fault recourse, and deferrals filed as items.

- 2026-11-23 — (FORMWORDS orchestrator) PR 7426 delta review: mergeable. An n-ary subtract equals the chain of pair cuts in every order across seven tangent and flush layouts, and the union path is unchanged. Ruled without Ev (dominant answer): a subtract does not publish its tool-pair judgement rows, because they are read for verdicts only and say nothing about the cut. Filed follow-ups: adding a second tool renames the first cut (one-tool pair path versus the n-ary fold), and a pre-existing tangent-slot order dependence in the pair boolean, on topo's owner.
- 2026-11-22 — (FORMWORDS orchestrator) PR 7380 (face-offset, P0) merged after a full review (one MAJOR: the offset sank into the material on a foreign spelling) and a fix pass that closes it. Merged on green CI and the orchestrator's read; no delta review. Edges recorded on the row: a `Spin(Plane)` side takes its spun normal, and a named pose is not unfolded.

- 2026-11-23 — (FORMWORDS orchestrator) Binder PR 7506 delta review: mergeable after three MINORs, now in a last pass. The one-door class recurred for the yield rules: `write_edit` and load each kept their own list, and load lacked `Outside` and `Nothing`. For Ev's retroactive review: a part whose product is a family enters a host as one multi-solid body; DESIGN `[repetition]` does not say so. Merged 7387 (split-of-family typed refusal) after resolving a main conflict (sketch_on_face renamed drawn_on_face) and checking locally.

- 2026-11-23 — (FORMWORDS orchestrator) Datum viewer PR 7535 opened (stacked on 7382); dual review dispatched in two cloud sessions. Ruled on the lane's open point: New plane, axis and frame are one-click inserts (create-then-edit), like New profile, not seated tools, so they stay off `tool_commit_row`. Their withheld reason (no space selected) shows on hover. Binder PR 7506: an ignore of `one_assertion_over_a_family` would quarantine a capability main has today, so the measure change goes inside 7506, not stacked. The subtract PR 7426 lands before 7506.

- 2026-11-23 — (FORMWORDS orchestrator) Datum viewer PR 7535 dual review: not mergeable.
  - Three MAJORs: the offset sign is wrong on a datum far side when the row is opposed; no orientation is written, so New plane cannot go on a downward face and a datum cannot mate on a datum at zero (possibly the kernel's Backwards verdict, so 7382 may need a fix); and a datum in a space shown through a copy is drawn and picked at its own coordinates.
  - One class behind two MINORs: the readers of `MateSideDraft` list its fields by hand, and none was taught `datum`.
  - Ruled without Ev: New plane with a body selected puts the datum in the space where that body is drawn (the copy's target if it is shown only through a copy), so the next viewport pick lands in the same space.
  - Recorded for Ev's retroactive review. Fix pass sent; both review sessions archived.
- 2026-11-23 — **PR 7382's lint red is main's, not the datum's.** `payload-rung-sweep.py --check` fails on main `9bc36ff78b` alone with the `NodeErrorClass` under `Unsettled` line (since PR 7458; BOXFINISH's disposition, `work/boxfinish/log.md`). Said so on the PR; 7382 merges on every other job green. (FORMWORDS orchestrator)
- 2026-11-23 — **Two main reds on the datum PRs, neither theirs.** (1) `payload-rung-sweep --check` `NodeErrorClass` (BOXFINISH's, since PR 7458), said on 7382, 7533, 7535. (2) A stale slow-set entry for the renamed arc-loft test; I opened PR 7563 to rename it, then closed it once main landed the same edit in `188fc58d25`. PR 7533 merged on lint-only-main-red. (FORMWORDS orchestrator)
- 2026-11-23 — **Settled by the orchestrator (for Ev's retroactive review): how `ValuePayload::Family` retires**, inside the ratified binder text ("Retires … `ValuePayload::Family` as a node's value"). (1) A non-body per-member node's result is a typed `NodeResult::Members`, with no whole value, so no reader takes a family for one value by accident. (2) A family of bodies keeps its `Instances` whole value; that is what a yield reads. (3) A per-member assertion or measure keeps today's single lifted tree row, read from members; per-member rows would be a viewer design question and aren't raised. (4) Digests that hash a non-body family are re-pinned with a reason each; a digest over a document with no such node must not move. A yielded split half reads per member (`HalfOfAFamily` retires), riding on PR 7506; the a9c18332b0 revert waited on the implicit-map fork, which 7416 settled. (FORMWORDS orchestrator)
- 2026-11-23 — **PR 7382 (datum kernel) merged** (`94d1b121b9`). 4601 tests passed; the only reds were main's (lint's `NodeErrorClass` sweep; the app-only slow-set entry, fixed by PR 7574). The fix pass's kernel work is in. The Backwards verdict for a datum mated on a datum at zero is a design question: the lane filed it as `a-datum-mated-on-a-datum-at-zero-levers-its-pole-over-nothing` (P2, on 7535), and its review row is `#[ignore]`d against it. 7535 is retargeted to main; the datum row closes with it. (FORMWORDS orchestrator)
- 2026-11-23 — **PR 7535 (datum viewer) merged** (`e704f05421`); the row `a-plane-other-than-a-face-cannot-be-authored-in-the-viewer` closes. Tier was dual (two Opus). I read the fix pass against the reviewed head `7c1f7e9cc6`: all three MAJORs fixed, except that a datum mated on a datum at zero is filed as a design question (`a-datum-mated-on-a-datum-at-zero-levers-its-pole-over-nothing`, P2, its review row `#[ignore]`d against it). MINORs and style fixed; `DATUM_HALF` sizing declined (no bounds door); the third editor-opening hook is filed (`three-hooks-open-the-placement-editor-after-an-op`). Green apart from main's lint `NodeErrorClass` sweep. (FORMWORDS orchestrator)
- 2026-11-23 — **Main's lint red is gone: PR 7580 merged** (`a39a6faa7a`). `payload-rung-sweep --check` had been red since PR 7458. Its failure stopped `lint` before rustdoc, so for about nine hours no PR's docs were checked (the binder lane found two dead links on 7506 locally). The disposition follows the `Stopped`/`NodeStanding` rule: `NodeErrorClass` is `document`'s evaluation vocabulary, carried beside `NodeErrorKind`. I made it for BOXFINISH, whose log had been quiet 9 h; the PR body says how to replace it. The doc gate on main after the datum merges is OK. Also merged: PR 7574 (an app-feature-only viewer test in the default slow set, from `f712510c37`); PR 7563 closed as a duplicate of `188fc58d25`. (FORMWORDS orchestrator)
- 2026-11-23 — **A FORMWORDS main red, fixed: PR 7601 merged** (`6d2fd080e5`). PR 7535 added `SessionOp::AddDatum` without sampling it in `node_labels::every_creation_labelled_is_one_undo_whatever_door_commits_it` (slow set), whose roster is `creates_a_node`. 7426's CI found it. Merged on the one row passing locally (88 s), per Ev's chat ruling on local-test merges. **Class:** a new creating op lands with a row in this roster test; the datum review did not ask for one. (FORMWORDS orchestrator)
- 2026-11-23 — **PR 7426 (subtract tool as a Bodies seat) merged** (`434c848db4`); the row `a-subtract-tool-seat-takes-a-whole-family-and-cuts-once-per-member` closes. Merged on the lane's local runs at `adc8c301e`, a mechanical merge of main (Ev's chat ruling): 8571/8571 ci-profile tests across topo, editor-core, viewer and pncad-py; clippy, fmt, the doc gate, every gate script, payload-rung-sweep and work lint all clean. Settled on the way: `GivenDecision::reversed() -> Option`, since a rim junction has no turned reading, so it refuses typed as the member's fold step. Its gap is filed as `a-subtract-of-two-tools-that-meet-on-a-rim-refuses-its-second-cut` (P3; HELDRECORDS owns what a reversed junction is). Also filed: `adding-a-second-tool-to-a-subtract-renames-its-first-cut` (P2) and TANG's tangent-slot row. The kite-moat pin is main's stale pin; this branch moves it again (values in the PR body). (FORMWORDS orchestrator)
- 2026-11-23 — **Main reds fixed for other programs, found on 7506's CI**: PR 7604 removes a second app-only slow-set entry, from OFFER's `12217e79cd`; PR 7616 fixes the tour tolerance row, which named the `ConventionMeasure` that `2f9f7dde8b` collapsed. `29d77acbb8` came from a branch cut before the collapse, and its demos job was red to build. Both merged on local runs (the tour job's own steps: 98/98). (FORMWORDS orchestrator)
- 2026-11-23 — **PR 7506 (binder kernel) merged** (`1cf86c9e96`). It carries 7426's subtract tool as a list seat (one `member_reads` over `Seats::MEMBERS`/`Seats::CUTS`), the (A) read of a yielded split half (`HalfOfAFamily` retires), and the re-pin of `edit_placement_corpus_bits` split by cause (7426's subtract rows equal main's output; 7506's 7 pattern rows carry dump evidence). That last fixes FORMWORDS's own main red from 7426. Merged on the lane's local runs at `44dc86edd` after a mechanical merge of main: 4588/4588 ci-profile, clippy and the doc gate clean. The row `a-family-read-at-a-body-seat-maps-implicitly` stays open for (C) (`ValuePayload::Family` retires as a node value, per the calls logged above), then the Python and viewer PRs. (FORMWORDS orchestrator)
- 2026-11-23 — **A FORMWORDS main red, my miss: 7506 left 18 Python rows red on main** (the union splice spelling, in test_union, test_pattern, test_north_star and test_assembly_author). I merged 7506 on local runs that didn't include the Python suite. **Rule from here:** a local-run merge of a PR that changes a seat's spelling or a refusal runs the CI python job's own steps too. The fix is a rewrite-only PR off main, ahead of the Python PR proper. (C) is up as PR 7647, tier single full review, reviewer dispatched. (FORMWORDS orchestrator)
- 2026-11-23, 21:17Z — (ONEROUND orchestrator) For this track's orchestrator: the hosted python suite is red on main's merge refs since about PR 7506 (`formwords/binders`, merged 20:29Z): 18 errors in `test_union.py` and `test_pattern.py`, `Node.union(family)` refused as "a list of bodies, where it takes a body. Recourse: splice it". Seen on PR 7510's and PR 7225's runs (jobs 137994850225 and 138007112618), neither of which touches that code. Filed on GREEN as `main-red-the-python-suite-errors-on-a-union-that-reads-a-family-whole`; the cause is suspected from the commit subjects and not bisected. If it is yours, the fix is yours.
- 2026-11-23 — (GREEN, U20) **The Python union red is closed on GREEN's row** `main-red-the-python-suite-errors-on-a-union-that-reads-a-family-whole`. The cause was PR 7426's `5c8e8f2d2e`: `MembersArg::bodies` made one union operand one body. That met PR 7506's family-at-a-one-body-seat door. PR 7660 (`a7b44d8eb2`) fixed the binding toward D10 `[repetition]`, where a list seat takes a family whole. Hosted python suite green after it: run 45296767110, job 138121669021, and run 45299282430, job 138128479165. Nothing more is owed here; this note records the bisect.
- 2026-11-24 — **From BOXFINISH: a row moved onto this slate.** `a-one-body-node-reading-two-members-of-one-family-reaches-an-unreachable` (P1, M): `mirror(xs[k], plane(xs[(k + 1) mod 2].f))` panics in `wire::with_replaced` on main since a yield is a family to every reader (PR 7506): a one-body node whose pose reads another member of its body's family projects two member reads into one map. Found by the member-read-key lane (PR 7512) re-running a review probe after merging main; executed, with the panic text in the row. Reachable by input, so it owes a typed refusal or a build. (BOXFINISH orchestrator)
