---
id: feature-tree-row-labels-draw-an-unbounded-pose-in-an-extend-row
kind: issue
title: viewer: row_label draws '{kind} - {pose}' with pose unbounded, in a non-wrapping row
status: closed
opened: 2026-09-22
priority: P1
cost: D
closed: 2026-09-23
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

## Read against the tree (`chrome/message-floor`, 2026-09-22) — left open on a layout question

**The pose is bounded, so "unbounded" overstates it.**
`crate::tree::frame_pose` composes a plane name from a closed set
(`plane_name`), the words `at` / `origin driven` / `on …'s face`, a node
number (`crate::tree::node_number`, a `u64`), and up to three
coordinates through `crate::props::render_number` — `{:?}`'s shortest
round-trip spelling, at most 24 characters for any finite `f64` — with
one unit symbol from a closed set. So a pose is at most about ninety
characters. Every part is bounded by something this crate controls,
which is the census test's definition of a NAME. What the test's soft
edge ("short enough to read at a glance") does not cover is a
ninety-character name, and a pose with three long coordinates is one:
several hundred points, in an `Extend` row, in a pane that scrolls both
ways (`crate::app`'s `ViewerBehavior::pane_ui`). So the label is never
unreadable. It is reached by scrolling right, which is the "then
scroll" half of Ev's ruling with no floor in front of it.

**Why it was not changed here.** Every layout that bounds the label's
width moves something on every row of the tree, and which one is a
question about how the tree looks:

- wrap the label (a selectable button over a pre-laid-out galley, at
  the message floor). The badge and the `shown` checkbox that follow
  it in the same `ui.horizontal` then land past the pane's right edge
  whenever the label wraps, which hides a failing row's badge exactly
  when its label is long;
- move the pose to its own line under the row, at
  `crate::pane::features`'s `message_indent`, which makes every frame
  row two lines tall;
- keep one line and put the pose on hover. This drops what
  `a_frame_row_says_which_frame_it_is` and
  `two_frame_rows_a_centimetre_apart_read_differently` hold, which is
  that two frames are told apart without pointing at them.

The question for Ev: is a long pose in the tree fine to reach by
scrolling, or which of these does the tree want?

## Ev's ruling (in chat, 2026-09-23) — closed

Asked whether a long pose should stay reachable by scrolling or the
tree should (a) wrap the label, (b) put the pose on its own line, or
(c) show it on hover only, Ev answered: *"keeping them scrolling is
good!"*. The row's corrected premise stands (a pose is bounded, at
most about 90 characters), and the `ScrollArea::both()` every chrome
pane sits in already reaches it. No change.
