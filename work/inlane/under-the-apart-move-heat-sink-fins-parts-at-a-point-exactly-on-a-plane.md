---
id: under-the-apart-move-heat-sink-fins-parts-at-a-point-exactly-on-a-plane
kind: issue
title: Under the apart move, heat_sink_fins parts at a point-in-solid plane test whose margin is exactly zero at the nominal
status: open
opened: 2026-10-28
priority: P1
---

Found in INTENT stage 3 G (PR 5954) by `revalue_corpus::revalue_heat_sink_fins`.

**What happens.** `heat_sink_fins` is a `Union` of one family: five fins, one placement over `k = index(fins)`. On main the document was a `PlacedUnion`, which grafted the fins without judging them: 3170 decisions, compared whole under `Move::Apart` (`crates/editor-core/tests/revalue_corpus.rs:134`), red only on four `segment_gap` flips at decisions 32, 33, 58 and 59. Those flips are main's, from #5542 (`work/green/main-reds-editor-core-slow-set-and-the-k-probe-sweep.md`).

Under G the union decides 22250, among them the n-ary pass's reads of each pair of fins. The `p0` and `p1` builds part after decision 6956, at `point_in_loop_segment`. The decision there is a `bool_point_in_solid_plane` whose margin is exactly `0.0` at the nominal and `-1.08e-7` at `p1`. Before it the same four `segment_gap` flips stand. In step the builds part nowhere: 22250 compared, 10450 moved, no flip.

So the parting is new with G. The row stays pinned at main's move (`Apart`), and reds with "compared under another move" until this closes.

**What closing it takes.** Find the point and plane the zero-margin decision reads: a fin's vertex on a neighbour's coplanar cap is the likely pair, since every member is a copy of one fin. Decide whether a pair the separation certificate holds apart should reach a point-in-solid test at all. If it should, the decision must not branch on an exact tie at the nominal. Then the row compares under `Apart` again, red on main's `segment_gap` flips alone until that item closes.

**2026-10-30, nightly triage** (nightly [41008276467](https://github.com/evgunter/cad/actions/runs/41008276467), main `39bbdfc777`): `revalue_corpus::revalue_heat_sink_fins` (`crates/editor-core/tests/revalue_corpus.rs:393`) is still red at every ε: it compares `InStep` against the pinned `Apart`, with 22242 compared. The message is unchanged from 2026-10-29.
