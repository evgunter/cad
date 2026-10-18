---
id: the-curve-pair-line-line-crossing-is-read-from-the-carriers-origins-uncharged
kind: issue
title: curve_pair's line_line places a crossing by the closed form from the two carriers' origins, with no rounding charge: its f64 error is about |o2 − o1|·u / sin φ
status: open
opened: 2026-10-18
priority: P2
cost: E
---


Found by RESTREAD's sweep for closed forms read from an arbitrary
origin (the branch `restread/no-shared-key-crossing`, which fixes the
census's own instance in `crates/topo/src/census.rs` `ee_lines_closest`).

## What

`crates/geom-brep/src/curve_pair/pose.rs` `line_line`, its `crossing`
closure: `along = (o2 − o1).cross(u2).dot(n) / s²`, read from the two
line carriers' origins `o1`, `o2`, which can stand anywhere on their
lines, metres from either span. `n = u1 × u2` cancels to an absolute
error of about `u`, and the reach's `f64` rounding is about
`(|o2 − o1| + |along|)·u / s`. RESTREAD measured the census's copy of
this form: at ε = 1e-12, 1 456 of 100 000 true crossings (read in both
orders) placed a definite span outside one edge. Nothing here charges
that rounding before `ta`/`tb` are read against the spans.

## Not traced

Whether `polish.rs` re-meters a generic candidate's place before a
verdict reads it, which would make this only a seed. The census's fix
reads each reach from the ends nearest it and charges
`geom_brep::rounding_charge((|e| + |reach| + gap/s)/s)`, deciding both
ends of the enclosure; the same shape would serve here.
