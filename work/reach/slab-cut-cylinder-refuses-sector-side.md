---
id: slab-cut-cylinder-refuses-sector-side
kind: issue
title: A slab cut through a cylinder wall refuses CurvedSectorSideUnsupported — the documented second-order recourse is wired into no lane
status: closed
opened: 2026-09-01
github: 1455
refs: [1377, 347]
priority: P0
cost: H
closed: 2026-10-01
pr: 3627
branch: reach/slab-cut-sector-side
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

## Evidence (2026-10-01, `reach-snowman`): a bar through a ball

The sphere form reaches the same site. A square bar
`brick((0.5, 2.0), (-0.3, 0.3), (-0.3, 0.3))` poking out of the unit
ball (a full revolve about `y`) refuses `CurvedSectorSideUnsupported`
under ∪, ∩ and ∖ once the crossing layer has a line × sphere root lane;
before it, the same pair stopped at the pierce door. The variant's one
raise is `sectors::side_code`'s `(_, Ok(_))` arm, so the sagitta charge
`arm²/lever` (lever = the sphere's radius) swamped the first-order
displacement at both the sector's arm and the reach's length. Pinned
as `crates/sweep/tests/snowman.rs`,
`a_bar_through_a_ball_crosses_the_sphere`. With the charge read at
its peak (below) the bar's sector sides certify and every op stops at
the pierce-ring door instead; the row now pins that door. The sibling
`m5_s13_review_probes::probe_edge_escape_refuses_typed_before_the_scan`
(a ball whose section circle crosses a slab face's edge) moved the same
way, to the join's `SectionNotPolar`.

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

## Closed (2026-10-01, PR 3627)

The sector-side charge reads the curvature at its peak, `l* = |d̂·n̂|·R/2`,
capped at the reach. Both repros now pass the sector side and stop at
TANG's pierce-ring door, as do the bar through a ball and a newly
reached family of vertex-touch poses. Every body that now builds matches
its closed form.

The dual review found a WRONG ∩ body. A straight chord in a cylinder wall
was taken as the section structurally, and ten bar-on-cylinder poses
shipped bodies that failed `validate_geometric`. The planar-side join now
reads a line as the section only when its midpoint lies on the wall (a
ruling). A pin row covers that branch, and 716 bodies on the widened scan
match an independent oracle.

Residue, each in its own file:
- the near-tangent remainder above, which no fixture reaches;
- TANG's `pierce-ring-has-no-join-arm` (#1291), where the repros and the
  round crown now stop;
- HONE's `planar-side-join-takes-an-off-wall-arc-for-the-section-by-window-alone`;
- HONE's `join-adjacency-escalation-surfaces-as-a-section-invariant`;
- `boolean-door-passes-a-geometrically-open-result-the-backstop-cannot-see`
  (the backstop's negativity floor, with the door's tier-3 gap);
- `vertex-vertex-side-codes-take-no-curvature-charge-on-curved-sector-faces`.
