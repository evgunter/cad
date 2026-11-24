---
id: rounded-error-fields-are-read-by-hand-outside-the-home
kind: issue
title: A Rounded's error field is read into hand arithmetic outside geom_core::running
status: open
opened: 2026-11-24
priority: P1
cost: M
---

Found by the review of `oneround/spline-control-rounding` (PR 7652), whose
own sweep could not match it: that sweep looked for value/bound tuple types,
`UNIT_ROUNDOFF` beside spline vocabulary and struct fields named
`rounding`/`error`, never for a hand formula over a `Rounded`'s field.

The shape: a `geom_core::running::Rounded`'s `.error` (and `.value`) read
into `f64`/`T` arithmetic outside the home, so the bound is propagated,
scaled or combined by hand, uncharged, where a door of `running` would
carry it. It is the shape that unit removed from `topo::spline_trim`
(`Piece::shifted`, `image_pieces`' run-end reach).

Production sites, by name (line numbers ride along):

- `geom_brep::curve_face::circle_roots`, the root slack:
  `(hi·lo.error − lo·hi.error)/swing` (`circle_roots.rs:248`).
- `geom_brep::implicit`, the seam's phase error `e_u.error + e_v.error`
  (`implicit.rs:2030`), and the charge `x.error + h.frame_error`
  (`implicit.rs:2411`).
- `geom_brep::curve_face::pose`, the extreme's noise
  `h.lo.error.max(h.hi.error)` (`pose.rs:1491`).
- `geom_brep::chart_singularity`, `reading.error` taken as a total
  (`chart_singularity.rs:1128`) and `walk.area.abs() − walk.error` as a
  margin (`chart_singularity.rs:1151`; that one is `Rounded::room`).
- `geom_brep::pcurve_cache::incidence`, the anchor's error summed by hand
  `off[0].error + off[1].error + off[2].error` (`pcurve_cache.rs:13601`).
- `geom::curves::conic`, a doubling written as a literal
  `Rounded { value: two·sq.value, error: two·sq.error }`
  (`conic.rs:577`).

The pattern that finds them: `\.error[^_a-zA-Z(]` over `crates/*/src`
outside `crates/geom-core/` (58 hits on main at this filing), read by hand:
most are other types' `error` fields (`BooleanError`, a fault's `error`,
the viewer's), or tests' oracles. It cannot match a bound copied into a
local first and combined later, or a `Rounded` destructured
(`let Rounded { value, error } = …`); a second pass should grep those
destructurings.

Each site is one of: a door that exists (`room`, `magnitude`,
`with_charge`, `+`), a door `running` lacks (a scaled bound, a sum of
bounds as one), or a quantity that is not a rounding and should not be a
`Rounded` at all. P1 by `work/README.md` Priority (one logic, the
propagation of a rounding bound, written in several places); cost M, by
the count and by each site needing its own reading.
