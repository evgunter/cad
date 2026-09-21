---
id: persist-err-projects-fourteen-arms-through-a-fifteen-slot-positional-tuple
kind: issue
title: pncad-py — persist_err is a 319-line function projecting 14 arms through a 15-slot positional tuple, where a transposed pair is invisible
status: open
opened: 2026-09-15
priority: P1
cost: D
---


## Finding

Found in the S415 style review (2026-09-15), reading `py/doc.rs` for
the tag-minting class.

`crates/pncad-py/src/py/doc.rs`'s `persist_err` is **319 lines**. It
destructures `PersistError`'s arms and binds a **15-element
positional tuple** — `(variant, message, node, param, path, …)` — one
`match` arm per refusal, **13 `E::` arms**, and fills the slots an arm
does not carry with **171 calls to `none()`**.

The defect is not the length. It is that **every field is identified
by its POSITION in a 15-tuple**, so two adjacent slots of the same
type transposed in one arm is:

- invisible to the compiler — both are `Py<PyAny>`;
- invisible to a reviewer — the arm is a column of `none()` calls and
  the eye cannot count to the eleventh;
- invisible to the tag guard — `variant` is still right;
- visible only to a Python caller reading a payload attribute that
  holds another attribute's value.

**Its own sibling solves this.** `edit_fields`, thirty lines above in
the same file, projects the same shape through **named** fields, and
the `#[derive]`d payload structs in `crate::edit_payload`,
`crate::check_payload` and `crate::mate_payload` all do the same. So
this is not a missing idiom; it is one door that did not take the
idiom the file already has.

## Where to look

- `crates/pncad-py/src/py/doc.rs` — `persist_err` (the tuple and the
  match), `edit_fields` (the shape it should have).
- `crates/pncad-py/src/edit_payload.rs`, `check_payload.rs`,
  `mate_payload.rs` — the named-field precedent.
