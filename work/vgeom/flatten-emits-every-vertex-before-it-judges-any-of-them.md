---
id: flatten-emits-every-vertex-before-it-judges-any-of-them
kind: issue
title: A non-finite vertex position is emitted unconditionally, above the arc guards that would refuse it
status: closed
opened: 2026-09-17
priority: P1
cost: E
closed: 2026-09-21
branch: vgeom/sketch-infinity
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

## Closed — the population, not the vertex

`flatten` now asks one question, `drawable`, of **every coordinate it
emits**: the loop's own vertices before they are pushed, and each arc
point as it is minted. A point that is not a pair of finite numbers
returns `Err(index)` — the refusal the signature already had, the one
`preview` renders as `PreviewError::Unflattenable`. The loop is
refused rather than the segment skipped, for the reason the function's
own `# Errors` already gave about arcs: a dropped vertex would put
the legs either side of it through a corner nobody authored.

**Two arms, both with live producers, both executed.**

- **The vertex.** `At(1e308, 0)`, `Toward(1, 0)`, `Line(1e308)` —
  three finite literals — replays to a loop whose second vertex is at
  `inf`, with a bulge of zero on every segment. A polygon reaches
  none of the arc guards, so the point was emitted and `flatten`
  reported success. Refuses at vertex **1**, which is neither arc
  row's vertex 0.
- **The arc's far side**, which this row's *"population is every
  coordinate `flatten` emits"* sentence covers and its title does
  not. `At(8e307, -1e307)` then a bulge of `10.0` to `(8e307, 1e307)`
  gives radius `5.05e307`, centre `(1.29e308, 0)`, and a finite sweep
  and start — every value #2798's guard asks about is a number — and
  nine of the 256 points it draws are at `inf`. A guard on the frame
  is not a guard on the points.

**Reachability of the drawn case: LIVE in `preview`, NOT in
`committed`.** Both producers fail validation
(`segment_straightness`, `arc_diameter_clearance`), and `committed`
draws only validated values, so neither reaches the committed lane or
its badge. `preview` draws a loop that replays and does not validate
— that is the case the form exists for — so both were drawn.
`work/vnews/the-profiles-badge-names-the-arc-case-only.md` carries
what that leaves narrower than its mechanism.

**Two arms the review added, both inside this row's population.**
`drawable`'s second conjunct was asserted nowhere — every fixture
carried its non-finite coordinate in `x`, and deleting
`point[1].is_finite()` left 637 viewer rows green — so a vertex whose
ORDINATE is the bad one now carries it. And the arc FRAME's own
`drawable(centre)` was subsumed by the point guard in every fixture:
the arm that separates them is `arc_points` answering **one**, where
the interior loop never runs, `start` is `atan2` of a finite ordinate
over `-inf` and so finite, and only the frame asks about a centre of
`[inf, 5e-7]`. Without it that arc is drawn as a straight chord.

PR: `vgeom/sketch-infinity`.
