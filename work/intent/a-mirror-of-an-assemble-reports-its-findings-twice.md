---
id: a-mirror-of-an-assemble-reports-its-findings-twice
kind: issue
title: A mirror of a ⊔ or of an instance reports the source's findings again, and a pick of a census-less piece drops its records
status: closed
pr: 5999
opened: 2026-10-28
priority: P2
cost: M
refs: [a-placement-carries-its-sources-at-rest-result, patterns-are-index-variables]
branch: intent/one-copy-door
closed: 2026-10-29
---

Found by PR 5903's review (R1, head `38e6cdd3`; probes on
`review/intent-carry-R1` @ `0fe59811`,
`crates/editor-core/tests/zz_review_probes_5903.rs`). Same on main.

**A mirror reports its source's findings again** (M2). `wire_mirror`
(`crates/editor-core/src/eval/wire.rs:2533`) hand-builds its `Carry`:
records and findings, no census, `censused: false`. So a census reading
the image as one arrival finds the meetings inside it again, and an
instance's findings ride the image beside the source's. Against
`[assertions]` ("reported once there, however many times its body is
placed or read"):

- `probe_f_mirror_of_instance`: a mirror of an instance whose part
  overlaps reports 9 findings for one overlap;
- `probe_g_mirror_of_assemble_beside_source`: a mirror of an
  overlapping `⊔` beside its source gives 8 in-arrival rows plus the
  source's own.

**A pick of a census-less piece drops its records** (M3). `pick_of`
(`eval/wire.rs:2747`) carries no contacts, since the piece's records
cite the member's inputs, which the copy does not name. A piece whose
door kept a census brings its records there; an instance's piece has
none, so `probe_h_pick_of_an_instance_piece_keeps_records` gathers a
product with 0 contact records where the part's seat is 1.

**The cure's shape** (M4, Q1):

- one copy door for every copy: `wire_place`'s family arm
  (`eval/wire.rs:3991`), `wire_pattern` (`:4254`), `wire_part`
  (`:2590`) and `pick_of` go through `copy_of` as a placement of one
  body and an instance do, and a mirror's image takes its source's
  carry where its keys allow;
- `cited_inputs` inside `Carry`, so records carried by hand never cite
  through a list they did not come with (what makes `pick_of` drop
  them);
- records in one home: today a boolean's ride
  `BooleanValue::Body::contacts` and every other value's
  `Carry::contacts`, reconciled at `records_of` and
  `product::sources_of`.


