---
id: the-in-band-sweep-fuzz-finds-a-miss-answered-inside-the-escalation-gap
kind: issue
title: GERM's in-band sweep fuzz finds a circle clear of a wall by 9.991ε answered Miss inside the escalation gap at ε = 1e-6
status: open
opened: 2026-10-11
priority: P1
---


Found by CLEAVE on PR 4607's CI (run 38114075940, job `test`), and
replayed on main at `50537331ae`, where it fails the same way:

```
CAD_FUZZ_SEED=0x71b4eb0ade4c1576 CAD_FUZZ_EFFORT=1 \
  cargo nextest run -p topo --lib \
  -E 'test(=boolean::conic_quadric::in_band_sweep::no_pose_inside_the_band_is_answered)'
```

`crates/topo/src/boolean/conic_quadric/in_band_sweep.rs:160` reports one
in-band pose answered:

```
ε 1e-6, depth -9.991394530896835e-6 (true 9.991e-6, 0 true roots): Ok(Miss)
  Circle { center: (70.86155278926769, 0.00027419437321786835, 0.0006195764874900972),
           axis: (0.052157748280677596, 0.4041414718003223, 0.9132082128766467),
           radius: 0.012990167364053808,
           u_ref: (0.9986388582937725, -0.021107839917183423, -0.047695804844253176) }
  wall r 70.8745352665677
```

The answer is right: the circle clears the wall by 9.991ε and has no
roots. But the clearance sits inside the escalation gap (`< 10ε`), where
the row requires `Uncertain` or a refusal. So either the door decides
`Miss` on a margin that should have read in the gap, or the row's judge
reads the depth from a frame whose rounding at `r ≈ 70.87` (`ρ` 0.013)
moves a pose just past `10ε` onto the in-band side. Which one it is has
not been traced.

The row draws a fresh seed per run, so this turns any PR's `test` job red
at random. That is why it is P1.

**A second seed** (INTENT stage 3 A's CI on PR 4556, head 1bf7c00e2a).
The INTENT orchestrator replayed it on main at 84498e62be, where it
fails the same way:

```
CAD_FUZZ_SEED=0x9433bbf2eb0a13c0 CAD_FUZZ_EFFORT=1 \
  cargo nextest run -p topo -E 'test(no_pose_inside_the_band_is_answered)'
```

```
ε 1e-6, depth -9.679071350377893e-6 (true 9.679e-6, 0 true roots): Ok(Miss)
```

A clearance of 9.679ε, further inside the `< 10ε` gap than the first
seed's 9.991ε, so frame rounding at the gap's edge is a less likely
explanation for this one.
