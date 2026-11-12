---
id: a-cut-across-its-tools-seat-leaves-a-sliver-between-the-seats-two-sections
kind: issue
title: A cut across its tool's declared seat leaves a sliver of the cut part between the seat's two sections, which the finish keeps: X − (bolt ⊔ plate) refuses ToolSeatCrossed
status: parked
blocked_on: [6765]
opened: 2026-11-09
priority: P1
cost: H
design: true
---


Filed by SEAL's lane on `a-vertex-piercing-two-declared-coincident-faces-reads-as-one`,
whose prototype measured it (the fork `seat-vertex-one-interface`'s
assumption 3).

## What

A ⊔ whose two pieces rest on each other, the seat a face–face record
(the bolt's foot and the plate's top), is read as one two-sided wall:
a vertex of the other operand piercing both faces mints each face's
crossing at one vertex (`vtxfac::Wall`, `vtxfac::WallRead`), the first
face's Out run and the second face's In run, so the vertex's up side
and down side each take one face's crossing and the copy between them
holds neither. That builds `S − X`, both `∩`, a bead on one piece, and
the bridged unions' self-touch (`a_vertex_through_a_declared_seat`).

It does not build `X − S`, the seat as the cut's tool
(`t − (bolt ⊔ plate)`), because the two sections cut `X` along each
face apart, and the zero-thickness crack of `X` between them is a
region of its own:

- **Its class is order-dependent.** Read against each face alone, the
  crack is below the foot and above the top, Out of both; with the
  copies minted the other way round it is In both. Shells are classed
  by their section faces (`finish::classify_shell`), so no single class
  drops it in every op: Out, `X − S` and `∪` keep it, zipped with the
  reverted seat patches into a lamina (the prototype measured
  `ResultInvalid` `LaminaWedge` for `post − seat` and `pin − seat`); In,
  `∩` and `S − X` would keep it instead.
- **The finish has no seat for dropping it.** Dropping the crack and
  the two seat patches it zips with in `X − S` breaks
  `setopfinish`'s seam bookkeeping: every completed polygon pair
  contributes a seam whose kept A and B faces must both survive the
  carve ("kept A section face missing from the carve"), and a seamed
  operand that loses every kept component is a `JoinDesync`.
- **The crack is not always its own shell.** In `bead − seat`, the bead
  across the bolt's side, the top's section runs past the bolt's foot
  edge (an edge of the bolt lying in the plate's top), so the crack
  over the foot opens into the bead's piece outside the bolt through
  one section face. Dropping it needs the top's patch divided along
  the bolt's foot edge, which no reader does: the prototype measured
  `EdgeInFace { gap: Crossed }`.

Today every such cut refuses `BooleanError::ToolSeatCrossed`, naming
the record. It is raised at the vertex pass where a vertex of `A` crosses
both faces of a wall of `B`, after the mirror check and before the
second face writes (`vtxfac::classify_vertex_on_face`). It is also raised
at the result gate where a shell holds both faces of a record `B`
carried in (`ops::Held::apart`): the slab and the cap, which cross the
foot by a face alone. A tool that only touches the seat from one piece
(a ridge in the bolt or in the plate) is cut away whole, the typed empty
result. Pinned by `a_cut_across_its_tools_seat_refuses_typed`
(`crates/topo/tests/a_vertex_through_a_declared_seat.rs`) and the
editor's `a_seat_as_a_cuts_tool_is_read_whole_and_refuses_where_the_cut_crosses_it`
(`crates/editor-core/tests/intent_dprime3_assemble_product_slot.rs`).

## Done when

`post − seat` is the typed empty result, `pin − seat` the pin's stub
below the plate, `bead − seat` the bead's piece outside both, each
at tier 3 and 3′ with its volume, and the two rows above flip. The
design question first: whether the crack is dropped by a rule in the
finish (its shell and the seat patches it zips with, the seam
bookkeeping taught to lose a completed pair), or the X-side
construction mints no crack at all; and how a wall's patch is divided
where the wall turns one-sided (`bead − seat`).

## Fork `seat-crack` (2026-11-10)

Three designers, drawn by the protocol, independently recommended one final state: **a cut reads its tool as material**. `t − (p ⊔ q)` is `t − (p ∪ q)`: at the cut's door the tool's pieces are fused where the `⊔` recorded their meetings, on the records' own decisions, and then an ordinary cut runs.
- The crack is never minted.
- `ToolSeatCrossed`, `seat_crossed`, `vtxfac`'s cut-tool branch and `Held::apart`'s tool arm are deleted.
- The union's coplanar imprint divides the plate's top for `bead − seat`.

Measured with the tool fused, each union deciding the seat itself: every refused cut builds or is typed empty, at tiers 3 and 3′. That covers the faces-only seats (slab, cap) and the `VertexReadTwice` rows (above, below, blade, corner); the volumes are in PR 6765. One designer measured the coaxial and flush curved cuts: two solids each.

The build checks first that a union can be handed the seat's record as its verdict; no designer exercised that path. The new `[operations.operators]` sentence is put to Ev on `[ev]` PR 6765, fork row 260.
