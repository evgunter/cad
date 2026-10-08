---
id: rust-document-authoring-has-no-profile-frame-or-insert-sugar
kind: issue
title: Rust's document authoring has no Node::profile(&ClosedLoop), no sketch frame and no Doc::insert, which Python spells in one line each, so the tour carries a copy of the insert/eval/body_at/len/scl helpers per scene
status: open
opened: 2026-10-02
priority: P3
cost: M
---

Found by SHOW's `split-node-chords-by-name-has-no-demo` (PR 3842's
review). A library finding: the friction a Rust user of the recipe layer
meets, which the Python surface already removed.

## What Python has and Rust does not

| Python | Rust today |
|---|---|
| `Node.profile(outline, plane=...)` takes a closed PATHS loop | `LoopProgram::from_recorded(&closed.program)` reaching into the loop's `program` field, wrapped by hand in `ProfileProgram { plane, loops, ids: Vec::new() }` |
| `doc.sketch_frame()` | `Node::Datum(Datum::Frame { origin, u, v })` — nine hand-written `Expr`s for the world xy frame |
| `doc.insert(node)` returns the id | `apply(doc, &DocEdit::InsertNode { node: Box::new(node) }, tol, &RefusingReach)`, then `applied.doc` and `applied.record.minted` |

Plus the value read: Python's `ev.value(n).body()` against a match on
`ValuePayload::Body(b) => (**b).clone()`, and `Expr::literal(v,
Dimension::Length)` against Python's `Expr.length_in(v, m)`.

## Evidence: the copies in `demos/tour/src`

Each scene that authors a document re-spells the same helpers (names as
of this filing):

- `insert`: `assembly.rs`, `bracket.rs`, `chain.rs`, `checks.rs`,
  `diefillet.rs`, `plate.rs`, `teapot.rs` (fns); `heatsink.rs`,
  `impeller.rs`, `ring.rs` (closures).
- `eval`: `bracket.rs`, `diefillet.rs`.
- `body_at`: `bracket.rs`, `diefillet.rs`, `mcchain.rs`, `mcplate.rs`,
  `teapot.rs`.
- `len` / `scl`: `bracket.rs`, `chain.rs`, `diefillet.rs`, `teapot.rs`,
  `plate.rs` (fns); `checks.rs`, `heatsink.rs`, `impeller.rs`, `ring.rs`
  (closures).
- the hand-written world frame (`Datum::Frame` with literal axes):
  `bracket.rs`, `chain.rs`, `checks.rs`, `diefillet.rs`, `heatsink.rs`,
  `impeller.rs`, `plate.rs`, `ring.rs`, `teapot.rs`, `assembly.rs`.

## Done when

A Rust consumer can write frame, profile-from-a-closed-loop and insert as
one call each through the `pncad` façade, and the tour's copies above are
replaced by it.
