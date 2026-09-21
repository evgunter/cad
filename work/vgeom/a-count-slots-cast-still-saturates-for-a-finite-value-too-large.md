---
id: a-count-slots-cast-still-saturates-for-a-finite-value-too-large
kind: issue
title: A finite Count value outside i64 still saturates to i64::MAX, and there is no refusal vocabulary for it
status: open
opened: 2026-09-21
priority: P2
cost: E
---


## Finding

Found by the refusal-floor sweep while closing
`a-count-slot-launders-a-typed-nan-into-zero`. That row's fix makes
`props::SlotValue::of`'s `Count` arm refuse a value that is not
finite, by the name `Expr::literal` uses for the continuous half
(`DimensionError::NonFiniteLiteral`). **The other half of the same
cast is untouched**: `value as i64` saturates for a finite value
outside `i64`'s range too, and that arm still commits a number nobody
typed.

`crates/viewer/src/props.rs`, `SlotValue::of`'s `Count` arm — the
`value as i64`.

Executed under `rustc -O`:

| typed | committed |
|---|---|
| `1e30` | `9223372036854775807` (`i64::MAX`) |
| `-1e30` | `-9223372036854775808` (`i64::MIN`) |
| `9.3e18` | `9223372036854775807` |
| `f64::MAX` | `9223372036854775807` |

The route is the one the parent row establishes and is unchanged by
its fix: `props::field_edit` reads anything `f64::from_str` accepts as
a `FieldEdit::Number`, `crates/viewer/src/widgets.rs`'s typed arm
hands it to `SlotValue::of`, and `crates/viewer/src/pane/properties.rs`'s
`slot_field` sets no `egui::DragValue` range, so `1e30` is a value the
field accepts. The four Count-dimensioned slots are `SlotId::Count`,
`VDegree`, `Stations` and `Instance`
(`crates/editor-core/src/node.rs`, `SlotId::dimension`'s `Count` arm).

## Why it was not fixed with the parent row

**There is no refusal to raise.** The parent's fix works because
`DimensionError::NonFiniteLiteral` already exists and already says the
right sentence — *a literal value must be finite* — so the viewer
raises the kernel's own word rather than minting one. For "this count
does not fit an `i64`" there is no variant, in `DimensionError` or
anywhere else the viewer may raise: `crates/editor-core` is EDIT's and
MSOLVE's, and `work/vgeom/program.md`'s `keep_out` says *"a numeric
door the viewer consumes is a hand-off and never a diff from here."*

So this needs a decision before it needs a diff, and the fork is:

- **a door in `editor-core`** — a `DimensionError` (or `Expr::count`)
  arm for a count outside the representable range, which is where the
  continuous half's finiteness refusal already lives; or
- **a refusal of the viewer's own**, in `session::Refusal`, which is
  VNEWS's vocabulary and would be the first numeric refusal the viewer
  words for itself.

Whichever is taken, the shape at the door is the parent's: `of`
already returns a `Result` and the call sites already take it.

## Not established here

Whether `Expr::count` or the document below it bounds a count of
`i64::MAX` anyway. If it does, the user still gets a refusal about the
wrong thing — a count out of range rather than a number that was never
a count — which is the parent row's own "Not established here",
unmeasured for the same reason.

## Fence

`crates/viewer/src/props.rs` — this program's, with the decision half
reaching `crates/editor-core` (EDIT's and MSOLVE's) or `session/`
(VNEWS's and VSEAM's).
