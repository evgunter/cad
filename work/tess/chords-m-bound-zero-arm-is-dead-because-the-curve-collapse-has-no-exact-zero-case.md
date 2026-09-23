---
id: chords-m-bound-zero-arm-is-dead-because-the-curve-collapse-has-no-exact-zero-case
kind: issue
title: chords' m_bound == 0.0 fast path is dead: the curve sup collapse has no exact-zero case, so next_up turns the exact zero into 5e-324
status: open
opened: 2026-09-22
---


Found by TESS-3's sweep for exact-zero decisions on a certified sup in
`crates/mesh`, at the merge base of `tess/3-exact-zero`.

## The defect

`nurbs_cert::cell_component` — the surface bound's `Σ sup² → sup`
collapse — short-circuits `hi == 0.0` to exactly `0.0`, precisely so
that the split selection's `== 0.0` degenerate-direction arms are
reachable. `chords.rs` has the same collapse twice, one dimension down,
and **neither copy has that case**:

- the integral carrier arm, `nurbs_chord_count`: `sum_sq.hi().sqrt().next_up()`;
- the rational carrier arm, `rational_carrier_m_bound`: `s.hi().sqrt().next_up()`.

`0.0_f64.sqrt()` is `0.0` and `next_up(0.0)` is `5e-324`, so a carrier
whose certified `sup|C″|` enclosure is the exact `[0, 0]` reports
`m_bound = 5e-324`. The fast path twelve lines later —

```rust
if m_bound == 0.0 {
    return Ok(1);
}
```

— is therefore a predicate no runtime value can make true, which the
implementer discipline (§2, "write assertions a bug could break")
calls documentation rather than a decision.

The zero IS reachable: a collinear degree-2 carrier with exact
coordinates (control points `(0,0,0), (1,0,0), (2,0,0)` on a clamped
degree-2 knot vector) has exactly-zero second differences, and
`RingInterval`'s backend keeps `0 + 0` and `0²` exact since SCALAR's
RING-2, so `sum_sq` is `[0, 0]`. TESS-3 measured the surface analogue
of exactly that and found the exact zero surviving to `cell_component`.

## Why it is not urgent

The ANSWER is unchanged. `curvature_step(δ_s, 5e-324)` is ~4e160, and
`ceil_count` over an O(1) span gives `1` — the same count the fast path
would return. So this is a dead arm and a missing invariant, not a
wrong mesh.

Note that a degree-1 carrier never reaches either site:
`nurbs_chord_count` decides `p < 2 ⇒ Ok(1)` from the knot vector, the
never-infer doctrine's answer, before the ring is touched. What is left
here is the degree-≥2 carrier whose net is straight.

## The repair

Route both collapses through one exact-zero-preserving function
instead of two open-coded `sqrt().next_up()`s — `cell_component` is
already that function and is private to `nurbs_cert`. TESS-3 did not
take it: the change touches CHORD's certified assembly in a second
module with its own chord-count inventory, and wants its own
measurement of which counts move (none should). A row that would go
red on the defect: a collinear degree-2 carrier whose `m_bound` is
asserted `== 0.0`, with the count pinned at `1`.
