---
id: profile-pieces-are-named-by-minted-step-ids
kind: unit
title: Build N1's profile-piece rule: every program step carries a minted StepId and a profile locator is { step, role }
status: open
opened: 2026-09-25
priority: P0
cost: H
---


## What

Ev ruled on #3193 (2026-09-25, "yes this makes sense!") that a profile
piece is named by an id minted when its step is authored, not by its
position. The rule is `names/README.md` "N1, the profile pieces". This
unit builds it.

## Scope

- **The id.** Every profile program step carries a `StepId` in the recipe,
  minted from the document's monotone counter at `InsertNode` and
  `SetProgram` and validated at load. Ids are never reused.
- **The locator.** `ProfileEdgeRef` / `ProfileVertexRef` become
  `{ step, role }`, with no loop index and no segment index. Each verb has a
  fixed role list:
  - fillet: run in, arc, run out;
  - `circle_split`: piece `k`;
  - `circle`: carrier.
  The replay record carries a per-segment role.
- **The anchor.** The naming anchor (`eval/anchor.rs`) translates each
  canonical segment to `{ step, role }` when the name table is published.
  Emission, loft correspondence and the viewer's marks still iterate
  canonical numbering (V3, DM8).
- **Loft walls.** A wall is named by the pieces it pairs, one locator per
  section.
- **What goes.** The positional rename machinery is deleted, and its
  rows re-baselined or removed:
  - `LoopProvenance`;
  - the `SegmentMap` rewrite;
  - `RETIRED_FLOOR`;
  - the value-edit carry (`reanchor_report`, PR 3180).
- **Wire.** The saved-document format breaks. An old file refuses typed,
  with the regenerate recourse. Goldens and corpus files that carry
  `"segment": N` are re-baselined.
- **Python.** A piece is spelled by the step handle its authoring call
  returned.
- **Ratified text.** Re-word, with the change, the ratified text that
  describes the positional numbering:
  - V3 "index CANONICAL positions";
  - DM8's loft sentence;
  - DM7's `Rebound` arm and its value-edit arm.
  Ev ruled the design on #3193, so these re-wordings describe the ruled
  change.

## Ruled before build (2026-09-25)

Ev answered the three questions on #3202:
- **Roles.** Roles stand as the path-language side of the name, and
  whatever the user sees speaks the language the path was written in.
  Where a fillet run and an authored leg share one segment, the earlier
  piece in authored order names it.
- **Circles.** A circle is `Piece(0)`/`Piece(1)` for now, and this unit
  ships with that. PATHS's lowering question
  (`work/paths/lower-profiles-to-carrier-and-interval-not-vertex-and-bulge.md`)
  would make it one `Carrier`, re-spelled then, with a second format
  break.
- **Loft seams.** A loft seam takes one vertex locator per section. Caps,
  rims and cap vertices take their own section's locators.

## Closes

- `a-child-documents-rebind-leaves-the-parents-held-names-in-the-old-numbering`
  (P0). Its measured row `asm_parent_held_names` turns green.
- `a-value-edits-last-published-numbering-is-not-recipe-state` (P1).
  Its fork no longer arises.
- `the-value-edit-numbering-check-costs-a-replay-per-swept-profile` (P2).
  The check it costs is deleted.
- Not EMIT's to close: EDIT's
  `a-slot-edit-through-a-zero-fit-renumbers-a-loops-live-names`, whose
  question (#3163) this ruling answers, and #3158's retirement question.
  Under minted ids a dropped id is never re-minted, so no floor is
  needed. The unit states what it closes on EDIT's slate, and EDIT's
  orchestrator closes it.

## Cost

79 non-test locator references across 24 files (`sweep`, editor-core
names/anchor/program/edit, `pncad`, `pncad-py`, the viewer's marks),
plus id minting and validation. The count is from #3193's survey,
recorded in `work/emit/log.md` (2026-09-24). This is a kernel unit and
is split if it runs long.
