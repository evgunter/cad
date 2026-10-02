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
`a_bar_through_a_ball_crosses_the_sphere`, which flips when this row
lands.

## Evidence (2026-10-01, TANG's circle × cylinder cell): an ARC through a wall

With the circle × cylinder root lane, a rim CIRCLE piercing a cylinder
wall reaches this site too: it is now the first wall for the
parallel-axes cylinder pairs that pierce below. Two unit cylinders,
`z ∈ [0, 2]` and `z ∈ [0.5, 2.5]`, axes `d` apart: `d ∈ {0.3, 0.8, 1.2,
1.6}` refuse `CurvedSectorSideUnsupported { Negative }` under ∪, ∩ and
∖ (margins −1.31, −0.40, −0.55, −0.22, the same with both operands
spun about z by 2 rad). The same door is now where a 30° rod through a
cylinder's rim stops, and where the transversal cylinder × ball and the
cylinder × torus of `verbs_cylsph_opening.rs` stop (which of their
pierces raises first is not pinned). Pinned by
`crates/sweep/tests/tang_circle_cylinder.rs`,
`parallel_cylinders_that_pierce_stop_at_the_sector_side`, which flips
when this row lands.

What the arc adds to the diagnosis: `side_code` reads an arc bound's
side from its departure DIRECTION levered at its extent
(`Reach::Extent`), against the wall's sagitta. For an arc, that ignores
the arc's own bend as well as the wall's, so the first-order model is
wrong twice. The arc's certified roots against the pierced wall bound
where it can next cross, so the residual at any interior point of the
arc between the pierce and the next root has the side's sign exactly.
That is a second-order-free reading that an arc lane could use, beside
the `enters_material_order2` recourse this row already names.
