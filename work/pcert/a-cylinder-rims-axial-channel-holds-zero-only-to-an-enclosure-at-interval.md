---
id: a-cylinder-rims-axial-channel-holds-zero-only-to-an-enclosure-at-interval
kind: issue
title: A circle's image on a cylinder's chart carries its axial cos and sin terms as raw dot products, so at Interval a rim's height holds zero only to an enclosure where the cone arm decides it
status: open
opened: 2026-10-30
priority: P3
cost: M
---



## Finding

Found by CLEAR's CAVITYDIAG. `geom_brep::pcurve_cache::derive_class`'s
cylinder arm writes the axial channel of every carrier as
`Channel::Free { c: w·axis, a: form.a·axis, b: form.b·axis, l: form.l·axis }`,
in both the meridian and the moving case. For a rim circle (its plane
⊥ the axis) `a` and `b` are zero, but at `Interval` they are enclosures
of zero: on the corpus pot's foot rim, `a ∈ [−6.9e−18, 6.9e−18]`, the
rim's `u_ref` enclosing `y ∈ ±1.1e−16` against the point axis
`(0, 1, 0)`. At `f64` they are `0.0`.

The cone arm beside it decides the same question at
`pcurve_cone_chart_axial` (`Margin::of(harmonic_sup_scalar(aa, ba))`)
and writes `Channel::constant(v0)` for a rim; the cylinder arm has no
such decision.

Every reader that tests the axial terms exactly reads a cylinder rim
as a line at `f64` and as a curve at `Interval`. CAVITYDIAG fixed its
own consumer at the reader (`topo::clearance::line_of` and
`topo::injective::Use::iso`, the `self_intersection_level` site),
because `pcurve_cache.rs` is this program's ground. Readers that
still test them exactly:

- `topo::clearance::monotone_line` (an extent row read as bulging, a
  sound superset);
- any other `Channel::Free { a, b, .. }` zero test; a sweep for
  `zero(a) && zero(b)` over `crates/topo/src` finds the one above.

## The fix at the home

A rim decision in the moving arm, as the cone's: decide the axial
`cos t` and `sin t` terms at a named site and write
`Channel::constant(c)` where they are `Zero`. Gate it on the
enclosures holding zero, or `f64` pcurves move for every cylinder rim
whose axial terms round to a nonzero value inside the band (oblique
pipes). With it in place, `line_of`'s amplitude arm is dead, and can
come out with its concession.
