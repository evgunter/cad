# DOCM-IDENTITY-DESIGN: a held value names the world it came from

Status: **RATIFIED in-chat (Ev, 2026-09-04): DI1–DI5.** The PR that
carries this doc is the record; DI5's stronger reading (below) is
stated as the interpretation taken and stands unless Ev says
otherwise. Companion-table row at `docs/DESIGN.md`. This doc answers
DOCM's identity questions (`work/docm/plan.md`: layer-3 identity
across rewinds, document identity, the memo and the store, free-move
commit) as one conversation, because each is the same defect: a held
value with no witness of which world it belongs to. Mechanics are
measured (file:line at the time of writing), not assumed.

## 0. Grounding (committed; this doc does not re-litigate)

- **Identity and version are distinct** (ASSEMBLY-DESIGN A4):
  `DocumentId` answers which part and survives every edit;
  `ContentPin` is the SHA-256 of the canonical bytes and answers
  which version; `DocRef` pairs them; the store returns a document
  only when its bytes hash to the pin (`workspace.rs:340`), and a
  moved pin is a recorded edit (A13, `UpdateReference`).
- **The document is a value and undo is keeping the old value**
  (GUI-DESIGN G1). `viewer::history` retains every `Doc` an action
  produced and never replays (`history.rs:60`); undo and redo move a
  cursor. `Doc::next_id` is part of that value (`doc.rs:313`).
- **The memo is per node, keyed by content and naming keys**
  (`eval/mod.rs:1955`), and an `InstantiatePart` node's content key
  hashes its `DocRef`, its solved placement and its interface, and
  nothing about the store — "the pin IS the referenced content"
  (`eval/mod.rs:2271`).
- **Determinism** (D9): same build, same inputs, same outputs. The
  store is an input of an evaluation only where the evaluation
  crosses the seam.
- **Free-move is display state** (GUI-DESIGN G3): a display frame
  over an instance's placement, no solver, admitted only for an
  instance no mate names (`display.rs:351`). `DocEdit::SetPlacement`
  exists, keys on the cluster gauge (A11), and the viewer never emits
  it (`edit.rs:227`; grep of `crates/viewer/src`).

## DI1 — A held node id is valid on the branch that minted it

`RecipeNodeId` is a small per-document counter, and because
`next_id` is part of the `Doc` value, an undo past a node's insert
followed by a fresh insert re-mints the same id on a sibling branch
of the history. Layer-3 state that holds an id across turns —
`Selection::Node`, `FaceSelection::node`, the seats behind the
revolve and combining tools, `BlendTarget::node`, and every held
`StableName`, since a name embeds its minting node — then denotes a
different node with no refusal (`seats.rs:67`, issue 1384).

The rule, one for every holder: **an id denotes the same node iff
the current history entry descends from the entry that minted it,
in the same history, and the node is live.** Along any forward path
from the mint the counter is monotone, so the id cannot be re-minted;
on any other branch it can. A hold therefore carries the id plus its
minting entry, which the history computes at pick time by walking up
until the counter drops below the id (`History::entry`,
`Doc::next_id`), and the per-frame `reconcile` / `standing` checks
descent before liveness.

- Undoing an unrelated later edit keeps a pick valid; undoing past
  the mint invalidates it; redoing onto the original branch restores
  it; a sibling branch that re-mints the id is refused.
- A history REPLACEMENT (`Open`, `NewDocument`) is not a rewind:
  entry ids are indices a fresh history reuses from zero, so tools
  clear on replacement the way selection already does
  (`session.rs:2641`, `:2706`). Selection keeps surviving undo:
  `Standing` stays a state, not an event (`session.rs:286`).
- Headless callers have no history; for them the rule is the
  documented obligation that an id is comparable only along a path
  of forward `apply`s.
- What this rule does not touch: a live node on the right branch
  whose geometry changed under a held name. That is `Standing`'s
  per-frame question and stays so.

The build is VIEW's (`layer3-recipenodeid-aliases-across-rewinds`).

## DI2 — The memo is a pure function of the document; the store is the session's

An instantiate node served from the memo is never consulted against
the resolver, so an evaluation with a prior serves a part whose file
has since changed or vanished, while the same evaluation without a
prior refuses `PinMismatch` or `Unresolved`
(`memo-admission-and-resolver-state`, measured). Two answers were
proposed: memo admission checks resolver state, or the memo is a pure
function of the document and the store is checked by whoever mounts
it. **The second is ruled.** The memo's claim is "same content key,
same value", and for an instantiate node the pin is the content: the
served value is exactly what the document pins. Putting store state
into admission would make `evaluate` with a prior depend on the
filesystem, against D9, and cost a seam crossing per reused node.

