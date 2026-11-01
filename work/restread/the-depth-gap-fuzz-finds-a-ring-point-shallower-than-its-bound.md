---
id: the-depth-gap-fuzz-finds-a-ring-point-shallower-than-its-bound
kind: issue
title: The depth-gap fuzz finds a point on a zero-minor ring 4.5e-14 shallower than its centre bound (seed 0xb4d28f0aaf5101ad)
status: open
priority: P2
cost: M
opened: 2026-10-21
---

Found by `tier-3-prime-reads-no-side-for-a-face-touching-a-vertex-of-its-own-solid`'s
lane, in a full `cargo nextest run -p topo` at the default ε. The lane
did not touch `crates/topo/src/census/depth_gap.rs`.

## What

`census::depth_gap_fuzz::no_sampled_depth_lies_past_its_bounds`
(`crates/topo/src/census/depth_gap_fuzz.rs`) failed at draw 40:

> Interval: Point3 { x: 2.9574539024146675, y: -0.20064889977029, z:
> -0.8359455165007845 } on Ring(Ring { center: (1.8744043230431866,
> 0.705929995463215, -1.5148813724340533), normal: (-0.4155440886894936,
> -0.6498738190544249, -0.20488980989626765), major: 1.5671105835681742,
> minor: 0.0 }) is 4.4853010194856324e-14 shallower than the bound
> Interval(lo: 1.6186984043841461, hi: 1.6186984043841481) on Center {
> (1.6633310049300403, 0.37583018878499264, -1.6189540164674816) }

Reproduce: `CAD_FUZZ_SEED=0xb4d28f0aaf5101ad CAD_FUZZ_EFFORT=1 cargo
nextest run -p topo -E 'test(no_sampled_depth_lies_past_its_bounds)'`.

The bound is an Interval enclosure and the sampled depth falls below
its lower end by 4.5e-14 on a ring of zero minor radius (a circle).
Either the ring's depth bound is not an enclosure at `minor = 0`, or
the sample's own depth is computed outside the enclosure. Pin the seed
as a deterministic row beside the fix.

## Another seed (2026-10-28, PR 5956 CI, eps 1e-6)

Draw 340 of seed `0xe2bf3f192c072cb8`: a point on a zero-minor ring
(major 1.1416) read 8.06e-14 deeper than its centre bound 1.2484.
PR 5956 (cone apex) does not touch `census/`. Reproduce with
`CAD_FUZZ_SEED=0xe2bf3f192c072cb8 CAD_FUZZ_EFFORT=1 CAD_TOLERANCE_EPS=1e-6`.
