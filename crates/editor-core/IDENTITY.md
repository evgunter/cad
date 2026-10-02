# A held value names the world it came from

This page is the ratified design of record for identity across time in
the editor-core document layer: which world a held id, a memo entry, an
evaluation, a file and a gesture each belong to. It answers DOCM's
identity questions — layer-3 identity across rewinds, document
identity, the memo and the store, free-move commit — as one
conversation, because each is the same defect: a held value with no
witness of which world it belongs to. Ev ratified DI1–DI5 in chat on
2026-09-04, DI5 in the stronger reading stated below. The page was
`docs/DOCM-IDENTITY-DESIGN.md` until DOCM's exit on 2026-09-13, when it
moved beside the code it governs. The reference vocabulary is the
companion page
`crates/editor-core/REFERENCES.md`. Mechanics here are measured, not
assumed; where a file:line has drifted, the name beside it is the
stable half.

## 0. Grounding (committed elsewhere; not re-litigated here)

- **Identity and version are distinct** (`crates/editor-core/ASSEMBLY.md`
  A4): `DocumentId` answers which part and survives every edit;
  `ContentPin` is the SHA-256 of the canonical bytes and answers
  which version; `DocRef` pairs them; the store returns a document
  only when its bytes hash to the pin (`workspace.rs`), and a
  moved pin is a recorded edit (A13, `UpdateReference`).
- **The document is a value and undo is keeping the old value**
  (G1, `crates/viewer/README.md`). `viewer::history` retains every
  `Doc` an action produced and never replays (`history.rs`); undo and
  redo move a cursor. `Doc::mint`, the chain node and step ids are
  minted from, is part of that value (`doc.rs`).
- **The memo is per node, keyed by content and naming keys**
  (`eval/mod.rs`), and an `InstantiatePart` node's content key
  hashes its `DocRef`, its solved placement and its interface, and
  nothing about the store — "the pin IS the referenced content".
- **Determinism** (D9): same build, same inputs, same outputs. The
  store is an input of an evaluation only where the evaluation
  crosses the seam.
- **Free-move is display state** (G3): a display frame over an
  instance's placement, no solver, admitted only for an instance no
  mate names (`display.rs`). `DocEdit::SetOffset` exists and sets the
  instance's offset in its gauge (A11 (2)).

## DI1 — A node id names one authored node wherever it is minted

`RecipeNodeId` is a digest of the document's mint chain (N1): the
chain digests every minting edit since the empty document, and an
insert's id digests the chain with the node as authored, its inputs'
and names' ids included. So every document whose mint log holds an id
holds the same insert of the same node, after the same minting edits,
whichever history branch, file or session it came from; a different
insert mints a different id. An undo past an insert followed by a
different insert therefore mints a new id, and the undone node's id is
foreign on that branch; an undo followed by the byte-identical insert
re-mints the same node, which is the same node.

The rule, one for every holder — `Selection::Node`,
`FaceSelection::node`, the seats behind the revolve and combining
tools, `BlendTarget::node`, every held `StableName`, the API's
callers: **a holder keeps the bare id, and asks the document in hand.
An id the mint log does not hold is foreign; one it holds that is not
live is deleted (`Doc::has_minted`, `Doc::node`); a live one is the
node.** History position is not part of the question.

- Undoing past the mint drops a pick, redoing restores it, and a
  sibling branch's different insert leaves it foreign — each because
  of what the document in hand holds, not because of how the user
  navigated there.
- A history REPLACEMENT (`Open`, `NewDocument`) clears every holder,
  tools included, the way it clears selection (`session.rs`): held
  picks are session state about the document replaced.
- What this rule does not touch: a live node whose geometry changed
  under a held name. That is `Standing`'s per-frame question and
  stays so.

*Record: the build is VIEW's, `layer3-recipenodeid-aliases-across-rewinds`:
the tools' clear on replacement and the rows pinning the rule.*

## DI2 — The memo is a pure function of the document; the store is the session's

An instantiate node served from the memo is never consulted against
the resolver, so an evaluation with a prior would otherwise serve a
part whose file has since changed or vanished while the same
evaluation without a prior refuses `PinMismatch` or `Unresolved`. Of
the two answers — memo admission checks resolver state, or the memo is
a pure function of the document and the store is checked by whoever
mounts it — **the second is ruled.** The memo's claim is "same content
key, same value", and for an instantiate node the pin is the content:
the served value is exactly what the document pins. Putting store
state into admission would make `evaluate` with a prior depend on the
filesystem, against D9, and cost a seam crossing per reused node.

Consequences:

- **A4's refusal sentence is the seam's, and only the seam's**: an
  evaluation that crosses the seam refuses a moved pin.
  `crates/editor-core/ASSEMBLY.md` A4 and the `pncad-py` audit page
  say so in those words.
