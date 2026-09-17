---
id: load-door-appearance-key-id-check-is-pinned-by-no-row
kind: issue
title: The load door's appearance-key id check is pinned by no row
status: open
opened: 2026-09-17
refs: [document-stablename-carriers-have-no-enumeration, 2784]
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
