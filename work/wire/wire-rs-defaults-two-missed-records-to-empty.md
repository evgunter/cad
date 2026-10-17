---
id: wire-rs-defaults-two-missed-records-to-empty
kind: issue
title: eval/wire.rs: two record reads default a miss to empty, so an internal break reads as an empty table or an uncited row
status: open
opened: 2026-10-17
priority: P4
cost: E
refs: [lane-profile-defaults-a-missing-replay-record-to-empty]
---

Found by the `wire-endings` lane's sweep for the class of
`lane-profile-defaults-a-missing-replay-record-to-empty` (an
`unwrap_or_default()` over a record whose miss is an internal break),
over `crates/editor-core/src/eval/`.

## Finding

1. `crates/editor-core/src/eval/wire.rs`, `split_port`:

   ```
   let names = value.output(port).cloned().unwrap_or_default();
   ```

   A split's value carries one output table per half; `port` has
   already been checked to name a half (`SplitHalf::of_output_body`).
   A miss is the value disagreeing with its own shape, and the default
   hands `projected` an EMPTY `OutputNames`, so the half projects as a
   body whose entities have no names. Whatever reads them next refuses
   (or does not) about names, not about the lost table.

2. `crates/editor-core/src/eval/wire.rs`, `wire_combine`'s union fold,
   the `contacts.recited` closure's `topo::Backing::Carried { input: 0,
   record }` arm:

   ```
   held.rows().get(record as usize).map(|c| c.cites.iter().collect()).unwrap_or_default()
   ```

   A carried backing names a record of the accumulation it came from;
   a miss is the fold losing its own record, and the default re-cites
   the contact as backed by NOTHING — a coincidence row with no
   citation rather than a refusal.

Neither was driven; both are `sure` as readings of the source and
`unlikely` to be reached while the value and the fold keep their
shapes.

## Direction

Refuse each miss as the internal break it is, with
`geom_core::KERNEL_DEFECT_ENDING` (the `ProfileRecordMissing` shape
`lane_profile` now takes), rather than defaulting.

## Not this row

- `eval/mod.rs`'s mate `face_parts` (`bundle_of(..).map(..).unwrap_or_default()`):
  `None` there means the node is not a mate, which is the documented
  meaning, not a miss.
- `eval/wire.rs`'s assertion arm (`node.payload_reads(doc).unwrap_or_default()`):
  the empty default falls into an `unreachable!`, so it panics loudly
  rather than lying.
