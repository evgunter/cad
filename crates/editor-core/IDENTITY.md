# A held value names the world it came from

This page decides identity across time in the editor-core document layer:
which world a held id, a memo entry, an evaluation, a file and a gesture each
belong to. Each clause answers the same defect — a held value with no witness
of which world it belongs to — for one kind of held value: layer-3 ids across
history rewinds (DI1), the memo and the store (DI2), evaluations (DI3), saved
files (DI4) and the free-move gesture (DI5). The reference vocabulary is the
companion page `crates/editor-core/REFERENCES.md`. File citations name the
symbol beside them; the symbol is the stable half.

## 0. Grounding (committed elsewhere)

- **Identity and version are distinct** (`crates/editor-core/ASSEMBLY.md`
  A4): `DocumentId` answers which part and survives every edit;
  `ContentPin` is the SHA-256 of the canonical bytes and answers which
  version; `DocRef` pairs them. The store returns a document only when its
  bytes hash to the pin (`crates/pncad/src/workspace.rs`), and moving a pin
  is a recorded edit (A13, `UpdateReference`).
- **The document is a value and undo is keeping the old value** (G1,
  `crates/viewer/README.md`). `viewer::history` retains every `Doc` an
  action produced and never replays (`history.rs`); undo and redo move a
  cursor. `Doc::mint`, the chain node and step ids are minted from, is part
  of that value (`doc.rs`).
- **The memo is per node, keyed by content and naming keys**
  (`eval/mod.rs`). An `InstantiatePart` node's content key hashes its
  `DocRef`, its solved placement and its interface, and nothing about the
  store: the pin IS the referenced content.
- **Determinism** (D9): same build, same inputs, same outputs. The store is
  an input of an evaluation only where the evaluation crosses the seam.
- **Free-move is display state** (G3): a display frame over an instance's
  placement, no solver, admitted only for an instance no mate names
  (`display.rs`). `DocEdit::SetOffset` sets the instance's offset in its
  gauge (A11 (2)).

## DI1 — A node id names one authored node wherever it is minted

`RecipeNodeId` is a digest of the document's mint chain (N1): the chain
digests every minting edit since the empty document, and an insert's id
digests the chain with the node as authored, its inputs' and names' ids
included. So every document whose mint log holds an id holds the same insert
of the same node after the same minting edits, whichever history branch, file
or session it came from, and a different insert mints a different id. An undo
past an insert followed by a different insert mints a new id, and the undone
node's id is foreign on that branch; an undo followed by the byte-identical
insert re-mints the same node.

The rule, one for every holder — `Selection::Node`, `FaceSelection::node`,
the seats behind the revolve and combining tools, `BlendTarget::node`, every
held `StableName`, the API's callers: **a holder keeps the bare id and asks
the document in hand. An id the mint log does not hold is foreign; one it
holds that is not live is deleted (`Doc::has_minted`, `Doc::node`); a live
one is the node.** History position is not part of the question.

- Undoing past the mint drops a pick, redoing restores it, and a sibling
  branch's different insert leaves it foreign — each because of what the
  document in hand holds, not because of how the user navigated there.
- A history replacement (`Open`, `NewDocument`) clears every holder, tools
  included, the way it clears selection (`session.rs`): held picks are
  session state about the document replaced.
- The rule does not cover a live node whose geometry changed under a held
  name. That stays `Standing`'s per-frame question.

*Open: the tools' clear on replacement and the rows pinning the rule,
`layer3-recipenodeid-aliases-across-rewinds`.*

## DI2 — The memo is a pure function of the document; the store is the session's

The memo is a pure function of the document, and the store is checked by
whoever mounts it. An instantiate node served from the memo is never
consulted against the resolver, so an evaluation with a prior may serve a part
whose file has since changed or vanished, where the same evaluation without a
prior refuses `PinMismatch` or `Unresolved`. That is the ruled behaviour: the memo's
claim is "same content key, same value", and for an instantiate node the pin
is the content, so the served value is exactly what the document pins. The
alternative, memo admission checking resolver state, would make `evaluate`
with a prior depend on the filesystem, against D9, and cost a seam crossing
per reused node.

- **A4's refusal sentence is the seam's, and only the seam's**: an
  evaluation that crosses the seam refuses a moved pin.
  `crates/editor-core/ASSEMBLY.md` A4 and `docs/guide/north-star-audit.md`
  say so in those words.
