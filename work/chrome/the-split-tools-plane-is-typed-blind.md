---
id: the-split-tools-plane-is-typed-blind
kind: issue
title: the split tool's plane is typed blind, its default lies on the body, and a split leaves the picture unchanged
status: closed
opened: 2026-10-31
priority: P1
cost: H
branch: chrome/split-plane
closed: 2026-11-01
pr: 6240
---

## Finding

Found driving the viewer (2026-11-01; Ev: P1). Three things together make the split tool hard to use:

- The plane is typed as a point and a normal **in the body's own coordinates**, and the viewport draws no preview of it while the tool is open.
- The default (through the origin, normal z) lies on the startup plate's bottom face, and a plane through the origin with normal x lies on its edge; both commit, and the tree row says `below half empty`.
- A real split (through x = 0.03, normal x) commits as `Split "Split 1" … Ok`, and the picture does not change: the Split is not placed, and the plate still shows whole. Its halves are read by later tools (the seats' half pickers), but nothing says so.

Replay with `cargo run --release -p viewer --features app --example drive -- script.txt`.

```text
window 1280 1600
click "Zoom to fit"
settle
click "Combine bodies"
click "Split tool…"
vclick 0.3 0.45
settle
click "1.00"
type "0"
key Enter
click "0.00" 0
type "1"
key Enter
click "0.0000" 0
type "0.03"
key Enter
click "Commit split"
settle
tree
picture after.png
```

## Closed (2026-11-01, PR 6240)

While the split tool is open, its plane is drawn in the frame of the copy the person picked. A tree pick, or a copy not in the picture, draws nothing, and the panel says why. The panel names the coordinates its fields are in. The default plane runs through the target's centre, so it cuts the body. Before commit, the panel reads the split door's own verdict from a staged evaluation run off the UI thread: "checking…", "cuts the body in two", or "misses the body or lies on a face". The stage lives in the session's `Derived`, so every document change drops it; that machinery is general (`SessionOp::Stage`, `StagedService`, one `Cancelling` handle shared with the prover). The half where the split result is drawn nowhere is SPACES' drawn-bodies rule, `[ev]` #6209, with evidence on `work/spaces/a-new-body-lands-placed-in-the-world.md`. Filed: `numbered-placement-forms-draw-no-preview`.
