---
id: edge-carrier-kind-has-no-readback-door
kind: issue
title: query::edge_carrier_kind has no readback twin, so the two-homes ruling does not reach the edge side
status: dispatched
opened: 2026-09-06
branch: topo/edge-carrier-kind-readback-door
---


## What

Ev's ruling on `face-kind-read-has-two-homes` (PR 1948): the typed
readback door is the one reading of a stored tag and the predicate seat
flattens it. `query::edge_carrier_kind` (`crates/topo/src/query.rs:296`)
reads an edge's certified carrier kind with no typed twin in
`readback.rs`, so on the edge side the query seat IS the only reading
and there is nothing to flatten. Either the shape is fine (one reading,
it just lives in `query`) or `readback` owes `edge_carrier_kind ->
Result<CurveKind, ReadbackError>` for the read-back consumers that
want to know which lookup missed, with the query seat then flattening
it as the face side does. Not a defect today; a question about where
the one reading lives. Reported by the two-homes lane; filed by the
TOPO orchestrator, 2026-09-06.

## Brief (TOPO, 2026-09-14) — block TOPO-B4 slot 2, dual at review

**The answer to give.** The edge side gets the shape Ev ratified for
the face side (`face-kind-read-has-two-homes`, PR 1948): the typed
readback door is the ONE reading of a stored kind tag and the
predicate seat flattens it. `readback::edge_carrier_kind(body, edge)
-> Result<CurveKind, ReadbackError>` copies the certified carrier's
kind out, refusing `Dangling` (stale edge key; stale curve key
reached from a live edge) and `NoCarrier` (null-edge scaffolding) and
nothing else — every certified carrier, NURBS included, has a kind,
so `NoCanonicalFrame` is not one of its refusals. `query::edge_carrier_kind`
becomes `readback::edge_carrier_kind(body, e).ok()`, its `None`
contract unchanged ("no carrier is an honest no"), and
`edge_carrier_matches` keeps reading through it. The document-layer
twin `editor-core::names::interrogate::edge_carrier_kind(ev, node,
name)` lands beside `face_carrier_kind` there (same node ladder,
`WrongKind` for a non-edge name, the wrapped `ReadbackError`), with
the `names/mod.rs`, `editor-core/src/lib.rs` and `pncad` re-export
lists carrying it where the face twin's name already sits.

**Phase 1 decides, and says why.** (1) `edge_pose` already walks
edge → curve geom → certified carrier with exactly the refusals the
new door needs: the two doors share ONE walk (a private helper the
rows pin bit-identical for `edge_pose`), not a second copy — a fresh
copy of the walk would be the two-homes defect minted again one
level down. (2) `CurveKind` lives in `query.rs` under the sentence
"the mirror lives where it is used"; once `readback` is the reading,
say where the mirror lives from what its readers now are (stay and
import, or move and re-export — both crate-internal) and keep the
placement paragraph true either way. (3) `pncad-py`'s
`Evaluation.face_carrier_kind` has no edge twin; that layer is not
this unit's — say in the PR body what its owner would add and
announce it on the owning slate, do not build it.

**Rows.** In `crates/topo/tests/readback_sense_kind.rs` beside the
face rows: the door copies the tag out for a line, circle, ellipse
and NURBS carrier; refuses `Dangling { Entity(Edge) }` for a stale
key and `NoCarrier` for null-edge scaffolding, and nothing else (the
dangling-curve-key case is a row if the body can be put in that
state through its own doors, otherwise the PR body says so); the
flattening is exact — on every case above `query::edge_carrier_kind`
equals the door's `.ok()`, and the existing `None` assertions
(`query.rs`'s `edge_carrier_kind` rows) stay. The delta a red-first
row shows: on the merge base a stale edge and a null-edge scaffold
are the same `None`; at the head the typed door tells them apart.
`edge_pose`'s existing rows pin the shared walk. The editor-core twin
gets one row beside `docm1_face_frame.rs`'s face-kind row (a named
edge answers its kind; a face name is `WrongKind`).

**Receipt.** Every reader of an edge's carrier kind in the workspace
(`CurveKind::of` sites; `.certified()` + `.carrier()` walks in
`query.rs` and `readback.rs`) with which door it now reads through;
every caller of `query::edge_carrier_kind` and `edge_carrier_matches`
(unchanged in behaviour, shown); the face twin's chain through the
four layers, and where the edge chain stops and why. The readback
header's rule 1 names `face_carrier_kind` as the tag door — re-word
it to name both (a description moved by the code, not a decision;
say so in the PR body with the `git log -S` check).

**Seams.** `crates/topo/src/query.rs` and `readback.rs` are this
program's; `crates/editor-core/src/names/` and `crates/pncad/src/`
are touched only in the delegate-and-re-export shape the face twin
already has — say what `work.py territory` names and announce on that
slate's log if it is not TOPO.

Branch `topo/edge-carrier-kind-readback-door`. PR title: "TOPO: the
edge-side kind read has one home — readback::edge_carrier_kind, query
flattens it". Do not close the item; the dual runs at review.
