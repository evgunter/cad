---
id: a-near-collinear-kink-on-a-full-revolve-refuses-as-a-kernel-defect
kind: issue
title: revolve: a near-collinear kink (15–60ε) that validates makes the full revolve refuse at the Euler attachment gate as a kernel defect
status: open
opened: 2026-10-25
priority: P2
cost: M
---


Found by the review of PR 5707 (PRED, the full revolve's axis cavities),
reproduced on main `4a44083275`. It is not that PR's: there is no pocket,
just one axis run.

## What happens

A single-run profile with a near-collinear kink validates, and its full
revolve then refuses as a kernel defect. The kink is a slightly tilted
bottom edge followed by the short and long legs of the outer wall:

```rust
let eps = revolve_common::eps(); // 1e-9 at the default row
let pts = vec![
    Point2::new(0.0, 0.0),
    Point2::new(1.0 + c * eps, 0.0),
    Point2::new(1.0, 1.19),
    Point2::new(1.0, 1.2),
    Point2::new(0.0, 1.2),
];
// Profile::new(SketchPlane::xy(), vec![ProfileLoop::polygon(pts)])
//     .validate(Tol::witness())  -> Ok
// revolve(&vp, axis_y(), Revolution::Full, Tol::witness())
```

- `c = 15`: "an Euler operation refused: geometry attachment gate: the
  unified conventional residual at sample 0 definitely exceeds the
  tolerance band (the cache does not represent the description, D4 ¶2).
  There is no way through: this is a kernel defect; report it".
- `c = 60`: the same gate, with the residual escalated inside the band
  (margin 1.34e-9 in (1e-9, 1e-8)).
- `c = 150`: validation refuses (a chord-side decision in band), as it should.

## Where

The gate raising it is `topo::euler`'s geometry attachment gate
(`crates/topo/src/euler.rs`). It is doing its job: the description it is
handed does not match the cached curve. The likely cause is on this
slate. The cosurface verdict (`swept::cosurface`, read by
`revolve/runs.rs::collapse_runs`) joins segments 1 and 2 into one run
within its margin. The collapsed segment then keeps the run's first
carrier and its stored last endpoint (`Traversed::continued`'s doc: the
two differ "by at most that margin plus rounding"). The line from
`(1 + cε, 0)` to `(1, 1.2)` therefore does not run where the first
leg's line does, and the wall or rim minted from it fails attachment.

The cosurface margin of two lines also depends on direction: it reads
the next chord's far end against the previous chord's line. So the
same kink can be one run in one traversal and a corner in the other.
PR 5707 makes the full revolve's pockets reuse the outer loop's one
collapse, so its own rows do not depend on that asymmetry.

## Owed

Either a run whose collapsed chord leaves its first carrier by more than
the band is not collapsed (it stays a corner, which tier 3 may then call
a sliver, typed), or the collapse refits the carrier to the stored
endpoints. Either way, a profile that validates should not reach a
"kernel defect" refusal here.
