---
id: layer3-recipenodeid-aliases-across-rewinds
kind: issue
title: Layer-3 state holding RecipeNodeIds aliases across history rewinds (id reuse)
status: open
opened: 2026-08-31
github: 1384
refs: [1375]
---

## From GitHub issue 1384

opened 2026-08-31, 0 comments.

Class defect, found in GAUTH-1 review (PR #1375): `RecipeNodeId` is a small per-document monotone counter, so any layer-3 value that HOLDS one across a document replacement can silently start denoting a different node when fresh inserts re-mint the same small ids.

Demonstrated scenario (reviewer probe, reproduced): fill a `RevolveTool` with picks (profile id 0, axis id 1), perform `SessionOp::NewDocument`, skip `reconcile`, insert a new profile and a new axis into the fresh document (they re-mint ids 0 and 1), commit the stale tool — the session's kind gates PASS (the new ids denote the right kinds) and a real `Node::Revolve` of nodes nobody picked is authored, with no refusal anywhere. The same shape arises from an undo past the picks' inserts followed by new inserts, and from `Open`.

Sweep targets (everything in layer 3 that holds a `RecipeNodeId` across turns):
- `viewer::revolvetool::RevolveTool` (both seats) — module doc now states the hazard honestly and names this issue;
- `viewer::matetool::MateTool` — `FaceSelection::node` inside its held picks (its `StableName` half resolves honestly; the node field is the aliasing half);
- `viewer::session::Selection::Node` (and `FaceSelection::node` inside `Selection::Face`) — `Standing` answers presence by id lookup, so a reused id reads as present.

Per-frame `reconcile`/`standing` guards the deleted-node case only; it cannot distinguish "the same id, a different node". Candidate directions (not adjudicated here): a document identity+generation stamp carried beside held ids and checked at consume time; clearing tools/selection on every history REPLACEMENT (open/new already clear selection — undo/redo do not clear tools); or stable-name-shaped node references for layer-3 holds. Whatever the mechanism, the fix should be one rule for all three holders, not three local patches.

## Home

Viewer ground (`crates/viewer/src/*`): GAUTH's closing entry names this issue as its residue, and both GAUTH and GUI are closed programs, so it lands in `work/issues/`.
