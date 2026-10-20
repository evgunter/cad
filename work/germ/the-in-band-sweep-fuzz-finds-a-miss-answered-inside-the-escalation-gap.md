---
id: the-in-band-sweep-fuzz-finds-a-miss-answered-inside-the-escalation-gap
kind: issue
title: GERM's in-band sweep fuzz finds a circle clear of a wall by 9.991ε answered Miss inside the escalation gap at ε = 1e-6
status: closed
branch: germ/in-band-sweep-gap-miss
pr: 4964
opened: 2026-10-11
closed: 2026-10-13
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
- 2026-10-12 (CLEAVE, PR 4858, CI run 38258750526, default-eps row): another witness. Internal row ε 1e-6, true depth 9.796e-6, circle radius 0.01418 against a wall of r ≈ 54.72, answered `Ok(Miss)` inside the escalation gap. Replay: `CAD_FUZZ_SEED=0xd94af3a7b70f78b6 CAD_FUZZ_EFFORT=1`. PR 4858 touches no code this row reads (`conic_quadric_roots` takes no body).
- 2026-10-13 (TANG, PR 4413, local debug run at the default ε, head 6e7f07bcd, main merged at 95d344176): `CAD_FUZZ_SEED=0x100c76901b4364a8 CAD_FUZZ_EFFORT=1` replays it deterministically. Internal row ε 1e-6, drawn depth −9.507e-6 (true 9.507e-6, 0 true roots), answered `Ok(Miss)`, so a circle clear of the wall by about 9.5ε, the same shape again. Circle centre (58.7466, 0.011136, −0.0042817), axis (−0.84551, −0.49839, 0.19163), radius 0.014111.

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

## Root cause (GERM, 2026-10-13)

Two causes, one on each side. All nine seeds above reproduce on main
at `0ddfd40980`, and each is pinned as a row of `GAP_WITNESSES`
(`crates/topo/src/boolean/conic_quadric/in_band_sweep.rs`,
`the_sweeps_gap_witnesses_are_judged_in_the_residual`).

**The door (six seeds: `0x71b4eb0ade4c1576`, `0x9433bbf2eb0a13c0`,
`0xe8586f5f5e218549`, `0x50cc8da379c68fde`, `0x67e238e4dd9155cf`,
`11`).** Each takes the first-harmonic arm. `first_harmonic_arm`
(`crates/topo/src/boolean/conic_quadric/mod.rs`) drops the second
harmonic `A₂` and charges it to the extremes' noise. But
`first_harmonic_roots_in_band`
(`crates/topo/src/boolean/circle_roots.rs`) decided the extremes on
`Margin::of(lo)` / `Margin::of(hi)` bare. Its noise meter only checks
that the error is within the zero band, and that is no bound on where
the extreme sits relative to the gap's far edge `Kε`. On the first seed
`hi = −1.0088e-5` and `A₂ = 0.099ε`, while the true extreme is
`−9.991e-6`. On the second, `A₂ = 0.54ε` (hence its deeper 9.68ε).
Fixed by `extreme_sign`, which decides each extreme as the interval
its error bound spans: both ends decided, and an end in the gap
escalates.

**The judge (three seeds: `0xedc36ace8b877740`, `0x883d9762675a3ba8`,
`18`, true distance 9.9995ε to 9.9998ε).** Each takes the ladder from
OUTSIDE the wall. The door classifies the linearized residual
`(ρ² − r²)/2r = d + d²/2r`, which `crates/geom-brep/src/implicit.rs`'s
module docs state is what classifies against the linear ε, agreeing
with the distance `d` only to first order. Outside the wall it
overstates `d` by `d²/2r`: up to 0.0019ε at `r = 0.027`. So these poses
truly lie past `Kε` in the band's own metric, and their `Miss` is right.
The row's judge read the oracle's distance. The oracle now also returns
`residual_depth` (`crates/topo/src/boolean/conic_oracle/exact.rs`,
`Truth::residual_depth`), and the judges that gate on the band read it.
It was not the far-frame rounding at `r ≈ 70.9`: the oracle is
double-double on the stored carrier.


**The seeds above no longer replay their poses.** The row draws every
pose's values before its judge now, so a seed replays its poses
whatever the judge or the door does; when these seeds were found it
drew the arc after the judge's `continue`, so any change to the judge
reshuffled the stream from there on (seed `11` at effort 10 went green
under the pre-fix door with the judge alone changed). Moving the draw
reshuffles it once more, so the seeds above are not replays of their
poses on this branch. `GAP_WITNESSES` is what pins those poses.

## Another seed (CLEAVE, PR 4861's CI, run 38272226876, 2026-10-13)

`CAD_FUZZ_SEED=0xc15ec3dfe26134a8 CAD_FUZZ_EFFORT=1` at ε = 1e-6
answers `Ok(Miss)` for a circle `9.660e-6` clear of the wall (`r`
32.836, `ρ` 0.0226, 0 true roots), again inside the escalation gap.
It reproduces locally on the branch, whose diff on this path is the
`Curve3::Ellipse` field rename alone.
It was found before the root-cause fixes above and has not been
replayed against them.
