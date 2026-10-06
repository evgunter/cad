---
id: public-body-option-getters-answer-none-for-a-torn-record
kind: issue
title: Body's Option getters answer None alike for the caller's stale key and a torn record
status: open
opened: 2026-10-04
---


(TOPO implementer, from the sweep of
`stale-key-and-not-same-edge-answer-for-a-callers-key-and-a-torn-body`'s
read-back unit.)

## What

Seven public `Body` walks answer `Option`, and their `None` is both
"the key you passed does not resolve" and "a record on the walk names
nothing":

- `Body::face_of_half_edge` (`body.rs`): `None` for a stale `he`, and
  for a live `he` whose `parent_loop` dangles (pinned as the same
  answer by `body::tests::face_of_half_edge_walks_and_refuses_on_either_stale_key`).
- `Body::mate`: `None` for a stale key and for an edge that does not
  claim the half (`mate_is_none_on_stale_or_unclaiming_edge`).
- `Body::half_edge_end`, `Body::loop_cycle`, `Body::faces_of_solid`,
  `Body::edges_of_vertex`, `Body::faces_of_vertex`: `None` for the
  caller's stale key and for a dangling `next`, a broken orbit or a
  shell that does not resolve.

D2 row 4 (the bullet *A torn body is a kernel bug*) makes the second
half a panic naming the record. Every crate-internal caller that reads
a key out of the body and hands it to one of these maps the `None` to
a refusal of its own or skips (`query::continuation` now panics on
it; many sites in `boolean/`, `splitting/`, `sweep` and `mesh` still
read it as a refusal or a skip).

## Direction

Each walk resolves its argument typed and every later hop through
`live::linked` / `Walk::closed`, so `None` means only the caller's
stale key. Then sweep the internal callers that hand them record keys
for the `.ok_or(…Corrupt…)` / `let Some(..) else { continue }` shape.
