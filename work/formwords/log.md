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

- 2026-11-22 — (FORMWORDS orchestrator) PR 7380 (face-offset, P0) merged after a full review (one MAJOR: the offset sank into the material on a foreign spelling) and a fix pass that closes it. Merged on green CI and the orchestrator's read; no delta review. Edges recorded on the row: a `Spin(Plane)` side takes its spun normal, and a named pose is not unfolded.
