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

## Question for Ev (2026-09-08, LIB orchestrator; `[ev]` PR)

The binding census's rule 1 accounts a curated Rust name by SPELLING:
`pncad.pyi` declares a top-level `Datum` and the census took the
authoring enum `Datum` as bound, so `Datum::FaceFrame` — a whole
authoring arm — was invisible for the life of its family. The guard's
own docstring disclaims semantics; this is that disclaimer with a
bill. What should the guard's alphabet do?

- **(A) A hand-maintained `SAME_SPELLING_DIFFERENT_TYPE` list that
  REFUSES rule 1 for the names on it**, so each such Rust name needs an
  explicit `BOUND_AS` (per arm, where the arms cross under other
  spellings — `Datum::Frame` → `Node.datum_frame`, …) or `NOT_BOUND`
  row. `Datum` is the first entry. The coincidence becomes a
  declaration a reader can check; a new same-spelled pair is caught
  the day it is written only if someone adds it — the list is the
  known blind spot, stated. Recommended.
- **(B) Rename the Python read-side class** (`DatumValue`) so the
  spelling no longer collides. Removes the instance and not the shape,
  and renames shipped surface for a guard's convenience.
- **(C) Leave it** — the docstring already says semantics are not
  checked.

Recommendation: **(A)**; small, mechanical afterwards, and the census
is the one guard the binding surface has.

### (D), added 2026-09-09 after Ev asked for a structural check that keeps the name match

**Keep rule 1's name match, but a match accounts MEMBERS, not the
type.** When a curated Rust name resolves to an enum (or a struct),
the same-spelled Python namesake accounts only the arms (or pub
fields) it actually spells — an arm as a class attribute
(`SurfaceKind.Plane`) or as a snake-cased constructor/property
(`Node::Extrude` → `Node.extrude`). Every arm the namesake does not
spell needs its own `BOUND_AS` (`Datum::FaceFrame` →
`Node.datum_face_frame`) or `NOT_BOUND` row. The resolver that turns a
curated name into its declaration is the one `LIB-SWEEP` commits as a
script (`payload-rung-sweep-is-prose-and-a-third-run-disagrees`), so
the machinery is shared rather than new.

Why it is structural: the read-side `Datum` spells none of the
authoring enum's five arms, so the name match would have demanded five
rows on the day it was written, and `FaceFrame` would have been the
one with nothing to write in it. It would also have caught the other
instance of the same hole: `Node::Union` and `DocEdit::SetMembers`
were arms behind bound names with no door until LIB-DOORS-3, invisible
to the census and visible only to a hand-kept roster in
`test_north_star.py`. Two known instances, no list.

Costs and blind spots: a one-time backfill of per-arm rows for every
curated enum whose arms cross under other spellings (`Node`, `DocEdit`,
`Datum` the big three; the unit's first step is the count); an arm
whose snake-cased name coincidentally matches an unrelated attribute
still slips; generic or aliased members are outside the source
reader's reach, as they are for the payload sweep.

Recommendation revised: **(D)** over (A); under (D) the list (A)
proposes is unnecessary.

## Ruled (2026-09-09, Ev, `[ev]` PR 2230)

**(D).** Ev: "D sounds good!" Rule 1's name match stays, but a match
accounts MEMBERS, not the type: when a curated Rust name resolves to
an enum (or a struct), the same-spelled Python namesake accounts only
the arms (or pub fields) it actually spells — an arm as a class
attribute or as a snake-cased constructor/property — and every other
arm needs its own `BOUND_AS` or `NOT_BOUND` row. The resolver is the
one `scripts/payload-rung-sweep.py` (LIB-SWEEP) commits. Mechanical
afterwards: the census rule, the declaration reader, and the one-time
backfill of per-arm rows (`Node`, `DocEdit`, `Datum` the big three;
the unit's first step is the count). (A)'s list is unnecessary.
