---
id: after-a-duplicate-the-original-cannot-be-picked-into-a-tool
kind: issue
title: after a duplicate, a viewport pick on the original body reaches no tool seat, silently
status: dispatched
opened: 2026-10-31
priority: P1
cost: M
branch: chrome/duplicate-pick
---

## Finding

Found driving the viewer (2026-11-01; Ev: "does sound like an issue"; banded P1 by the walker, since the click is silent). On the startup plate, the duplicate tool's copy lands beside the original. Then, with the boolean tool open, a viewport click on the **original** adds no member and shows no notice (`no picks yet`); a click on the copy adds `member 1: Place "Part 1" (…)`. Without the duplicate, the same click on the plate adds `member 1: Extrude …`. The original's body is now read by two placements (its own and the duplicate's), and the pick reaches no seat; the routing is `Tools::feed` → `on_body_pick` → `Selection::seat_node` (`crates/viewer/src/tools.rs`), which is where to look first.

Replay with `cargo run --release -p viewer --features app --example drive -- script.txt`.

```text
window 1280 1600
click "Combine bodies"
click "Duplicate tool…"
vclick 0.3 0.4
settle
click "Commit duplicate"
settle
click "Zoom to fit"
settle
click "Boolean tool…"
vclick 0.3 0.47
settle
dump
```
