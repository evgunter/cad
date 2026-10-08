---
id: a-moving-door-carries-minted-rows-onto-an-unminted-face-and-half-mints-it
kind: issue
title: kef, kfmrh and ring_move carry a loop's or run's rows across one chart onto a face that stores none, leaving it half-minted
status: open
opened: 2026-09-30
priority: P3
cost: M
design: true
---

Found by `loop-reparenting-doors-drop-rows-they-could-now-re-mint-under-decide`
(branch `topo/reparent-remint`, PR 3531), whose measurement it is.

That unit made the doors that move a loop or run onto a face owe the
destination a re-mint when the moved rows do not stand there and the
destination was complete (`Body::plan_moved_rows`,
`crates/topo/src/euler_ring.rs`): the `_minting` twins run it through
the site mint, and the keys-only doors refuse `KeysOnly`. The site mint reads the destination
as found (`StoredRows::remints`), so a destination that stored NO row
is left as found: the minting pass owns it. But where the moved rows
DO stand — one chart, every moved half carrying a row — the door
carries them, and a destination that stored none of its own is then
half-minted: rows on the moved loop, none on its own. A rowless loop
carried onto a half-minted face does the same.

**Measured** (probes on every moving door, not committed): topo's
suite, four calls —
`euler_site_pcurve_rows::kef_merging_a_complete_face_into_an_unminted_one_leaves_it_half_minted`
(`kef`, which pins the state as documented), and three in
`loop_reparenting_pcurve_rows::ring_move_and_mfkrh_carry_every_row_across_one_payload`
(`kfmrh` twice and `ring_move`, on spline charts). Sweep's `ci` profile:
none at that PR's head.

`PcurveMintError::MissingCache`'s docs list the state among the doors
that can still produce it.

**The fork.** Either the door drops the carried rows when the
destination stored none (the face leaves unminted, as a chart change
leaves it, at the price of rows that were right), or it mints the
whole destination through the site mint, reading "minted" off the face
as the door leaves it rather than as found — a face storing a row after
the door is a minted face — which mints loops no mint ran over. The
site mint's own rule ("a face storing no row was never minted, is the
minting pass's, and stays rowless") reads the face as found.
