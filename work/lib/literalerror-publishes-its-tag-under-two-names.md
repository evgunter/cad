---
id: literalerror-publishes-its-tag-under-two-names
kind: issue
title: pncad-py — LiteralError spells its tag kind at two doors and variant at a third, so a caller cannot read it without knowing which door refused
status: open
opened: 2026-09-16
---



## Found by

PORT-DIMS-1's full review (2026-09-15). Pre-existing and outside that
unit's fence; filed because PORT-DIMS-1's taxonomy argument — *"the
class names the DOOR and the type's own word rides beside it"* — does
not currently account for it.

## The finding

`ErrorClass::Literal` is raised at three doors, and two of them publish
the same tag under a different attribute name:

| door | raise site | attribute |
|---|---|---|
| literal construction | `py/expr.rs:102` (`literal_err`) | `kind` |
| measurement arithmetic | `py/measure.rs:148` | `kind` |
| the recorded-program lift | `py/path.rs:1351` (`loop_program`) | **`variant`** |

All three carry the same vocabulary: `py/path.rs` mints its word from
`recorded_program_error_tag`, whose `Literal` arm is
`expr_dimension_error_tag(inner)` — the map the other two use directly.
One word, one class, two attribute names.

`pncad.pyi` declares `LiteralError` with `kind` and `value` and does not
declare `variant` at all, so the third door's attribute is undeclared as
well as inconsistent. A caller writing `except LiteralError as e:
e.kind` gets an `AttributeError` for a refusal raised through the
profile lift, and has to know which door refused in order to read the
class's own payload — which is the thing a typed class is for.

## Why it matters to the dimension family specifically

This is the **third route** by which the document layer's
`DimensionError` reaches Python (`ErrorClass::DIMENSION_DOORS` in
`crates/pncad-py/src/errors.rs` is the roster), and the only one whose
attribute name disagrees with its siblings. PORT-DIMS-1 argued that the
vocabulary is uniform across all six doors, which is true of the WORDS
and not of the NAMES they arrive under. That argument should either
account for this row or this row should close.

## The decision in front of it

Two shapes, and the choice is not obvious:

1. **Spell it `kind` everywhere.** `LiteralError`'s own declared
   attribute, and the two doors that already agree are the majority.
   Costs: `variant` is the spelling every OTHER class in the taxonomy
   uses for "the refusing arm's tag", so `LiteralError` becomes the
   exception in the other direction.
2. **Spell it `variant` everywhere**, and re-declare `LiteralError`
   with `variant` beside (or instead of) `kind`. Costs: `kind` is what
   `.pyi`, the guide and the Python suite read today, so this is a
   surface change with rows to update.

Either way the census should gain something that would have caught it:
a class raised from several sites with different attribute sets is
exactly what `surface_census.rs` is for, and it does not ask this.

## Where to look

- `crates/pncad-py/src/py/path.rs:1343-1356` — `loop_program`, the odd
  one out.
- `crates/pncad-py/src/py/expr.rs:95-110`, `src/py/measure.rs:140-155`
  — the two that agree.
- `crates/pncad-py/src/tags.rs` — `recorded_program_error_tag`, whose
  `Literal` arm forwards `expr_dimension_error_tag`.
- `crates/pncad-py/pncad.pyi` — `class LiteralError`, which declares
  `kind` and `value`.
- `crates/pncad-py/src/errors.rs` — `ErrorClass::DIMENSION_DOORS`, the
  roster this row is the footnote to.
