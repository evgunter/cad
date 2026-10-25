---
id: delete-the-boolean-operand-edge-gate
kind: unit
title: delete gate_operand_edges once every site behind the sweep refuses a spline or spiric edge typed
status: open
opened: 2026-10-03
priority: P1
cost: M
refs: [3984]
---


Filed by `planar-crossing-lane-reads-a-curved-carrier-as-a-line` (PR
3984) on the REACH orchestrator's ruling for that PR's last fix pass:
the PR keeps the gate, because the dual review was frozen on a head
that keeps it, and the deletion is this unit.

## The decision it carries out

The NURBS/spiric operand fork's reconciled reports
(`analysis/design-fork/nurbs-spiric-operands-d1` and `-d2`,
`design.md`, round 1), adopted by the orchestrator: delete
`boolean::reduce::gate_operand_edges` once every site it silently
protects refuses a spiric or spline edge typed at its own site, naming
the edge and face. A refusal then names the pair that lacks a rung
(C12.1, "retire per arm, never wholesale"), and no second roster of
admitted carriers sits beside the lanes.

## What is left to type

PR 3984 typed the sweep's planar and curved arms
(`BooleanError::CrossingCarrierUnsupported`, unreachable through the
public doors while the gate stands). The sites behind the sweep are
`join-and-continuation-sites-blame-the-edge-gate-for-a-spline-edge`,
plus the sector walk's NURBS chord (`boolean/sectors.rs`
`build_sectors`; the split's twin is
`a-nurbs-edges-sector-departure-is-its-chord`).

## Prior art on the PR branch

PR 3984's branch carries a full attempt, backed out by revert before
review: c4f3840bd (one `EdgeCarrierUnsupported { operand, edge, face,
site }` variant raised at six sites — the two crossing arms, the germ
frame, the ring run, the vertex sector walk and the continuation
scan's face extent — the gate and `CurvedEdgeUnsupported` deleted,
each site with a row that reds without its fix), af3e825c0 (a census
of the loft prism and the spiric-rimmed vessel cavity through every op
against five brick placements in both orders, every `Ok` held to its
closed-form volume), 9f9b87add and 5501a3f03. Measured on that attempt,
at ε 1e-9, 1e-6 and 1e-12:

- nothing builds in the census; every outcome refuses typed;
- teapot wall 3 and lily wall 8 move to `CurvedBooleanUnsupported{Nurbs}`,
  klein wall 3 to `CurvedPairUnsupported{Cone, Plane}` at the operand
  gate, `spiric_rim` row 11 to `CurvedPierceUnsupported`;
- `EdgeCarrierSite` has to be curated in the façade prelude
  (`scripts/payload-rung-sweep.py --check`) and listed `INTERIOR` in
  the Python binding census.

That attempt was never reviewed; re-derive it, do not merge it.

## Follow-ons

`nurbs-edge-crossing-rung-is-the-ring-composite` (parked on frontier
(d)) and `spiric-operand-edges-reopen-with-their-first-producer`
(deferred).

## Note from emit (2026-10-07)

Retiring this gate, or the spline crossing-root arm it waits on, owes an
end-to-end row: a face crossing one NURBS edge twice with one sense,
named through the boolean. Today only unit rows reach
`emit_topo::chord_along`, which orders the two crossings
(`emit_topo`'s `nurbs_crossings_rank_by_parameter`,
`work/emit/a-crossing-of-a-nurbs-edge-ties-for-want-of-its-parameter.md`).

## The klein bottle is refused for edges nowhere near its contact (2026-10-25)

Measured on main at `20658b1d` (asked in chat whether the bottle's
join really needs a spline edge). `klein::bottle` at `Tol::witness()`:
the loop (`lofted_loop`, the union's operand B) has 12 edges: **8 are
`Curve3::Circle`**, the inner and outer rims of its two annular caps
(at z = 3.0 and z = 0.6), and **4 are `Curve3::Nurbs`**, the loft's
longitudinal seams, one per wall, running the length of the tube. The
bulb has 16 circles and 6 lines and no spline. `union(bulb, loop)`
refuses `CurvedEdgeUnsupported { operand: B, edge: EdgeKey(5v1) }`,
and `5v1` is one of the seams.

So the edges where the two bodies meet, the cap rims against the
bulb's rims, are plain circles and need nothing from a spline lane;
the gate refuses the whole operand because a seam elsewhere on it is
a spline. The only part of the bottle's join that genuinely asks for
NURBS work is the descent through the flare: the loop's NURBS walls
piercing the bulb's cone (wall 4's subject), along with whatever seam
crosses it. Deleting the gate should therefore move klein wall 3 to
that pierce, or to the contact itself, and the row that retires the
gate should say which. The unreviewed attempt above moved it to
`CurvedPairUnsupported{Cone, Plane}`, which names neither a NURBS face
nor a seam; nobody has said which faces that pair is.

One thing the circles do not settle: the loop's caps are tilted
3.99e-4 rad off the rims they meet (klein finding 11), so their circles
are not the bulb's. The rims can be up to about 1.1e-4 m apart, which a
declared `Rest` contact would have to accept. That tilt goes away when
the loop is swept along its exact spine
(`work/carvetail/a-half-turn-spine-sweeps-only-off-its-exact-tangents`,
P0).
