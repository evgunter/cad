---
id: from-computed-claims-a-zero-tangent-at-a-dual
kind: issue
title: Real::from_computed returns tangent zero at a dual, so a re-entered value that moves with the inputs yields a plausible wrong sensitivity
status: parked
opened: 2026-11-19
priority: P1
cost: H
blocked_on: [7155]
refs: [dl5-has-no-arm-for-a-selection-whose-result-is-the-payload, census-rows-whose-bracket-read-re-enters-the-scalar-are-filed-as-payload-or-selection]
---


Filed by the SCALAR orchestrator, 2026-11-20. The build of DL5.2
(`crates/geom-core/README.md`, "Reading a bracket where nothing
certifies"), parked on Ev's answer to PR 7155.

## What

`Dual::from_computed` is `Self::constant(T::from_computed(x))`:
tangent zero, the claim that the value does not move with the inputs.
`Sym::from_computed` is an opaque unknown. `Real::from_computed`'s own
doc says a scalar that defaulted it to `from_f64` "would re-enter
every computed value as a constant", and at a dual that is what it
does.

A survey of the 157 calls outside tests (read, not executed; counts
and lists in `work/scalar/log.md`, 2026-11-20): 57 a dual cannot
reach, about 60 only decided on or reported, 9 constants, about 10
flat to first order by a comment's argument, 10 wrong, 10
undetermined. The wrong one on the path production runs is ATREST's
`link-lowest-on-freezes-its-candidates-so-a-dual-sensitivity-through-a-tie-drops-a-term`.

## What closing it takes

A second count (two designers, round 7 and 8 of the fork; read, mostly
not traced to consumers) puts 114 lines, 128 calls, in a dual's reach
by the enclosing function's own bound. By what the `f64` feeds: about
42 bounds decided against, 55 places evaluated at, 9 solved values the
result is built on, 10 free choices, 13 lifts of `f64`-built data, 3
constants. Every unit re-sorts the sites it touches.

- **The backstop**: `from_computed` at a dual returns an unknown
  tangent: the degraded tangent `copysign` gives at a kink
  (`KinkJacobian`), NaN over `f64` and the whole line over an
  enclosure. First or last; first makes the rest loud.
- **Bounds stop being `T`**: a pure `f64` bound is decided at `f64`
  (`pcurve_cache/projected.rs` `hull_sector` lifts `x_lo` only to call
  `holds`); a bound subtracted from a `T` quantity is an allowance
  term of the margin. `Margin::<f64>::lift` is itself a re-entry and
  `Margin::value` hands the `T` back, so it is not the door.
- **The solve door**, on `Real`, declared and never defaulted: the
  `f64` answer and the residual at `T` in, the same value out with the
  implicit-function tangent at a dual; identity at `f64` and
  `Interval`, opaque at `Sym`, the unknown tangent at
  `Dual<Interval>` and at a singular slope. About 9 sites
  (`curve_face/composite.rs` and `spline_face.rs` `settle`,
  `replace_face.rs` `corner_of_root`, both polishes).
  `topo::Link::lowest_on` needs none: it keeps the `T` roots it
  already computes.
- **The zero door**: zero at a dual, opaque at `Sym`, every use on a
  gated list with its kind and pinned against finite differences
  (box centres, patch splits; 10 to 14 sites).
- **`f64`-built data** is built at `T` or lifted behind
  `CertifiedEnclosure` (the loft's enclosure arm, already owed as
  `launders` in `from-f64-sites.tsv`; `on_carrier_domain`; `skin.rs`).
- **Certification interiors take the certifying bound** (DL5.3):
  `pcurve_cache.rs`, `pcurve_cache/projected.rs`,
  `spline_incidence.rs`, most of `geom-brep/src/certify.rs`;
  `carrier_span`, fenced today only by its seven callers. Each item
  needs checking that no construction caller instantiates it.

## Check before building (nobody has)

- DL2 memo keys hash tangent bits, so the unknown tangent's NaN must
  be canonical (`ContentBits for f64`).
- No dual-path code may branch on a tangent's finiteness (E9).
- Whether the whole line or NaI is what E7's consumer reads as "do not
  prune" at `Dual<Interval>`.
- How many stackup sensitivities turn unavailable when it lands: the
  "decided only" bulk was classified by reading. The dual corpus and
  `blend_dual_sensitivity` show it.
- Whether a pure-`f64` bound decided at `f64` gives the verdict it
  gave decided at `T` at every scalar.
- `props.rs` `volume_lo`/`volume_hi`/`surface_area` hold bracket ends
  in `T`-typed fields of a reported `VolumeEnclosure<T>`: whether the
  stackup reads them.
- `Interval` division of an exact zero by a slope enclosure, for the
  solve door's bit identity.
- Re-entry that avoids the door: `Interval::point`, the `launders`
  rows of `from-f64-sites.tsv`, and a `value_channel` map followed by
  evaluation at `T` (`SplineWall::outward`, which also goes round
  `project`'s `CertifiedBounds` door).
