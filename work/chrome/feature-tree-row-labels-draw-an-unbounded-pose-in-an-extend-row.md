---
id: feature-tree-row-labels-draw-an-unbounded-pose-in-an-extend-row
kind: issue
title: viewer: row_label draws '{kind} - {pose}' with pose unbounded, in a non-wrapping row
status: open
opened: 2026-09-22
---


`crates/viewer/src/pane/features.rs`'s `row_label` composes
`format!("{} — {pose}", row.kind)` and draws it with
`ui.selectable_label` inside `features_ui`'s per-row
`ui.horizontal` — a non-wrapping row, so
`egui::TextWrapMode::Extend`, so the galley is laid out at infinite
width.

`row.kind` is a closed set this crate enumerates. **`row.pose` is not**:
it is composed from the node's own placement, and nothing bounds its
length. Under the test
`messages-in-the-creation-and-properties-panes-still-draw-past-their-row`
states — *a text is a name only when its drawn width is bounded by
something this crate controls* — this is a sentence, and it is drawn
by the layout's rule rather than the message's.

The PR that added `crate::widgets::message` reported this site as
*"noted, not filed"*. It is filed here because
`docs/prompts/reviewer-style-lane.md` Q6 is explicit that a disclosed
deviation owes a concretely scheduled followup, and "noted" is not a
schedule.

**It is not simply a `message` call.** A `selectable_label` is a
control whose click selects the row, and handing it a pre-laid-out
galley changes what its rect is and therefore what a click hits. The
cheap read is whether the pose belongs in the row at all or on hover;
the expensive one is a `message`-shaped door for a selectable row.
