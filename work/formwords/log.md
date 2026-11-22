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