Consequences:

- **A4's refusal sentence narrows** to: an evaluation that crosses
  the seam refuses a moved pin. `crates/editor-core/ASSEMBLY.md` A4
  and the `pncad-py` audit page say so in those words (the audit
  page already does, from LIB-G18a).
- **The session owns store freshness**, and already gates the memo
  on resolver identity (`evalseam.rs:174`, `same_resolver`): a
  replaced resolver is a full re-evaluation. What it lacks is a
  signal when the mounted directory's CONTENTS change. The smallest
  complete door: **`SessionOp::Reevaluate` re-mounts the store** — a
  fresh `DirResolver` over the same directory — which through the
  existing gate re-evaluates fully, while ordinary edits keep the
  memo. This is the head of
  `document-seam-no-in-session-change-detection`; the item's two
  adjacent edges (save-as into a partless directory, the chooser's
  vocabulary) are chrome and go to CHROME with DI4.
- The premise in the item that the session hands the previous
  evaluation as `prior` is stale: `request_eval` carries none
  (`session.rs:3030`); the memo lives in `evalseam::PriorRun`. Only
  `probe_bounds` hands a prior directly (`session.rs:2321`), and it
  hands the landed evaluation of the same document.

## DI3 — An evaluation carries its document's identity

`Evaluation` carries an `Epoch` and nothing that says which `Doc` or
which resolver produced it (`eval/mod.rs:55`). `product`, `assemble`
and `SolvedPoses::placement` each take a document plus an evaluation
that must be OF that document, and nothing can check the pairing;
mispairing is silent misbehaviour (the LIB comment on
`memo-admission-and-resolver-state`). **`Evaluation` gains
`document: DocumentId`**, stamped by `evaluate` from `doc.id()`, and
every door that takes the pair refuses a mismatch typed
(`ProductError::EvaluationOfAnotherDocument { expected, found }` and
its siblings). The version half is not stamped: within one document,
the per-node content keys already decide reuse, and a pin per
evaluation would cost a canonicalization per run for a check the keys
make. The memo lookup itself (`prior.nodes.get(&id)`) adds the same
id check, so a prior from another document is refused rather than
mined for coincidental hits.

## DI4 — Saving at a path never forks identity; forking is its own act

Save-as beside the original writes the same `DocumentId` to a second
file (`docio.rs:166`; the id is set only at construction,
`doc.rs:377`), and the directory then refuses `DuplicateId` for every
resolution through it (`workspace.rs:83`). Identity is the
document's, not the file's (A4), so the save door cannot mint a fresh
id without forking. Two acts, both typed:

- **`Save { path }`** keeps the id, as today, and **refuses**
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

The build is CHROME's (`save-a-copy-duplicate-id-bricks-store`).

## DI5 — Releasing a free-move gesture is the placement edit

G3 ratified free-move as display state that is never persisted, and
excluded committing it from v1's scope. **Ruled (Ev, 2026-09-04): the
viewer may record a free-moved placement persistently**, and the
reading taken is the stronger one, under G1's preview-versus-commit
rule: the gesture's previews stay display frames, and **its release
emits one `DocEdit::SetPlacement`** on the instance — one undo step,
one document transition, and the placement survives save and reopen,
which is what a user expects of a part they placed. Consequences:

- `CommitFreeMove` becomes the committed edit; `moves` in
  `DisplayState` (`display.rs:434`) empties, since a committed frame
  is document data. `hidden` stays display state.
- Admission is unchanged: only an instance no mate names may be
  free-moved (`free_move_check`), so the edit's target is a singleton
  cluster and keys on itself as gauge (A11 rule 3). A later mate that
  joins clusters re-keys the record through `ClusterMaintenance`
  as any placement is today.
- G3's sentence "hiding and free-move are display state, never
  persisted" narrows to hiding; `crates/viewer/README.md` and the
  `display.rs` module doc say so, and the round-trip row that pinned
  the old boundary flips to pin the new one.

The build is CHROME's (`no-persistent-setplacement-session-op`).

## What this doc does not touch

The reference vocabulary (`docs/DOCM-REFERENCES-DESIGN.md`); the
instantiation seam's mate-identity channel; the check registry's
subject; the certified range query. Every viewer build named above
is CHROME's or VIEW's.
