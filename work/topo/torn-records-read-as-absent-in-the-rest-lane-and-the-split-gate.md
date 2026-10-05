---
id: torn-records-read-as-absent-in-the-rest-lane-and-the-split-gate
kind: issue
title: The REST lane, the split gate and null_site read a torn record as absent
status: open
priority: P3
cost: M
refs: [torn-body-refusal-families-beyond-the-six-doors, public-body-option-getters-answer-none-for-a-torn-record]
opened: 2026-10-05
---

## What

The second-pass sweep of the PR that converted `rest.rs`', `sectors.rs`'
and the split's torn-read refusals looked for the shape no refusal
pattern matches: a lookup that reads a torn record as *absent*
(`None`, `continue`, `false`) and so answers as if the record were
not there. In the files that PR touched:

- `boolean/rest.rs` `face_ball` and `face_witnesses` (`?` on the face,
  its surface, its loops, the loop walk, half-edges and points) and
  `face_carrier` (a face whose surface does not resolve reads as a kind
  outside the ladder);
- `boolean/rest.rs` `mint_chord`'s `ring_loop_of` (a ring loop that does
  not resolve reads as no ring) and `realize_seam`'s `has_edge` (a
  vertex that does not resolve reads as unjoined);
- `splitting/classify.rs` `gate_face_reach` (`?` on the face and its
  surface: the gate reads no reach, and refuses the face as
  unsupported) and `insert_crossings`' curve read (a curve that does
  not resolve refuses as `ScaffoldingOperand`);
- `chord_join.rs` `null_site` (`edges_of_vertex(v).unwrap_or_default()`
  and `continue` over a curve that does not resolve).

Each reads a key either from a record or one its caller carries; which
it is decides the fix, as on the families row.

## Direction

Per site, as `torn-body-refusal-families-beyond-the-six-doors` does it:
a record hop goes through `live::linked` / `live::proven`; a key the
caller carries keeps an honest typed or `None` answer that states the
fact. The REST lane and the split run mid-operation, so a site there
needs its own proven premise before it panics.

