---
id: a-per-kind-sentence-lives-in-the-widget-not-on-the-choice
kind: issue
title: A sentence true of one datum kind is hardcoded in the widget rather than beside the choice it describes
status: open
opened: 2026-09-21
priority: P4
cost: E
---


## Finding

Found by AUTH-1's style reviewer (AUTHOR, PR 2955, 2026-09-21),
confidence `sure` that the class is live and `likely` on the newest
instance. In `crates/viewer/src/pane/create.rs`'s `add_datum_ui`,
three sentences true of exactly one `DatumKindChoice` are hardcoded
inside that kind's match arm rather than living beside the choice:

- the `Frame` arm's *"y is squared against x; the normal is x × y"*;
- the `AxisInPlane` arm's *"x and y are the frame's own; a revolve
  needs its profile on this frame"*;
- and the `FaceFrame` arm's *"sketch +x, turned about the face's
  outward normal; the origin is the face's"*, which AUTH-1 added.

**The vocabulary for doing this right already exists in the same
crate**: `DatumKindChoice::unmet_seat` (`crates/viewer/src/forms.rs`,
AUTH-1) and `blend_kind.size_label()`
(`crates/viewer/src/blend.rs`) both put a per-kind sentence on the
kind, where the compiler's exhaustiveness holds the roster. A sentence
in the widget has no such holder.

## Not a MAJOR, and why it is P4 rather than P1

These three sentences are each TRUE, and a per-kind explanatory line
in a form is not wrong the way a duplicated rule is wrong — nothing
computes with them and nothing can drift out of step with a second
copy, because there is no second copy. What it costs is that a fourth
kind gets no such line unless its author remembers, which is the
`ALL`-roster problem one level down. Prose and naming hygiene,
`work/README.md`'s P4 band.

## The reason it is worth a row at all

The reviewer's framing, which is the part to keep: AUTH-1's whole
subject was replacing a hardcoded per-kind sentence
(`"pick a frame to write the axis in"`) with a roster, and **the same
diff added a hardcoded per-kind sentence twelve lines away**. That is
`docs/prompts/reviewer-style-lane.md`'s fresh-instance trap landing
exactly where it says it lands, and only a reader who did not write
the fix caught it. Where else to look, per the reviewer:
`add_profile_ui`'s `blocked` strings in the same file.
