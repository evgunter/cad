---
id: face-line-germs-read-unit-in-the-strut-and-walk-orders-amplify-a-within-zero-tilt
kind: issue
title: Face-line germs read unit in the strut and walk orders amplify a within-zero tilt by 1/sin φ (insert strut_order, walks_after; vtxfac germ_order)
status: review
branch: splitdoor/face-line-germ-lean
opened: 2026-11-01
priority: P1
cost: M
refs: [main-red-a-prism-tilted-within-the-zero-escalates-at-the-sector-reach-since-arm-vs-reach]
---


Found by the FULL review of PR 6212 (`splitdoor/prism-tilt-reach`), which has the same root cause.

## What

PR 6212 reads two faces' line un-normalized wherever the sector pass asks which of its rays lies within a sector (`boolean/sectors.rs` `rays_within`: `pair_search`, `insert.rs` `germ_dir`, `vtxfac::pierce_germ_dir`). That drops the `1/sin φ` conditioning a unit line between near-parallel faces carries. The germ itself still leaves as a **unit** direction, and the order machinery downstream reads it against bounds and against other germs at a reach. There the factor is back.

- `insert.rs` `strut_order`, `bool_strut_side`: the germ's side of the arrival edge, `(g × e)·n` at the edge's reach. It is reached from `walk_faces_first` and from `vtxfac`.
- `insert.rs` `strut_order`, `bool_strut_order`: two germs' order, `(g0 × g1)·n` at `arm`.
- `insert.rs` `walks_after`, `bool_shared_cut_order`: two cuts' order at the sector's arm. `cut_order` and `walks_before` read through it.
- `vtxfac.rs` `germ_order`: `side` and `apart`, each `bool_strut_side`, germ against germ in the pierced face. It also calls `strut_order` with `germs[0]` as the edge.
- `vtxfac.rs` `tie_breaks_after`: `strut_order` with a germ as the edge.

## Witness

This is the review probe (`splitdoor/prism-tilt-reach-review` 13af36d6, `insert.rs` tests `rv_strut_side_reads_the_unit_germ_amplified`), re-run on PR 6212's head:

```rust
let band = Band::linear(geom_core::Tol::witness()).unwrap();
let at = |deg: f64| { let (s, c) = deg.to_radians().sin_cos(); Vec3::new(c, s, 0.0) };
let down = Vec3::new(0.0, 0.0, -1.0);
let n_b = at(30.0);
for tilt in [0.0, 0.2 * band.zero()] {
    let n_a = (at(20.0) + Vec3::new(0.0, 0.0, -tilt)).normalize();
    let mut g = n_a.cross(n_b).normalize();
    if g.z > 0.0 { g = -g; }
    strut_order((down, 1.23), n_b, (g, at(120.0)), 0.11, || Ok(None), band)
}
```

Upright it is `Ok(true)`. Tilted by 0.2ε it is `Escalated { Coincidence(Sectors, Moot) }` at `bool_strut_side`, margin −1.4167e-9: the 10° pose of `sectors::tests::a_tilt_within_the_zero_between_near_parallel_faces_decides_as_upright`, where the faces' unit line is steeper than the tilt by `1/sin 10° = 5.76`.

## Why it is not a local fix

- The germ would have to carry its line's lean `|n_a × n_b|` (1 for a germ along an edge or a tangent locus). That runs through `Cut`, `vtxfac`'s `Germ` and `pierce_germ_dir`'s return.
- Against a true edge, the plane-offset reading is exact: levering at `reach · lean` reads the edge's far end off the germ's other face.
- **Germ against germ** is a lever choice the PR 6212 ruling did not make. Both directions carry a conditioning, so whose plane does the reading measure off? A candidate is `min(lean₀, lean₁)`: the noise then stays within the two faces' own tilts, `arm · min(s₀, s₁) · (δ₀/s₀ + δ₁/s₁) ≤ arm · (δ₀ + δ₁)`. It needs a ruling before it is built.

## 2026-11-01 (SPLITDOOR, `splitdoor/face-line-germ-lean`)

Built per the orchestrator's ruling. Every germ now carries its lean (`sectors::Leaned`): `|n_a × n_b|` for a face line from `insert.rs` `germ_dir`, `vtxfac::pierce_germ_dir` or the held pierce, and 1 for an edge, a bound or a tangent locus. `Cut`, vtxfac's `Germ` and `PlannedRun.order` carry it. Every reading of two directions in the order machinery levers by `sectors::leaned_lever`, `lever · min(lean₀, lean₁)`: `strut_order`'s side and order, `walks_after` (and through it `cut_order` and `walks_before`), and vtxfac `germ_order`'s side and apart, plus `tie_breaks_after`. The derivation is at the helper. The review probe is now `insert.rs` `a_germ_tilted_within_the_zero_sides_the_arrival_edge_as_upright`, and the germ-vs-germ row is `two_germs_tilted_within_the_zero_walk_as_upright`.
