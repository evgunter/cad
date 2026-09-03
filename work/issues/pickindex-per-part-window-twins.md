---
id: pickindex-per-part-window-twins
kind: issue
title: PickIndex has no home for one part's window of entity X - seven hand-parallel edge/patch twins
status: open
opened: 2026-08-31
github: 1395
refs: [1098]
---

## From GitHub issue 1395

opened 2026-08-31, 0 comments.

GAUTH-2 added edge picking to `crates/viewer/src/pick.rs` by writing, beside each patch-shaped member, an edge-shaped twin with the same body. A reviewer counted seven, and the count is the finding — each pair is correct today and each is kept correct by hand:

- `ids_in` / `edges_in` — one body's window, sliced out of a flat list
- `by_target` / `edges_by_target` — the window maps those two read
- `name_of` / `edge_name_of` — position to name, with the loud arm
- `ids_of_target` / `edges_of_target` — the (node, body) narrowing of a selection
- `names` / `edge_names` — the flat parallel name lists
- `highlight` / `edge_overlay` — the marking, per kind
- `PatchId` / `EdgeId` — the display-coordinate address

Plus an eighth site inside `PickIndex::build`: the per-part window is computed twice, once for patches and once for boundaries, in two loops that must agree about part order and about what "contiguous" means. They do agree — the parts are walked in the same order both times — but nothing checks it, and the failure mode if they ever diverge is the #1098 class: a window that names the next body's entity answers a plausible, confidently wrong name.

**The missing home** is a per-part window index generic in the entity kind: one structure that owns "the parts, in order; each part's run of entity K; the flat names parallel to it; the (node, body) narrowing", instantiated twice. Every twin above then collapses to one implementation with a kind parameter, and `build`'s two loops to one.

**Why it is worth doing before the next kind.** A vertex-pick unit is the obvious next consumer, and it would mint copies eight and nine of the same shape by the same method. Two kinds is the point where the shared shape is arguable; three is where it is not.

Fold in the same question one layer up: `FaceSelection` and `EdgeSelection` are field-for-field twins in `session.rs` (`name`, `node`, `body`, `feature()`), deliberately distinct types so a tool that takes faces cannot be handed an edge — accepted at two kinds, and a third selection kind is where that call should be re-taken rather than repeated.

Not urgent, and nothing is wrong today. This is a note about where the next unit should look first.

## Home

Viewer ground (`crates/viewer/src/pick.rs`): GAUTH's closing entry names this issue as its residue, and both GAUTH and GUI are closed programs, so it lands in `work/issues/`.
