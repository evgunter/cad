---
id: sketch-segment-eval-could-be-exact-at-both-ends
kind: issue
title: SketchSegment::eval could be bit-exact at both ends (a comparison-free blend); it is exact only at s = 0
status: open
opened: 2026-09-26
---


Found in #3254's dual review (R1, claim C6); filed on the coordinator's
ruling. Not adopted there.

## What stands

`geom_brep::SketchSegment::eval` (`crates/geom-brep/src/mapped.rs`) is
the a-anchored rotation `a + (R(s·Δθ) − I)·(a − centre)`. At `f64` it
returns `a` bit for bit at `s = 0`, and at `s = 1` it returns `a`
turned by the whole sweep, which misses the stored `b` in the last bits.
R1 measured `eval(1) ≠ b` on 379 of 398 arcs.

#3254's first cut said an exact end at `s = 1` was "not possible" over
`Real`, which has no comparison. That was wrong. The comparison-free
blend

```text
eval(s) = (1 − s)·rot_a(s·Δθ) + s·rot_b((s − 1)·Δθ)
  rot_a(φ) = a + (R(φ) − I)·(a − centre)
  rot_b(φ) = b + (R(φ) − I)·(b − centre)
```

is `a` exactly at `s = 0` and `b` exactly at `s = 1` at `f64`. R1
measured it exact on all 398 of the same arcs.

## Why it was not chosen

- **Interval.** The blend evaluates two rotations and sums their
  enclosures, where the anchored form carries one. The anchored form was
  chosen for its width at the endpoints' scale
  (`geom-brep/tests/arc_eval_anchor.rs`, `review_arceval_r1_probes.rs`).
- **Sym.** The blend builds a second rotation's nodes at every
  certification sample.
- **Endpoint authority** is held by the vertices, and certification
  meters `eval` against the carrier at every sample, `s = 1` included.

## What is owed

Measure the blend's Interval widths against the two anchor rows and its
Sym cost on the m10 pins, then decide.
