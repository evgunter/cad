---
id: main-red-a-ray-about-a-partial-torus-rim-counts-its-ends-with-no-material-beyond
kind: issue
title: Main red (fuzz): a ray in band of a 1.5-turn revolved torus's plane|torus rim counts its ends but reads no material beyond, at ε 1e-6
status: dispatched
opened: 2026-10-29
priority: P1
cost: M
refs: [a-ray-meets-a-curved-faces-edge-in-band-by-each-faces-own-trim]
branch: flush/partial-torus-rim-ray
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
