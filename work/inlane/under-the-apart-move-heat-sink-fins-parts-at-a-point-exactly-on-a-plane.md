---
id: under-the-apart-move-heat-sink-fins-parts-at-a-point-exactly-on-a-plane
kind: issue
title: Under the apart move, heat_sink_fins parts at a point-in-solid plane test whose margin is exactly zero at the nominal
status: open
opened: 2026-10-28
---

Found in INTENT stage 3 G (PR 5954) by `revalue_corpus::revalue_heat_sink_fins`.

**What happens.** `heat_sink_fins` is a `Union` of one family: five fins, one placement over `k = index(fins)`. Under `Move::Apart` (`crates/editor-core/tests/revalue_corpus.rs:134`), the `p0` and `p1` builds part at decision 6956 of 22250, after `point_in_loop_segment`. The decision there is a `bool_point_in_solid_plane` whose margin is exactly `0.0` at the nominal and `-1.08e-7` at `p1`. So the row compares in step (`InStep`, 22250 decisions, 10450 moved).

Before the parting, the apart move also flips four `segment_gap` decisions (32, 33, 58, 59). Those are main's, from #5542 (`work/green/main-reds-editor-core-slow-set-and-the-k-probe-sweep.md`), and they red an apart row until that item closes.

**What closing it takes.** Find the point and plane the zero-margin decision reads: a fin's vertex on a neighbour's coplanar cap is the likely pair, since every member is a copy of one fin. Decide whether a pair the separation certificate holds apart should reach a point-in-solid test at all. If it should, the decision must not branch on an exact tie at the nominal. Then move the row back to `Apart`.
