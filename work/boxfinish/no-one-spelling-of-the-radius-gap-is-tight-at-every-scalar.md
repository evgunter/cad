---
id: no-one-spelling-of-the-radius-gap-is-tight-at-every-scalar
kind: issue
title: No single generic spelling of the radius gap is tight at f64, Interval and Sym at once: the product form is wider at Interval by (1+ρ)/(1−ρ), and is not a number where the radius's box reaches zero
status: open
opened: 2026-11-28
design: true
---


Found by the signed-radius unit's fix pass (PR 7686), measuring
spellings of `geom_core::radius_gap` after its review showed the form
PR 7370 landed, `(N − r²)/(n + r)` (`N = w·w`, `n = √N`), reading a
rounding of the squares at far scales.

**The three wants.** At `f64` the gap is `n − r` to the root form's
accuracy, exactly zero at `n = r`. At `Interval` it is no wider than
`n − r`. At `Sym` its numerator is the zero form for a point built at
`c + r·x`, with nothing known of the radius's sign.

**What each spelling gives** (`s = n + r`, `ρ` the relative half-width
of `s` over the box):

| spelling | `f64` | `Interval` width against `n − r` | `Sym` zero form |
|---|---|---|---|
| `n − r` | exact near the locus | 1 | no: `√(r²) − r` is opaque |
| `(N − r²)/s` | a rounding of `N` over `2r`: a far arc's lens is red at ε `1e-12`, 71 of 5,040 generated documents move | `×(1 + ρ)` | yes |
| `(n − r)·s/s` (shipped) | `n − r` to two roundings of the difference, 0 at `n = r` | `×(1 + ρ)/(1 − ρ)` | yes |
| `(n² − r²)/s` | the squares round, as the second row | as the second row | not run |

Measured at `Interval` (an offset `r·(cos 0.4, sin 0.4)`, `r = 0.3`
boxed `±rel·r`; the same ratios at `r = 4e6`), widths of the gap:

| rel | `n − r` | `(N − r²)/s` | `(n − r)·s/s` |
|---|---|---|---|
| 0.33 | 3.960e-1 | 5.910e-1 | 7.861e-1 |
| 0.1 | 1.2000e-1 | 1.3333e-1 | 1.4667e-1 |
| 1e-2 | 1.2000e-2 | 1.2121e-2 | 1.2242e-2 |
| 1e-3 | 1.2000e-3 | 1.2012e-3 | 1.2024e-3 |
| 1e-6 | 1.2000e-6 | 1.2000e-6 | 1.2000e-6 |

The one row that moved on width: `geom-brep`
`radial_reader_rows::a_point_built_on_a_normalized_tilt_is_not_proven_on_it_over_a_box`,
whose wide case (half-width 0.1 on a radius of 0.3) reads `[0, 0.42]`
where the second row's form read `[0, 0.32]` and the difference reads
`[0, 0.21]`.

**Why no one spelling does all three.** The zero form needs `N − r²`,
or a product that expands to it, as the numerator. `f64` needs that
numerator computed as `(n − r)·s`, the only way the difference is
taken before anything rounds. Then `Interval` pays `s/s`, which it
does not read as one. Any factor `Interval` does read as exactly one
(a literal, a quotient the scalar cancels) the symbolic tier reduces
first, leaving `√N − r`.

**No bounded enclosure over a box that reaches zero.** A norm is never
negative, so `s` holds zero only where the radius's enclosure reaches
zero or below (with a norm reaching zero too). There both quotient
forms are unbounded where `n − r` still reads an enclosure
(`geom-core` `the_gap_reads_the_difference::over_a_wide_box_the_gap_is_invalid_only_where_the_radius_reaches_zero`:
radius boxes `[0, 0.5]` and `[−0.1, 0.5]` read the whole line,
uncertified, which a decision reports as `MarginDiag(Invalid)`;
`[1e-6, 1e3]` reads an enclosure). Over any box of positive radii, however wide, the
gap is an enclosure.

On `work/boxfinish/the-two-parameter-plates-volume-refuses-on-a-junction-margin-that-is-not-a-number.md`:
the reader of `props_chart_junction_closed` that goes through the gap
is the closed form's joints, `geom-brep` `props/curved.rs`'s `wall`
reading (`w.radius_gap(radius).abs()`), whose value is a joint's flat
term in `props::closure::require_joints_closed`. **Not reproduced
here**: that this is what reads invalid in `m10_3`'s replays, and what
radius a replay hands it whose box reaches zero. The interval lane's
junctions (`interval_loop_turns`) read chart images and no gap.

**What a tight one would take.** A scalar whose value channel and
expression channel may be spelled differently for one operation: the
value as `n − r`, the expression as the quotient with the polynomial
numerator. `Sym<T>` evaluates one spelling on both today. That is new
machinery in the tier, with its own soundness argument (the two
spellings are one real wherever `s ≠ 0`).

**The tests that say it is closed.**
- The gap's `Interval` width equals the direct difference's on the
  table above's boxes.
- `at_interval_the_gap_is_within_the_boxs_relative_width_of_the_difference`
  tightens to a factor of one.
- The far lens, the 5,040-document differential and the zero-form rows
  stay as they are.
