---
id: an-interval-revolve-of-a-cone-wedge-a-thousand-units-out-refuses-its-cones-rung-3-at-1e-12
kind: issue
title: A revolve of the unit cone wedge a thousand units from the origin refuses its certification at eps 1e-12, at f64 (AnalyticRung3 KernelLimit on its cone) and at Interval (MappedSource escalates), where the same wedge a hundred units out builds
status: open
opened: 2026-11-22
priority: P3
cost: M
---


## Finding

Filed by lane `embed/apex-separator` (EMBED, PR 7315's fix pass,
2026-11-22), on main `7669fcfdd`.

`CAD_TOLERANCE_EPS=1e-12`, at f64: the triangle `(0,0) (1,0) (0,1)` revolved by
`θ = 1.5π + 0.01` about the sketch's `y`, on a sketch plane moved to
`(640, −570, 520)`, refuses in `sweep::revolve`:

```
Op { source: Certification { error: AnalyticRung3(KernelLimit { operand: Cone,
  bound: 1.28621974215376e-12, margin: MarginDiag(Value(1.28621974215376e-12, None,
  ZeroWord(0))), width: 2.2737367544323206e-13, extent: 1.414213562373256 }) } }
```

The f64 build refuses so; the `Interval` build of the same body refuses
on another check at 1e-12:

```
Op { source: Certification { error: Escalated { check: MappedSource, sample: 0,
  cause: Indeterminate { margin: MarginDiag(Enclosure { lo: 0.0, hi: 3.487425976840761e-12, .. }),
  band: Band { zero: 1e-12, escalate: 1e-11 }, predicate: Some("carrier_matches_mapped_source") } } } }
```
 At both
scalars it builds at the default ε and at 1e-6, and a hundred units out
(`(64, −57, 52)`) at every ε.
The bound is the band at 1e-12 scaled to the extent, and the width is
an ulp of 640, so at this distance an `Interval` cone's rung-3 margin
is the coordinates' own rounding. Whether that refusal is the kernel
limit doing its job (the row's expectation) or a margin read at the
wrong scale is not measured.

**The same at a hundred times the size.** The triangle
`(0,0) (100,0) (0,100)`, the same `θ`, plain frame: at `Interval` and
1e-12 its pcurve certification escalates:
`Pcurve(Certify { half_edge: HalfEdgeKey(10v1), error: Escalated { check:
EnvelopeTerm(FidelityV), .. } })`, margin `[0, 1.86e-12]` against the
band `1e-12`. (PR 7115's branch files a long cone a hundred units out
escalating its pcurve envelope at 1e-12 on PCERT; not on main yet.)

**Where it shows:** `self_intersection_proves_injectivity::a_cone_wedge_past_a_half_turn_is_embedded_at_its_apex`
skips those two poses' builds loudly at 1e-12 on a certification
refusal only; any other refusal fails the row, and every other pose is read at
1e-12.

- 2026-11-23 — **by `−θ`, a third kind of refusal at the same limit**
  (EMBED, lane pole, PR 7115): the wedge a hundred times the size
  revolved by `−4.7224` or `−5.5`, at `Interval` and ε 1e-12, refuses
  the cone's rung 3 with limb 2 undecided
  (`AnalyticRung3Refusal::Stalled`: enclosure `[0, 1.06e-12]` against
  the band `(1e-12, 1e-11)`), where by `+θ` the same pose refuses its
  pcurve's `FidelityV` envelope term. `at_the_build_limit` in
  `crates/sweep/tests/self_intersection_proves_injectivity.rs` names it.
