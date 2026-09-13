---
id: the-entity-kind-door-has-six-spellings
kind: issue
title: Read a name, test its EntityKey kind, refuse: three copies in eval/wire.rs and six spellings of the refusal across the crate, with no shared door
status: open
opened: 2026-09-12
---



## Finding

Found by the full review of PR 2480 (MINOR 1), which read the operand
door's sweep and found its blind spot: that sweep grepped for
`NodeErrorKind::WrongOperand`, so a kind refusal raised under a
DIFFERENT error kind was invisible to it. PR 2480's hit list disposed of
`Selected::faces` as *"one enum, two arms, no duplication yet"*, which is
**false of the class as it stands** and is retracted here.

## The three copies, in one file

`crates/editor-core/src/eval/wire.rs` carries the same five lines three
times — one `ladder::resolve_in`, one `let EntityKey::X(k) = ent.key
else`, one refusal carrying `name` and `found: ent.key.kind()` —
differing only in the wanted kind and the error arm:

- `resolve_open_faces` → `NodeErrorKind::ShellOpenKind { name, found }`
  (wants `EntityKey::Face`)
- `resolve_selection` → `NodeErrorKind::BlendSelectionKind { verb, name,
  found }` (wants `EntityKey::Edge`)
- `wire_datum`'s `Datum::FaceFrame` arm, inline →
  `NodeErrorKind::FaceFrameKind { name, found }` (wants
  `EntityKey::Face`)

All three already share their RESOLUTION door (`ladder::resolve_in`) and
its N5 refusal trio. What none of them shares is the kind test and the
refusal after it — which is exactly the shape `eval::wire`'s `operand` /
`node_operand` now has for VALUE kinds.

## Six spellings of one question, crate-wide

The class is "read a thing, test its kind, refuse", and the crate says
it six ways. The field names are the census:

| spelling | where |
| --- | --- |
| `expected` / `found` | `NodeErrorKind::WrongOperand` — the value door, given one home by PR 2480 |
| `name` / `found` | `ShellOpenKind`, `BlendSelectionKind`, `FaceFrameKind` |
| `verb` / `found` | `MeasureSelectionKind` (`Selected::faces`, in the same file) |
| `wanted` / `found` | `crates/editor-core/src/names/interrogate.rs` `kind_mismatch` |
| `expected` / `found` | `crates/editor-core/src/stackup.rs` |
| the recipe road | `crates/editor-core/src/mate/member.rs` — filed separately as `work/docm/the-third-datum-axis-phrase-lives-in-mate-member.md` |

## What a taker owes

A decision about the ENTITY-kind door, which is a different vocabulary
from the value-kind one: entity kinds are `EntityKey`'s variants, not
`ValuePayload` families, and the refusal carries the authored NAME
rather than an input id. The three `wire.rs` copies are the instance
worth fixing first; whether the other three spellings collapse onto it,
or stay distinct because their subjects genuinely differ, is the
question this row asks and does not answer.

Read beside `eval::mod`'s `operand_vocabulary_census`, whose own doc
records this class as what it cannot see.
