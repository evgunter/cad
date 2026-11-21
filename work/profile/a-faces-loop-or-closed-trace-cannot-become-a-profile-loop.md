---
id: a-faces-loop-or-closed-trace-cannot-become-a-profile-loop
kind: issue
title: A picked edge loop, or a face's whole closed trace, cannot become a profile loop: an elbow off a hollow cylinder's end is re-typed by hand
status: open
opened: 2026-11-20
priority: P1
cost: M
design: true
---


Asked by Ev in chat (2026-11-20), and banded P1 at Ev's request: "is it
possible to use an edge selected in the viewer as a profile that can
then be revolved or something? like if i have a hollow cylinder, can i
make an elbow by clicking on the outer and inner loops, putting them
into a profile on the cylinder end face, and revolving it?"

## What exists

- **New profile on a face** opens a profile on the latched face's chart
  (`Recording::draw_on_face`; `crates/viewer/README.md`, "New profile on
  a face").
- **Traces** (`crates/editor-core/src/trace.rs`; DESIGN "Constructing a
  coincidence"). A profile drawn on a frame reads a face's carrier cut
  by the frame's plane. Three document forms read one, each lowering to
  a step of the profile's own vocabulary: a straight piece running to
  `ProgramTarget::OnTrace`, an arc about a centre running to `OnTrace`,
  and `ProgramStep::AlongTrace` (a heading). In the viewer, a second
  latched face makes the profile's one chain run onto that face's trace
  and along it.

## What is missing

Nothing takes a picked edge, a picked loop, or a face's whole closed
trace and makes it a loop of a profile. For the elbow, the outer and
inner walls of the hollow cylinder, cut by the end face's plane, are
exactly the two circles wanted, and the end face's own two loops are
those circles again. Today the person opens a profile on the end face
and types two circles (centre and radii, or the variables the cylinder
was built from), then revolves.

## The design question (`design`)

What a profile loop that comes from existing geometry is: what the
person picks (edges, a loop, a face, a face's trace), what the document
stores (a read that re-resolves when the body changes, as a trace does,
never a copy of numbers), how it lowers to the profile's vocabulary so
the replay sees an ordinary program, what it refuses (an open chain of
edges, a non-planar loop, an edge with no closed form in the plane), and
how it sits beside the three trace forms rather than as a second way to
say what they say (`[one-way]`).

## Not checked

- Whether the three trace forms can already spell a whole circle (an arc
  about a centre from the trace back to itself), which would make part
  of this sugar over what exists.
- Whether the hand-typed elbow builds: an annulus on the end face
  revolved about an axis in that plane. Read, not run.
