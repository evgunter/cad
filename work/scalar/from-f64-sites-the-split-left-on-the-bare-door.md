---
id: from-f64-sites-the-split-left-on-the-bare-door
kind: issue
title: from_f64 sites the split left on the bare door: constants whose holder is an f64, and two that are none of the four
status: open
opened: 2026-11-24
priority: P2
cost: M
parent: from-f64-is-split-by-where-the-number-comes-from
---


Found by `from-f64-is-split-by-where-the-number-comes-from`, which built the four doors a constant enters by (`geom_core::Lit` and `Real::lit`, `Real::count` and `Real::count_ratio`, `geom_core::BandWidth`, `editor_core::expr::authored`) and moved the sites whose source the call site itself shows. Each site below still calls `Real::from_f64` and is listed `owed` to this row in `scripts/gates/from-f64-sites.tsv` with its function; its entry goes when the site does. The rule is `crates/geom-core/README.md` DL5.2.

Constants of one of the four sources, not moved because the number reaches the site as an `f64` some other code holds, so moving it means changing that holder's type:

- A count held as `f64`: `geom_core::spline::basis::ders_rows` (`factor`, a falling factorial accumulated in `f64`), `spline::derivative` `Vector::times` and `over` (`k`, a product of `binomial`s, which returns `f64`), `spline::projective::form_coef` (`Coef::Constant { num, den }`, `f64` fields), `geom::curves::nurbs::rational_corrections` (`binom`). The move is `binomial` and those fields as integers, entered through `Real::count`; an integer product can overflow where the `f64` one rounds, so the type is chosen with `QUOTIENT_MAX_DEGREE` in hand.
- A count ratio held as `f64`: `sweep::skin::section_parameters` (`shares`, a path sweep's `i/(stations − 1)`, a slice of `f64`).
- A table of numbers the source wrote, held as `f64`: the ray schedules (`topo` `chart_region::point_in_polygon`, `solid_contain::point_in_faces`, `sphere_region::contains`, `splitting::containment::walk_schedule`, `splitting::order::in_plane_frame`, each `r.map(T::from_f64)` over a `const` table of `Vec3<f64>` that `ray_walk::walk` also reads at `f64`), and the Gauss rule of `geom_brep::props::loop_area::nurbs_vector_area`. The move is the table as `Lit`s (`half_angle::SPLITS` and `region::TARGET_SHARES` moved this way).
- A constant computed at run time: `topo::side_reader::aimed_ray` (`aim_turn()`, the golden angle's sine and cosine from `sin_cos`, which is no `const fn`), `profile::validate::point_in_loop` (`RAY_BASE + k·RAY_STEP`). Each is a `Lit` once its values are written out, or a table built in a `const`.
- A constant handed down a helper as `f64`: `profile::seg` `resolution_headroom` and `read_back` (`ulps`, an ulp count every caller writes as a literal), `topo::solid_contain::along_half` (`f`, a fixed fraction), `geom::surfaces::nurbs::placeholder` (the unit knots, through `Knots::lift(T::from_f64)`).
- A band, or a knob's override of it: `topo::separating::apart_along` (`2.0 * pad`). Every production caller passes `boxes::sweep_pad(band)`, and `boolean::reduce` passes `knobs.pad_override` where a test sets one, so the parameter is not a `BandWidth` until the knob is one too.

None of the four, and on no other row:

- `geom_core::Real::scale_pow2`'s default (`self * Self::from_f64(k)`): a helper that enters whatever `f64` its caller hands it. Its callers pass a subdivision's `2^-r`; the signature admits any number.
- `geom::curves::banded::solve` (`l`): a dropped term's signed zero, an `f64` read off the pivot on the value channel and re-entered so the product keeps its sign.

Cost: M. No site here is a known wrong answer; each is a constant the list holds by a reviewer's word where a signature could.

## A run option's derived frame (PR 7721's fix pass)

`editor-core/src/eval/mod.rs` `ComputingTwist::at` (four sites: an axis's three components and an angle; the translation goes through the same `lift`): the frame `ComputingTwist::for_node` computes at run time from a node's ordinal and mint digest. The PR first moved them onto `expr::authored`; both reviews read them as not authored, and they are back on the bare door, `owed` here. They are a constant of the run and not of the document, so a literal at `Sym` is true of them; which class that is, is question 5 of `the-from-f64-split-leaves-five-questions-dl5-2-does-not-settle`.
