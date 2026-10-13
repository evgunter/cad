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

## 2026-10-11 — a second seed (FLUSH, local release run)

On FLUSH's `flush/apex-edge-in-face-plane`, whose diff touches none of
the row's gated paths:

```
CAD_FUZZ_SEED=0xedc36ace8b877740 CAD_FUZZ_EFFORT=1
ε 1e-6, depth 9.999757644744384e-6 (true 1.000e-5, 0 true roots): Ok(Miss)
  Circle { center: (-0.24244944920625458, 0.08343253614355985, -1.0225101464360302),
           axis: (-0.949015772808044, -0.02563613020061992, 0.3141844232133222),
           radius: 1.0810235207384276,
           u_ref: (0.3152285884267972, -0.07717920520967814, 0.9458722468291643) }
  wall r 0.0983100695346417
```

The clearance here is 9.99976ε against a true 1.000e-5: the same
boundary of the gap, at a unit-scale pose rather than `r ≈ 70.87`, so the
far-frame rounding reading above does not explain this one.

## Another seed (NURBS, 2026-10-12)

Seed `0xe8586f5f5e218549` reddens `topo` `conic_quadric::in_band_sweep` in one default run.

NURBS build A replayed it on two trees, each on its own isolated target dir:
- on PR 4551's branch: red;
- on main at `5160aff448`: red.

It is not PR 4551's change, and it is a witness for this row. Replay:

```
CAD_FUZZ_SEED=0xe8586f5f5e218549 cargo nextest run -p topo -E 'test(in_band_sweep)'
```

The clearance was not taken. — (NURBS orchestrator)

## A second seed (CONTACTHOLD, 2026-10-12)

PR 4781's CI (run 38214475352, job `test`, the ε 1e-6 row) drew another
seed in the same class, and it replays on main at `faf9995c39`:

```
CAD_FUZZ_SEED=0x50cc8da379c68fde CAD_FUZZ_EFFORT=1 CAD_TOLERANCE_EPS=1e-6 \
  cargo nextest run -p topo --lib in_band_sweep::no_pose_inside_the_band_is_answered
```

```
ε 1e-6, depth 9.7516991133157e-6 (true 9.752e-6, 0 true roots): Ok(Miss)
  Circle { center: (3.9496123832118526, -0.0005186744695551243, 0.0032386832317805856),
           axis: (-0.3082326362143074, -0.15043530224655058, 0.9393411849857152),
           radius: 0.010641160947759434, … }
```

A circle clear of the wall by about 9.75ε answered `Miss` inside the
escalation gap, the same shape as the first seed. Any PR whose CI draws
such a seed goes red on it; the fuzzer's seed varies per run.

- 2026-10-12 (ENCL orchestrator): a second witness on ENCL's PR 4811 (editor-core only; CI run 38226984714, default-eps row): `topo boolean::conic_quadric::in_band_sweep::no_pose_inside_the_band_is_answered` panicked at in_band_sweep.rs:160, '1 in-band poses answered'. Internal row ε 1e-6, true depth 9.894e-6 (inside the <10ε escalation gap, r≈37.55 region), answered Ok(Miss). Replay: `CAD_FUZZ_SEED=0x67e238e4dd9155cf CAD_FUZZ_EFFORT=1`. The fuzz draws a fresh seed per run, so it reddens random PRs.

## Three more seeds at effort 10 (GERM, PR 4886's review, 2026-10-12)

The review pair of PR 4886 (head `b9e0bafb`) found the row red at
effort 10 on three more seeds, identically on main:

```
CAD_FUZZ_SEED=0x883d9762675a3ba8 CAD_FUZZ_EFFORT=10 \
  cargo nextest run -p topo --lib in_band_sweep::no_pose_inside_the_band_is_answered
CAD_FUZZ_SEED=11 CAD_FUZZ_EFFORT=10 …
CAD_FUZZ_SEED=18 CAD_FUZZ_EFFORT=10 …
```

Replayed on PR 4886's branch (fix-pass head), each answers `Miss` at
ε 1e-6 for a circle clear of the wall inside the `< 10ε` gap, with no
true roots:

| seed | depth (true) |
|---|---|
| `11` | 9.176e-6 |
| `18` | 1.000e-5 (9.99977e-6 read) |
| `0x883d9762675a3ba8` | 9.999e-6 (9.99946e-6 read) |

PR 4886 does not touch `conic_quadric`. Its changes to
`certified_subdivision` act only on a bracketed root, and a `Miss` has
none; the reviewers saw the same reds on main.
