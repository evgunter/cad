---
id: layer3-recipenodeid-aliases-across-rewinds
kind: issue
title: Layer-3 state holding RecipeNodeIds aliases across history rewinds (id reuse)
status: parked
opened: 2026-08-31
github: 1384
refs: [1375]
blocked_on: [next-id-has-no-layer3-door]
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

`docs/DOCM-IDENTITY-DESIGN.md` DI1 adjudicates the candidate directions
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

## Home

Viewer ground (`crates/viewer/src/*`): GAUTH's closing entry names this issue as its residue, and both GAUTH and GUI are closed programs, so it landed in `work/issues/` and was re-homed here by DOCM's 2026-09-04 hand-off.
