---
id: a-cut-across-its-tools-seat-leaves-a-sliver-between-the-seats-two-sections
kind: issue
title: A cut across its tool's declared seat leaves a sliver of the cut part between the seat's two sections, which the finish keeps: X − (bolt ⊔ plate) refuses ToolSeatCrossed
status: closed
opened: 2026-11-09
priority: P1
cost: H
design: true
closed: 2026-11-12
pr: 6860
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

## Decided (Ev, PR 6765, 2026-11-12)

Ev approved the final state: a cut reads its tool as material, the tool fused on the `⊔`'s records, each record taken as the verdict. Ev ruled that it is semantically the same as `[operations.operators]`'s statement, so `docs/DESIGN.md` is unchanged. The rule and why it is sound go in an architecture note in `crates/topo/README.md`, under "The body, its operators and its tiers", with a clause id.

## Dispatch (the build, 2026-11-12)

Lane `seal-material`, branch `seal/cut-tool-material`. Review tier: full (a P1 on shared ground, `crates/topo/src/boolean/*`). The build first checks that an area record can be handed to the union as its given verdict; if it cannot, it stops and reports, without re-deciding the seat.

## Decided (orchestrator, 2026-11-12): the class is the record's meaning

The build found that a face–face record carries no class and no margin: at the cut's door it is `CarriedRecord<FfContact>`, two faces and an index, and the class and margin live only in the row it cites, which the door is never handed. A given one-carrier pair needs only its class; the margin and line are read only for a tangency (`given_verdicts`).

- **What the record already says.** A `⊔` records a touch and reports an overlap as an interference finding (`[operations.operators]`). So a face–face record is a touch: the two materials lie on opposite sides of where the faces meet.
- **Why the class follows.** For two planar faces the record certifies one plane, the reading `contact::area_record` documents and `vtxfac::Wall` and `Held::apart` already build on. One carrier with opposed senses is `GlueClass::Rest` by that class's definition. So the class is what the record means, not a new band decision, and nothing is decided twice.
- **Curved faces.** A face–face record of curved faces may be a line tangency, which the record does not tell from an area. The fuse refuses typed and names the pair. The census cannot record one today (`work/atrest/the-census-cannot-record-a-curved-seat-between-two-solids.md`).
- **What it does not move.** No ratified text: `[topology.records]` keeps "the pair and nothing else", and `[coincidence.booleans]` holds, because the record is taken, not re-decided. Carrying the class from the record's backing, or storing it on the record, stay where `a-cut-by-a-curved-seat-keeps-its-two-faces-held-apart-by-the-record` leaves them.

## Closed (2026-11-12, PR 6860)

A cut reads its tool as material (`crates/topo/README.md` `[topo.cut-material]`). The cut's door (`boolean::nary::cut_by_material`) makes every solid of a tool whose solids rest on each other a member, carved out with its keys (`carve::solid_alone`). It unites the members in `run_members`, every step given each planar face–face record between two solids as a `Rest` pair (`nary::tool_parts`), and deciding no carrier pair. Then the ordinary cut runs, and `boolean::material::Through` restates the result in the tool's cells. The crack is never minted.

- **Deleted.** `BooleanError::ToolSeatCrossed` with its kind, tag and editor row; `seat_crossed`; `vtxfac`'s cut-tool branch; the cut arm of `Held::apart`.
- **Given verdicts.** `GivenDecision` is now `Rest`, `Continuation`, or `Tangent { margin, line }`: a one-carrier given pair carries its class alone.
- **Refusals.** A curved or lower-cell record between two of the tool's solids refuses `ToolSeatUnread` naming it.

**Evidence** (`crates/topo/tests/a_vertex_through_a_declared_seat.rs`, at rest and turned, tier 3 and 3′):
- `a_cut_by_its_tools_seat_reads_the_tool_as_material`:
  - post: typed empty;
  - pin: 6 faces, 0.020;
  - bead: 6 faces, 0.006;
  - slab: 14 faces, 2.276;
  - cap: 10 faces, 0.168;
  - over: 8 faces, 0.030;
  - collar: 10 faces, 0.096;
  - corner: 6 faces, 0.018.
- `a_block_inside_the_seats_material_is_cut_away`: ridge, down, above, below and blade are typed empty.
- `a_cut_by_the_seat_is_the_cut_by_each_piece_in_turn`: the fold identity.
- `a_tool_of_three_stacked_solids_is_one_material`.
- The split post, both ways, in `a_seat_divided_along_a_ring_or_met_with_a_seat_refuses_at_the_carry`.

**Elsewhere:**
- `nary.rs`'s `a_tools_seat_is_taken_and_decided_nowhere`: no step decides a carrier pair.
- `crates/sweep/tests/a_cut_by_a_curved_seat_refuses_its_material.rs`: the coaxial and flush shafts refuse `ToolSeatUnread`.
- The editor's `a_seat_as_a_cuts_tool_is_read_as_its_material`: the post is empty, and a cap's floor is named as the plate's top.
- `S − X`, both `∩` and `∪` are unchanged; the bridged `∪` still refuses `TouchesItself`.

Five mutants were each killed by a named row (PR body).

Residues:
- `a-cut-by-a-tool-whose-pieces-also-meet-at-a-line-refuses`;
- `a-row-on-a-merged-face-of-a-cuts-material-names-one-tool-face`;
- the narrowed `a-cut-by-a-curved-seat-keeps-its-two-faces-held-apart-by-the-record`;
- the narrowed `a-seat-held-apart-across-two-shells-of-one-solid-passes-the-gate`.
