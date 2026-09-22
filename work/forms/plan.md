# FORMS — the plan

the creation forms' vocabulary and the seats they gate

Opened 2026-09-22 by CHROME's priority-seam cut (`work/README.md`,
Track size). Nothing dispatched.

## The slate

**23 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P1 | `a-creation-forms-held-pick-survives-a-document-swap` | D | A creation form's held pick survives a document swap and resolves against the new document |
| P1 | `a-fifth-spelling-of-this-seat-is-empty` | D | A fifth spelling of 'this seat is empty' |
| P1 | `body-seat-reads-through-the-placer-chain` | D | denotes_body must read the body seat through the placer chain |
| P1 | `denotes-body-enumerates-its-gaps-against-the-operand-door-and-misses-one` | D | denotes_body names two of its differences from the operand door and there is a third |
| P1 | `dimension-to-unit-ladders-have-six-homes-in-the-viewer` | D | Six Dimension-to-unit ladders in the viewer |
| P1 | `drag-tick-has-three-homes` | E | The drag tick has three homes and a count field has two answers |
| P1 | `four-pick-state-vocabularies-in-one-create-module` | D | create.rs answers 'what is this button waiting for' in four vocabularies |
| P1 | `probe-seed-respells-props-authored-in` | E | probe_seed re-spells props::authored_in by hand |
| P3 | `the-circle-split-cap-offers-counts-the-document-refuses` | D | The circle_split count cap offers counts the document escalates on |
| P3 | `the-kernel-takes-any-count-has-four-homes-in-the-viewer` | D | The circle_split count rule is restated at four viewer sites |
| P4 | `a-per-kind-sentence-lives-in-the-widget-not-on-the-choice` | E | A per-kind sentence is hardcoded in the widget rather than beside its choice |

## Order

**The one live wrong answer first**:
`a-creation-forms-held-pick-survives-a-document-swap` — a pick held
across a document swap resolves against the NEW document, which is a
confident wrong answer rather than a refusal and wants a correctness
arm on its review.

**Then the seat gate, as one unit**: `body-seat-reads-through-the-placer-chain`
and `denotes-body-enumerates-its-gaps-against-the-operand-door-and-misses-one`
are the same function and the same repair — read the family through
the placer chain, as `editor-core`'s `eval::node_value_kind` already
does. AUTH-4 (#3052) added a third seat with the same shape
(`NodeKindWanted::Instances`) and pinned it as a named exception in
`crates/viewer/tests/combine_ops.rs`; the unit retires both exceptions.
Take it after #3052 lands.

**Then the vocabulary rows**, cheapest first: `drag-tick-has-three-homes`
and `probe-seed-respells-props-authored-in` (both E, fix written in the
row), then `four-pick-state-vocabularies-in-one-create-module` with
`a-fifth-spelling-of-this-seat-is-empty` (both are "what does this
form say about its seats", and one pass over `create.rs` serves both),
then `dimension-to-unit-ladders-have-six-homes-in-the-viewer`.

**The circle-split pair goes together**, count rule before cap.
`a-per-kind-sentence-lives-in-the-widget-not-on-the-choice` rides with
whichever unit next edits `add_datum_ui`.

## Review posture

OPEN, for this program's first dispatch. CHROME's posture is the
natural inheritance — **no duals and no row in `docs/MODEL-AB-LOG.md`**
(Ev, in chat, 2026-09-04, reaffirmed 2026-09-15 and 2026-09-22: *"no
AB protocol"*), a style review against
`docs/prompts/reviewer-style-lane.md`, and a correctness arm where a
unit's failure mode is a confident wrong answer — but nobody has
re-asked it for this slate, so the first orchestrator answers it here.
