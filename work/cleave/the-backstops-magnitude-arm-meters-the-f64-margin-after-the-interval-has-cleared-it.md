---
id: the-backstops-magnitude-arm-meters-the-f64-margin-after-the-interval-has-cleared-it
kind: issue
title: The volume backstop's magnitude arm meters the f64 margin after the interval re-derivation has cleared it, so a correct boolean refuses ResultVolumeImplausible on its closed forms' rounding
status: closed
opened: 2026-10-21
priority: P1
cost: E
refs: [the-backstop-reads-an-unpadded-f64-volume-as-exact-where-it-passes-or-switches-off-a-bound, a-planar-face-sums-its-area-about-a-far-carrier-origin, a-correct-intersect-a-sub-ulp-outside-its-operand-refuses-at-the-backstops-exact-band, a-block-resting-a-zero-band-gap-above-another-refuses-at-the-volume-backstop]
pr: 5842
closed: 2026-10-27
---


Found by FLUX unit #2 (`flux/far-frustum-pad`) while auditing the readers
that take a zero `volume_pad` as exact.

## The path

`crates/topo/src/boolean/ops.rs`, `bound_holds`:

1. `:2530` sets `metered = hi / lever`, `hi` the margin's upper end
   summed at the walk's scalar. With every face in closed form the
   margin is `SignReading::Exact` (`MassProperties::reading`, pad
   `0.0`), so `hi` is the `f64` sum with nothing stated for its
   rounding.
2. Arm 1 (`:2539`): the `f64` margin reads `Violated` at the exact band,
   so it is re-derived in interval arithmetic. Where the interval margin
   does not certify the violation, `open` is set and `lo` moves to the
   interval's low end (`:2555`).
3. No round moves a closed form, so the loop falls through. The open
   arm (`:2574`) reads the interval `lo`.
4. Arm 2 (`:2583`) then decides `metered`, which is still the `f64`
   margin, against the band, and refuses `ResultVolumeImplausible`.

So a margin the interval has cleared is refused on the rounding it was
re-derived to get past.

**The rule this breaks:** a reader that decides on a zero-pad value
re-derives it (`props::certify_role`: the interval re-derivation, or
`rounding_bound` with no lane), or decides on an enclosure of it. Check 7,
check 10, `shell_role` and point containment do. Arm 2 is not the only
backstop reader that doesn't, as FLUX #2's audit first said: its review
found three more, filed together as
`the-backstop-reads-an-unpadded-f64-volume-as-exact-where-it-passes-or-switches-off-a-bound`
(the Bound arm's held read, a silent pass; the `bounded` operand gate;
the +V posture).

## Witness, through the public doors

A unit box split by `x = 0.3 + 0.1·z`, the plane stated through a point
`far` metres along itself (cleave's `rigid-turn-sliver-volume` witness,
filed on the FLUXTAIL P1). Then the above side is cut by a box below
`z = 1 − d` (`∩`), less a box above it (`∖`), or with it (`∪`). At
every refusal, the inequality's interval margin holds by 4e-6 m³ or
more.

| ε | rows | refused | where |
|---|---|---|---|
| 1e-9 | 149 | 14 | far 1e5 and 1e6 |
| 1e-12 | 91 | 0 | (the split refuses at far ≥ 1e5) |
| 1e-6 | 149 | 6 | far 1e5 and 1e6 |

Instrumented, every refusal is arm 2, after an interval margin that
holds. For example, far 1e5, ε 1e-9, `d = 1e-3`, `A ∩ low`:

```
arm1 vol(A ∩ B) ≤ vol(A) interval margin [6.000500000000741e-4, 6.000500000007403e-4]
arm2 refuses vol(A ∩ B) ≤ vol(A) metered -5.0174571138747066e-5
ResultVolumeImplausible { which: "vol(A ∩ B) ≤ vol(A)", got: "0.6493999500006974", bound: "0.6489379992416014" }
```

The `f64` sums are the FLUXTAIL P1's error: A reads 0.648938 for an
exact 0.65. At far 1e6 the below side reads −1.46 for +0.35, and its
`∪` with the top box refuses `vol(A ∪ B) ≥ vol(B)` against an interval
margin of +0.35.

