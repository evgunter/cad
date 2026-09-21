---
id: flatten-emits-every-vertex-before-it-judges-any-of-them
kind: issue
title: A non-finite vertex position is emitted unconditionally, above the arc guards that would refuse it
status: open
opened: 2026-09-17
priority: P1
cost: E
---

Found by the review of #2798, against the guard that PR added.

## Finding

`crates/viewer/src/sketch.rs`, `flatten`:

```
    for (index, vertex) in vertices.iter().enumerate() {
        let from = vertex.pos();
        let to = vertices[(index + 1) % vertices.len()].pos();
        at.push(out.len());
        out.push([from.x, from.y]);
        let bulge = vertex.bulge();
        if bulge == 0.0 {
            continue;
        }
```

`out.push([from.x, from.y])` is **unconditional and above every
guard**. #2798 made the flattener refuse an arc whose frame is not
numbers, and the refusal covers the arc's INTERIOR points only: the
loop's own vertices are emitted before the question is asked, and a
loop with no bulges at all — a polygon, which is what the rectangle
template lowers to — passes the arc guards without ever reaching them.

So a vertex position that is not a finite number is drawn, with no
refusal, five lines above a refusal written for exactly that.

**This is the class-not-instance half of #2798's own repair.** That
PR's module header now says the fourth question this module judges is
*whether a replayed loop can be DRAWN* — and the check it added
answers that for arcs and not for points. The population is every
coordinate `flatten` emits.

## Reachability: unsure, and the same answer as its parent

#2798 found no `NaN` producer for the flattener's inputs and a live
non-finite one for the arc frame. A non-finite VERTEX is a different
input: it would have to come out of `replay` for a loop whose literals
are all finite, which is the same question that row left open.

## Fence

`crates/viewer/src/sketch.rs` — VIEW's, the standing double claim with
CHROME.
