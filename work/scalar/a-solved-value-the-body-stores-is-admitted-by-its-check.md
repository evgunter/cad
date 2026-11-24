---
id: a-solved-value-the-body-stores-is-admitted-by-its-check
kind: unit
title: A solved value the body stores is admitted by its check, through one door that is from_computed's only caller
status: dispatched
opened: 2026-11-20
priority: P1
cost: H
parent: from-computed-claims-a-zero-tangent-at-a-dual
---

The rule is `crates/geom-core/README.md`, DL5.2's two paragraphs after the table (Ev, PR 7176). Dispatched 2026-11-23 (letter H, dual review: one door every stored solved value passes, and a gate that makes it `from_computed`'s only caller; later code is written against both). Both units it follows have merged (PR 7190, PR 7250). It follows `certification-interiors-are-closed-to-a-dual-by-a-marker-bound` (the door is bounded `Certifies`) and `a-place-is-an-f64-argument-answered-on-the-value-channel` (which takes the evaluate-only solved values off `from_computed` first).

## Scope, as the designers read it (re-measure)

- One door in `geom-core`: the `f64` candidate and the residual at `T` in, the site's check run inside, the admitted `T` or the refusal out. The sketch put to Ev is on PR 7176; names and bounds are this unit's.
- The storing sites move onto it: `geom_brep::curve_face::spline_face::settle` (into `split_at`), `composite::crossings` (into `split_at`), `topo::replace_face::carrier_foot` (an edge's re-anchored end). Whether `SplineWall::paths` stores its pieces as edges was not settled; trace it.
- `curve_pair::Span::read` / `Hit::On{t}` reaches `split_other_at_point` through `OnCarrier::On`, which is already a typed admit: count it, and say whether it is the same door.
- `replace_face::corner_of_root` admits an `f64` root with no residual check at `T` (only pairwise corner agreement) and is generic over an `f64`-only solver: decide whether it goes through the door or stops being generic.
- A gate: `Real::from_computed` has no caller outside the door.
- `ChartMeeting.uv` is `f64` beside `t: T`: right under the rule (a place beside a stored value), worth a doc line.

## Decides, where a spec would otherwise assert

- The check inside the door is the one each site ran by hand: a site whose hand check differs in kind (a pairwise agreement, a window bound) is a finding before it is a port.
- At `Interval`, a wide family refuses at the door. Pin one row per site where it does, so the refusal is typed and named, not a later certification failure.

## Note from `a-place-is-an-f64-argument-answered-on-the-value-channel`

A place is handed to `geom`'s place-taking evaluators as an `f64` (`crates/geom/src/places.rs`: `eval_at`, `deriv_at`, `eval_in_span_at`, `project_at`), which exist only at a scalar that carries no tangent. Their one step to the scalar, `at_place`, is private to that module and is `T::from_computed(x)`: at `Sym` a place has to be its own unknown, or a re-valuation reads at a point chosen for the nominal values (pinned by `ssi::one_arc::tests::a_window_ends_parameter_enters_as_a_computed_value` and the projection sweep pins in `geom`). So it is a second caller of `from_computed` beside this unit's door. It is private, so no caller can spell a solved value through it. The gate this unit adds either names `geom::places::at_place` as the second caller or gives a place its own `Sym` spelling.

The modules that keep a place's step inside their own evaluator (`topo::spline_trim`'s split point and inserted cut point, `edge_nurbs::Schedule::station`) are further `from_computed` callers of the same kind; the gate's list would name them too.
