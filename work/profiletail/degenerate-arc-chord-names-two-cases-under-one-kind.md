---
id: degenerate-arc-chord-names-two-cases-under-one-kind
kind: issue
title: paths: DegenerateArcChord carries both a coincident chord and a tangent arc whose end lies on its tangent line behind it; a separate variant would let each message name its own case (an [ev] candidate: the kind is ruled in PATHS-DESIGN)
status: open
opened: 2026-10-19
priority: P3
cost: M
design: true
---


Found by #5403's review (F3).

`PathError::DegenerateArcChord { chord }` refuses two different
geometries under one kind, as `docs/PATHS-DESIGN.md` rules ("behind it
is the full turn (`DegenerateArcChord`)"):
- the arc's ends coincide (every arc mode);
- a tangent arc (`tangent_arc_to`, and `arc_onto` run reversed) has
  one end on the line tangent at the other, on the side only a full
  turn reaches. Its chord is whatever the ends span, 2.7 m in the
  rows.

#5403 first carried a `behind: bool` field to word each case. That is
a tag, which PATHS-DESIGN's two-types-over-a-tag ruling retired
elsewhere (`line_close`, `site`). The message cannot read the band, so
the variant's data cannot tell the cases apart. So the message now
names both cases, with recourse for either, and the `chord` it
reports tells the reader which one it is.

The message is shared by every arc mode, and only a tangent arc has a
tangent line at its tip (a `Via`, `Center`, `Bulge` or `arc_onto`-close
leg has none). #5403's re-review found the first merged wording, "one
end lies on the other's tangent line", false for those legs, so it now
reads "its ends coincide within tolerance, or a tangent arc's far end
lies on its tangent line where only a full turn reaches it", true of
every leg. A separate variant would let each leg's message drop the
case it cannot reach.

**The candidate question for Ev.** Should the full-turn case be its
own variant, with its own kind and its own message per verb (the
tangent line belongs to the tip for `tangent_arc_to` and to the target
for `arc_onto`)? That changes a ruled kind, so it is an `[ev]` PR.

## Re-homed at PATHS's close (2026-10-21)

Moved from `work/paths/` to PROFILEPROSE (`work/profileprose/`) when PATHS closed. PATHS's live rows went to four programs opened at this close and cut along the priority seam (PROFILE, PROFILEREST, PROFILETAIL, PROFILEPROSE; `docs/doc-ledger/paths-leaves-the-tracker.md`). The item's id, content and history are unchanged.
