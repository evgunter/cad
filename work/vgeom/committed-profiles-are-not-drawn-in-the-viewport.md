---
id: committed-profiles-are-not-drawn-in-the-viewport
kind: issue
title: A profile vanishes from the viewport once it is committed (Ev-requested, high priority)
status: closed
opened: 2026-09-17
branch: vgeom/overlay-lanes
pr: 2859
closed: 2026-09-19
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

## The profile being edited draws its preview, not its committed loops

(Added by VSEAM, `editing-a-profile-does-not-share-the-create-forms-interface`.)
A committed profile can now be opened in the add-profile form's editor
from the Properties pane (`ViewerBehavior::edit_profile_ui`), and while
it is, its held loops are previewed in the viewport exactly as the
create form's are (`ViewerBehavior::profile_previews`, the `edit`
door). The committed-profile pass must SKIP the node being edited, or
the picture shows the old program and the edited one on top of each
other — wired by PR 2862 as `sketch::committed`'s `except`.

## Closed

Landed by PR 2859 (`vgeom/overlay-lanes`). `sketch::committed` draws every evaluated profile each frame into `EdgeOverlay::profiles` in `Theme::profile`. The live preview keeps its probe tint. A profile whose evaluation refused draws nothing, and one that cannot be flattened is counted by `frame::profiles_badge`. The add-profile form settles on an accepted add (`Drafts::accepted`), so a new profile is not drawn twice. The `except` seam that leaves the node being edited out is wired by the profile-editor unit, PR 2862 (VSEAM, `editing-a-profile-does-not-share-the-create-forms-interface`). Residue: `zoom-to-fit-frames-no-committed-profile` (this slate).
