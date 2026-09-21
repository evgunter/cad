---
id: a-count-slot-launders-a-typed-nan-into-zero
kind: issue
title: A typed NaN in a Count slot commits 0, past the finiteness refusal props.rs promises names it
status: open
opened: 2026-09-17
priority: P0
cost: E
---

Found by the sweep `a-clamp-is-not-a-bound-against-nan` ran over
saturating float-to-int casts in `crates/viewer/src`. Every line below
was executed.

## Finding

`props::field_edit` reads a value field's text and returns
`FieldEdit::Number` for anything `f64::from_str` accepts. Its doc says
so deliberately, and says what pays for it:

> A non-finite spelling (`inf`, `NaN`) reads as a Number here on
> purpose: `Expr::literal`'s refusal names the problem ("a literal
> value must be finite"), where the parser would only say the word is
> not a parameter.

**That promise is false for a structural slot.** `props::SlotValue::of`
splits on the DIMENSION before any expression is built:

```
        if dimension == Dimension::Count {
            Self::Count(value as i64)
```

`f64::NAN as i64` is **0** and `f64::INFINITY as i64` is
**`i64::MAX`** — a saturating cast, not a conversion. So the value
that reaches `SessionOp::SetSlot` is an ordinary integer, and
`Expr::literal`'s finiteness refusal is never asked. The word the user
typed is gone by then.

The four Count-dimensioned slots are `SlotId::Count`, `VDegree`,
`Stations` and `Instance` (`crates/editor-core/src/node.rs`,
`SlotId::dimension`'s Count arm); `is_structural` is defined as that
dimension, so these are exactly the structural slots.

## The route, which is the part a reader will doubt

The properties field is an `egui::DragValue` with a `custom_parser`
(`crates/viewer/src/pane/properties.rs`, `slot_field`). egui clamps a
parsed value into the widget's range before writing it back, and its
`clamp_value_to_range` orders with `total_cmp`, under which a positive
`NaN` sits ABOVE `+inf` — so the number egui stores for a typed `NaN`
is `+inf`, not a `NaN`. That is not the value the commit reads. The
parser stashes the RAW `FieldEdit` in a `RefCell` first, and the
commit arm reads that cell, so the `NaN` reaches
`SlotValue::of(row.dimension, field.authored(written))` untouched.

Both spellings therefore land, by two different roads: `NaN` commits
`0` through the cast, and `inf` commits `i64::MAX` through the
saturation.

## Not established here

What the document does with `Count(0)` or `Count(i64::MAX)` once the
op is performed. If either is refused downstream the user still gets a
refusal about the wrong thing — a count out of range rather than a
word that is not a number — but the size of the consequence is not
measured on this row.

## Fence

`crates/viewer/src/props.rs` and `crates/viewer/src/pane/properties.rs`
— VIEW's, the standing double claim with CHROME.
