---
id: segment-curve-takes-a-section-index-only-to-fill-an-error-payload
kind: issue
title: segment_curve's section index exists only to fill an error payload and is the first line a sweep caller writes
status: open
opened: 2026-09-15
---

## Finding

**Raised by SCALAR's `S393` fix pass** (2026-09-15), from the demo
tour, which is the library's usage oracle (`memories/demo-purpose.md`).

`sweep::skin::segment_curve(section: usize, seg, place)` is the public
door that turns one authored sketch segment into a world-space
`NurbsCurve3`. Its first argument is a SECTION INDEX, and the body uses
it for exactly one thing: filling `SkinError::DegenerateSection {
section, what }`. Nothing about the curve depends on it.

That is invisible to a caller inside the loft, which has a section
index to hand. It is not invisible to a caller OUTSIDE one: the tour's
sweep cell converts a single arc into a path and has to open with
`segment_curve(0, …)` — a zero that means "there is no section here",
written as though it named one. It is the first line of the first
sweep a user writes, and it asks them to supply a coordinate in a
structure they are not building.

Shapes that would fix it, none of them decided here: the payload could
carry the segment's own description instead of an index; the index
could move to the caller that has one (the loft's loop wrapping the
call, adding the index to the refusal it already maps); or the door
could take an `Option<usize>` and say so.

**Where**: `crates/sweep/src/skin.rs`, `segment_curve` (~280) and
`SkinError::DegenerateSection`; the outside caller is
`demos/tour/src/skinned.rs`'s sweep cell.

**Confidence**: sure (the argument's only use is the payload).

**Verdict:**
