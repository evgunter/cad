---
id: committed-profiles-are-not-drawn-in-the-viewport
kind: issue
title: A profile vanishes from the viewport once it is committed (Ev-requested, high priority)
status: open
opened: 2026-09-17
---

## Ev's note (verbatim)

> profiles seem to be invisible after they're actually written and are out of preview. i think it makes sense to display them always, like the datums, though the preview profile could have a different color

## Priority

**High priority — requested directly by Ev** (in chat, 2026-09-17, from
Ev's own list of UI nits). This row goes ahead of the rest of the
program's order; see the plan's *Ev's requests* section.

## Where it lives

- The profile form's preview is the only place a profile's loops reach
  the viewport. `sketch::preview` replays the form's shapes, and
  `pane::viewport` draws the result into `EdgeOverlay::preview`.
- Once the profile is committed, the form is at rest and nothing draws
  the node's loops. A profile node has no body until something extrudes
  or revolves it, so the scene mesh has nothing for it either.
- Datums are drawn every frame from the landed evaluation
  (`datums::draws`). Committed profiles want the same treatment: a pass
  over the evaluation's profile nodes, flattened by the same
  `sketch::flatten`, in a lane of their own.
- Ev: the preview may keep a different colour from committed profiles.
  That is one more `EdgeOverlay` lane or mark.

Probably shares its lane and width work with
`datum-grid-lines-are-too-prominent-and-cover-profile-lines`, which
wants profile lines drawn over the grid.
