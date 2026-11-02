---
id: a-fresh-pick-off-the-older-picture-commits-a-stale-reference
kind: issue
title: A face clicked while an edit's run is owed is read off the older picture, and a gesture committed on it writes a stale reference
status: open
opened: 2026-11-02
priority: P2
cost: M
---


Found by the PR 6264 re-verify 2 (probe `rv2_a_fresh_pick_off_the_older_picture`, landed `#[ignore]`d on this row in `crates/viewer/tests/takeovers.rs`). It holds on main, before PR 6264.

The pick index answers the picture on screen (`PickCache`, "current or absent", `crates/viewer/src/pickcache.rs`; `drawn_index` in `crates/viewer/src/pane/viewport.rs`), which is the landed run's. Between an edit's commit and its landing the landed run answers the older document. A face clicked in that window is named as the older document names it, and a gesture committed on it writes that name into the newer document.

Probe: a fillet of the block committed; before it lands, the block's top picked afresh off the landed picture, and a boss sketched on it. The sketch is accepted. After the landing, the small block mated on the copy's top fails its placement ("the recorded reference disagrees … rebind it"). The failure is typed and loud, not silent.

PR 6264 closes the same window for a pick HELD across the commit (it follows the takeover at the commit, `DocSession::follow`; `a_gesture_on_a_held_pick_before_the_landing_reads_it_moved`), and judges held names only by a run that answers the document they live in (`DocSession::name_run`). A fresh click is a new name read off the drawn picture, which neither reaches.

The code it touches is picking, not the takeover door: the pick cache and the viewport's picking (`crates/viewer/src/pickcache.rs`, `drawn_index` and the pick loop in `crates/viewer/src/pane/viewport.rs`). It sits on the DOORS slate because a takeover is its sharpest trigger and it is the same window PR 6264 closed for held picks; the window opens on any edit's commit.

**Options when dispatched:** refuse a pick while the picture is older than the document (the click waits for the landing); or move a fresh pick through the takeovers committed since the picture's run, as a held pick is moved (it needs the `Moved` of each commit kept until its landing). A door that refuses a name by the mint log is no option: the stale name is spelled in minted ids (the block's `Cap(End)`), and it is stale because the copy now reads the fillet, which the mint log cannot see.
