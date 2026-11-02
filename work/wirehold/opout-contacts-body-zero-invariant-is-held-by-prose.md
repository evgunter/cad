---
id: opout-contacts-body-zero-invariant-is-held-by-prose
kind: issue
title: A carry's record invariants are held by prose: a family has no records of its own by construction only, and the split sentence goes with the split carry
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

## Parked on the D10 hold (2026-10-17, WIRE triage)

Read against the tree at `fd218851f9`. The doc is already wrong (it says only `wire_instantiate_part` fills `contacts`; `wire_place` does too, when `bodies.len() == 1`). Stage 4 I rebuilds that channel and stage 3's index variables reshape the multi-output case; re-read after both. Parked on `mates-declare-no-contact`, the INTENT row whose landing reaches this ground, and moved to WIREHOLD by WIRE's priority-seam cut, keeping its id.

Signed (WIRE orchestrator).

## Re-checked after D10 (2026-11-01, WIRE orchestrator)

From the read-only `wirehold-triage` against origin/main 8509df2510. INTENT closed on 2026-10-30, and every trigger this row waited on has fired.

**Changed.**
- `OpOut.contacts` is now `carry: Carry<T>` (`eval/carry.rs:39`).
- A family is structural: each member has its own carry (`Carry::of_family`, `carry.rs:217`). The multi-output pattern case is gone.
- Left is prose only: "A split never fills them" (`eval/wire.rs:51-52`). `wire/a-split-in-the-document-drops-its-operands-contact-records` (in flight) makes that sentence false, so that PR rewrites it, and this row narrows to whatever invariant remains after it.
