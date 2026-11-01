---
id: pinch-carrying-machinery-valence-4
kind: issue
title: design - pinch-carrying machinery to support the intersecting equal-radius boolean family (valence-4 section vertices)
status: open
opened: 2026-08-31
refs: [1353, 1372]
priority: P1
cost: H
design: true
---

## From GitHub issue 1377

Opened 2026-08-31; 0 comments.

## Ruling (Ev, in-chat 2026-08-31)

"we definitely need to support that geometry at some point." The typed refusal shipped by VERBS-GERMARMS PR-2 (#1353, `GermFrameCylinderPinch`) is the honest **interim** state, not the permanent answer.

## What this is

The intersecting equal-radius cyl×cyl family's section is two bisector-plane ellipses that always cross at `p ± r·(â₁×â₂)` — self-intersections of the section curve, physically the two sharp points of a Steinmetz solid where the walls are mutually tangent. Producing the union/intersection body requires building **valence-4 vertices** (four arc branches meeting at a point). The at-rest vocabulary already admits them; what is missing is the Boolean's minting of the node (Ev's decision below).

## The measured inventory of what must widen (from PR-1/PR-2; the ordinal-109 dual is re-verifying — fold its findings in before speccing)

- **Vertex factory**: a split vertex currently joins exactly two fragments; the pinch joins four. PR-2's enabling correction already landed one adjacent premise fix (`vertex_on_curved_face` no longer assumes vertices are never interior to a face — falsified by PR-1's own pierce vertex).
- **Chord pairing / join**: runs are paired two at a time; at a pinch, four arcs must be distributed into loops with the correct crossing structure (branch selection is point-dependent — see the chord-lane widening below).
- **Loop walking / containment**: section traversal assumes simple loops; the pinch is a branch point.
- **Tier-3 validation / census**: valence expectations; the result body's census carries two valence-4 vertices.
- **Tangency honesty**: at the pinch the walls are mutually tangent — PR-1's sagitta charge (`bool_pierce_sector_side_curved`) is on the critical path; the second-order sector trilean is the open technical question for definite side-verdicts near the pinch.

## Prerequisites and sequencing

1. **#1372 first** — the family cannot even be *recognized* without the parameter-identity (declared radius equality) channel; that design conversation gates this one.
2. The **chord-lane widening** (PR-2's pre-registered STOP: `SectionCtx` generalized from one-plane-one-conic to a carrier set + point-resolved branch selection, under a bit-identity fence on the plane×wall path — Ev leans widen-in-place over a parallel variant, in-chat 2026-08-31) is naturally the first implementation unit; it is needed for ANY cyl×cyl join, pinch or not.
3. Pinch machinery proper (this issue) builds on both.

Fenced out deliberately (spec, ratified): NURBS-fitting/approximating around the pinch — a plausible body whose census and metering lie near the tangency.

Spec to be drafted as a design doc for ratification after the ordinal-109 dual and its union fix pass land; until then the typed refusal stands.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_016pYMaeU4woYZN8YGdTLfSK

## Home

VERBS: this is the cyl×cyl germ lane's geometry-side acceptance — Wave 2's curved boolean breadth over analytic pairs, VERBS' charter and its `crates/geom-brep/src/ssi*` / `intersect.rs` territory — and the issue names itself as VERBS-GERMARMS' successor.

**VERBS closed** (exit walk ratified, PR #1793); re-homed to
`work/issues/` awaiting an owner.

**Adopted by CURVED** at its opening for dispatch (2026-09-04, Ev's
in-chat direction): the plan's lane that carries this item is in
`work/curved/plan.md`.

## Trigger fired (SEAT, 2026-09-05)

`parameter-identity-channel-to-boolean` closed as delivered by SEAT-6
(PR 1593): the channel exists and `RadiusEvidence` has its production
caller at `germ_section_frame`. Re-opened here by the SEAT orchestrator
because lint refuses a row parked on a fired trigger; CURVED schedules
it. Note for the pinch design: the channel carries stored SCALAR
fields only — axis-flavoured declarations are
`work/issues/axis-flavoured-declarations-have-no-channel.md`.

## Re-pointed from the D10 hold (2026-10-08)

Waits on `intent-stage4-is-built`, not on the whole program: the equal-radius family is recognised by ParamSource tokens and RadiusEvidence::Declared; stage 4 replaces that with canonical-form identity at the door. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)

## Released by INTENT stage 4 E (`intent/s4-e-glue-on-zero`) (2026-10-09)

E retires the prerequisite's channel. Equal radii are decided by their margin, and `RadiusEvidence` and `ParamSource` are deleted, so the family is recognised without a declaration. The pinch machinery itself is unbuilt: the intersecting equal-radius pair still refuses `GermFrameCylinderPinch` (`crates/topo/src/boolean/join.rs:2088`). The design and its sequencing stand, less item 1.

## Ev's decision (PR 5020, 2026-10-14)

Approved by Ev. Weighed by a designer pair over two reconciliation rounds (row 137 of
`docs/DESIGN-FORK-LOG.md`); both hold one recommendation. The August
inventory is stale: nothing caps valence, and since PRs 4057 and 4728
the boolean splits a pinch into one vertex per cone and holds several
runs in one ring. What is missing is one event, a point where one face
pair's section crosses itself with no operand edge there (a *node*).

- Mint each node from the section table, a ring of four struts in each
  wall with the sectors' sides fixed by construction (a sibling of
  `vtxfac`'s `RingPlan`); every germ carries the ellipse it rides; each
  ellipse joins as a plane section against its bisector plane, as
  sphere pairs ride their radical plane; the zip's one-vertex-per-cone
  split finishes the body. One new margined read: where each node lies
  in each face.
- At rest: ∩ is V2 E4 F4 (node valence 4), ∪ has valence-4 nodes (6
  where a seam passes), ∖ is two solids on one point key at each node.
  No tier-3 change; ∖'s census touch is a witness for
  `census-cross-solid-curved-pairs-undecidable-on-shell-results`.
- Ratified text: D2's `Intersection` clause, the witness selecting the
  branch (Ev's words, 9f0bb4d915). `GermFrameCylinderPinch` retires; its
  unequal-radius arm is the skew pair's unbuilt quartic class.
- Units: U1 the node (bisector lane first), U2 a node on an operand
  edge, the rename alone.

## Built: U1 and the rename (2026-10-14)

- **The rename** (PR 5072): the intersecting unequal-radius pair refuses as the skew pair's class, `GermFrameUnsupported`.
- **U1, the node**:
  - **PR 5092, the bisector-plane lane.** A branch datum is decided on each germ at mint. Each ellipse joins against its own bisector plane.
  - **PR 5111, the node.** Each node inside a face of both walls is minted as a ring in each wall (`topo/src/boolean/node.rs`, `vtxfac::mint_node`), with the runs' sides fixed by the walls' senses.
  - **What builds:** ∪, ∩ and ∖ for two rods, two bores, and a rod against a bore, at the closed forms (`sweep/tests/bisector_nodes.rs`). A slanted bore's difference is one solid pinched to itself at both nodes, which D1 admits.
  - **The retired refusal:** `GermFrameCylinderPinch` is gone. A node not built refuses `NodeUnbuilt { why }`.
  - **∖'s census touch:** filed on `census-cross-solid-curved-pairs-undecidable-on-shell-results`.

## Ev's decision on U2 (PR 5178, 2026-10-15)

Approved by Ev after four reconciliation rounds (row 147 of
`docs/DESIGN-FORK-LOG.md`); both designers hold the design below.

## Built: U2a, U-lever and U2p (2026-10-16)

U2a (the factored crossing, PR 5192), U-lever
(`u-lever-the-wedge-is-levered-at-the-faces-extent`, PR 5204) and U2p
(the pair reading at the pierce, PR 5203) are on main. The near-node
seam and cap poses build at their closed forms in every op; the √band
ring's one open row is its pcert item. U2b and U2c below remain.

## Remaining

- **U2, a node on an operand edge.** A seam, rim or other edge passes through a node. The common case is a node at an operand *vertex*: an extruded wall's one seam ruling ends at the rims' only vertices, so the usual tee puts one node at the branch rim's vertex, and the usual pipe cross puts both seams through one node and none through the other. The edge is not avoidable: a rim is real geometry, and a seam's place is convention (D2), so whether an op builds must not depend on it.
  - **The refused ring on main.** The crossing sweep reads an edge lying on one wall against the partner wall through the cylinder equation, whose root lane decides "two roots or one" on a discriminant quadratic in the roots' separation (`solid_contain::line_wall_roots`, `WallRoots::Tangent`). Near a node such an edge is nearly tangent to the partner wall, so every node carries a refused region about √band wide: two extruded rods at 90°, at the witness band, refuse `CurvedPierceUnsupported` with a seam turned up to 3e-5 rad off a node or a cap up to 1e-4 m off the meeting station, and build from 3e-4 (`a-sqrt-band-refused-ring-around-every-pinch-node`).
  - **The crossing, read through the bisector planes.** For two equal cylinders whose axes meet at p, cylinder B's residual on wall A is exactly the product of the two bisector-plane distances, f_B|A = (w·(â₁−â₂))·(w·(â₁+â₂)), w = x − p. The sweep reads an edge on one wall against the partner wall as transverse line×plane and conic×plane roots on the existing planar lanes, each landing on the ellipse its plane carries, placed in the partner's face by the cylinder chart trim (`reduce::curved_face_arm`, gated on `join::crossing_cylinder_reading`). A seam δ off a node has two roots 2δ/sin θ apart, linear in the offset, so the refused region closes to one band, and the boolean reads the section one way, as the join already does.
  - **The sweep splits at a node; the node pass claims and mints.** In the factored arm the deciding quantity is the separation of an edge's two bisector-plane roots: Zero is one crossing at a node, and the sweep splits the edge once, snapped onto the section table's node point (`split_edge_onto`, as its vertex-landing sites snap onto a partner vertex); in band escalates `VertexOnVertex` with the move-or-turn recourse; definite splits twice. An end of a wall edge with both plane distances Zero is an operand vertex at a node (the tee's rim vertex, the pipe cross's seam ends), recorded by the existing endpoint posture with no separation read. Every contact the sweep records at a node carries the node as a tag: the wall pair and which of its two nodes, the sign of (v − p)·û, read from decisions the sweep already made. Every split of an operand edge thus stays in the sweep, and a second direction landing at a split vertex reads `OnVertex` (the containment door's pre-pass tests vertices before edges), deduplicated in `ContactAcc`. After the sweep, ahead of the vertex-on-face loop, the node pass claims every tagged vertex, so no pierce or pair reader reads one, and never compares positions (records are written by the op that decides them, D1's F1); it places an untagged node in or out of its faces by the containment door's interior walk, the boundary pre-pass bypassed for the family's wall edges; and it mints each node with a *site* per operand (the claimed vertex and its fan; U1's lone node is the empty fan), hanging the struts into the fan at the germ directions ±(â₁ ± â₂) and giving each edge through the node the side of the sector its direction lies in (two plane-side reads, levered at the radius). An edge lying along a branch has one factor identically zero and refuses typed. `NodeWhy::OnBoundary` and `GermOnNode` retire.
  - **At rest.** Seam through a node: ∪ valence 6 (8 where both seams pass), ∩ the bicylinder V2 E4 F4, the difference whose operand's seam passes valence 3 there. The tee: ∪ valence 2 (a right-angled corner of two arcs), ∩ V2 E4 F4 with valence 4, main ∖ branch valence 4, branch ∖ main valence 2. Both capped at the station: ∩ valence 4 with the transverse cap×cap line. Arcs end at nodes under D2's branch clause; seam and rim pieces keep their descriptions; nothing becomes joinable. No ratified text changes, no new vertex kind, no tier-3 change.
  - **An edge near a node, not through it.** An operand edge passing a node at a distance δ beyond the band lands its two crossings about 2δ/sin θ apart, and the pieces between each pierce and the node are short arcs about 2δ long whose dihedral is about δ. Two reads stand between U2a's crossing and those bodies.
    - *The pierce reads the partner wall through the branch the sweep decided.* The sweep records which bisector plane each crossing lies on (U1's `Branch` datum; a node is both). The pierce classification consumes it: the germ is n̂_wall × n̂_plane, the ellipse's own tangent, and each bound's side is a product of decided signs (the constant, the other plane's distance at the pierce, the bound's rate across the plane), with no sagitta charge, since a bound lying on one wall cannot re-enter the other within the exact identity. One owner per fact: the sweep decides and the pierce consumes.
    - *At rest, a wedge is levered at min(the faces' curvature radii, the farther face's extent), never the edge's own chord.* Check 4, `Intersection` certification and the boolean's must-carry read share one lever (`wedge_decided`). Folding in the edge's chord makes a 2δ arc at dihedral δ read δ², so certification refuses `NotTransverse` up to about 2e-5 at the witness band, escalates near 2e-4 and passes from 6e-4: that is the whole of the √band ring at rest. A dihedral is how far the faces separate as they leave the edge, bounded by how far the faces run and how they curve; levered so, the same arc reads Transverse from about one band. The change is general, not the family's: every short, shallow edge between large faces now reads Transverse, its locus determined transversely to about ε/α as a long edge's at the same angle already is, and a conventional description stored by the old read can now draw `TransverseNotIntrinsic`. The face extent is derived from the body, never stored.
    - *No node arm and no node certificate.* The node's 2-jet is the indefinite form whose null lines are the branches; restricted across a branch it is zero, so tier 3's cusp arm would read the saddle as a lamina, but with the faces' lever every station of a short arc reads Transverse and the cusp arm is never reached. The thin patch within √(εr) of a node is legal already (tier 3′'s rule on the neighbourhood of what two cells share).
    - *Containment needs nothing.* Loop roles are read through the witness ladder (`resolve_roles_geometric` → `shell_witness::complex_side` → `stands::ladder` → `point_in_operand`). Near a node its witnesses read On or in band, which the ladder takes as evidence only; the partner loop's far rim vertices decide. A loop flanked only by near-node regions would surface as `SectionLoopUndecided`, and is the day to give `point_in_operand` the pair reading.
    - *D4 across δ.* Within the zero band the edge passes through the node (U2b). In band, one decision on the crossings' separation escalates `VertexOnVertex`, with the move-or-meet or tighten-ε recourse. Beyond the band the poses build; up to a few K·ε the short arc's first station may escalate `SliverDihedral` with the tighten-ε recourse. Nothing refuses between the band and √band. No ratified text changes.
  - **Units.** U2a (M), the factored crossing (PR 5192): the sweep reads the family's walls through the planes; the near-node poses still refuse at the pierce until U2p. U-lever (M, general, its own lane): the wedge lever at check 4, certification and must-carry from one home; acceptance a digest fence over topo, sweep and editor-core at all three ε rows, plus one short, shallow, non-node edge at rest. U2p (M), the pair reading: the branch on the sweep's contact record, the pierce's sides and germ through it; acceptance seams and caps 1e-8, 1e-6 and 1e-4 off a node, and both operands' caps, building at the closed forms for ∪, ∩ and ∖, with one ∩ row at 1e-6 whose loop roles resolve; its acceptance waits on U-lever. U2b (M), the sweep's node split and tag, and the node pass's claim and mint with sites: the pipe cross as extruded and with one seam turned off, every op, both orders, re-posed. U2c (M, after U2b), rims (the one-sided fan; a germ whose gap holds no face gets no strut): the tee with its seam at a node and off it, both capped. Rows that go red knowingly: `bisector_nodes::a_rim_through_the_nodes_refuses_typed`, `verbs_germarms2::the_steinmetz_seams_are_tangent_at_the_sections_pinch_points`. The sphere pair's radical plane factors the same way; no symptom is known.
  - *The U2p acceptance at 1e-8, re-read by the orchestrator (PR 5203).* 1e-8 is the witness band's escalation edge (K·ε), so D4's ε < δ < K·ε row governs it there: a pose 1e-8 off a node builds or escalates in band. Measured, the node's placement stands 9.99e-9 off a seam, both caps' sectors 1e-9 to 4e-9 apart, and a single cap's rim 9.999999995e-9 off in one re-posed order, against an escalation of 1e-8. The line above is Ev's as approved; this one records how its 1e-8 case reads at the measured band.
