---
id: load-door-appearance-key-id-check-is-pinned-by-no-row
kind: issue
title: The load door's appearance-key id check is pinned by no row
status: closed
opened: 2026-09-17
closed: 2026-09-17
refs: [document-stablename-carriers-have-no-enumeration, 2784]
branch: edit/stablename-carriers
pr: 2797
---

(Measured by the carrier-enumeration unit's mutant pass, 2026-09-17.)

## The finding

`persist/check.rs`'s `validate_snapshot` checks that every node id a
document's `StableName`s derive from is below the mint counter
(`check_id`, `SnapshotError::IdBeyondCounter`). That check covers both
carriers — the nodes' payload names and the appearance store's keys.
**Only the payload half is pinned.**

Measured on the enumeration branch, against the whole `editor-core`
`all` binary (1414 rows):

- drop the WHOLE name pass → **2 red**
  (`asm_r2a_mate_solve::row6i_the_load_check_refuses_a_mate_head_past_the_mint_counter`,
  `m10_2_r1_probes::r1_corrupt_v16_files_refuse_typed_at_the_load_door`);
- drop only the STORE half of the same pass → **0 red**.

The check is reachable, not dead: `SetAppearance` is the one
name-carrying edit the insert door deliberately does not check
(`resolve/mod.rs`'s `walk_names` match says so — an appearance name
resolves at evaluation, where a miss is a typed `AppearanceLoss`), so
a document CAN hold a store key whose minting node never existed, and
the load door is the only place that refuses it. A bug that dropped
the store from that walk would ship green.

## The shape

One row in the load door's refusal suite that corrupts an appearance
key's `name.node` past the mint counter and asserts
`SnapshotError::IdBeyondCounter` — the store's twin of
`row6i_the_load_check_refuses_a_mate_head_past_the_mint_counter`. The
corruption needs `pub(crate)` reach (no edit door mints a key past the
counter), so it belongs in `persist/check.rs`'s own `tests` module
beside `structurally_invalid_documents_refuse_at_save`, or in the
`m4_pr6_refusal` byte-level suite if a saved file can be doctored to
the same end.

## Why it was not built at the enumeration unit

That unit's spec bounds it to the enumeration and the four sites that
read it; its own mutant table is what disclosed this. Filed at the
moment it was disclosed. The unit's change makes the gap *smaller* —
the two halves are now one loop, so dropping the pass reds the two
rows above — but the store half specifically is still unheld.

## Closed (2026-09-17, PR #2797)

The row the shape asks for is
`persist::check::tests::rv_an_appearance_key_past_the_mint_counter_refuses_typed`
— a document whose only fault is an appearance key minted by node 7
over a mint counter of 0, asserting
`SnapshotError::IdBeyondCounter { id: 7, next_id: 0 }`. It sits in
`persist/check.rs`'s own `tests` module, beside
`structurally_invalid_documents_refuse_at_save`, which is where this
file said it belonged: the corruption needs `pub(crate)` reach because
no edit door mints a key past the counter.

It arrived as a review probe of the enumeration unit and was adopted
into it, the file being one that unit rewrites.

Re-measured on the enumeration branch at the fix pass, with the row in
place: the mutant that made this finding — the validator's name pass
skipping its `Store` arm — now reds, and reds this row alone (1 red in
`--lib`, `--test all` green at 1425). The payload half's twin,
`asm_r2a_mate_solve::row6i_the_load_check_refuses_a_mate_head_past_the_mint_counter`,
is unaffected by it, which is the asymmetry this row removes.
