---
id: a-bracket-middle-is-also-spelled-half-lo-plus-half-hi-and-f64-midpoint
kind: issue
title: A bracket's middle is also spelled ½lo + ½hi and f64::midpoint, beside its two homes
status: open
opened: 2026-11-09
priority: P3
cost: M
---


Found while closing
`inline-bracket-midpoints-are-spelled-in-two-forms` (branch
`onecopy/bracket-mid`). That row homed and gated the two hand
spellings: `½(lo + hi)` goes to `geom_core::mid_param` (re-exported
as `geom::mid_param`), `lo + ½(hi − lo)` to `geom::bracket_mid`, and
`scripts/gates/bracket-mid-by-hand.sh` refuses both. Two more forms
remain, and the gate names both as blind spots.

## Form 3, `½lo + ½hi` (4 production sites)

- `geom::net::middle` (`crates/geom/src/net.rs`), with a stated reason:
  "halved before the sum so it cannot overflow".
- `geom_brep::intersect::section_bounds::witness` and `miranda`'s
  `mid` closure (`crates/geom-brep/src/intersect/section_bounds.rs`).
- `<Interval as SolveScalar>::quoted` (`crates/editor-core/src/eval/mod.rs`).

These give the same bits as `mid_param` wherever neither half is
subnormal and `lo + hi` does not overflow.

## Form 4, `f64::midpoint` (about 20 production sites)

`geom_core::ParamRange::mid` (`crates/geom-core/src/spline/range.rs`),
`geom_brep::props::quad::{trimmed, net, rational, cells, area}`,
`geom_core::k_stats`. std's form is correctly rounded and finite for
every finite pair. It gives the same bits as `mid_param` wherever both
ends lie within `f64::MAX / 2`.

## Asked

Decide whether a form-3 or form-4 site's overflow behaviour is
load-bearing (say so at the form), or route it to `mid_param`, then
gate the spelling. The census is
`grep -rnE '0\.5 \* [a-z_.()]+ \+ 0\.5 \* |\.midpoint\(' crates/*/src`.
Not every `.midpoint(` is `f64`'s: `editor-core`'s
`BoxAxis::midpoint` and `ParamBox` leaves are the axis door, which
already reads `mid_param`.
