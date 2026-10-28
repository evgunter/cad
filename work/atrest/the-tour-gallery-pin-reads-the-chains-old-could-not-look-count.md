---
id: the-tour-gallery-pin-reads-the-chains-old-could-not-look-count
kind: issue
title: demo-tour gallery pin expects 196 could-not-look on the chain scene; since #5853 it reads 10 at-rest overlap findings (red on any PR that pulls the demos job)
status: closed
opened: 2026-10-27
closed: 2026-10-28
priority: P1
cost: M
refs: [5853, 5869]
branch: atrest/the-tour-gallery-pin-reads-the-chains-old-could-not-look-count
---


Filed by SECT's orchestrator (2026-10-27), from SECT's lily lane on PR 5869.

## Measured

`demo-tour gallery::tests::each_gallery_document_denotes_its_scene_or_says_why_not`
fails on `origin/main` at 71d97e07 (after #5853, "atrest: curved copies
filling one volume interfere"): the tour's `chain` scene now yields 10
at-rest findings (overlap) where the pin expects 196 could-not-look.
Reproduced on a clean worktree of main by the lily lane. Main's own CI
skips the `demos (tour + wild)` job, so main reads green; any PR whose
diff touches `demos/tour` pulls the job in and goes red on it (PR 5869's
`gate ok`, run 40580799170).

## What the owner decides

Whether the 10 findings are true. The chain's links interlock without
sharing volume, so a finding that two links overlap would be a false
interference (a live wrong answer, P0), while true findings make this a
stale pin (re-pin, and say what moved). The band above is provisional
until that is read.

## Closed — the 10 findings are true; the pin is stale and re-pinned

Every one of the 10 is an `Interference` (none could-not-look), and
every one is a real shared volume. The pins are separate extrudes
rather than bores (`chain.rs` §"What the chain is made of"), so at
nominal each pin of radius 0.8 mm sits half inside the 3 mm-high bar
whose end it marks. These pairs were checked by the closed form of a
box against a cylinder on the built bodies, and confirmed `In`/`In` by
`topo::point_in_solid`:

- base pin × bar 1;
- joint pin k × bars k−1 and k, for k = 2..4 (6 pairs);
- tip pin × bar 4;
- target × bar 4;
- target × tip pin (coincident solids; `Overlap::Unlocalized`).

At a half-disc's incentre, `(x_pin ± 0.4 mm, 0, 1 mm)`, the depth is
0.4 mm in both solids. For target × tip pin it is 0.8 mm on the axis.
That is about 4·10⁵ ε at ε = 1e-9.

No other pair of the 10 solids shares volume. The bar × bar joints
only touch at their end caps, with a closed-form depth of ≤ 0. Those
pairs are not reported at rest; only `UnprovenCoincidence` rows name
them.

The issue's framing ("the chain's links interlock") does not hold for
this document: its links butt end to end, and the pins overlap them by
construction. `gallery.rs`'s chain row is re-pinned from 196 to 10,
and its `why` now names the pairs.
