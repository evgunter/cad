---
id: the-whole-body-property-fold-depends-on-face-order
kind: issue
title: The whole-body property fold sums face terms in arena order, so one solid stored in two face orders measures a volume a few ulp apart
status: open
opened: 2026-10-11
priority: P2
design: true
---


## What

`crates/topo/src/props.rs`'s `fold_runs` sums each face's flux and
area term in plain arithmetic, in the order of the face slice it is
handed, which for the whole-body walks is face-arena order. That is
DESIGN.md's D9 addendum, idiom 2 ("a *sequential* fold in arena
order"). A body's measured volume is therefore a function of its face
storage order as well as of its faces. Two representations of one solid
can measure a few ulp apart, with identical per-face terms.

It surfaced on the STEP round trip:
`step-import::nurbs_import::arc_loft_natively_computes_its_rational_volume`
(`work/exch/arc-loft-round-trip-volume-differs-by-face-order.md`;
`work/nurbs/main-slow-set-reds-on-two-rows-since-4553.md`, PR 4684).
- Measured, native against imported: every face hands the quadrature
  identical bits (knots, net, weights, rectangle, perimeter, defect)
  and gets the same flux back. The rational wall is
  `[9.738934194323258, 9.738940253487032]` both ways.
- The reader's Euler-operator assembly allocates faces in its own
  order. Its rebuilt `Shell::faces` list IS the native arena order:
  native arena = shell `[1..6]`; imported arena `[1..6]` against
  imported shell `[1, 3, 6, 2, 4, 5]`.
- So the six terms are summed in two orders. Native:
  `[9.14, 0, 6.2, 9.7389…, 6.2, 6.2]`, total `37.48052987749494`.
  Imported: `[9.14, 9.7389…, 0, 6.2, 6.2, 6.2]`, total
  `37.48052987749493`.
- PR 4553 changed the rational wall's flux bits, and with them whether
  the two orders happen to round alike: 0 ulp apart before, 1 at
  ε = 1e-6 and 2 at 1e-9 after. No enclosure was lost.
- PR 4684 re-scoped that row: per-face terms bit-identical native
  against imported (through `topo::test_support::mass_properties_with_face_terms`),
  each body's own paths bit-identical (the import's gate against
  `mass_properties` of the imported body), and native against imported
  bodies overlapping and within the reordering bound,
  `2(n − 1)·u·Σ|xᵢ|` plus an ulp a side.

**PR 4551's row `quad-de-boor-through-the-blossom-breaks-the-import-bit-identity`
(on its branch) is this same fold.** It reports one ulp on the same
body with identical pads, and reads it as "the two walks hand
`de_boor` inputs that differ". The measurement above refutes that
premise: the inputs agree, and the sum's order differs. Its blossom
change moves the rational wall's bits, which is what decides whether
the two orders round alike.

## Option (a), built and backed out: sum in a stored order

The reader rebuilds each shell's face list exactly as the native body
stores it, so summing in shell-list order makes the round trip agree.
NURBS built it on PR 4684's branch (`Body::faces_in_walk_order`:
`mass_properties`, check 7's subjects, the assembled certificate and
the boolean backstop all walked solids → shells → faces). Two green
rows went red:
- `sweep::fillet_h5_hostless_rim::a_hostless_rim_composes_with_a_shared_wall_neighbour`
- `sweep::blend_tworims::a_seam_split_rim_pair_on_shared_half_band_walls_composes_in_one_call`

Both pin a one-call result bit-identical to a sequential composition,
and those bodies agree in arena order but not in shell order. Any
storage order is canonical for one equivalence and not another, so
choosing a different one moves the seam rather than closing it.
Backed out; topo's sources are main's.

## Option (b), not built: an order-independent fold

An exact summation of the face terms (a two-sum expansion, rounded
once) makes the volume's bits a function of the multiset of faces.
Its cost:
- **the scalar-trait surface**: a per-scalar summation on `geom_core`'s
  scalar traits, exact at `f64`, directed at `Interval`, and sequential
  (or its own canonical form) at the symbolic and probe scalars, since
  `fold_runs` is generic over `T: Decide`;
- **D9 idiom 2's wording**: the ratified addendum says "a *sequential*
  fold in arena order", so (b) is a revision of a ratified clause and
  an `[ev]` question;
- **the goldens that re-cut**: every mass-property golden where the
  sequential sum rounds differently from the exact one. At least the
  sweep thread-count digests
  (`crates/sweep/tests/thread-count-digest/eps-*.txt`, every body
  whose sum is not already exact) and any row pinning a volume's bits;
  the census is part of the design.

## The question

Should the whole-body fold be order-independent ((b)), or is
order-dependence the contract, with cross-representation claims stated
as bounds (the status quo PR 4684 adopts on the round-trip row)?
