---
id: disabled-reasons-still-pass-as-strings-through-gated-button
kind: issue
title: Disabled reasons still pass as strings through widgets::gated_button and commit_row, which take &str
status: open
opened: 2026-11-22
priority: P3
cost: M
---


## Finding

Found by FORMWORDS' seat-vocabulary unit (PR 7374). That unit gave the
creation tools' commits one gate. `pane::create`'s `withheld_commit_row`
takes the tool's formed op and reads the withheld reason off its typed
fault (`CommitFault::withholds`), and draws it under the commit as well
as on its hover. Under that row, `crates/viewer/src/widgets.rs`'s
`commit_row` and `gated_button` still take `blocked: Option<&str>`. Any
caller can hand them a sentence of its own, and these do:

- `crates/viewer/src/pane/create.rs`, `ViewerBehavior::all_edges_row`:
  `Select all edges` is disabled through a bare `add_enabled(false, …)`.
  Its literal hover is *"click an edge or a face of the body first, and
  let it evaluate…"*, it is said only on hover, and the condition
  (selection, landed pair, index) is computed beside the load it gates.
- `crates/viewer/src/frame.rs`, `SELECT_A_BODY_TO_RE_POINT_ONTO`: a
  re-point button's `op: Err(String)`, rendered through
  `widgets::repoint_button` in `pane/create.rs` and
  `pane/properties.rs`. The reason is a string rather than a refusal
  value, and it is said only on hover.
- `crates/viewer/src/pane/properties.rs`, the add-variable form's gate
  (`"name the variable"`, `"pick a dimension first"`): already drawn in
  plain view and on the hover from one evaluation, but as `(String,
  Tone)` rather than as a typed fault.
- `crates/viewer/src/matetool.rs`'s refusals:
  `work/doors/the-mate-tool-says-an-empty-seat-in-words-of-its-own.md`.

## Owed

Decide whether the `&str` parameters become a typed reason (a
`Refusal`, a `CommitFault`, or a trait both render through), so that no
caller can pass a sentence of its own. Decide too whether every
disabled reason is also drawn in plain view, as the creation commits
now are (`work/chrome/a-disabled-pane-control-says-why-only-on-hover.md`
is the per-control half of that). `work/chrome/a-disabled-controls-reason-has-one-home.md`
is the README clause the result would state.
