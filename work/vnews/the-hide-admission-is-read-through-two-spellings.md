---
id: the-hide-admission-is-read-through-two-spellings
kind: issue
title: the properties pane's hide toggle reads display_check directly while the tree's checkbox reads DocSession::instance_hidden_refusal
status: open
opened: 2026-09-28
priority: P4
cost: E
refs: [the-feature-trees-shown-checkbox-is-drawn-over-a-refusal-the-op-will-give]
---

Left by `the-feature-trees-shown-checkbox-is-drawn-over-a-refusal-the-op-will-give`,
whose lane could not touch `pane/properties.rs` (another lane held it).

## What is there

Two controls push `SessionOp::SetInstanceHidden`, and each is gated on
that op's admission, read two ways:

- `ViewerBehavior::instance_ui` (`crates/viewer/src/pane/properties.rs`)
  calls `crate::display::display_check(self.session.committed_doc(), node)`
  itself and hands the `AdmissionFault` to the private `hide_toggle`,
  which draws the fault's sentence visibly under the checkbox.
- `ViewerBehavior::feature_row` (`crates/viewer/src/pane/features.rs`)
  reads `DocSession::instance_hidden_refusal`
  (`crates/viewer/src/session.rs`), the same call wrapped as the op's
  `Refusal::Display`, and puts its sentence on the checkbox's disabled
  hover.

Today they agree by construction: one function, one document, and
`Refusal::Display` renders the fault verbatim. The second spelling is
where they would stop agreeing if the op's admission grew a layer.

## Fix

Have `instance_ui` read `instance_hidden_refusal` and let `hide_toggle`
take the `Option<Refusal>`. `instance_ui`'s early return before the
free-move probe (`addressable.is_err()`) reads the same value.

