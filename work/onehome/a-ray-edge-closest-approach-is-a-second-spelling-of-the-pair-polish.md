---
id: a-ray-edge-closest-approach-is-a-second-spelling-of-the-pair-polish
kind: issue
title: solid_contain's curved-edge passes spells a ray x edge closest approach by alternating feet, a second spelling of the curve-pair pair polish, read off the sampled spline foot
status: open
opened: 2026-11-16
priority: P2
cost: M
---

Found by the walk-stop fork's designers (2026-11-16); checked against main
`2c17deabf`.

`passes` (`crates/topo/src/solid_contain/curved_edges.rs:338`)
carries a seed near the edge's carrier to the ray's closest approach by
alternating feet, `FEET = 8` rounds (`curved_edges.rs:66`, `:361-364`):

```rust
for _ in 0..FEET {
    near = carrier.foot(r, near).seed();
    r = ray.foot(carrier.eval(near));
}
```

That is a closest pair of a line and a curve, a second spelling beside the
curve-pair polish (`polish::pair_held`,
`crates/geom-brep/src/curve_pair/polish.rs:69`; the unheld `polish::pair`
it was beside went in PR 7137), and it reads
`Curve3::foot`, whose spline foot is sampled
(`crates/geom/src/curves/on_carrier.rs`, `work/onehome/a-spline-foot-misses-its-true-minimum-on-a-fifth-of-random-draws.md`).
A fixed eight rounds with no convergence reading is a budget, not a
decision.

## Done when

The ray × edge closest approach reads through the one pair polish (or the
one it becomes), and its stop is decided.

**Read the walk-stop fork first.** The open design fork `work/onehome/the-curve-pair-walks-stop-verdict-rests-on-which-heuristic-foot-it-polishes.md` may change
the fix; whoever takes this row reads that fork's outcome before writing
anything.

**Two more spellings of the clamp** (noted by the review of PR 7048, fork 275 step 3). A pair polish held to its runs is spelled twice besides this row's:
- `Curve3::foot`'s `near.max(lo).min(hi)` (`crates/geom/src/curves/on_carrier.rs`);
- `polish::pair_held`'s order-agnostic clamp (`crates/geom-brep/src/curve_pair/polish.rs`).

The fold-in pass should take all three to one.
