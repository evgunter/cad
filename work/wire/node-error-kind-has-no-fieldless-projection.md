---
id: node-error-kind-has-no-fieldless-projection
kind: issue
title: NodeErrorKind carries payloads and has no fieldless projection, so three doors render a node refusal to prose — the fifth instance of the kind-mirror class
status: open
opened: 2026-09-11
---


Found by the `checks-product-refusal-degrades-to-string` lane (PR 2344)
while re-running that class's sweep. It is the **fifth** instance of
the class `kind-mirrors-have-no-single-declaration` describes, and the
first one where the mirror does not exist at all.

## The correction this row carries

`kind-mirrors-have-no-single-declaration`'s earlier text implies
`NodeErrorKind` is a missing sibling. **It is not missing** — it exists
(`crates/editor-core/src/eval/mod.rs:675`) and is payload-carrying. The
gap is narrower and more useful stated precisely: there is no
**fieldless** projection of it, so a consumer that wants the class has
only the rendered sentence.

## The three doors

Each has a `NodeErrorKind` **value in hand** and renders it away:

- `crates/editor-core/src/drive.rs:1129` —
  `WitnessDoesNotBuild { cause }`, built as `e.kind.to_string()`.
  **M10's** fence.
- `crates/editor-core/src/mc.rs:368` — `NominalDoesNotBuild { cause }`,
  the same shape. `crates/editor-core/src/mc.rs` is in **no open
  program's `paths`** — which is why this row is homed here.
- `crates/editor-core/src/stackup.rs:231` and `:1510` —
  `MeasureRefused` / `MeasureRefusedAtNominal`, `cause: String` from a
  node error. **PROPS'** fence.

These were invisible to PR 1806's sweep because its field-name pattern
did not include `cause`; that is the blind spot, now measured rather
than inferred.

## Why these are worth a unit

The degradation is not hypothetical here: the value exists, is typed,
and is thrown away one line before the consumer needs it. That is a
sharper case than the doors the earlier units fixed, where the class
had to be minted first.

## What this needs decided

Whether a fieldless projection is the right answer at all, or whether
these three doors should simply carry `NodeErrorKind` itself — it is
already an enum the consumer could match on, and the reason the other
mirrors are fieldless (a payload the consumer must not depend on) may
not apply. **Answer that before minting a sixth hand-written mirror**:
PR 2344 established that the pairing direction of a hand-written mirror
is closable by a derive and by nothing else, and three of them now
carry a copied two-part guard. A fifth hand-rolled pair should not land
just because four already have.

## Re-homed to WIRE, 2026-09-20

(FIX orchestrator) Ev, in chat, 2026-09-20: *"can you kick all the design decisions back to
the track they actually belong to, leaving fix design-free?"* FIX is the
program for rows whose fix is already written; a row whose blocking
question is a DESIGN decision belongs to the track that owns the surface
the decision is about.

**The decision this row is blocked on:** whether a fieldless projection is the right answer at all, or whether the
three doors should carry `NodeErrorKind` itself — *answer that before minting
a sixth hand-written mirror*.

**Why WIRE.** `NodeErrorKind` is declared in `crates/editor-core/src/eval/mod.rs`, which
territory says is **owned by wire**, and WIRE's slate already carries a row on
this very type: `node-error-kind-renders-the-slot-id-through-debug`. The
decision is about the type's shape, so it belongs where the type is declared.

**The row's own fence claim is now stale, and that is part of why it moves.**
It was homed on FIX because *"`crates/editor-core/src/mc.rs` is in no open
program's `paths`"*. Territory now says `mc.rs` is **owned by props** — as are
the other two consumer doors, `drive.rs` and `stackup.rs`. So all three doors
that render the kind away are PROPS's ground and the declaration is WIRE's;
nothing about it is FIX's any more.

Nothing about the finding is changed by the move: same id, same
evidence, still `open`, and no part of its question is answered for
you except where this note says Ev answered it.
