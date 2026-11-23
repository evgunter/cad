---
id: a-record-the-split-parts-has-no-consumption-rule
kind: issue
title: A contact record whose cells the split's plane parts has no consumption rule in topo's carry; the document drops it after a check instead
status: closed
priority: P1
cost: M
opened: 2026-11-01
design: true
closed: 2026-11-23
---


Found by the FULL review of PR 6268 (2026-11-01). topo's carry
(`crates/topo/README.md:807-826`; `CarryRefusal` docs at
`crates/topo/src/boolean/records.rs:46-47`, "refused, never dropped")
consumes a record only when one of its cells is removed, divided with its
remainder removed, or moved, and calls any other unlanded record stale.
A record whose two cells the split's one plane puts on different sides
(`landings`, `records.rs:855-857`) is none of those, so topo refuses it
`Parted`. Such a touch lies in the cut plane: the two sides meet there as
they meet along the rest of the cut, which no single side's records can
state. A legal split refuses on it (`name_invariance`'s plate pinch at
y = 1).

PR 6268 handles it as an interim in editor-core: `wire_split`'s
`sides_carry` drops a parted record only after checking its cells meet the
cut, and refuses `ContactCarry { Parted }` otherwise. The policy's proper
home is a consumption case in topo's carry plus a sentence in that README
paragraph. The paragraph is a design page, so the change is an `[ev]` PR,
weighed by a designer pair first. heldrecords'
`a-finished-body-holds-its-contact-records-at-every-door` moves the carry
into the split's door, which must then adopt or replace the policy.

## After PR 6268 (2026-11-01)

The interim landed in topo itself as an opt-in:
`SplitResult::carry_contacts_across` (`crates/topo/src/splitting/finish.rs`)
retires a parted record only when each cell has an image on the cut (a
plane face its side holds, `SplitNaming::plane_pieces`, or a cell bounding
one), and refuses `Parted` otherwise; `carry_contacts` still refuses every
parted record, so the README's carry paragraph stays true of it. Only
`wire_split` calls the `_across` form. The `[ev]` question is unchanged:
whether the README's carry paragraph gains this as a consumption case.

## The split's door carries (2026-11-18, `embed/records`)

`topo::split` now carries its operand's records itself, with the
across-the-cut policy, and `SplitResult::carry_contacts[_across]` are
gone. A parted record with a cell that has no image on the cut refuses
at the kernel door (`SplitFinishError::Carry { Parted }`), where the
kernel API dropped it silently. Witness:
`crates/topo/tests/split_along_a_pinch_face.rs`: two bricks united
along the line `(1, 1, z)`, split by `x = 1`, which holds a face of
each and cuts neither. The pinch's vertex pair is parted, neither side
holds a face of the cut for it to lie on, and the split refuses; the
same bits finished bare split as before. The consumption rule this row
asks for would retire that record (the touch lies in the cut plane).

Moved from WIREHOLD to DOCTAIL on 2026-11-20, by `git mv`, when WIREHOLD merged into DOCTAIL: WIREHOLD held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one DOCTAIL carries.

## A recorded slit cut along its line (2026-11-22, HELDRECORDS split-on)

`heldrecords/split-on` records a slit's two coincident edges (the split's
own edge–edge record, citing the `SplitOn` row of the operand edge under
them), so a later cut along a slit now parts a record. Witness:
`crates/topo/tests/a_keel_wrapped_at_both_ends_splits_as_a_ring.rs`
`a_later_cut_across_the_ring_builds_and_so_does_one_through_its_tip`, the
ring's side cut by `y = 2`, upright along the ring: each of the ring's
two edges lies in the plane and goes to its own side, bounding no face
of the cut there, and the split refuses `Carry { Parted }`, where it
built `7.85 | 7.85` while the ring held no record. The consumption rule
this row asks for (a touch lying in the cut plane retires) would build
it again. Every recorded coincident pair a later cut runs along reaches
this, curved ones included, so the rule now gates the slit forms'
reuse as well as the union pinch.

## Claimed by HELDRECORDS and re-banded P1 (2026-11-23, HELDRECORDS orchestrator)

HELDRECORDS claimed this row from DOCTAIL by `git mv`; the id and body are unchanged. Plan step 3 (PR 7481, `heldrecords/split-on`) makes the split record its own coincident edges, per Ev's PR 5967 ruling. A later cut whose plane contains a recorded slit's or ring's two edges then refuses `Carry { Parted }` where it built before. The witness is `a_keel_wrapped_at_both_ends_splits_as_a_ring::a_later_cut_across_the_ring_builds_and_so_does_one_through_its_tip`, a cut at y = 2: 7.85 | 7.85 before. Every recorded slit or ring a later cut runs along reaches it, so a legal split refuses: P1. The row blocks step 3's landing, and a blocker inherits the band of what waits on it.

Two Opus designers are weighing the question (Fable is unavailable this sitting). Their statement is `origin/heldrecords/briefs:briefs/fork-parted-record.md`. The answer changes the README's carry paragraph, a design page, so it goes to Ev unless the designers agree and the text they would change turns out unratified.

## Ruled (2026-11-23, Ev on PR 7489)

The carry consumes a record whose two cells the op puts in different
result bodies where each cell reaches the cut by a decision the op made:
an image of each holds a vertex of the split's ON set (the operand
vertices it decided ON, the crossings and tangencies it minted there,
`SplitReduction::on_vertices`, and their copies). Otherwise the record
refuses `Parted`, a lineage defect. `crates/topo/README.md`'s carry
paragraph states it.

## Closed (2026-11-23, `heldrecords/split-on`, PR 7481)

Built in topo's one carry (`boolean::records::landings`, every caller's,
the Boolean's included): the ON set travels on the lineage
(`MintedBacking::OneCut { on }`, from `SplitNaming::on_the_cut`), and the
interim section-face test, the opt-in carry and its `Parting` list are
gone. `split_along_a_pinch_face` builds, each brick with no record; the
ring's `y = 2` cut builds `7.85 | 7.85` again, each side at 3′ against its
records. The precondition is pinned by
`splitting::finish::parted_on_the_cut::a_parted_record_is_consumed_only_on_the_cut`
(the same carry with the ON set emptied refuses `Parted`). The
postcondition the ruling leans on, that each side's cells lie in its
closed half-space, is not checked by the split: filed as
`work/heldrecords/the-split-checks-no-postcondition-that-each-sides-cells-lie-in-its-half-space.md`.
