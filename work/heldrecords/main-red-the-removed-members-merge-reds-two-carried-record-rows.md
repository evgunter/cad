---
id: main-red-the-removed-members-merge-reds-two-carried-record-rows
kind: issue
title: main red since PR 7745: face_face_touch's divided-faces record refuses CarryRefused Unread, and the curved-face pierce row refuses JoinDesync on a minted carried meeting
status: open
opened: 2026-11-26
priority: P1
cost: E
---



## Finding

Filed by INSIDE's measurement lane `inside-attribute-reds` (2026-11-26).
Two slow-set rows are red on origin/main `649a6026b` (release, default
ε). Both were green at nightly 45553333820 (`4dc697d543`), and no row
named either test.

1. `sweep` `face_face_touch::a_record_whose_faces_the_op_divides_lands_on_the_pair_holding_its_line`,
   at `crates/sweep/tests/face_face_touch.rs:799:22`:

       in A∩B: Err(CarryRefused { row: Found, refusal: Unread { record: 18, cells: [Face(FaceKey(5v1)), Face(FaceKey(25v1))] } })

   The raising site is `crates/topo/src/boolean/records.rs:943`, where
   `RecordConfirm::meets(pair)` returned `Err` on the pair the record
   lands on.

2. `sweep` `a_pierce_held_in_a_curved_face::a_plane_face_holding_the_pierce_over_a_cylinder_builds_in_every_op_and_order`,
   at `crates/sweep/tests/a_pierce_held_in_a_curved_face.rs:456:26`:

       intersection of (sector ∪ triangle) and the lying cylinder over (1.47, 3.0), the held operand first: true: Err(JoinDesync { what: "two cells the boolean minted on a carried record meet at a point that is no vertex pair" })

   The raising site is `crates/topo/src/boolean/ops.rs:4633`. A landing
   marked `minted` has cells that are not both vertices.

## Bisect

Release-profile runs over origin/main's first-parent merges from
`4dc697d543`. Both tests pass at `747154c6a` (#7770, which touches
only work/) and fail at `0ac627ec9`, **PR 7745
(heldrecords/removed-members**, "the carry consumes a record only where
it lands on cells a division names removed"), the next merge. The
payloads at `0ac627ec9` match main's byte for byte. PR 7745 rewrites
`crates/topo/src/boolean/ops.rs` and `records.rs`, the two raising
sites.

Reproduce:

    cargo nextest run --release -p sweep --test all -E 'test(a_record_whose_faces_the_op_divides_lands_on_the_pair_holding_its_line) | test(a_plane_face_holding_the_pierce_over_a_cylinder_builds_in_every_op_and_order)'

## What a fix is

Read both raising sites against PR 7745's landing rule. Either the
carry lands these records where the rows expect, or the rows are re-read
and re-pinned with the reason. P1 because a red slow test is a red
nightly.
