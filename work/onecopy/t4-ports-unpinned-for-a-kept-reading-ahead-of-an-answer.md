---
id: t4-ports-unpinned-for-a-kept-reading-ahead-of-an-answer
kind: issue
title: Four of QML T4's Any ports have no pin for a kept reading ahead of an answer
status: open
opened: 2026-11-10
priority: P3
cost: M
---


Filed by QML T4 (`onecopy/refusal-standing`) on the orchestrator's ruling (c), 2026-11-10. Each of T4's ports onto `geom_core::ladder::climb(Any)` should have a pin: the production function, with a reading kept in band ahead of a later rung that answers, red under the old first-refusal loop. A mis-wired abort or `Blocked` arm then goes red.

T4 pins four:
- `ray_walk::on_boundary`;
- `boolean::region::Region::on_boundary`, by a stub region over the production method;
- `boolean::sectors::rays_within`;
- `chart_region::polygon_relation`.

These four are not pinned, because no cheap fixture reaches a kept rung ahead of an answer. The reach probe saw none in-suite either. Each needs the fixture named:

- **`boolean::carrier_touch::ball_off_face`** (`crates/topo/src/boolean/carrier_touch.rs`). A member's reading refuses only where `edge_clear_of_ball`'s `reduce::certified` refuses: an isolated member's `clear` and the box test never refuse. **Fixture:** a body whose face boundary holds an edge with an uncertifiable curve geometry (an in-band certificate), ahead of a member inside the ball.
- **`boolean::carrier_touch::off_face`** (same file). A cluster refuses only through `ball_off_face`, so it needs that fixture, plus a carrier whose clusters reach that face: a cluster whose ball meets the uncertifiable edge, ahead of a cluster whose ball holds a face point.
- **`shell::band_seam`** (`crates/topo/src/shell.rs`). **Fixture:** a band face on a periodic chart whose two seam sides are half-edge runs, one read in band by `winds_once`, the other decided not to wind once. `winds_once` refuses where its first point's radial lever is in band of the chart's axis (`shell_chart_lever`) or its winding is in band of half a turn (`shell_chart_winds`); the lever is the cheaper, a side starting within the band of the axis.
- **`shell::wraps_its_period`** (same file). **Fixture:** a face with two boundary cycles on a periodic chart, the first read in band by `winds_once` (as above, starting within the band of the axis) and the second winding once.

## Done when

Each of the four has a pin as described, red under the old first-refusal loop, or a stated reason it cannot be reached.
