---
id: the-fresh-body-cylinder-sheet-wrapper-is-written-six-times
kind: issue
title: The fresh-body cylinder-sheet wrapper is written six times across two probe suites
status: open
opened: 2026-09-20
priority: P4
cost: E
---


## Finding

- **Where**: `crates/topo/tests/mate5_cyl_eps_rung.rs` — `sheet_a`,
  `sheet_b`, `sheet_b_r` and the two closures `tilted` / `small` inside
  `one_axis_tilt_two_levers_two_answers` — and
  `crates/topo/tests/r1_mate5_probe.rs`'s `sheet`.
- **Importance**: low — six copies of a four-line wrapper, no verdict
  rides on it
- **Confidence**: sure; all six read source-to-source at merge base
  `cd9fdfd6b`
- **Raised by**: the `dup/src-cyl-sheet` lane, 2026-09-20, from the
  reviewer's instruction to census the direct `cyl_wall_sheet(` sites
  once the `&mut Body` adapter had a home

The shape is one thing, six times:

```
fn X(frame, src, u0, u1, v0, v1) -> (Body<f64>, FaceKey) {
    let mut body = Body::<f64>::new();
    let f = cyl_wall_sheet(&mut body, frame, Some(src), (u0, u1), (v0, v1), Tol::witness());
    (body, f)
}
```

It is **not** the adapter PR #2925 homed. That one takes `&mut Body`
and returns a `FaceKey`; `crates/topo/tests/probe_support/mod.rs`'s
`wall_sheet` is its one spelling and `r1_mate5_probe.rs`'s `sheet` is
now three lines over it. This class is the *fresh-body* wrapper, which
returns the body it made, and the five in `mate5_cyl_eps_rung.rs` still
spell the door's whole argument list themselves.

## What a unit here owes

Decide whether the fresh-body shape gets a home beside `wall_sheet` in
`probe_support`, or whether returning the body is enough of a per-suite
judgement to leave. Two things a taker should have in hand:

- **`mate5_cyl_eps_rung.rs` was out of the finding lane's fence**, so
  the five there were read and not touched. Nothing is known to be
  wrong with them.
- **The `interval_lane` pair in that file is NOT in this class.**
  `a_half_period_tie_declines_period_fold` builds `Body<Interval>`, and
  both `wall_sheet` and this shape are `f64`-only; a shared home that
  wanted them would have to be generic over the scalar, which
  `cyl_wall_sheet` already is and these wrappers are not.

## Why this row is not on the territory owner's slate

`scripts/work.py territory` reads both suites as `tcost`'s and
`tint`'s. The finding is a wrapper written six times, which is this
program's charter; it is filed here so it sits with the rest of the
cylinder-sheet class, and either owner may claim it.
