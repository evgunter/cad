---
id: the-is-this-a-frame-door-was-deleted-and-its-classification-dispersed
kind: issue
title: PR 2435 deleted frame_kind, whose doc said 'is this a frame' is answered once with one refusal vocabulary, and inlined the classification at three sites; the same value_of + payload-match + WrongOperand shape recurs over seven other kinds
status: open
opened: 2026-09-12
refs: [2435]
---


## Finding

From the full review of PR 2435 (S1, confidence `sure`). **Verified
independently by the WIRE orchestrator** against `origin/main` before
filing, because it is an accusation about a deletion.

`frame_kind`'s doc on main said, in as many words:

> Classifies the frame node a profile's `plane` names — **the variant
> read every by-value reader of a frame shares, so "is this a frame" is
> answered once with one refusal vocabulary.**

PR 2435 deleted it — reasonably, since it had one caller left after the
carry — and inlined the classification. `crates/editor-core/src/eval/wire.rs`
now carries the same `value_of` + `DatumValue::Frame` match +
`WrongOperand { expected: "datum frame" }` block at **`:978-985`,
`:1021-1028` and `:1082-1088`**, and the replacement's own doc
(`:970-973`) *names* the duplication in prose — *"the door every
operand's kind is checked at, the same one `frame_plane_lane` uses"* —
rather than giving it a home.

**This is the standing trap in its exact form**: a unit closing a
duplication deleted the door that existed to prevent that duplication.
Sixth instance on this program, and the only one where the deleted thing
had written down why it existed.

## It is a class, and the fix must not stop at three

The same shape — `value_of`, match the payload kind, raise
`WrongOperand` — recurs over **other** kinds at `wire.rs:1474-1480`,
`:1588-1594`, `:1605-1611`, `:1709-1715`, `:2572-2578`, `:3114-3120`,
`:3966-3972`. A fix that unifies only the three frame sites is a
**half-fix** and should be labelled one.

Read beside `composed-expected-phrases-are-hand-copied-across-sites` and
`frame-plane-lane-and-axis-frame-are-one-door`: all three are the same
file's operand-door vocabulary seen from different angles, and a taker
of any one should read the other two before choosing a shape.

## Not a reason to restore `frame_kind` as it was

Deleting it was right on its own terms — the carry removed its last real
caller, and the reviewer confirms the two remaining doors agree today
(only `Datum::Frame` and `Datum::FaceFrame` produce `DatumValue::Frame`).
What is owed is **a** home for "is this a frame, and which kind", not
that particular one. The shape the reviewer's S2 asks for — an explicit
carried enum matched exhaustively, so a new `Datum` variant is a compile
error rather than a silent lane placement — would supply it, which is
why PR 2435's fix pass may close part of this row. Check before taking
it.

## Beside it, same file, not the same shape

**Two fields named `placement_f64`, same type, one copied from the
other** (S7, `likely`): `NodeValue::placement_f64`
(`crates/editor-core/src/eval/mod.rs:313`) and
`ProfilePre::placement_f64` (`crates/editor-core/src/eval/anchor.rs:140`),
with `wire.rs:1283` copying the first into the second. Not wrong — it is
a carry, not a re-derivation — but same name, same type, one derived
from the other, two hops apart is what makes a reader stop and check
which one they hold. Recorded here rather than given its own row.
