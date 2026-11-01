---
id: a-face-inward-direction-is-spelled-in-five-places
kind: issue
title: The direction into a face from its half-edge (N × τ) is spelled at five sites beside stands::inward
status: open
opened: 2026-10-27
priority: P3
cost: E
---

Found by PR 5808's style review.

The direction into a face from a half-edge of it, `N × τ` (`N` the
face's outward normal at the point, `τ` the half-edge's tangent), has
one home, `crates/topo/src/stands.rs` `inward`, and is spelled again by
hand at:

- `crates/topo/src/boolean/slit.rs` `walls_inside` (`normal.cross(dir)`);
- `crates/topo/src/side_reader.rs` `edge_frames` (`normal.cross(tau)`);
- `crates/topo/src/chord_join.rs` (`normal.vec().cross(tangent)`, in the
  ring-side reading near `join_lone_ring`, and
  `quadric.outward(q, sense).cross(travel)`).

`boolean/reduce.rs` `sheets_part` calls `inward`. Each site also
re-derives `τ` from the carrier and negates it for `he_minus`; a helper
taking the half-edge would hold both.

## Slate triage

2026-11-01: moved from `work/cleave/` by CLEAVE's split on its semantic seams (Ev, in chat: CLEAVE measured 114 budget points against 30). ONEREAD holds the boolean questions read by two homes; id and body unchanged. (CLEAVE orchestrator)
