---
id: delete-the-boolean-operand-edge-gate
kind: unit
title: delete gate_operand_edges once every site behind the sweep refuses a spline or spiric edge typed
status: closed
opened: 2026-10-03
priority: P0
cost: M
refs: [3984]
closed: 2026-10-25
branch: splice/gate
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

## Re-banded to P0 (on request in chat, 2026-10-25)

This gate is what stops three demo joins: klein wall 3, lily wall 8 and teapot wall 3. Each one refuses a lofted or swept operand because its seams are spline edges, even where the contact itself is plain circles (the klein measurement above). Any body built by loft or sweep is refused by every boolean.

## Moved to SPLICE (2026-10-25)

From ORBIT. Moved by `git mv` in the chat-approved refold that opened SPLICE (`work/splice/program.md`), the track for a body built by loft or sweep entering every boolean. This row stays P0; the rows it waits on moved with it.

## Closed

Landed on `splice/gate`. `reduce::gate_operand_edges` and
`BooleanError::CurvedEdgeUnsupported` are deleted. The sites are typed
first (`join-and-continuation-sites-…`,
`a-nurbs-edges-sector-departure-…`).

**A drive-by on the sweep's curved arm.** `curved_face_arm` refused a
NURBS or `Approx` face of `y` naming `x_is`, the edge's operand, so
every wall below read operand A. It now names `x_is.other()`, witnessed
by the census's face-ownership check.

**The emit note's end-to-end row is the crossing rung's.** A spline
edge claims no box, so the boolean refuses the first spline edge it
reads against the first face it reads, wherever the other operand
stands: no row here can witness a face crossing one NURBS edge twice.
`editor-core`
`names::emit_topo::wavy_loft::a_union_across_a_nurbs_edge_refuses_until_a_crossing_rung_builds_it`
is the tripwire that reds the day a rung builds that union. The row
itself moved, dated, to `nurbs-edge-crossing-rung-is-the-ring-composite`.

**The census.** `sweep` `spline_spiric_operands`. It runs the loft
prism, the lofted cylinder and the spiric-rimmed vessel cavity through
all four ops, in both orders, against bricks. Each `Ok` is held to its
closed form, or for a cut to inclusion–exclusion (every op that builds
implies one shared `V(O ∩ B)`). Measured at ε 1e-9, 1e-6 and 1e-12:

- every spline outcome refuses typed and names a face of the operand
  its site says owns it; the bricks are fresh-keyed
  (`topo::test_support::brick_with_fresh_keys`), so a face key read in
  the wrong body resolves nowhere;
- the cavity builds **30 ops, pinned**: far, enclosing and inside
  bricks, a poke through its neck wall and a peg through its mouth, all
  clear of the spiric rims. Re-adding a spiric refusal at the operand
  gate reds the row;
- a poke into the torus band between the two spiric rims refuses
  `GermSectionSpiric { section: RoutesToGeneralRung }`, pinned by
  `a_poke_into_the_cavitys_torus_band_refuses_typed`.

**The demo walls**, measured with the gate off on main and re-pinned:

- **klein wall 3** refuses `CurvedBooleanUnsupported { operand: B, kind: Nurbs }`
  at the contact. The sweep reads the seam line of the bulb's outer
  neck cylinder (r = 0.275, z ∈ [2.58, 3.0]; it ends on the z = 3 rim
  the loop's top cap meets) against one of the loop's four NURBS walls,
  which rises from that rim. That is a line against a spline face, and
  no crossing layer reads one. Neither a far seam nor the cone pierce
  is read first.
- **lily wall 8** refuses the same variant, on the sheath, operand B.
  The blade's base-section rim circle, between a NURBS wall and the
  planar cap, is read against a NURBS wall of the sheath rising from
  that same rim.
- **teapot wall 3** refuses the same variant, on the spout, operand B.
  The belly's revolve seam — edge `9v1`, a quarter meridian in z = 0
  on the circle of centre (0, 4/64) and radius 5/64, between two of the
  belly's sphere faces, on which the spout's axis leaves the belly at
  about (−0.076, 0.080, 0) — is read against a NURBS wall of the spout
  rising there.

The prose in each scene, `demos/README.md` and the Python north-star
row says so.

**Split side.** No boolean path reaches
`splitting::join::certify_section_area` or
`splitting::finish::describe_section_boundary`: both are reached only
from `split_scratch` and `split_direct`, and the boolean imports only
`finish::{carve, single_solid}`. Both rows stay as they are.

**Filed:** `a-spline-rider-reads-as-lying-in-no-plane-face`. It is the
slit pass's `lies_in`, latent behind the planar arm.
