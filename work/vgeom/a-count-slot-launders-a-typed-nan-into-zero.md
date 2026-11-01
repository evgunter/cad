---
id: a-count-slot-launders-a-typed-nan-into-zero
kind: issue
title: A typed NaN in a Count slot commits 0, past the finiteness refusal props.rs promises names it
status: closed
opened: 2026-09-17
priority: P0
cost: E
closed: 2026-09-21
branch: vgeom/refusal-floor
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

## Closed — the Count arm raises the literal door's own refusal

`props::SlotValue::of` returns `Result<Self, DimensionError>`, and its
`Count` arm answers `DimensionError::NonFiniteLiteral` for a value
that is not finite — **the same error, by name, that `Expr::literal`
raises for the continuous half**. That is what makes `field_edit`'s
promise true rather than merely repaired: the promise was that the
refusal downstream names the problem, and this is that refusal, raised
where the literal door is not on the path because `Expr::count` takes
an integer.

The four call sites take it the way each already refuses:
`session/probe.rs`'s `probe_edit` through its own documented `None`
("the value cannot be expressed there at all"),
`session.rs`'s `preview_gesture` through `Refusal::Dimension`, which
is a named refusal on screen, and `widgets.rs`' typed arm and
`pane/properties.rs`' create-param by emitting no operation. **After
the fix the poison is not representable in the operation**: a
`SessionOp` carries a `SlotValue`, and there is no longer a `SlotValue`
for it.

**Row**: `crates/viewer/tests/panel_edits.rs`,
`a_count_slot_refuses_a_value_that_is_not_a_number`. It reads BOTH
sides rather than restating one — the expected error is obtained by
calling `Expr::literal(f64::NAN, Dimension::Length)` and comparing, so
a row that spelled `NonFiniteLiteral` as a literal and agreed with
itself is not what is here. It carries the pair a value door needs:
the three poisons refuse, all four Count-dimensioned slots refuse, and
every legitimate count — including the truncation toward zero at both
signs — comes back as itself. The continuous arm's `NaN` is checked by
`matches!` and not `assert_eq!`, because `NaN` is equal to nothing.

**Mutation**: deleting the `is_finite` conjunct from `of`'s `Count`
arm reds that row and nothing else in the 798-row app-feature suite.

## What is NOT fixed, and it is the other half of the same cast

`value as i64` still saturates for a FINITE value outside `i64` —
`1e30` commits `i64::MAX`, executed. It is not fixed here because it
has no refusal to raise: `DimensionError` has no arm for it and
`crates/editor-core` is EDIT's and MSOLVE's. Filed as
`work/vgeom/a-count-slots-cast-still-saturates-for-a-finite-value-too-large.md`
with the fork it needs.

PR: `vgeom/refusal-floor`.
