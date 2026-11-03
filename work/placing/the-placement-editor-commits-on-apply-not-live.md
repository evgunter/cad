---
id: the-placement-editor-commits-on-apply-not-live
kind: issue
title: The placement editor commits on Apply and has a Create door; Ev's rule is Place-then-edit with each change committed live
status: dispatched
opened: 2026-11-02
priority: P1
cost: M
refs: [creating-a-thing-and-editing-it-are-two-implementations]
branch: placing/place-live
---


## Finding

Ev ruled on `[ev]` PR 6388 (2026-11-02). The rule is in
`crates/viewer/README.md`, "A create gesture commits the node". It
binds the placement editor (`crates/viewer/src/matetool.rs`) in two
ways.

- **Place, not Create.** On an unplaced body, Place inserts its
  placement where the body stands, with no mates and every slide and
  spin at zero, and opens the editor on it. Every mate is then added
  in the editor. `PlaceDoor::Create` and a `SessionOp::Place` carrying
  a bundle go. The README's placement section already says this.
- **Live commits, no Apply.** Each change the person makes commits as
  one `SetBundle` and one undo step, provided the edited bundle passes
  the document's door. A change the door refuses is not applied: the
  editor holds it as a draft over the document, shows the refusal, and
  commits the draft whole on the first later change that makes it
  pass. The README's placement section still describes the Apply, the
  verdict before Apply and `BundleStale`. Rewrite those sentences with
  the build.

FORMS built the same rule for the profile editor (#6403): live
commits, a held refused draft, and a "Discard changes" control. Read
that before designing this one, so the two editors behave alike.
The rule's census row is
`work/forms/creating-a-thing-and-editing-it-are-two-implementations.md`.

Signed (FORMS orchestrator).
