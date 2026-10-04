---
id: cycle-walks-refuse-loop-cycle-broken-for-a-stale-next-link
kind: issue
title: A kill or make whose cycle walk meets a dangling next refuses LoopCycleBroken, where a dangling prev refuses StaleKey
status: closed
opened: 2026-10-03
priority: P4
cost: E
closed: 2026-10-04
pr: 4029
---


(TOPO lane `topo/kill-anchors-collision-refuses-its-own-variant`, from
its §5 receipt.)

## What

`EulerOpError::StaleKey` (`crates/topo/src/euler.rs`) decides "a key
the operator must follow to do its work (a `prev` link, a spine
parent, a start vertex) does not resolve". `Body::loop_cycle`
(`crates/topo/src/body.rs`) returns `None` both for a walk that
overruns and for a stale link (`Walk::Broken`), and every operator
that walks a cycle maps that `None` to `LoopCycleBroken`. So one
dangling link refuses two variants depending on which direction it
points. Executed probe on `declined_cube`, `kef(halves[0])`:

- `next(next(he))` set to `HalfEdgeKey::default()`:
  `Err(LoopCycleBroken { loop: 3v1 })`;
- `prev(he)` set to `HalfEdgeKey::default()`:
  `Err(StaleKey { key: HalfEdge(null) })`.

Sites that map the walk's `None` to `LoopCycleBroken`:

- `Body::mef`'s plan (`euler.rs`, `loop_cycle(he1)`);
- `Body::site_cycle_from` (`euler.rs`);
- `Body::site_cycle` (`euler.rs`), whose `LoopRows::Corrupt` also
  covers a loop key that does not resolve (`pcurves::loop_rows`);
- `Body::kef`'s plan (`euler_kill.rs`, `loop_cycle_live(he)`);
- `Body::kemr`'s plan (`euler_ring.rs`, `loop_cycle_live(he1)`);
- `Body::mekr`'s two ring-walking sites (`euler_ring.rs`,
  `loop_cycle_live(ring)`).

The same mapping outside the `euler*.rs` files (found by PR 3975's
review; the receipt's pattern covered only those three files):

- `movefac` (`movefac.rs`, `loop_cycle(first).ok_or_else(broken)`);
- `attach.rs`'s walk at the re-chart door (~:570);
- `merge_faces::outermost_survivor` (`merge_faces.rs`).

## The question

Which variant decides a link that does not resolve during a walk?
Either `StaleKey` (split `Walk::Broken` from `Walk::Overrun` and name
the key), or `LoopCycleBroken` with `StaleKey`'s doc narrowed to say
that a `next` followed by a cycle walk is not one of its keys. Both
variants answer `reports_tier1_corruption` `true`, so no render
changes class either way.

## Closed 2026-10-04

PR 4029. The question dissolved: neither variant decides a link that
does not resolve, because only a torn body reaches one, and a torn
body now panics naming the record (D2 row 4). `StaleKey`,
`LoopCycleBroken` and `reports_tier1_corruption` are deleted.

- A dangling `prev` panics in `live::linked` / `live::proven`
  (`crates/topo/src/live.rs`), naming the holder, its link and the key.
- A dangling `next` panics at the walk. `Walk::closed`
  (`crates/topo/src/body.rs`) names the hop that broke (`Walk::Broken
  { at }`); `movefac`, `merge_faces::outermost_survivor` and the
  re-chart door's walk (`attach.rs`, through `site_cycle_from`) and
  `mef`'s plan read through it or through `site_cycle_from` /
  `site_cycle` (`euler.rs`), which panic naming the loop and the first
  half-edge. `kef` (`euler_kill.rs`, `loop_cycle_live(he)`), `kemr`
  (`euler_ring.rs`, `loop_cycle_live(he1)`) and `mekr`'s
  `ring_cycle` (`euler_ring.rs`) panic naming the loop and the walk's
  first half-edge, not the hop: `loop_cycle_live` returns `Option`
  and drops `at`. That is a message's precision, not a second answer
  to one state, so it is not this row's question.
