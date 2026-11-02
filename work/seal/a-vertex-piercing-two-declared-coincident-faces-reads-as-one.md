---
id: a-vertex-piercing-two-declared-coincident-faces-reads-as-one
kind: issue
title: A vertex piercing two faces a recorded face–face contact declares one interface refuses VertexReadTwice: a union bridging two pieces of a ⊔, and a ⊔ as a cut's tool, cannot build
status: open
opened: 2026-10-24
priority: P1
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
