---
id: value-error-pairs-are-carried-beside-rounded-in-four-modules
kind: issue
title: Four modules carry (value, error) pairs of their own beside Rounded and spell room and magnitude on them by hand
status: review
opened: 2026-11-19
priority: P1
cost: M
branch: oneround/rounded-pairs
pr: 7221
---


Found by the sweep of `oneround/rounded-room` (shape: a `.abs() - <error>`
or `.abs() + <error>` read off a value/error pair, and `(value,
error)`-shaped tuples beside `Rounded`), which closed that shape in
`geom_brep::half_angle`. Each module below carries a value and its error
bound as two names of its own, a vocabulary parallel to
`geom_core::running::Rounded`, and spells `room` and `magnitude` on them:

- `geom_brep::curve_pair::pose::window` (`taylor`): takes `[T; 4]`
  values and a `[T; 4]` of errors and builds `Bounds` as `v.abs() - e0 -
  fall`, `d1.abs() - e1 - turn`, `d1.abs() + e1`, …. It is the Taylor
  bound `half_angle`'s walk reads (`half_angle`'s doc says so), a second
  implementation of it. Every production caller passes a zero error
  array (`[z0; 4]`, `[T::zero(); 4]`), so the error argument is carried
  and never charged; its `lo` and `slope_lo` are not clamped and are
  read through `positive(row, …)`, which a negative fails.
- `geom_brep::pair_section::gap` (`Reading` with its `err: Rounding<T>`):
  `r.g1.abs() - r.err.g1 - (r.g2.abs() + r.err.g2) * w - …`, and
  `r.d + r.err.d`. The least slope goes to `Margin::levered` through
  `slope`, not into a division by itself.
- `geom_brep::intersect` (the plane × cone apex arm and `pair_bound`):
  `(gap, gap_err)` and `(c, c_err)` tuples, `size = gap.abs() +
  gap_err`, and `undecided = (gap.abs() - gap_err).min(c.abs() -
  c_err)`, two rooms read through `select_le_zero`.
- `geom_brep::curve_face::conic_quadric` (`conic_quadric.rs`, the cone arm's root slack):
  `least = slope(theta).abs() - two * noise - width * bend` and
  `(f(theta).abs() + noise) / least`, the division guarded by
  `least.select_le_zero(unbounded, …)`.

Two more, found by the review of PR 7145, the first inside the blind
spot stated below (so the title's "four" undercounts):

- `geom_brep::implicit` (the sphere × circle harmonic,
  `CircleSphereHarmonic`): two `Rounded`s are unpacked into `lo` /
  `lo_error` and `hi` / `hi_error` fields, and the ends are read by hand
  (`h.lo - h.lo_error - h.frame_error`, `h.hi + h.hi_error +
  h.frame_error`).
- `topo::vertex_section` (`read_sections`): a `Rounded` is rebuilt off
  another's fields, `Rounded { value: offset.value.value, error:
  offset.value.error + skew(…) }`, to add a charge before the quotient.

The same review saw three restatements beside them, of one class with
this row (one logic written out per module):

- the bisection count: `half_angle`'s `BISECTIONS = 64` and
  `pair_section::gap`'s `GAP_BISECTIONS = 64`;
- the Taylor ladder's factorials (2, 6, 24), written out in
  `half_angle`'s `subdivision`, `pose::window` and `pair_section::gap`;
- `pair_section::gap` building a throwaway `Rounded { … }` to borrow
  `toward_zero`.

Not read, and where this row's sweep is blind: a bound held under a
field or variable of another name and subtracted through an
intermediate (`noise`, `rounding`, `lo_error` / `hi_error` fields in
`geom_brep::roots`, `curve_face::circle_roots`, `implicit`,
`carrier_net`, `pcurve_cache`, `topo::boolean::section_cert::ruling`).
A pass over fields of those names lists them
(`rg '(err|error|noise|rounding)\w*: (T|f64)' crates/*/src`); their
readers were not walked.

`editor-core::resolve::pick`'s `quotient` divides by `det.abs() -
bound_det` on `(f64, f64)` pairs; `certify` returns `None` unless
`det.abs() > bound`, so the room is positive there, and
`site-rounding-charges-owed-a-re-posing` already schedules the file's
re-posing. `geom_brep::ssi::compensated` is a double-double with its own
bound and a guarded division; an error-free transformation, not this
row's.

Fix: carry each as `Rounded<T>` and read it through `room()`,
`magnitude()` and `divisor()`; `window::taylor` and `half_angle`'s walk
then share one Taylor bound. Each of these files is also listed by
`site-rounding-charges-owed-a-re-posing`, so a re-posing that removes a
charge removes its pair with it.

P1 by `work/README.md` Priority (two implementations of one underlying
logic).
