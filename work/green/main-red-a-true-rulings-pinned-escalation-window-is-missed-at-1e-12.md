---
id: main-red-a-true-rulings-pinned-escalation-window-is-missed-at-1e-12
kind: issue
title: Main red at ε 1e-12: geom-brep's a_true_ruling_keeps_the_closed_form_on_both_nappes pins an escalation in (1e-12, 1.1e-12) and reads 1.112e-12
status: open
opened: 2026-11-23
priority: P1
cost: E
---


Reported by the implementer of ONEROUND's PR 7442 on 2026-11-23, from a run of the one test on a clean `origin/main` at `aa7754fa9b` (its own worktree and target directory, `CAD_TOLERANCE_EPS=1e-12`). Not run by me.

`geom-brep` `pcurve_cache::ruling_reading_rows::a_true_ruling_keeps_the_closed_form_on_both_nappes` fails at ε 1e-12: "σ 1, φ 1, over (0.5, 40.0): pinned to escalate pcurve_envelope in (1e-12, 1.1e-12) at Interval at ε 1e-12", with the enclosure reading `{ lo: 0.0, hi: 1.1122737889672985e-12 }` (`ruling_reading_rows.rs:170`). The row pins the window an `Interval` enclosure lands in, and the enclosure's upper end now stands just past it.

The file last changed on main in PR 7323 (`onehome/check1-frame-half`, commit `01ebb1c2ca`, "Frame rounding bounds the distance to the polar factor"), merged on 2026-11-23; that it is the cause is likely and not established (nobody ran the row on the commit before it). The 1e-12 row of geom-brep runs on a PR only where the diff touches the crate, so every such PR meets it.

Owed, by the lane that moved it: whether the wider enclosure is right (then the window is re-pinned with what moved and why) or a loosening to undo.
