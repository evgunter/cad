---
id: inline-bracket-midpoints-are-spelled-in-two-forms
kind: issue
title: Inline bracket midpoints are spelled in two forms, lo + ½(hi − lo) and ½(lo + hi), with one home for the first
status: open
opened: 2026-11-04
priority: P3
cost: M
---

Found in the delta review of PR 6516 (`onecopy/meet-interval`).

## The class

A bracket's midpoint is spelled inline across the tree in two forms:
- `lo + ½(hi − lo)` (also written `lo + (hi − lo)·0.5`). Its home is
  `geom::bracket_mid` (`crates/geom/src/projection_policy.rs`), which
  `projection_policy::mid`, `curve_pair::polish`'s `place` and
  `geom_brep::krawczyk::midpoint` read. Inline copies remain, for example
  `geom-brep/src/props/quad/rotational.rs`, `props/taylor.rs`,
  `certify.rs`, `curve_face/composite.rs`, `ssi/pair.rs`,
  `mesh/src/walk.rs` and `mesh/src/validate.rs`.
- `½(lo + hi)` (also `(lo + hi)·0.5` and `(lo + hi)/2`), with no home.
  It is spelled in `ssi/enclose.rs`, `ssi/march.rs`, `ssi/one_arc.rs`,
  `ssi/section.rs`, `pair_section/ruling.rs`,
  `pair_section/spline_implicit/sweep/critical.rs` (`mid`),
  `circle_roots.rs`, `topo/src/clearance.rs`, `editor-core/src/analysis.rs`,
  `step-import/src/knot_split.rs`, among others.

## Census (2026-11-04, on `onecopy/meet-interval` `ed74313f07`)

```
A='0\.5 \* \([a-z_.()]*lo[()]* \+ [a-z_.()]*hi|\(([a-z_.]*)lo(\(\))? \+ [a-z_.]*hi(\(\))?\) \* 0\.5|\(([a-z_.]*)lo(\(\))? \+ [a-z_.]*hi(\(\))?\) / 2'
B='lo(\(\))? \+ \(?[a-z_.]*hi(\(\))? - [a-z_.]*lo(\(\))?\)? \* 0\.5|lo(\(\))? \+ 0\.5 \* \([a-z_.]*hi(\(\))? - '
rg -n --type rust "$A" crates   # ½(lo + hi): 66 in src, 80 in tests
rg -n --type rust "$B" crates   # lo + ½(hi − lo): 9 in src, 9 in tests
```

*Blind spots:* names other than `lo`/`hi` (`a`, `b`, `t0`, `t1`,
`min`, `max`), a midpoint split across lines, and `mid_param`-style
helpers. Not every `A` hit is a bracket's midpoint: some are the middle
of two parameters or two points, which is a different fact and may
keep its own spelling.

## The bit question

The forms are not interchangeable. On a finite bracket they can differ
in the last bit, and `½(lo + hi)` overflows where `hi + lo` does
(`(1.5e308, 1.6e308)` gives `+∞`), where `lo + ½(hi − lo)` overflows
only when the width does. So moving a `½(lo + hi)` site onto
`bracket_mid` can move `Interval` bits and f64 results, for example a
Krawczyk box, a march step or a subdivision split, and needs that
crate's suites run at every ε row before and after.

## Asked

Give each bracket midpoint one home. That is `bracket_mid`, or a named
second form where a site's bits or overflow behaviour are load-bearing
(say why at the form). Move the `lo + ½(hi − lo)` inline copies onto
`bracket_mid` first, since that move changes no bit. Then weigh the
`½(lo + hi)` sites one crate at a time against their suites.
