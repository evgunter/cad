---
id: face-normal-guard-gained-a-pcurves-hit-at-6900-behind-mains-red
kind: issue
title: The face_normal guard gained a hit in pcurves.rs at PR 6900, unseen because the guard was already red on main
status: closed
closed: 2026-11-13
opened: 2026-11-13
priority: P1
---


Found by S173's lane (2026-11-13). `topo` `face_normal::tests::no_source_file_folds_the_bit_by_hand` was already red on main: its residual hits are `mesh/src/walk.rs` ("TAU") and a second `topo/src/injective.rs` ("1.0"). Since PR 6900 merged (`22cce0e278`), it also reports a hit in `crates/topo/src/pcurves.rs`. PR 6900 classed this guard's red as main's by its residual at the frozen head, before the fix pass's `pcurves.rs` changes landed, so the new hit was masked by the existing red.

Owed:
- Read the `pcurves.rs` hit. It is either a hand fold of a face's sense bit, which must move onto the one door, or a false positive the guard should sanction with its reason.
- Make the guard green on main by resolving the two older residual hits (`walk.rs`, `injective.rs`) or pinning them with their filed rows, so a new hit can never hide behind them again.

Class: a red guard on main hides every new hit. Fixing the guard to green is what restores it.

## Withdrawn (ONEDOOR orchestrator, 2026-11-13)

The premise is false. PR 6911's review checked the guard by execution at the head and at main. The `pcurves.rs` "1" hit is in the guard's sanctioned list (`097288f34`, on main), and the guard's surplus is only the two older hits, `mesh/src/walk.rs` "TAU" and a second `topo/src/injective.rs` "1.0". PR 6900 added no hit. The orchestrator filed this from a lane's reading of the guard output without checking it. The standing concern, that a red guard on main hides new hits, still holds for those two older hits.
