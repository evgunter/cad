---
id: slab-cut-cylinder-refuses-sector-side
kind: issue
title: A slab cut through a cylinder wall refuses CurvedSectorSideUnsupported — the documented second-order recourse is wired into no lane
status: open
opened: 2026-09-01
github: 1455
refs: [1377, 347]
priority: P0
cost: H
---

## From GitHub issue 1455

Opened 2026-09-01; 0 comments.

Found by the `story_authoring` integration lane building a chess rook through the GUI's op vocabulary: the natural crenellation move — a rectangular slab subtracted through a cylindrical crown — is the *first* thing a user carving a round tower reaches, and it fails.

**Repro (creation ops, headless):** `AddProfile` circle r = 0.013 → `AddExtrude` 0.008 (the drum); `AddProfile` rectangle 0.040 × 0.006 crossing the wall → `AddExtrude`; `AddBoolean Subtract` → the node fails `Boolean(CurvedSectorSideUnsupported { band … })`, no value, tree badge Failed.

The refusal is honest and typed, and its own doc-comment (`crates/topo/src/boolean/mod.rs`, the `CurvedSectorSideUnsupported` variant) names the recourse: the second-order sector trilean `geom_brep::enters_material_order2`, "which the declared-`Tangent` lump already consumes and which no lane wires into this verdict yet." That is a disclosed deviation with no scheduled followup — this issue is the schedule. Related context: #1377 (pinch-carrying family), and issue 347 (carrier-crossing refusals) is the planar-side sibling.

The story suite works around it with a square crown, so the workaround is recorded in-tree beside the ops that wanted the cylinder.

(story-suites orchestrator)

## Home

`work/bool/` — the refusal is `CurvedSectorSideUnsupported` in `crates/topo/src/boolean/mod.rs`, inside S-BOOL's territory glob `crates/topo/src/boolean/*` and its charter of operand gates that refuse legal inputs.

## Re-homed at S-BOOL's exit (2026-09-16)

Moved from `work/bool/` to CURVED (its charter names S-BOOL's ceded ground and inherits at S-BOOL's exit) when S-BOOL closed (`docs/S-BOOL-EXIT-WALK.md`); the item's content, id and history are unchanged.

## Evidence (2026-09-30, the `Borders` build)

The union form reaches it too. A round boss
`circle_split(1.5, 1.0, 0.6, 3, 0.0)` extruded 1.8 from z = 0.44 on the
plate `[0,3] × [0,2] × [0,1]`, crossed by a slab x∈(1.4,1.6),
y∈(−1,3), z∈(0.5,2.0), refuses `CurvedSectorSideUnsupported` in all six
member orders. Found by the obstacle-mechanism measurement (branch
`emit/borders-mechanism-probe`, `crates/editor-core/tests/borders_probe.rs`,
fixture `round_boss_slab`); it keeps the naming layer's curved dividers
untested.

## Measured (REACH slab-cut lane, 2026-10-01)

Both repros raise at `boolean::sectors::side_code`'s curvature charge,
called from `vtxfac` on a slab edge's pierce vertex in the cylinder
wall, with a DEFINITE first-order side:

| fixture | bound | `d̂·n̂` | arm = reach | lever `R` |
|---|---|---|---|---|
| drum ∖ slab | bisector `+y` in the slab floor | −0.231 (In) | 0.00735 | 0.013 |
| boss ∪ slab, order [0,1,2] | chord `+y`, the slab side's edge | +0.986 (Out) | 1.18 / 1.41 | 0.6 |

The charge was read at the sector's arm and at the bound's far end,
both past `R·|d̂·n̂|`, where the sagitta outgrows the first-order term.
Second order is not the missing information: the charge peaks at
`l* = |d̂·n̂|·R/2` (`1.7e-4` and `0.146` m of certified separation
here). It is now read there, capped at the reach. What still refuses
is a bound leaving the face within about `2·sqrt(band/R)` radians of
tangent (or with a reach too short to witness its slope), whose side
is second order: the residue the `enters_material_order2` recourse
still names. No real pose has been found that reaches it end to end
(72 near-tangent bars and 16 vertex-vertex touches, 2026-10-01: each
stops earlier, at `CurvedPierceUnsupported` or an escalation); its
witness is `boolean::sectors`' unit rows.

## What blocks the repros now, and what is left of this row

Both fixtures stop at the join, `SectionArcWindow { NoChartedRun }`:
a pierce ring in a wall face has no join arm. That is TANG's
`work/tang/pierce-ring-has-no-join-arm` (#1291), and the round rook
crown waits on it. `editor-core`'s `reach_slab_cut_sector_side` rows
admit that door or the closed-form volume, so they become volume
checks the day the ring lane lands.

Left on this row: the near-tangent residue above, which no fixture
reaches; and the round crown in the story suite, which is #1291's.
