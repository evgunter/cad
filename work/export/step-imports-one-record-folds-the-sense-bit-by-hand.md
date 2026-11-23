---
id: step-imports-one-record-folds-the-sense-bit-by-hand
kind: issue
title: step-import's one_record.rs folds a face's sense bit by hand at from_n, a site face_normal's census does not sanction: no_source_file_folds_the_bit_by_hand is red on main
status: open
opened: 2026-11-23
priority: P2
---

Filed by GERM's `germ/co-surface-table` lane (PR 7431), from that PR's
review, which saw this red outside the PR's diff.

## Finding

`topo`'s `face_normal::tests::no_source_file_folds_the_bit_by_hand` is
red on `origin/main` (run on a main worktree, 2026-11-23). Its census of
hand folds under a face's sense bit holds one site more than the
sanctioned list: `crates/step-import/src/one_record.rs`, `from_n`. The
assertion at `crates/topo/src/face_normal.rs` (the row's `assert_eq!`)
names it as the left side's extra entry. The site came in with
export's import-records work (`1ef6fc8324`, "export: import-records fix
pass"; `6bb72c51fd`).

## To do

Read that site's fold through `OutwardNormal::from_chart` or a
`face_normal` door, or sanction it in the row's list with why it is a
genuine fold of the bit.
