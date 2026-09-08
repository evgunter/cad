---
id: datum-crosses-name-for-name-as-two-types
kind: issue
title: the census accounts `Datum` name-for-name across two different types, so a whole authoring arm hid behind it
status: open
opened: 2026-09-06
---


Found by LIB-B-FACE-FRAME's sweep, banked rather than acted on.

## What happened

`crates/pncad-py/tests/test_binding_census.py`'s rule 1 accounts a
curated Rust name when `pncad.pyi` declares a top-level name spelled
identically. `Datum` is curated at `crates/pncad/src/document.rs:40` —
the `editor_core` AUTHORING enum, whose arms are the datum kinds a
recipe can hold — and `crates/pncad-py/pncad.pyi:2223` declares a
top-level `Datum`, which is the READ-side value
`crates/pncad-py/src/py/value.rs:367` projects and `Value.datum()`
answers with.

The two are different types answering different questions, and rule 1
cannot tell. `Datum::FaceFrame`
(`crates/editor-core/src/node.rs:704`) was therefore invisible to the
census for the whole life of the `B-FACE-FRAME` family: the family's
charter named `Datum.face_frame` as one of three doors, and only one
of the three (`face_carrier_kind`) could be a `NOT_BOUND` row. Nothing
mechanical would ever have said the arm was unbound.

This is not a defect in the guard as written — the module docstring
says outright that semantics are not checked ("nothing here verifies
that Python's `Frame` is the `Frame` the façade curates") — but it is
that sentence with a bill attached, and the bill is a whole authoring
arm.

## The shape of the hole

Rule 1 is a NAME match, so it is exactly as strong as the coincidence
that the two sides picked the same word. The failure needs three
things at once and this is the only place in the tree where all three
hold today:

1. a curated Rust name that is a SUM of arms (an enum), so that a
   missing arm is a missing door rather than a missing type;
2. a Python class of the same spelling that is a different type — here
   the read-side value of the same family, which is exactly the kind
   of name a binding naturally reuses;
3. the arms crossing under a different spelling (`Node.datum_*`), so
   no `BOUND_AS` entry is written and nothing points at the enum.

Swept for the shape by comparing the census's `self.top` accounting
against `pncad.pyi`'s class list: the other curated enums Python
spells identically (`BooleanOp`, `SurfaceKind`, `CurveKind`,
`EntityKind`, `SegTag`, `SplitHalf`, `CapEnd`, `MeridianEnd`,
`RimSupport`, `Cmp`, `OpGroup`) all cross as fieldless mirrors of the
SAME enum, so `py/select.rs`'s growth tripwire covers their arms.
`Datum` is the one whose Python namesake is a different type. **What
the sweep could not match**: a curated STRUCT whose Python namesake is
a different type with fewer fields — the census is equally blind to
that and this pattern says nothing about it.

## What a fix would look like (not decided here)

The census reads source text and no compiled module, so it cannot ask
whether two `Datum`s are the same type. What it could do is refuse
rule 1 for a curated name a hand-maintained list marks as
"identically spelled, different type" — turning the coincidence into
a declaration a reader can check. That is a change to the guard's
alphabet and belongs to whoever owns the census next, not to a family
unit.
