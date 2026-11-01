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

- **Identity and version are distinct** (DESIGN `[assemblies.pins]`):
  `DocumentId` says which document, `ContentPin` which version, `DocRef`
  pairs them. The store returns a document only when its bytes hash to
  the pin (`crates/pncad/src/workspace.rs`), and moving a pin is a
  recorded edit (ASSEMBLY A13, `UpdateReference`).
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
- **Free-move is display state** (G3, over shown spaces by DI5): a display
  frame over one shown space's picture, no solver, keyed by that space
  (DI5; `crates/viewer/src/display.rs`).

## DI1 — A node id names one authored node wherever it is minted

DESIGN `[assemblies.pairing]` states the rule for every holder; this is
why it holds and what it covers.

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

The rule is DESIGN `[assemblies.pairing]`; this is how an evaluation
keeps it. **`Evaluation` carries `document: DocumentId`**, stamped by `evaluate` from
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

The rule is DESIGN `[assemblies.saving]`. Identity is the document's,
not the file's, so a save cannot mint a
fresh id without forking. And a directory holding one id under two filenames
refuses `DuplicateId` for every resolution through it (`workspace.rs`), so
writing the same `DocumentId` to a second file beside the original is not a
save the store can serve. Two acts, both typed:

- **Save at a path** keeps the id, and **refuses** `SaveWouldDuplicateId`
  when the target directory already holds this id under another filename —
  the store's own scan answers that before anything is written. No
  warning-then-continue.
- **Save as a new document** forks: the same content under a fresh random
  id, written at the file name the person chose in the target directory's
  store, and the resolver rebound to that directory. The session's history
  is re-identified under the new id as one operation, so an undo after a
  fork never returns the session to the original's identity. Inbound
  `DocRef`s to the old id do not follow, which is what a fork means.

*Built at the workspace door, `crates/pncad/src/workspace.rs`:
`Workspace::save_at`, refusing `WorkspaceError::SaveWouldDuplicateId { id,
existing, requested }` (and `SaveWouldReplaceDocument { id, path }` for a
target holding another document), and `Workspace::save_as_new_document`,
with a random id at the chosen target.*

## DI5 — A free-move frame belongs to the space it was drawn for

Several spaces can be shown at once in the GUI, overlapping. A free-move
repositions the picture of one shown space and changes no geometry in any
space: releasing the gesture writes nothing to the document, and the
release leaves the display frame where it is. The frame is a held value
like any other on this page, so it names the space it was drawn for: it is
keyed by that space, and it is dropped when that key stops naming a shown
space (an undo or an edit removes it; `DisplayState::prune`).
Consequences:

- Keeping a relation means stating it: a placement's mates read poses off
  geometry and its values are numbers the user types. No edit reads the
  display frame, and no gesture writes one into the model.
- A free-move may write display metadata (Ev, PR 5312), so a layout can
  outlive the session. It is viewer data outside the canonical bytes, never
  hashed into a `ContentPin` and never read by evaluation; without it, a
  reopened document draws its shown spaces at a default pose.
- G3's "hiding and free-move are display state, never persisted" stands,
  over shown spaces.

## What this doc does not touch

The reference vocabulary (`crates/editor-core/REFERENCES.md`); the
instantiation seam's mate-identity channel; the check registry's subject; the
certified range query.
