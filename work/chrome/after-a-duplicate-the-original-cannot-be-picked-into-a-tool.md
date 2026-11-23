---
id: after-a-duplicate-the-original-cannot-be-picked-into-a-tool
kind: issue
title: a viewport click that meets nothing while a tool is open takes no pick, silently
status: closed
opened: 2026-10-31
priority: P1
cost: M
branch: chrome/duplicate-pick
closed: 2026-11-01
pr: 6239
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

## Premise corrected (2026-11-01, CHROME partial-picture lane)

The original body is pickable after a duplicate: a click on its material seats `Extrude …`, a click on the copy seats `Place "Part 1"` (`tests/drive.rs`, `after_a_duplicate_a_click_on_either_body_seats_it`). The replay's point falls in the plate's 12 mm hole once zoom-to-fit frames both bodies, so the index answers a miss (`Ok(None)`); without the duplicate the same fraction lands on material. What is real is that a miss with a tool open said nothing. Ruled B by the CHROME orchestrator: the viewport's miss path says it on the line, naming the tool; a tree deselect stays quiet.

## Closed (2026-11-01, PR 6239)

The premise was false: after a duplicate, both bodies seat. The walk's click landed in the plate's hole once zoom-to-fit framed two bodies. What was real is that a viewport click meeting nothing while a tool is open said nothing. It is now `ToolNotice::Missed`, worded from the tool's pick kind ("no edge under the cursor" for an edges-only tool), on the new `Subject::Tool`, which retires when the open tool changes, so a cancelled tool takes its miss with it. A tree deselect, a click with no tool open and a seating click stay quiet, each pinned by a row. The guard row seats both bodies after a duplicate. Filed: `a-seated-tool-previews-what-a-click-would-take` (P3).
