---
id: a-seated-tools-held-node-is-drawn-nowhere
kind: issue
title: A seated tool's held node seats are drawn nowhere in the viewport, so a tool can commit against a body the picture does not single out
status: open
opened: 2026-09-30
priority: P2
cost: M
design: true
---


Found by AUTH-10's census of latched picks (the held-face-mark unit),
which the spec scoped out: *"If held node seats are also undrawn, that
is a row, not this unit."* They are.

## What

Seven modal tools hold NODE seats in tool state across frames —
revolve, boolean, split, transform, pattern, projection and duplicate,
each over `seats::Seats` (`crates/viewer/src/seats.rs`, the `held`
field; the tools in `combine.rs` and `revolvetool.rs`). A seat is
filled from `Selection::seat_node`, so a face or edge click fills it
with the node whose drawn body the ray met, and a tree click fills it
directly. Nothing clears a seat but the tool.

The viewport draws none of them. `pane::viewport::frame_marks` gathers
every held pick the picture marks — the add-datum form's face, the mate
tool's two faces, the blend tool's edge set — and there is no node in
it: the seated tools are named in its doc as the one holder it does not
draw. The only evidence a seat is filled is the panel's line
(`seats::picks_line`), which says `feature 3` and not where feature 3
is. So a boolean's two operands, picked by clicking two bodies, stop
being marked the moment the selection moves on, which is the defect
`held-face-pick-is-invisible-in-the-viewport` closed for faces.

## Why it is a design question and not the face fix again

A face has one drawn patch, so its held mark is an id in
`marks::Highlight::held`. A node seat names a whole body — possibly
several (a pattern), possibly none drawn (a profile, a datum, a split
the projection hides) — so the mark is an EXTENT, which is what
`marks::focus` answers for the side panel's selection. Whether a held
node wears the held mark (stripes, `marks::Held`) over that extent, and
through which lane — `focus` is a scene flag and rebuilds the mesh,
`Highlight::held` is one uniform id per holder — is the decision.
