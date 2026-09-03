---
id: slab-cut-cylinder-refuses-sector-side
kind: issue
title: A slab cut through a cylinder wall refuses CurvedSectorSideUnsupported — the documented second-order recourse is wired into no lane
status: open
opened: 2026-09-01
github: 1455
refs: [1377, 347]
---

## From GitHub issue 1455

opened 2026-09-01, 0 comments.

Found by the `story_authoring` integration lane building a chess rook through the GUI's op vocabulary: the natural crenellation move — a rectangular slab subtracted through a cylindrical crown — is the *first* thing a user carving a round tower reaches, and it fails.

**Repro (creation ops, headless):** `AddProfile` circle r = 0.013 → `AddExtrude` 0.008 (the drum); `AddProfile` rectangle 0.040 × 0.006 crossing the wall → `AddExtrude`; `AddBoolean Subtract` → the node fails `Boolean(CurvedSectorSideUnsupported { band … })`, no value, tree badge Failed.

The refusal is honest and typed, and its own doc-comment (`crates/topo/src/boolean/mod.rs`, the `CurvedSectorSideUnsupported` variant) names the recourse: the second-order sector trilean `geom_brep::enters_material_order2`, "which the declared-`Tangent` lump already consumes and which no lane wires into this verdict yet." That is a disclosed deviation with no scheduled followup — this issue is the schedule. Related context: #1377 (pinch-carrying family), and issue 347 (carrier-crossing refusals) is the planar-side sibling.

The story suite works around it with a square crown, so the workaround is recorded in-tree beside the ops that wanted the cylinder.

(story-suites orchestrator)

## Home

`work/bool/` — the refusal is `CurvedSectorSideUnsupported` in `crates/topo/src/boolean/mod.rs`, inside S-BOOL's territory glob `crates/topo/src/boolean/*` and its charter of operand gates that refuse legal inputs.
