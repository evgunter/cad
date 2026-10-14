---
id: deep-snapshot-cannot-see-a-consumed-key-slot
kind: issue
title: fixtures::deep_snapshot compares rows, so a refusal that minted and removed an arena entry, consuming a key slot against D1, passes every assert_err_deep_unchanged row
status: closed
opened: 2026-09-30
priority: P3
pr: 3566
branch: topo/snapshot-key-slots
closed: 2026-09-30
---

## What

Found closing `deep-snapshot-does-not-walk-the-body-side-tables`.
D1's per-op contract (`docs/DESIGN.md`, ~`:211`) says a refused op
"consumes no key slots". `fixtures::deep_snapshot`
(`crates/topo/src/fixtures.rs`) now walks one line per row of every
table on `Body`, but a `SlotMap`'s free slots and their versions are
not rows. An operator that inserts an arena entry and removes it
before refusing leaves every row as it was and bumps the vacated
slot's version, so the next mint returns a different key. Every
`assert_err_deep_unchanged` row passes it.

Two rows check the slot half of the contract, by lineage rather than
by snapshot: `euler.rs`, `failed_ops_leave_the_key_sequence_pure`, and
`review_m1_pr4.rs`, the "failing calls must consume zero key slots"
row. Each interleaves a handful of refusals (stale keys and a few
structural ones) into one construction and compares the result with
the clean run. The refusals the other atomicity rows pin (pcurve mint,
rebased carrier, tear gates, graft, movefac) are not among them.

## Shape to consider

Give the snapshot a per-arena "next key" line: the key an insert into
a clone of each arena would mint. That observes the free-list head, so
a consumed slot moves it. It needs a value to insert per arena; the
arena's first entry cloned, or a fixed placeholder, will do. The
snapshot's own row gains a mutant that inserts and removes one entry.
