# AUTHOR — the plan

The doors a person needs to build geometry without leaving the viewer.

Opened 2026-09-20 by CHROME's priority-seam cut (`work/README.md`,
Track size). Nothing dispatched yet.

## The slate

**24.5 budget points** of dispatchable work against a ceiling of 30 —
one sitting, which is what the cut was for.

| pri | item | cost | title |
|---|---|---|---|
| P0 | `a-negative-extrude-distance-probes-as-valid` | E | The probe reports every negative extrude distance as valid, so a length field's only floor is the single point zero |
| P0 | `add-parameter-form-authors-canonical-only` | E | The add-parameter form authors only the canonical unit, though the kernel's written_length/written_angle doors are total |
| P0 | `add-profile-mints-no-frame` | D | The add-profile form cannot mint the frame it needs, and names the ones it finds by node number |
| P0 | `add-profile-placement-on-picked-face-frame` | H | Nothing in the viewer can mint a Datum::FaceFrame, so a profile still cannot be placed on a picked face |
| P0 | `addboolean-doc-names-a-vocabulary-that-does-not-exist` | D | SessionOp::AddBoolean's doc promises a declaration vocabulary that no DocEdit provides |
| P0 | `parameter-row-field-has-no-text-door` | D | A parameter row's value field is a bare DragValue — no parser, no unit authoring, no no-op guard |
| P0 | `path-preview-draws-nothing-for-a-refused-step` | D | viewer: the add-profile path preview draws nothing once any authored step refuses, so the author cannot see what to fix |
| P0 | `the-gui-shows-no-measure-value-and-no-clearance` | D | the GUI shows a measure's existence and never its value |
| P0 | `viewer-cannot-author-a-duplicate-node` | D | viewer: a 'duplicate' node reachable from the UI (Ev's request) |
| P0 | `viewer-cannot-author-a-part-node` | D | The viewer has no AddPart op, so a Part { Instance(i) } node — the road to a nested copy the mate tool now admits — is reachable only from a file or the Python API |

## Order

**`add-profile-placement-on-picked-face-frame` first, and nothing else
before it.** — TAKEN, 2026-09-21, as **AUTH-1** (`docs/AUTH-1-SPEC.md`,
branch `author/face-frame-seat`). It went out as ONE unit, not two:
the sibling's two halves are a `commit_action` gesture and a picker
label, and neither is needed for a face frame to exist, so cutting
them in gave the lane a second form to change for no part of the wall
this row is about. The sibling runs next and is better informed for
it — a face frame's honest label cannot be written before a face frame
can be minted.

Placing a sketch on a face you picked is the most ordinary authoring
gesture there is and the viewer cannot do it at all, which is why this
row leads. `add-profile-mints-no-frame` is the same wall from the
form's side, and is **next**.

Then the two node-kind gaps (`viewer-cannot-author-a-part-node`,
`viewer-cannot-author-a-duplicate-node` — the second is Ev's own
request and is the cheaper of the pair), then
`parameter-row-field-has-no-text-door`, which is what makes every
other authored value un-typeable.

`addboolean-doc-names-a-vocabulary-that-does-not-exist` waits on
EDIT's `DocEdit` vocabulary rather than on anything here; ask before
specifying it. The two E rows
(`add-parameter-form-authors-canonical-only`,
`a-negative-extrude-distance-probes-as-valid`) are drive-bys for
whoever is next in `forms.rs` and `props.rs` — take them there rather
than dispatching them (`work/README.md`, "The tracker is not
comprehensive").

## Review posture

**Answered 2026-09-21, at AUTH-1's dispatch: CHROME's inherited
posture, adopted unchanged.** No duals; a style review
(`docs/prompts/reviewer-style-lane.md`) plus a second correctness
reviewer on the units whose failure mode is a confident wrong answer
rather than a refusal. Every row on this slate is such a unit — a door
that authors a node wrong is a model that is quietly not what the
author drew, where a door that refuses is visible — so the second
correctness arm is this program's default rather than its exception,
and a unit that drops it says why in its spec. No A/B row on any
dispatch while the account's Fable budget is spent
(`docs/MODEL-AB-LOG.md`); the band 6800-6899 stays claimed and
unspent.
