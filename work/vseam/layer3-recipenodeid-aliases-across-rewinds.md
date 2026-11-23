---
id: layer3-recipenodeid-aliases-across-rewinds
kind: issue
title: The RecipeNodeId aliasing hazard is gone under digest ids; the counter-era prose and two of DI1's rows remain
status: open
opened: 2026-08-31
refs: [1375]
priority: P4
cost: E
---

## From GitHub issue 1384

Opened 2026-08-31; 0 comments.

Class defect, found in GAUTH-1 review (PR #1375): `RecipeNodeId` is a small per-document monotone counter, so any layer-3 value that HOLDS one across a document replacement can silently start denoting a different node when fresh inserts re-mint the same small ids.

Demonstrated scenario (reviewer probe, reproduced): fill a `RevolveTool` with picks (profile id 0, axis id 1), perform `SessionOp::NewDocument`, skip `reconcile`, insert a new profile and a new axis into the fresh document (they re-mint ids 0 and 1), commit the stale tool — the session's kind gates PASS (the new ids denote the right kinds) and a real `Node::Revolve` of nodes nobody picked is authored, with no refusal anywhere. The same shape arises from an undo past the picks' inserts followed by new inserts, and from `Open`.

Sweep targets (everything in layer 3 that holds a `RecipeNodeId` across turns):
- `viewer::revolvetool::RevolveTool` (both seats) — module doc now states the hazard honestly and names this issue;
- `viewer::matetool::MateTool` — `FaceSelection::node` inside its held picks (its `StableName` half resolves honestly; the node field is the aliasing half);
- `viewer::session::Selection::Node` (and `FaceSelection::node` inside `Selection::Face`) — `Standing` answers presence by id lookup, so a reused id reads as present.

Per-frame `reconcile`/`standing` guards the deleted-node case only; it cannot distinguish "the same id, a different node". Candidate directions (not adjudicated here): a document identity+generation stamp carried beside held ids and checked at consume time; clearing tools/selection on every history REPLACEMENT (open/new already clear selection — undo/redo do not clear tools); or stable-name-shaped node references for layer-3 holds. Whatever the mechanism, the fix should be one rule for all three holders, not three local patches.

## Ruled — DI1 (2026-09-04)

`crates/editor-core/IDENTITY.md` DI1 adjudicates the candidate directions
above and hands the build here. The rule, one for every holder: **an id
denotes the same node iff the current history entry descends from the
entry that minted it, in the same history, and the node is live.** A
hold carries the id plus its minting entry; `reconcile`/`standing`
check descent before liveness; a history REPLACEMENT (`Open`,
`NewDocument`) clears tools the way it already clears selection, since
entry ids are indices a fresh history reuses from zero.

**DI1's holder set is wider than the sweep list above.** It adds the
seats behind the revolve and combining tools (`seats.rs:67`),
`BlendTarget::node`, and — the clause that widens it most — *every held
`StableName`, since a name embeds its minting node*. The sweep this
unit owes is over that set, not over the three holders this issue's
original text named.

**A door is missing, and it is not on this program's ground.** DI1
computes the minting entry by walking up the history until the counter
drops below the id, citing `History::entry` and `Doc::next_id`.
`History::entry` is public (`crates/viewer/src/history.rs:192`), but
`Doc::next_id` is `pub(crate)` in editor-core with no accessor
(`crates/editor-core/src/doc.rs:315`), so layer 3 cannot read it and
G1's boundary rules say layer 3 may not reach past the public surface.
The door is DOCM's to open — `crates/editor-core` is that program's
territory — and it is filed as
`work/view/next-id-has-no-layer3-door.md` on this slate with the
announce owed to DOCM, rather than assumed.

## Unblocked (2026-09-20, `edit/minted-id-door`)

The missing door is open: `Doc::has_minted(&self, id: RecipeNodeId)
-> bool` (`crates/editor-core/src/doc.rs`), true exactly when the id
is below the document's mint counter. It is the predicate the section
above argued for rather than the counter itself — `next_id` stays
`pub(crate)`, so layer 3 can ask *could this document have minted
this id* and cannot read where the counter stands, which keeps the
monotonicity argument in editor-core. DI1's parenthetical names it
now. The walk this unit owes is therefore `History::entry` up the
history until an entry's `doc()` answers `false`; the last entry that
answered `true` is the minting one, and liveness (`Doc::node`) is the
second question, asked after descent. The holder sweep can start.

## Home

Viewer ground (`crates/viewer/src/*`): GAUTH's closing entry names this issue as its residue, and both GAUTH and GUI are closed programs, so it landed in `work/issues/` and was re-homed here by DOCM's 2026-09-04 hand-off.

## Re-homed to VSEAM (2026-09-29, EDIT orchestrator)

The work left here is a viewer-side walk. Every layer-3 holder of a
`RecipeNodeId` in the session and history state has to be checked
against `Doc::has_minted` (EDIT's door, built on PR #2905) after a
rewind. VIEW, which owned that ground, has left the tracker, and VSEAM
claims `crates/viewer/src/history.rs`. EDIT's side, the door the walk
reads, is done. Id kept; priority unchanged.

## 2026-10-21 — re-priced after `[ev]` #3871 (P0 → P4, H → E)

The 2026-10-02 log notes asked for this row to be re-priced and it was
not; this section does it. Everything above it describes the
counter-era hazard and the minting-entry walk, and both are history now.

**The wrong answer is gone.** `RecipeNodeId` is a digest of the mint
chain (`crates/editor-core/IDENTITY.md` DI1, N1): a different insert
mints a different id, so a held id can no longer come to denote a node
nobody picked. DI1 as ruled on #3871 has a holder keep the bare id and
ask the document in hand (`Doc::has_minted`, then `Doc::node`); the
minting-entry walk is retired and is not owed.

**The replacement window is closed.** `ViewerApp::perform_batch`
(`crates/viewer/src/app.rs`) computes `replaced` for both `Open` and
`NewDocument` and calls `document_replaced`, which drops the drafts'
and the tools' holds. Rows: `a_replaced_document_drops_the_held_face`
and `a_replaced_document_drops_the_open_tools_picks` (`app.rs`).

**What is left:**

- **Prose that still states the counter-era hazard**, now false:
  `seats.rs`'s "Survival, and the hazard it does not cover" (ids as "a
  small per-document counter" that "re-mint the same small ids");
  `revolvetool.rs:21`, `combine.rs:18` and `:886`, `blend.rs:614`, and
  `crates/viewer/tests/creation_ops.rs:924`, each citing issue #1384;
  and `Doc::has_minted`'s doc (`crates/editor-core/src/doc.rs`, above
  `:1060`), which still describes "DI1's walk" and "descent first, then
  liveness". The last is editor-core's ground (EDIT), so it travels as
  a hand-off or with EDIT's next unit there.
- **Rows pinning DI1's bullets**, where none exists yet: undoing past
  the mint drops a pick and redoing restores it (`select_pick.rs:805`
  covers the undo half for a selection), and a sibling branch's
  different insert leaves a held id foreign while a byte-identical one
  keeps it.

That is prose and test hygiene, so P4.
