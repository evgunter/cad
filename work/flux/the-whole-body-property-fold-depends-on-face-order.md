---
id: the-whole-body-property-fold-depends-on-face-order
kind: issue
title: The whole-body property fold sums face terms in arena order, so one solid stored in two face orders measures a volume a few ulp apart
status: review
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
- ENCL's PR 4682 re-scoped that row on main to a `faces − 1` ulp bound
  on the volumes with the pad bit-identical; PR 4684 added, on top,
  per-face terms bit-identical native against imported (through
  `topo::test_support::mass_properties_with_face_terms`) and the
  import's gate bit-identical to the imported body's own measurement.

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

## The question for Ev

Is a body's measured volume a function of the body as stored, or should
it be invariant under the storage choices the data model declares free?
Those choices are face and loop order, ring order, and each loop's anchor
half-edge.

Two designers weighed it. A reconciliation round settled the fact they
disagreed on. A probe shows that rotating a loop's anchor moves per-face
terms by ulps: 1 to 5 of 8 to 14 faces, and up to 21 ulps of the volume
far from the origin. So an exact sum at the body fold alone, option (b)
above, makes nothing storage-invariant. Both designers withdrew or
rejected it.

Two coherent final states remain:
- **(A) storage-dependent and stated.** D9 names storage as part of the
  input. Cross-storage tests compare structure, or carry a bound computed
  from the face terms.
- **(S) storage-invariant all the way down.** Every reduction over a set
  or cycle (the edges of a loop, the loops of a face, the faces of a
  body, and the pads) is the rounded-once sum of its terms, through one
  summation on `geom_core::Real`.

The designers split: one recommends (A) (likely), the other leans to (S)
(unsure). Either way, the current `faces − 1` ulp bound is the wrong
shape for signed terms. D9 idiom 2's "arena order" wording also changes,
in a different direction under each answer.

## Ev's answer (PR 4830, 2026-10-19): (A), storage-dependent

"i think i want to pick storage-dependent for simplicity". D9's
determinism bullet now names a body's storage as part of the input,
and idiom 2 reads "the walk's own fixed order". The (S) spike is kept
findable (`docs/doc-ledger/exact-sum-spike.md`).

What remains, and keeps this row open: the STEP round trip's
`faces − 1` ulp bound
(`step-import::nurbs_import::arc_loft_natively_computes_its_rational_volume`)
is the wrong shape for signed terms. Replace it with a bound computed
from the face terms, or compare structure.

## Resolution (FLUX, `flux/round-trip-computed-bound`)

The cross-storage rows carry a bound computed from the face terms.

- `topo::test_support::fold_order_bound(terms)` (new, beside
  `mass_properties_with_face_terms`): `(volume, pad)` bounds on how far
  two fold orders of one body's face terms may round apart.
  - volume: `2·(E + u·(Σ|tᵢ| + E))/3`, `E = γₙ₋₁·Σ|tᵢ|`;
  - pad: `(γₙ₋₁ + 2u·(1 + γₙ₋₁))·Σpᵢ/3`, the pads summed and divided
    rounding up;
  - both with a 1% margin for the bound's own arithmetic. They scale
    with the terms, not the volume.
- Row `topo::props::fold_order_rows::two_fold_orders_round_apart_within_the_terms_bound`:
  over every order of six signed terms whose volume is small against
  them (a body `10⁵` m out), the volumes spread by `≈ 6.4e-12`,
  thousands of the volume's ulps, so `faces − 1` ulps fails them, and
  stay within the bound.
- `step-import::nurbs_import::arc_loft_natively_computes_its_rational_volume`:
  the per-face bit identity is kept; the volume's `faces − 1` ulp count
  and the pad's bit identity across the two bodies are now the computed
  bounds.
- `step-import::roundtrip` (every corpus fixture, native sidecar against
  the re-import): its `2·kernel_faces` ulps of `|V|` is now the computed
  bound from the imported body's own terms, plus the 2 ulps of the mm³
  conversion.

**Same shape, not converted** (each compares a volume at a multiple of
its own ulp, but not two storage orders of one body):

- `step-import::freecad` (against a closed form, `census.2` ulps of the
  volume): the fold's distance from the exact sum is one order's
  `γₙ₋₁·Σ|tᵢ|/3`, and the same helper would state it.
- `editor-core::asm2a_instantiate` (two rows, `8·ε_mach·|V|` between a
  body and its rigidly placed copy): there the transform moves every
  term, so a computed bound needs the transform's own error, not only
  the fold's.
- `step-import::review_probes` prints an ulp distance and asserts none;
  `sweep::reach_continuation` names one in a doc only.

