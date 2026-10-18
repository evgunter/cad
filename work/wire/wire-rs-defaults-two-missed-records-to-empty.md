---
id: wire-rs-defaults-two-missed-records-to-empty
kind: issue
title: eval/wire.rs: two record reads default a miss to empty, so an internal break reads as an empty table or an uncited row
status: closed
opened: 2026-10-17
priority: P4
cost: E
refs: [lane-profile-defaults-a-missing-replay-record-to-empty]
closed: 2026-10-18
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

## Resolution (2026-10-18)

Both defaults now refuse as `NamingError::Emission`, which ends with the
kernel-defect ending:

- `split_port` refuses `SPLIT_HALF_UNNAMED`.
- The fold's carried-record miss refuses `UNION_CARRY_LOST_RECORD`.

The sweep found two siblings in `wire_combine`, both a
`used.binary_search(&k)` miss that defaulted to no citation (the step
re-cite's `.into_iter().collect()` and the node re-cite's
`.unwrap_or_default()`). Those now refuse `UNION_STEP_UNDECIDED_CITE`.
Both fold re-cites now go through a new
`topo::ContactRecords::try_recited`, whose map can refuse, so a miss
refuses inside the map. Before this, a row that lost only one of
several citations was not caught at all.

No test drives these refusals: each is unreachable while the value and
the fold keep their shapes. The union and split suites stay green over
the new paths (`cargo nextest run -p editor-core -E 'test(union) |
test(split)'`, 345 tests). The rest of the sweep is in the PR body.
