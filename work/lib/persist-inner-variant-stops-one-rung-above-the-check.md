---
id: persist-inner-variant-stops-one-rung-above-the-check
kind: issue
title: pncad-py — PersistError.inner_variant gives an EditReplay the EditError's word and drops the check's, at the one arm that nests two levels
status: open
opened: 2026-09-16
---



## Found by

PORT-DIMS-1's full review (2026-09-15), which read the new
`PersistError::Dimension` arm against its stated model — `EditReplay`,
*"the arm the new one copies"* — and found the model one rung short.

## The finding

`crates/pncad-py/src/py/doc.rs:515` projects the `EditReplay` arm as:

```rust
E::EditReplay { index: at, error } => (
    word(edit_error_tag(error)),
    ...
```

`edit_error_tag` answers the `EditError`'s own arm. For
`EditError::Dimension(DimensionError)` that word is `"dimension"`, and
WHICH dimension check refused — `mismatch`, `trig_needs_angle`,
`unknown_display_unit` — is not projected anywhere. It survives only
inside the message string, which is the thing this library's error
contract exists to not do.

**The function that would supply it is already in the file.**
`crate::tags::edit_inner_variant_tag` maps `EditError::Dimension(inner)`
to `expr_dimension_error_tag(inner)`; `py/doc.rs` imports it and calls
it at **lines 150 and 211**, the two sites that project an `EditError`
onto its own Python class. The `EditReplay` arm is the third place an
`EditError` crosses and the only one that does not ask.

## It is reachable, and the kernel keeps both levels

`crates/editor-core/tests/m4_pr6_refusal.rs::a_replayed_edits_dimension_refusal_reaches_the_load_door`
is the executed fact this rests on: a save file's `edits` list is data,
so a hand-edited one carries a `SetExpression` whose replay is
ill-dimensioned, and `load` answers

```
PersistError::EditReplay { index: 0, error: EditError::Dimension(
    DimensionError::Mismatch { op: "add", left: Length, right: Angle }) }
```

Both levels are on the Rust value and matchable. Only the projection
flattens. (The row had to TAMPER a written file: `save` replays the log
through the same doors, so a log this bad is never written — which is
what the load-door re-check is for.)

## Why it is filed rather than fixed

`persist_err`'s tuple has **one** `inner_variant` slot and this arm
nests two levels, so there is no free slot with the right meaning —
`detail` is the reporter's own words, `found` is the header's text, and
the rest are the other arms' payloads. Projecting the deeper word means
either a SIXTEENTH field on a public Python class (`.pyi`, the census,
and a name for it), or redefining `inner_variant` for this arm alone,
which trades one flattening for an inconsistency. That is a payload
decision on `pncad-py`'s public surface, which is LIB's, and it is
bigger than the line PORT-DIMS-1 could have taken in passing.

Worth noting what the shape suggests: the sibling that already solved
this is `EditError` itself, which publishes `variant` AND
`inner_variant` — two levels, two names. `PersistError` wrapping an
`EditError` is three, and the class currently spends its two on the
outer pair.

## Where to look

- `crates/pncad-py/src/py/doc.rs` — the `EditReplay` arm (~515), and
  lines 150 / 211 where the same file asks the question this arm skips.
- `crates/pncad-py/src/tags.rs` — `edit_inner_variant_tag`,
  `edit_error_tag`, `expr_dimension_error_tag`.
- `crates/pncad-py/pncad.pyi` — `PersistError`'s fifteen attributes and
  the paragraph describing the nested arms.
- `crates/editor-core/tests/m4_pr6_refusal.rs` — the reachability row.
- Adjacent, same door, same fifteen-slot tuple:
  `work/lib/persist-err-projects-fourteen-arms-through-a-fifteen-slot-positional-tuple.md`.
