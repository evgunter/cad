---
id: opout-contacts-body-zero-invariant-is-held-by-prose
kind: issue
title: OpOut's 'a multi-output op never fills contacts' is held by a doc comment, not by the type
status: open
opened: 2026-09-28
priority: P4
cost: E
---

Found by the GATHER editorial pass over `eval/wire.rs`
(`wire-rs-accumulation-residue-comment-ratio-and-wire-sweep`).

## Finding

`crates/editor-core/src/eval/wire.rs`, `OpOut`'s doc:

> Declared records are keyed in the op's OUTPUT BODY 0 arena. ...
> **Invariant**: a multi-output op (`Split`, `Pattern`) never fills this
> field — "output body 0" would be a lie for its other bodies.

Nothing enforces it. `OpOut { payload, names, groups, contacts,
carried }` is constructible with any payload beside a non-empty
`contacts`, and the one filler today (`wire_instantiate_part`) is
single-body by reading, not by type. The paragraph the pass removed
went on to promise that "the day something does, the channel grows a
per-output shape rather than silently mis-keying" — a promise made by
prose to a future author who may not read it.

## Direction

Make the pairing a type: e.g. a constructor (or a payload-side field)
that only a single-body payload can carry contacts through, so a
multi-output op with records is a compile error rather than a
convention. Then the invariant paragraph can go.
