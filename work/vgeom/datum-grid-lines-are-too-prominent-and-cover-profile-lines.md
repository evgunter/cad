---
id: datum-grid-lines-are-too-prominent-and-cover-profile-lines
kind: issue
title: Datum grid lines are far too prominent and draw over profile lines (Ev-requested, high priority)
status: closed
opened: 2026-09-17
branch: vgeom/overlay-lanes
pr: 2859
closed: 2026-09-19
---

## Ev's note (verbatim)

> plane grid lines are far too prominent. maybe they could be thinner and/or partial opacity? they at the very least should be made to not occlude profile lines. (i think the plane grid lines should very likely be thinner, but idk about the profile lines; those should be more prominent, though idk if it should be done by thickness or opacity)

## Priority

**High priority — requested directly by Ev** (in chat, 2026-09-17, from
Ev's own list of UI nits). This row goes ahead of the rest of the
program's order; see the plan's *Ev's requests* section.

## Where it lives

- Every overlay line — selection and hover marks, the profile preview
  and the datum drawings — goes through one edge pass (`gpu.rs`,
  `EdgePass`, `vs_edge`/`fs_edge`). They all share one screen width
  (the `edge` uniform's `z`), and the pass has no blending, so there is
  currently no per-lane width and no opacity.
- The datums fill `EdgeOverlay::datums` (`marks.rs`) and the preview
  fills `EdgeOverlay::preview`. Both are filled in `pane::viewport` (the
  profile-preview block and the datums block just above it). Which one
  wins where they overlap is decided by draw order within the pass,
  not by what the lines mean.
- After the reversed-Z / seen-region change (branch
  `viewer/reversed-z-grid`), a plane is ruled out toward its horizon,
  so it draws more lines than before and this gets more pressing.

## What a fix has to decide

- The grid's width. Ev's lean is thinner.
- Whether the grid gets partial opacity. That means a blended lane, or
  pre-mixing the grid colour toward the background.
- Profile lines always drawn over grid lines. Ev: "at the very least".
- Profile lines more prominent. Ev is undecided between thickness and
  opacity.

Per-lane width is the likely shape: one width per `EdgeOverlay` lane,
carried in the vertex data or in one draw per lane.

## Closed

Landed by PR 2859 (`vgeom/overlay-lanes`). Each overlay lane has its own draw priority (`marks::EdgeLane`, where `DRAW_ORDER` runs datum grid, committed profile, preview, hovered, selected) and its own width and opacity (`gpu::lane_style`). The edge pass blends in straight alpha. The grid is 1 pt at `theme::DATUM_OPACITY` = 0.5 and draws first, so no profile, preview or mark is covered by it. Profiles stand out by thickness (3 pt, opaque). A theme test holds profile apart from grid and preview under all three palettes and the dichromacies.
