---
id: a-rigid-map-moves-an-enclose-chart-margin-past-its-rounding-allowance
kind: issue
title: A rigid map moves an enclose chart margin by 1.3e-12 relative, past the fuzz test's rounding allowance
status: closed
branch: nurbs/rigid-map-chart-allowance
closed: 2026-10-14
opened: 2026-10-14
---


## Found

`geom-brep`'s
`ssi::enclose::tests::a_rigidly_mapped_wall_reads_the_seated_chart_speeds_floors_and_tube`
(`crates/geom-brep/src/ssi/enclose.rs`, near line 1764) is a shape-1
fuzzer with a fresh seed each run. It went red on seed
`0x166d62361818555e` at main `416607159e`. The failure reproduces under
the ci profile and does not depend on the build flags:

```
CAD_FUZZ_SEED=0x166d62361818555e cargo nextest run -p geom-brep --lib a_rigidly_mapped_wall_reads_the_seated
```

```
two-span patch: a rigid map about Vec3 { x: -0.9254990197806239, y: -0.1674496796624283,
z: -0.38154305676260414 } through Point3 { x: 1.1971011302698784, y: -0.13768816972372022,
z: -0.5807137925187895 } by 0.5675790275065047 moved a chart reading 0.002672789803881874
to 0.002672789803885438
```

A second seed, `0xdfa19917eaceb89e` (at `CAD_TOLERANCE_EPS=1e-6`), fails
on the same two-span patch and the same reading: 0.002672789803881874
moves to 0.0026727898038849634. Both seeds are deterministic reproducers. The test drew a
failing seed on 2 of the 6 runs the build-time lane made on this tree, so
the per-PR gate goes red on it often.

The failing assertion is the `within_rounding(a, b, reach, scale)` check
in the same test (`enclose.rs`, near line 1813), on the speeds/pads/margins
chain. The moved reading is `margins[1]`. It moves by 3.6e-15 absolute, or 1.3e-12
relative. Every speed and pad on the same wall agrees to about 1e-15
relative.

## Likely cause

`within_rounding` (same file) allows `256 · ε · (1 + reach) · scale ·
max(|a|, |b|)`. That bound is relative to the reading's own magnitude.
A margin is a difference of quantities at the wall's own scale, so its
rounding error scales with that wall scale and not with the margin. A
small margin under a rotation that reorders the arithmetic can
therefore exceed an allowance that the speeds meet easily. Either the
margin rows need an absolute term at the wall's scale, or the margin
computation needs to be shown frame-invariant to the stated bound, in
which case this seed is a real defect.

Pin both seeds as deterministic tests beside whichever fix lands
(`docs/prompts/implementer-discipline.md` §8).

— found by the build-time lane (NURBS), 2026-10-14

## Resolution (NURBS build A, 2026-10-14)

This is the same defect as
`rigid-map-chart-readings-fuzz-reds-past-its-rounding-bound`: the same
reading and the same cause. It is closed there, and that item's
resolution carries the measurement and the fix. This item's "likely
cause" was right: a margin rounds at the wall's scale, not at its own
value. Both of its seeds are pinned as fixed rows there.
