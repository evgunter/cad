---
id: a-sign-change-bisection-is-written-once-per-crate
kind: issue
title: A sign-change bisection is written once per crate, not once
status: open
opened: 2026-11-21
priority: P2
cost: E
---


Found by the review of PR 7221 (`oneround/rounded-pairs`), which gave
`geom_brep::half_angle`'s and `pair_section::gap`'s bisection one home in
`geom_brep::piece::sign_change` (`pub(crate)`, 64 halvings). That home
serves geom-brep only. A sign-change bisection is still written out at
least here:

- `topo::offset_derive::bisect`: `f64`, 64 halvings, stops early where
  the midpoint stops moving, and returns the end nearer zero;
- `geom_brep::curve_face::composite::bisect_row`: its own
  `BISECTIONS = 64`, stops early, and falls back to the residual where
  the Bernstein row reads no sign;
- `geom_brep::ssi::march` (the leaving step, `SSI_LEAVING_BISECTIONS`):
  a predicate bisection rather than a sign change.

Three of the sites the review named are not bisections, and are not this
row's: `geom_core::spline::compose::patch`'s two 64-step loops
(`min_norm_point_iterates`, `frank_wolfe_iterates`) iterate an optimiser,
`topo::props::face_reading_with` walks a round schedule, and the
`pose::window` test `the_half_width_covers_a_piece_one_ulp_wide_far_from_zero`
walks one ulp at a time.

A shared home could live beside `geom_core::bisect` (the piece walker
every walk already reaches): a `sign_change` over a `Real` closure, with
its count and with the early stop the `f64` sites need as a parameter.
`piece::sign_change` would then re-export it. Where a site differs on
purpose (composite's residual fallback, march's predicate), the row
should say so beside the home rather than fold it in.

P2: no answer disagrees; one logic, several spellings.