The probe (a `topo` unit row beside `tier3_tests::far_anchored_slab`):

```rust
let b0 = finished("box", brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol), tol);
for far in [1e2, 1e3, 1e4, 1e5, 1e6] {
    let s = far / (0.1f64 * 0.1 + 0.25 + 1.0).sqrt();
    let origin = Point3::new(0.3 + 0.1 * s, 0.5 * s, s); // on the plane, `far` along it
    let plane = split_plane(origin, Vec3::new(1.0, 0.0, -0.1), tol);
    let split = crate::split(&b0, &plane, tol)?;
    for part in [&split.above, &split.below] {
        let a = finished("side", part.body()?.clone(), tol);
        for d in [1e-1, 1e-2, 1e-3, 1e-4, 1e-5] {
            let low = finished("low", brick::<f64>((-1.0, 2.0), (-1.0, 2.0), (-1.0, 1.0 - d), tol), tol);
            let top = finished("top", brick::<f64>((-1.0, 2.0), (-1.0, 2.0), (1.0 - d, 2.0), tol), tol);
            // Each of these refusing ResultVolumeImplausible is the defect.
            let _ = (crate::intersect(&a, &low, tol), crate::subtract(&a, &top, tol), crate::union(&a, &top, tol));
        }
    }
}
```

A planted witness through `volume_backstop` itself (its unit rows'
entry) needs no split. R is `tier3_tests::far_anchored_slab(0, 1e-3, t,
far)` and A is the same slab at `far = 0`, so R ≡ A geometrically and
every inequality holds exactly. Its interval margin is [−6.5e-27,
+6.5e-27], yet arm 2 refuses `∩` and `∪`: 24 of 79 rows at 1e-9 (far ≥
1e4), 63 of 159 at 1e-12 (far ≥ 1e3) and 0 of 19 at 1e-6. Any thickness
from 3e-11 to 1e-5 m refuses, so these are not slivers. The 1e-6 count is
this corpus's: R1's planted corpus on PR 5576's review
(`review/5576-017Nyiv2`, `review_5576_cleave_planted`) refuses 1 of 8 at
1e-6 (8 of 32 at 1e-9, 27 of 56 at 1e-12), and R2's refuses at 1e-6 too.

## The fix proposed

Once arm 1 has re-derived the margin, arm 2 meters the interval margin
(its upper end over the interval area) in place of the `f64` one.
Curing the FLUXTAIL P1 shrinks the error, but arm 2 would still read an
unpadded sum as exact, so this fix stands on its own.

## Neighbours

- `fluxtail/a-planar-face-sums-its-area-about-a-far-carrier-origin`
  (P1): the cause of the `f64` error in both witnesses.
- `a-correct-intersect-a-sub-ulp-outside-its-operand-refuses-at-the-backstops-exact-band`
  (P2) and
  `a-block-resting-a-zero-band-gap-above-another-refuses-at-the-volume-backstop`
  (P0): the same backstop, other arms.

## From PR 5576's review (2026-10-21)

- **Arm 2 has no test.** R2's mutant, arm 2 never refusing, survives all
  2250 `topo` tests. A fix lands with a row that reds when arm 2 refuses
  on an interval margin that holds (either witness above), and with one
  that reds when it passes a margin the interval certifies violated
  beyond the band.
- **The fix moves an opaque pin:** R2 measured that metering the
  interval margin in arm 2 moves `boolean::ops::from_computed_rows`'
  re-valued count from 2 to 4. Re-baseline it and say what moved.
- **The naming invites the read:** `SignReading::Exact`,
  `SignReading::exact()` and `is_exact()` mean "one value, no quadrature
  bracket", not "exact" (R1, style). The sibling row carries the case
  for one fix over all four readers.

## Closed (2026-10-27)

The volume backstop left the boolean's operation path (`[ev]` PR 5780: the volume relations are a test oracle the suites run on their own results, `topo::test_support::volume_relations`), so nothing refuses or passes on it. The magnitude arm went with it, and the witness's correct bodies are returned.
