---
id: node-operand-has-one-consumer-where-axis-datum-is-the-same-door
kind: issue
title: eval/wire.rs's node_operand and mate/member.rs's axis_datum are the same door written twice, and they disagree about the seat
status: open
opened: 2026-09-21
---



## Finding

Found while landing DOOR's
`the-third-datum-axis-phrase-lives-in-mate-member` (the literal
`expected:` in `mate/member.rs`'s `axis_datum`). That row names this
residue in its "What is NOT owed here" section and files nothing, so
it gets its own file here: a residue disclosed is not a residue
scheduled, and DOOR's directory is deleted when DOOR closes.

Confidence `sure`. Accurate at `3456b74`.

Two functions read a NODE (not a value), admit it through a closure,
and otherwise refuse `NodeErrorKind::WrongOperand` with `found:` from
`eval::node_value_kind`:

- `node_operand` in `crates/editor-core/src/eval/wire.rs` — generic
  over the document's program type and the read's result, taking
  `expected: &'static str`. **One consumer**
  (`wire.rs`'s profile-program read, `super::family::PROFILE`).
- `axis_datum` in `crates/editor-core/src/mate/member.rs` — the same
  three steps written out, with `crate::eval::phrase::DATUM_AXIS` as
  its `expected:` and `Node::Datum(Datum::Axis { .. })` as its read.

`axis_datum` is `node_operand`'s natural second consumer, which is
what would earn the abstraction its keep.

## What a lane taking this has to decide first — the seat

**They disagree, and the disagreement is the whole content of the
row.** `node_operand` DROPS the seat `node_value_kind` answers with
(`.map_err(|seated| seated.1)`) and returns a bare `NodeErrorKind`;
its doc comment argues that at ITS door no seat can arrive, because
the schedule evaluated the input `Ok` before the op ran. `axis_datum`
CARRIES the seat (`Err(seated)`) and returns `Result<_, Seated>` —
and it has to, because the recipe road reaches nodes the schedule has
not evaluated, which is exactly what MSOLVE-7 settled for it
(`work/msolve/axis-datum-names-the-pattern-where-the-evaluation-names-the-transform.md`).

So a shared door is not a rename. Either it returns the seated shape
and `wire.rs`'s one consumer re-drops the seat at its call site — which
moves an argument out of a doc comment and into a `map_err` — or it is
generic over the error and the two premises stay where they are, which
buys less than it costs. That choice is the row.

## Fences

The two files have two owners: `eval/wire.rs` is WIRE's,
`mate/member.rs` is MSOLVE's (`scripts/work.py territory`). Filed
here because the helper with a home is WIRE's; a lane that takes it
announces the crossing to MSOLVE.
