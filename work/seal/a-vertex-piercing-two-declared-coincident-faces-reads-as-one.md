---
id: a-vertex-piercing-two-declared-coincident-faces-reads-as-one
kind: issue
title: A vertex piercing two faces a recorded face–face contact declares one interface refuses VertexReadTwice: a union bridging two pieces of a ⊔, and a ⊔ as a cut's tool, cannot build
status: open
opened: 2026-10-24
priority: P1
cost: H
---


Filed by INTENT's D′3 lane (PR 5636's fix pass), on the boolean's
vertex-on-face owner's slate.

## What

A bolt standing on a plate, `bolt ⊔ plate`, is one body of two solids
that meet over the bolt's foot. A boolean that reads that body whole
and puts a vertex of its other operand through the seat refuses
`BooleanError::VertexReadTwice` (`crates/topo/src/boolean/mod.rs:3093`),
raised by `vtxfac::pierced_and_paired`
(`crates/topo/src/boolean/vtxfac.rs:117`, the refusal at `:128`): the
vertex pierces the bolt's foot and the plate's top, two faces, and the
pass reads each.

`pierced_and_paired` reads only the reduction's own contacts
(`ReductionContacts`, `vtxfac.rs:97`). It does not read the operand's
contact records. Measured on PR 5636's branch: a part whose product is
`bolt ⊔ plate`, its census recording the seat (one face–face contact
record), placed as an instance and cut by a pin through the seat,
refuses `VertexReadTwice` at the same site with the record carried in
`records_a`. A bead bridging the bolt and the plate does the same in a
union.

The editor now distributes `−` and `∩` over a `⊔` operand (PR 5636,
`crates/editor-core/src/eval/wire/distribute.rs`), which is the
refusal's own stated recourse ("the operation applied to each piece,
with the results then combined"). Two cases are left that no
distribution reaches:

- **`∪` with a member bridging two pieces.** `(bolt ⊔ plate) ∪ bead`,
  the bead through the seat. The result is one solid touching itself
  over the seat's face, which spec §5′.3 says refuses, typed, naming
  the pair (`docs/INTENT-STAGE3-SPEC.md`, "A solid that touches itself
  over an area ... refuses, typed and naming the pair"). Today it
  never gets that far.
- **A `⊔` as a cut's tool.** `t − (bolt ⊔ plate)` through the seat.
  It is `(t − bolt) − plate`, a fold of cuts rather than a cut per
  piece, which the distribution does not build.

## Done when

A vertex that pierces two faces a recorded face–face contact of its
operand declares coincident reads them as one interface. The bridged
union then refuses as the typed self-touch naming the seat's pair, and
`t − (bolt ⊔ plate)` builds. The pinned rows flip:
`a_seat_unioned_with_a_bead_keeps_its_pieces_and_a_bridge_is_read_twice`
and `a_seat_as_a_cuts_tool_is_cut_by_whole`
(`crates/editor-core/tests/intent_dprime3_assemble_product_slot.rs`).

The seat's record reaches the boolean once the census runs at the
`⊔`'s door and its value carries the verdict
(`work/intent/assemble-is-an-operator-and-the-product-is-a-slot.md`,
the fix pass's first item). Until then a `⊔` read directly carries no
record of its seat, and only an instance of a part carries one.

## Slate triage

2026-11-01: moved from `work/cleave/` by CLEAVE's split on its semantic seams (Ev, in chat: CLEAVE measured 114 budget points against 30). SEAL holds the Indeterminate seal and the declared-contact rows in its way; id and body unchanged. (CLEAVE orchestrator)

## Design (fork `seat-vertex-one-interface`, 2026-11-09)

Three designers weighed this and converged after one round. It is not a question for Ev, because it builds ratified text: D10 `[operations.operators]`, `[coincidence.booleans]`, the `[topology.conventions]` slit sentence, and tier 3's "the material decides the edges". The "Done when" above named a mechanism, "read as one interface". It is replaced by this section.

**The rule.** Where a record declares two faces of one operand coincident, every reader of that operand's material at a point reads the pair as **one two-sided wall**: material on both sides, each face owning its side. The readers are the vertex cone, the line wedge, face containment and the coplanar sector, and each one consumes that gathered wall. A crossing point is still read once. It makes one result entity per material sector: two coincident vertices and edges along the seat line, one in each solid. This is the vertex-on-face instance of the line-dimension precedent already built in `boolean/recl.rs`, where a slit's or ring's line places its germs per material sector (`a_ring_crossed_along_its_line_places_its_germs_apart`). It also extends `vtxfac::cone_classes`' gathered cone from touching to crossing. Every operation runs on the whole body, and the seat's record rides the operation's lineage, never re-decided.

**Outcomes** (the bolt-on-plate seat; a post, a bead, a ledge):
- `post − seat` gives typed empty.
- `seat − post` gives 2 solids, 22 faces (11 + 11), volume 0.948, with the seat record carried onto the remaining faces.
- `post ∩ seat` and `seat ∩ post` give 2 solids, with the seat record carried.
- `t − (bolt ⊔ plate)` builds.
- A bead touching one piece builds.
- A bead or ledge bridging bolt and plate refuses typed, naming the record's two faces. This is the `[topology.conventions]` refusal. It is raised where the result is sorted into solids, when one solid holds both faces of a carried face–face record. It needs a new `BooleanError` variant.
- Two solids touching with no record keep refusing `VertexReadTwice`. The existing negative rows hold until HELDRECORDS makes them gate refusals.

**Decisive first step (prototype, before the build).** On the lane's seat fixture, build the gathered read and measure the outcomes above. The probe suite is staged at `/root/.local/share/cad-work/fork-seat/seal-vertex-probe-suite.rs`. Three assumptions are unchecked:
1. The face–face stage puts two coincident section curves on a crossing wall, one per sheet. It is unbuilt today.
2. The join carries two strut runs at one vertex, each paired with its own face's ring.
3. Containment drops both seat faces inside `X` in `X − S`, so no lamina is left.

If the prototype shows the gathered read cannot be made sound, stop and report. The fallback weighed was grouping the solids at the door, and it goes back to the designers.

**Rejected.**
- Editor-only distribution: the measured re-decision, see TALLY `a-cut-distributed-over-a-seat-re-decides-the-seat-and-reports-its-split-tool`. Also, the fold `(t − bolt) − plate` re-decides the seat at its second step, and pieceless finished bodies such as imports or severed parts are never reached.
- Door grouping: a second structure that cannot reach a slit inside one solid.
- A proposed README clause, "an operator never reads across a contact inside an operand": it is the opposite of this rule and is not added.

**Follow-up:** once this lands, the editor's `distribute.rs` retires, and result pieces are keyed from lineage. That is its own row.