- **The session owns store freshness**, and gates the memo on
  resolver identity (`evalseam.rs`, `same_resolver`): a replaced
  resolver is a full re-evaluation. What it lacks is a signal when the
  mounted directory's CONTENTS change. The smallest complete door,
  and the one this design takes: **`SessionOp::Reevaluate` re-mounts
  the store** — a fresh `DirResolver` over the same directory — which
  through the existing gate re-evaluates fully, while ordinary edits
  keep the memo. That door is CHROME's build, at the head of
  `document-seam-no-in-session-change-detection`; the item's two
  adjacent edges (save-as into a partless directory, the chooser's
  vocabulary) are chrome too and travel with DI4.
- The session hands no previous evaluation as `prior`: `request_eval`
  carries none, and the memo lives in `evalseam::PriorRun`. Only
  `probe_bounds` hands a prior directly, and it hands the landed
  evaluation of the same document.

*Record: the memo half and A4's narrowed sentence are DOCM-4 (PR
1808), with DI3; the re-mount door is open at
`document-seam-no-in-session-change-detection`.*

## DI3 — An evaluation carries its document's identity

A pairing door takes a document plus a value that must be OF that
document, and without a stamp nothing can check the pairing;
mispairing would be silent misbehaviour. **`Evaluation` carries
`document: DocumentId`**, stamped by `evaluate` from `doc.id()`, and
every door that takes the pair refuses a mismatch typed
(`ProductError::EvaluationOfAnotherDocument { expected, found }` and
its siblings). Which doors those are is
`crates/editor-core/ASSEMBLY.md`'s A2a, the one place that list is
written. The version half is not stamped: within one document,
the per-node content keys already decide reuse, and a pin per
evaluation would cost a canonicalization per run for a check the keys
make. The memo lookup itself (`prior.nodes.get(&id)`) makes the same
id check, so a prior from another document is refused rather than
mined for coincidental hits.

*Record: built by DOCM-4 (PR 1808); which doors read the stamp is
`crates/editor-core/ASSEMBLY.md` A2a.*

## DI4 — Saving at a path never forks identity; forking is its own act

Identity is the document's, not the file's (A4), so a save door cannot
mint a fresh id without forking — and a directory holding one id under
two filenames refuses `DuplicateId` for every resolution through it
(`workspace.rs`), so writing the same `DocumentId` to a second file
beside the original is not a save the store can serve. Two acts, both
typed:

- **`Save { path }`** keeps the id, and **refuses**
  `SaveWouldDuplicateId { path, other }` when the target directory
  already holds this id under another filename — the store's own
  scan answers that before anything is written. No warning-then-
  continue.
- **`SaveAsNewDocument { path, name }`** forks: a new `Doc` value
  with a freshly minted id (derived from `name` as `NewDocument`
  does, or random through the workspace door) and the same content,
  a fresh history rooted at it, and the resolver rebound to the
  path's directory. Inbound `DocRef`s to the old id do not follow,
  which is what a fork means.

*Record: the build is CHROME's, landed 2026-09-08 in
`crates/pncad/src/workspace.rs` — the two acts are `Workspace::save_at`
and `Workspace::save_as_new_document`, the refusal
`WorkspaceError::SaveWouldDuplicateId { id, existing }` — closing
`save-a-copy-duplicate-id-bricks-store`.*

## DI5 — Releasing a free-move gesture is the placement edit

G3 ratified free-move as display state that is never persisted, and
excluded committing it from v1's scope. **The viewer may record a
free-moved placement persistently**, and the reading taken is the
stronger one, under G1's preview-versus-commit rule: the gesture's
previews stay display frames, and **its release emits one
`DocEdit::SetOffset`** on the instance — one undo step, one
document transition, and the placement survives save and reopen,
which is what a user expects of a part they placed. Consequences:

- `CommitFreeMove` is the committed edit; `moves` in `DisplayState`
  (`display.rs`) empties, since a committed frame is document data.
  `hidden` stays display state.
- Admission is unchanged: only an instance no mate names may be
  free-moved (`free_move_check`), so the edit's target is a singleton
  group and roots it (A11 (3)). A later mate that places it on another
  group clears its offset, as the mate door does for any first
  operand (A11 (2)).
- G3's sentence "hiding and free-move are display state, never
  persisted" narrows to hiding; `crates/viewer/README.md`, the
  `display.rs` module doc and the round-trip row that pins the
  boundary follow the edit.

*Record: the build is CHROME's and is not in the tree —
`no-persistent-setplacement-session-op` is open, the whole free-move
family in `SessionOp` is still display-only, and `display.rs` still
states the old boundary.*

## What this doc does not touch

The reference vocabulary (`crates/editor-core/REFERENCES.md`); the
instantiation seam's mate-identity channel; the check registry's
subject; the certified range query. Every viewer build named above
is CHROME's or VIEW's.