- **The session owns store freshness.** It gates the memo on resolver
  identity (`evalseam.rs`, `same_resolver`): a replaced resolver is a full
  re-evaluation. A change to the mounted directory's contents has its own
  door, the smallest complete one: **`SessionOp::Reevaluate` re-mounts the
  store** — a fresh `DirResolver` over the same directory — which through
  the existing gate re-evaluates fully, while ordinary edits keep the memo.
- The session hands no previous evaluation as `prior`: `request_eval`
  carries none, and the memo lives in `evalseam::PriorRun`. Only
  `probe_bounds` hands a prior directly, and it hands the landed evaluation
  of the same document.

*Built: the memo half and A4's sentence (PR 1808). Open: the re-mount door
and its two adjacent edges (save-as into a partless directory, the chooser's
vocabulary), `document-seam-no-in-session-change-detection`.*

## DI3 — An evaluation carries its document's identity

**`Evaluation` carries `document: DocumentId`**, stamped by `evaluate` from
`doc.id()`. A pairing door takes a document plus a value that must be of that
document; without the stamp nothing could check the pairing, and a
mispairing would misbehave silently. Every door that takes the pair refuses
a mismatch typed
(`ProductError::EvaluationOfAnotherDocument { expected, found }` and its
siblings); `crates/editor-core/ASSEMBLY.md` A2a is the one
place that list is written.

The version half is not stamped: within one document the per-node content
keys already decide reuse, and a pin per evaluation would cost a
canonicalization per run for a check the keys make. The memo makes the same
identity check: `evaluate` compares a prior's `document` with the document's
before scheduling, and a prior of another document is dropped — every node
recomputes and `Evaluation::prior_refused` records the mispairing — rather
than mined for coincidental hits.

*Built: PR 1808.*

## DI4 — Saving at a path never forks identity; forking is its own act

Identity is the document's, not the file's (A4), so a save cannot mint a
fresh id without forking. And a directory holding one id under two filenames
refuses `DuplicateId` for every resolution through it (`workspace.rs`), so
writing the same `DocumentId` to a second file beside the original is not a
save the store can serve. Two acts, both typed:

- **Save at a path** keeps the id, and **refuses** `SaveWouldDuplicateId`
  when the target directory already holds this id under another filename —
  the store's own scan answers that before anything is written. No
  warning-then-continue.
- **Save as a new document** forks: a new `Doc` value with a freshly minted
  id (derived from a name as `NewDocument` does, or random through the
  workspace door) and the same content, a fresh history rooted at it, and the
  resolver rebound to the path's directory. Inbound `DocRef`s to the old id
  do not follow, which is what a fork means.

*Built at the workspace door, `crates/pncad/src/workspace.rs`:
`Workspace::save_at`, refusing `WorkspaceError::SaveWouldDuplicateId { id,
existing, requested }`, and `Workspace::save_as_new_document`, with a random
id.*

## DI5 — Releasing a free-move gesture is the placement edit

The viewer records a free-moved placement persistently, under G1's
preview-versus-commit rule: the gesture's previews stay display frames, and
**its release emits one `DocEdit::SetOffset`** on the instance — one undo
step, one document transition — so the placement survives save and reopen,
which is what a user expects of a part they placed. This extends G3, which
made free-move display state, never persisted, and left committing it out of
v1. Consequences:

- `CommitFreeMove` is the committed edit; `moves` in `DisplayState`
  (`display.rs`) empties, since a committed frame is document data. `hidden`
  stays display state.
- Admission is unchanged: only an instance no mate names may be free-moved
  (`free_move_check`), so the edit's target is a singleton group and roots
  it (A11 (3)). A later mate that places it on another group clears its
  offset, as the mate door does for any first operand (A11 (2)).
- G3's sentence "hiding and free-move are display state, never persisted"
  narrows to hiding; `crates/viewer/README.md`, the `display.rs` module doc
  and the round-trip row that pins the boundary follow the edit.

*Open: `no-persistent-setplacement-session-op`. `CommitFreeMove` is still a
display-only op and `display.rs` still states the old boundary.*

## What this doc does not touch

The reference vocabulary (`crates/editor-core/REFERENCES.md`); the
instantiation seam's mate-identity channel; the check registry's subject; the
certified range query.
