---
id: main-red-a-ray-about-a-partial-torus-rim-counts-its-ends-with-no-material-beyond
kind: issue
title: Main red (fuzz): a ray in band of a 1.5-turn revolved torus's plane|torus rim counts its ends but reads no material beyond, at ε 1e-6
status: review
opened: 2026-10-29
priority: P1
cost: M
refs: [a-ray-meets-a-curved-faces-edge-in-band-by-each-faces-own-trim]
branch: flush/partial-torus-rim-ray
pr: 6168
---


Found by nightly triage 2026-10-29, run
[40823227622](https://github.com/evgunter/cad/actions/runs/40823227622)
(main `5648e577eb`), job `full suite`, ε row `1e-6` only.

## What

`sweep` `a_ray_meets_a_curved_edge_once_for_both_faces::every_crossing_of_a_ray_about_the_corpuss_edges_counts_right`
(`crates/sweep/tests/a_ray_meets_a_curved_edge_once_for_both_faces.rs:237`):

```
1 rays read wrong: [
  "rev torus 1.5pi (inside out: false) at ε 1e-6, Circle EdgeKey(6v1) [Plane | Torus]
   q=Point3 { x: 0.1654750844081492, y: -0.3689933861942981, z: -6.0031985322182555 }
   d=Vec3 { x: -0.020687219868856334, y: -0.01637304530398302, z: 0.9996519205811447 }:
   [(3.751781834279822, 3.751781834279822, -1), (4.246592668498707, 7.99890393475562, 0)]:
   ends counted true, material false beyond",
]; reproduce with CAD_FUZZ_SEED=0x003481ce0ba06522 CAD_FUZZ_EFFORT=1
```

The ray's crossing cluster at the plane|torus rim of the partial revolve
counts as entering. The analytic body reads no material beyond it, so
the side reading through the met edge
(`topo::boolean::solid_contain::curved_edges`, PR 5697) gives a wrong
membership for this ray.

## Attribution

**Latent, not tonight's.** Reproduced with the seed at `CAD_TOLERANCE_EPS=1e-6`. It
fails the same at `a1cfd1cfdb` (the 10-28 nightly's head) as at
`5648e577eb`. The row draws a fresh seed each night, and this is the
first seed to land on the case. It is the corpus walk of PR 5697's
curved-edge rule.

## What closing it takes

Replay the seed, find which face's half the cluster at `t ≈ 3.7518`
stands for, and see why the torus side reads no material past it. A
grazing ray on the rim's plane side is the likely reading. Fix the rule,
or show that the ray is outside the rule's in-band precondition and
narrow the fuzz. Then pin the seed as a row.

Banded P1, not P0: it is a wrong membership from a containment reader on
an everyday solid. No verb is yet shown to answer wrong because of it.

Reproduce: `CAD_TOLERANCE_EPS=1e-6 CAD_FUZZ_SEED=0x003481ce0ba06522
CAD_FUZZ_EFFORT=1 cargo test -p sweep --test all --
a_ray_meets_a_curved_edge_once_for_both_faces::every_crossing_of_a_ray_about_the_corpuss_edges_counts_right`.

## Measured

The seed reproduces on main (`45f6be756b`) and on PR 5781's head
(`ba313795a9`) alike, at ε 1e-6 only. Through `ray_crossings`
(`crates/topo/src/boolean/solid_contain.rs`) the ray meets two planes,
the cap `FaceKey(1v1)` at t 7.99890 and the far cap at 6.005, and the
torus carrier at two roots only, 3.7518 and 4.2466, where it crosses
the tube across the axis. Near the rim it grazes the tube's bottom
just outside, and crosses the carrier nowhere near there. The met
rim `EdgeKey(6v1)` stood for the cap's plane crossing and for the
torus face `FaceKey(4v1)`'s root at 4.2466, the root
`curved_edges::nearest` reads nearest, 3.75 along the ray from the
passage. That root is not near, but `FaceKey(4v1)` definitely holds
it, so it stood. The cluster from 4.2466 to 7.9989 counted
(−1 + 1)/2 = 0 where the ray leaves the tube at 4.2466: one exit lost.
The row's guess was right: a graze on the rim's plane side.

**The family.** A ray along a torus rim's tangent near the tube's top
or bottom, just outside the tube, tilted off the cap's plane, and
crossing the tube again across the axis. On main it reads 73 of 9025
rays wrong at ε 1e-6, on every rim of three partial turns (1.25π, 1.5π,
1.75π; 6 rays per rim, plus the seed). On PR 5781's head it reads 73,
72 and 72 wrong at ε 1e-6, 1e-9 and 1e-12, so 5781 does not change the
reading.

## Fix

A met edge's half stands for a curved face's root far from the passage
only where the face holds the root and `reaches_root_in_band`
(`crates/topo/src/boolean/solid_contain.rs`) says the ray reaches it
in band of the carrier. That predicate decides the rise off the carrier
over the distance `off` at the root's incidence,
`levered(|d·n̂|, off)`, against the passage's own clearance from the
carrier. The rise is charged for the root point's residual tilt and for
rounding. The comparison fails where it is definitely positive. Where it
lies in band the root is set aside, graze-marked. A straddling or
poisoned comparison (a root at a cone's apex or a torus's axis) is the
ray's `Escalated` fault, as the file's other decided sites do.

A plane's held crossing still stands, because a ray's distance from a
plane is linear. On a convex carrier (a cylinder, a cone's nappe, a
sphere) a root reached from outside satisfies `|f'|·off ≤ f(passage)`,
and a graze there is followed by no far root at all. A root reached
from inside, on a short inside chord, can rise to twice the chord's
depth, so it is set aside. On the torus the bound is necessary and not
shown sufficient (`work/flushtail/a-met-edges-reach-bound-is-necessary-not-shown-sufficient-on-a-torus.md`).
The root across the axis fails it by about six orders of magnitude.

The carrier normal has one home, `chart_normal` over the face's surface
and `geom_brep::implicit_gradient`. `root_outward`'s cone and torus
incidences read it too. A face with no carrier is `CorruptFace` in both.

**Measured.**

- **The first head's comparand.** That head compared the rise against
  the band. Of about 800 far, held roots one fuzz run at ε 1e-6 met,
  98.5% had a rise of at most Kε. Over three seeds per ε, 93% of the
  far roots had a rise in (ε, Kε]: they stood only because an in-band
  `Err` folded into "stands".
- **The passage-clearance comparand.** 1,713–2,645 of those far roots
  per run decide Zero and stand. 0–108 per run lie in band and are set
  aside. None decide Positive, and 0 rays read wrong over the 9 runs.
- **The reviewer's corpus probe.** 46,080 rays and 31,347 points per ε,
  head against `origin/main`. The lines that move are rays read
  `(0, false)` and now set aside: 2 at ε 1e-9, 38 at 1e-6 (mostly
  inside chords, and shallow or along rays at rims) and 1 at 1e-12.
  0 are wrong on either tree.

Pinned in `crates/sweep/tests/a_ray_meets_a_curved_edge_once_for_both_faces.rs`:

- the seed's ray by `the_nightlys_ray_grazing_a_torus_rim_reads_right`
  (fast). It reads wrong without the rule at ε 1e-6 only.
- the family by
  `rays_grazing_a_torus_rim_count_no_crossing_of_the_tube_elsewhere`
  (3 s in a debug build, so in the slow set). It reads wrong without the
  rule at every ε, 72 of 9024 rays. With the rule it reads 0 wrong and
  8160 rays whole, against a floor of 85%.

The fuzz rows ran with 18 fresh seeds plus the nightly's at effort 2,
at ε 1e-9, 1e-6 and 1e-12: 57 runs, 0 wrong.
