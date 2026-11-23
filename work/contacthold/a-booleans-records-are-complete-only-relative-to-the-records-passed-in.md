---
id: a-booleans-records-are-complete-only-relative-to-the-records-passed-in
kind: issue
title: A boolean's records are complete only relative to the records its caller passed in: a kernel op on a bare body whose own contacts were dropped ships a body that claims tier-3′ currency and fails 3′
status: open
opened: 2026-10-13
priority: P3
cost: M
design: true
---


Found by FUSE's carry-class re-measure after PR 4820 (2026-10-13, branch
`fuse/carry-rows-after-4820`). Measured on main `0d2117d0`.

## What

`BooleanBody`'s doc (`crates/topo/src/boolean/ops.rs:170`) says the
result's `contacts` are "the machine-checkable record of every
intentional touching the pipeline propagated", and that
`validate_pseudomanifold(&contacts)` is its tier-3′ pass. But the op can
only propagate records it was given. An operand's own contacts reach
the result only through `BooleanInputs::records_a` / `records_b`. The
façade doors (`union`, `subtract`, `intersect`) pass none, so a body
whose earlier contacts were dropped, such as a fold step handed the
previous step's bare `r.body`, comes back with records that miss those
contacts. When the result carries any record, the doc still makes it
tier-3′ currency, and it fails 3′.

## Evidence

- **A census at the door** (a temporary probe in `boolean_op_with`)
  over the topo/sweep/editor-core `ci` suites found 229 results that
  carry records and still ship an `UnrecordedContact`. They come from
  seven tests:
  - `a_vertex_read_by_two_sector_passes` (four tests);
  - `names::emit_topo::touch_reread_rows`;
  - `pierce_strut_at_a_pinch`;
  - `three_solids_on_one_line`;
  - `union_flush_onto_edge_contact::every_order_folds_a_flush_partner_onto_the_edge_contact`.
  
  A further 108 such findings carry no records, so they are tier-3
  currency and not this row.
- **The re-fold.** `standing_pyramids_folded_onto_the_plate_build_in_every_member_order`,
  folded through `boolean_op_with` with each step's records carried in
  (`records_a = prev.contacts.carried()`), passes strict tier 3′ on
  60/60 steps (5 poses × 6 orders × 2 steps). Folded bare, as the test
  does, its last steps fail at `MEET`.
- **The test files document this as an allowance:**
  - `a_vertex_read_by_two_sector_passes.rs`: `three_prime`'s doc ("an
    operand's own, which a kernel op called without `BooleanInputs`
    records does not carry");
  - `pierce_strut_at_a_pinch.rs`: the `Row` doc;
  - `three_solids_on_one_line.rs`: lines 230 and 292;
  - `union_flush_onto_edge_contact.rs`: `fold(…, carry: false)`.
- **The carried siblings pass:**
  `union_flush_onto_edge_contact::carried_records_certify_every_order_of_the_fold`
  (36/36), and `three_solids_on_one_line`'s carried variants.

So nothing is mis-minted or mis-carried. The op answers correctly for
the records it was given, and the currency claim is overstated for a
caller who gave it none.

## Options (design: the choice is open)

- **(a) State the dependence in `BooleanBody`'s doc.** A result's
  records are complete relative to the operands' records passed in, and
  a result built from an operand whose own contacts were not passed is
  tier-3′ currency only if the operand had none.
- **(b) Make the door refuse or flag an operand that carries contacts
  no record names.** That needs the census over the operand at the
  door, which REACHHOLD's
  `work/reachhold/boolean-door-runs-the-census-over-its-result.md`
  parks on the cross-solid curved lane.
- **(c) Make the type carry it.** A result built without its operands'
  records would carry "records relative to inputs", for example a
  marker on `BooleanBody` set when `BooleanInputs` carried none for an
  operand that is itself a boolean result, so the currency claim cannot
  be overstated.

## The other direction: a caller can pass records no op decided

Added by FLUSH's given-gate lane (`flush/given-gate`, 2026-10-13). The
same channel admits too much as well as too little.
`OperandRecords`' fields are public (`crates/topo/src/boolean/mod.rs`),
and so is each `CarriedRecord`'s, so a Rust caller can hand
`boolean_op_with` an operand record that no operation ever decided.
The door checks only that its keys are live (`validate_declarations`'s
`carried`), and the result carries it as a `Backing::Carried` contact.
`BooleanInputs::given` had the same shape and is now minted only from a
kernel verdict (`GlueVerdict::given_at`). A fix that has the kernel
carry an operand's records itself (option (c), or records that ride on
the body) would close both directions; minting `OperandRecords` only
from `ContactRecords::carried` would close this one.

## Measured (FLUSH, 2026-10-26): the fold's half closed

The n-ary union's and intersect's fold steps no longer pass bare bodies
from one crate to another. `topo::nary_boolean`
(`crates/topo/src/boolean/nary.rs`, `flush/nary-union-door`) carries
each step's records into the next itself (`records_a` is the previous
step's `contacts.carried()`, the first member's records at the first
step), and each member's own records into the step that joins it; the
caller hands records in only once, per member (`MemberInputs`). That is
what `editor-core`'s fold already did by hand, so no result moved
(every union and intersect evaluated in `editor-core`'s suite gave the
same body, names, rows and records). What is left of this row is the
pair doors (`union`, `subtract`, `intersect`, `boolean_op_with`), whose
results are complete only relative to the `BooleanInputs` records a
caller passes, and the other direction below.

