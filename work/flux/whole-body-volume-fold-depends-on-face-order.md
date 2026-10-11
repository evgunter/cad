---
id: whole-body-volume-fold-depends-on-face-order
kind: issue
title: props fold_runs sums face terms in arena order, so one solid stored in two face orders measures a volume a few ulp apart
status: open
opened: 2026-10-11
priority: P2
---


## What

`crates/topo/src/props.rs`'s `fold_runs` sums each face's flux and
area term in plain arithmetic, in the order of the face slice it is
handed, which for the whole-body walks is face-arena order. That is
DESIGN.md's D9 addendum, idiom 2 ("a *sequential* fold in arena
order"). A body's measured volume is therefore a function of its face
storage order as well as of its faces. Two representations of one solid
can measure a few ulp apart, with identical pads and identical
per-face terms.

It surfaced on the STEP round trip:
`step-import::nurbs_import::arc_loft_natively_computes_its_rational_volume`
(`work/exch/arc-loft-round-trip-volume-differs-by-face-order.md`;
`work/nurbs/main-slow-set-reds-on-two-rows-since-4553.md`).
- Measured, native against imported: every face hands the quadrature
  identical bits and gets the same flux back.
- The six terms arrive in two arena orders. Native:
  `[9.14, 0, 6.2, 9.7389…, 6.2, 6.2]`. Imported:
  `[9.14, 9.7389…, 0, 6.2, 6.2, 6.2]`.
- The totals are `37.48052987749494` and `…493`.
- That row now claims bit identity only within one body, and bounds
  native against imported by the fold's reordering error.

## Why a different stored order does not fix it

The reader rebuilds each shell's face list (`Shell::faces`) exactly as
the native body stores it, so summing in shell-list order makes the
round trip agree. NURBS tried that: `mass_properties`, check 7's
subjects, the assembled certificate and the boolean backstop all walked
solids → shells → faces. Two green rows went red:
- `sweep::fillet_h5_hostless_rim::a_hostless_rim_composes_with_a_shared_wall_neighbour`
- `sweep::blend_tworims::a_seam_split_rim_pair_on_shared_half_band_walls_composes_in_one_call`

Both pin a one-call result bit-identical to a sequential composition,
and those bodies agree in arena order but not in shell order. Any
storage order is canonical for one equivalence and not for another.

## The question

Should the whole-body fold be order-independent? For example, an exact
two-sum expansion of the face terms, rounded once, would make the
volume's bits a function of the multiset of faces. That needs:
- a per-scalar summation on `geom_core`'s scalar traits (exact at `f64`,
  directed at `Interval`, sequential at the symbolic and probe
  scalars);
- re-cutting every mass-property golden where the sequential sum
  rounds differently;
- an edit to D9's addendum, which is ratified, so it is an `[ev]`
  question.
