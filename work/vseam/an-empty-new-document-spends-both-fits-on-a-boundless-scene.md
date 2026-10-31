---
id: an-empty-new-document-spends-both-fits-on-a-boundless-scene
kind: issue
title: An empty NewDocument lands a boundless scene that spends both fits, so its first body is neither framed nor budgeted
status: open
opened: 2026-10-31
priority: P3
cost: E
---


Found by the review of PR 6125 (NOTE-1). That PR gives `NewDocument`
the same `DocumentDerived::owed()` an `Open` gets, which is an
improvement on its merge base, where `NewDocument` was owed neither fit.
This row is the gap that remains, not that PR's regression.

## What happens

An empty `NewDocument` evaluates to nothing drawable, and its first
landing still produces a scene, with `SceneMesh::bounds() == None`.
That landing spends both owed values together, in `ViewerApp::sync_scene`
(`crates/viewer/src/app.rs`):

- `fit_delta` is spent on the first landing "whether or not that landing
  has a body to price". The empty document has no `fit_request`, so δ
  stays the one the OUTGOING document was opened or typed at.
- `fit_camera` is spent on the boundless scene's rebuild. The viewport
  frames the origin, as `a_replaced_document_reframes_the_camera_through_both_doors`
  shows.

So the new document's first body (an `AddInstance` of a large part, say)
lands with nothing owed. It is not framed, and it is drawn at the
outgoing document's δ rather than at a budgeted one. The reviewer
measured this on the PR head: an empty `NewDocument` lands a scene with
`bounds=None`, both fits read spent after that one landing, and δ is
still the outgoing document's.

## What resolving it looks like

Spend each owed value on the first landing that has something to spend
it on: `fit_camera` on the first scene with `bounds()` `Some`, and
`fit_delta` on the first landing with a `fit_request`. Both are local
conditions in `sync_scene`. Check that the δ-budget's "a DEFAULT, not
a clamp" argument (`DocumentDerived::fit_delta`) still holds when a
document's first edit, rather than its opening, is what gets priced.
