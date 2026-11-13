---
id: face-normal-guard-gained-a-pcurves-hit-at-6900-behind-mains-red
kind: issue
title: The face_normal guard gained a hit in pcurves.rs at PR 6900, unseen because the guard was already red on main
status: open
opened: 2026-11-13
priority: P1
---


Found by S173's lane (2026-11-13). `topo` `face_normal::tests::no_source_file_folds_the_bit_by_hand` was already red on main: its residual hits are `mesh/src/walk.rs` ("TAU") and a second `topo/src/injective.rs` ("1.0"). Since PR 6900 merged (`22cce0e278`), it also reports a hit in `crates/topo/src/pcurves.rs`. PR 6900 classed this guard's red as main's by its residual at the frozen head, before the fix pass's `pcurves.rs` changes landed, so the new hit was masked by the existing red.

Owed:
- Read the `pcurves.rs` hit. It is either a hand fold of a face's sense bit, which must move onto the one door, or a false positive the guard should sanction with its reason.
- Make the guard green on main by resolving the two older residual hits (`walk.rs`, `injective.rs`) or pinning them with their filed rows, so a new hit can never hide behind them again.

Class: a red guard on main hides every new hit. Fixing the guard to green is what restores it.
