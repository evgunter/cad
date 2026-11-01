---
id: an-instances-pattern-draws-nothing
kind: issue
title: a pattern with output instances commits a family no product reads, so nothing is drawn
status: open
opened: 2026-10-31
priority: P0
---

## Finding

Found driving the viewer (2026-10-31; Ev: P0). The pattern tool with output **instances** commits, and nothing new is drawn: the viewport still shows the one original plate (View pane: `faces: 8`). The checks badge says `check unused: Place "Place 2" (…): used by nothing: not the product, not an assertion, and nothing reads it`.

`DocSession::add_pattern` (`crates/viewer/src/session.rs`): both outputs insert the pattern's placement; `Fused` then inserts the Union over the family and repoints the product at it (`repoint`), while `Instances` inserts the placement alone. Nothing adds the family to the product, and the viewport draws the product. Selecting the pattern's node offers no **Add to the product** (`product_ui` reads `world::world_copy`, which answers nothing for it).

Selecting it also shows its first slide row as `= ?`: `slide value: the definition of variable #38:… refused: index #38:… has a value only in one member's evaluation, and it was read outside one. Recourse: read the index where its reader is evaluated once per value`. The Properties pane reads the indexed slot outside a member.

The **fused** output of the same pattern draws three plates.

Replay with `cargo run --release -p viewer --features app --example drive -- script.txt`:

```text
window 1280 1600
click "Combine bodies"
click "Pattern tool…"
vclick 0.3 0.6
settle
click "0.0200"
type "0.07"
key Enter
click "Commit pattern"
settle
click "Zoom to fit"
settle
picture after.png
click "Place 2 *"
settle
dump
```

## Partly by design (third walk, 2026-11-01)

The projection tool says "the pattern it is read out of is not drawn itself — to project another of its copies, pick it again in the feature tree", and projecting copy 1 of an instances pattern draws that copy. So leaving the family undrawn is the design. What remains of this row: the pattern tool says nothing of it, so an "instances" pattern reads as a commit that did nothing (the badge's `used by nothing` adds to that); and the pattern's Properties still show the `= ?` refusal above.
