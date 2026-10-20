---
id: a-bore-tangent-to-a-plates-face-mints-a-face-face-record-the-census-contradicts
kind: issue
title: A plate minus a rod tangent to its face from inside mints a face–face record the census contradicts on the material crescent
status: open
opened: 2026-10-19
priority: P2
refs: [a-two-carrier-face-pair-off-its-tangent-line-reads-contradicted-not-stale, an-at-rest-shared-crescent-reads-as-a-contradicted-contact-not-an-interference-finding, the-census-discovers-no-same-solid-tangent-touch-through-face-interiors]
---

Found by `contacthold/ff-divided-reads-stale` (the unit
`a-two-carrier-face-pair-off-its-tangent-line-reads-contradicted-not-stale`),
re-measuring PR 4921's review probes on main.

## What

A plate `[-2, 2]² × [0, 1]` minus a rod of radius 0.5 about
`(0, 1.5)`, z ∈ [0.25, 0.75]: the bore is tangent to the plate's
y = 2 face from inside along x = 0, so the result is one solid pinched
to zero thickness along that ruling. The op mints a face–face record of
the plate's face and the bore's wall (the reduction's found row), and
its own 3′ contradicts it:

- PR 4921's `review_4921_tangent_battery`, case `p×c in`, A∖B:
  `solids=1`, volume right, `3'err: ContactContradicted { a: Face(5),
  b: Face(9), .. }` (`contact_tangent_overlap`), at s = 1, 1e-2, 1e2.
- `review_4921_both_faces_divided`, case `divided in` (the plate
  slotted through the bore's wall near y = 1, B = rod + a block across
  the plate's face at x ∈ [1, 1.5]), A∖B: the carry confirms the found
  row on the result's piece pairs, the line runs through both pieces,
  and the table contradicts it on the crescent, so the op refuses
  `CarryRefused { Found, Unread { cells: [Face(5), Face(11)] } }`.

Run either with `cargo test -p sweep --test all review_4921_ -- --ignored
--nocapture` after copying `crates/sweep/tests/review_4921_probes.rs`
from branch `contacthold/face-face-touch-row-review` and its `mod` line
into `crates/sweep/tests/all.rs`.

The reading: beside the line the region between the plane and the bore
(y < 2, outside the circle) is the plate's material, which both faces'
material sides contain. C4 (`crates/topo/README.md`): "`ContactClass::Tangent`,
the record's and the mate's class, keeps its material claim, and a
shared crescent contradicts it" (`contact_verify::tangent_locus_relation`,
`declared = true`, through `boolean::tangent_table_at_rest`). That
sentence was written for two materials overlapping; here the crescent is
one solid's, pinched, the complement of the rod resting outside the
plate (void crescent, materials beyond).

## Owed (a design fork)

Which is wrong, the mint or the confirm:

1. The pinch is a face–face touch and the record is right; the
   `Tangent` table reads a same-solid pair's shared crescent as a
   pinch (one material), not an overlap. A revision of C4's sentence.
2. A material-crescent pinch is not a `Tangent` contact; the reduction
   mints no face–face row for it (and the census's same-solid discovery,
   `the-census-discovers-no-same-solid-tangent-touch-through-face-interiors`,
   demands none), or mints it under another class.
3. The op refuses a material-crescent pinch outright.

Pins on the fix: both probes' A∖B rows, every scale, every eps row.



## W1′: the same point set in two topologies (2026-10-19)

Measured by both designers of the pinch fork's round 3 (`[ev]` PR 5395),
on main `f6ee5c7db8`, f64, `Tol::witness`, with throwaway probes:

- **Rod seams off the ruling** (this row's fixture): one solid,
  v = 16 − π/8, records ff(plate's y = 2 face, bore wall) plus two
  edge–face records (the cap circles). Tier 3 passes; 3′ refuses
  `ContactContradicted` on the face–face record.
- **Rod seam on the ruling** (the rod's seam placed on the ruling,
  x = 0, y = 2): the seam edge lies in the plate face (`Smooth`,
  `Aligned`), records ef(seam, plate face) plus two vertex–face
  records. Tier 3 and 3′ both pass.

Same point set, same volume: two topologies by vertex placement, with
opposite 3′ verdicts. Whichever answer the fork takes, the two forms owe
one verdict; the edge–face form passes only because no lane reads its
sides (RESTREAD's
`the-edge-edge-and-point-record-confirms-read-no-material-sides`).

## The through-bore slot (2026-10-19, unmeasured)

From round 4 of the pinch fork (`[ev]` PR 5395): a plate ∖ a rod tangent
from inside that runs through the plate (a through-bore), then ∖ a slot
cut through the wall beneath the bore, far from the tangent line. The
slot leaves the material joined only along the line, so the body is two
solids, each ending in a cusp there. Either answer must handle it: the
record form has to split the plate face and the bore wall at the
record's locus, which that op never sectioned; the edge form already
holds the cusp edges and only re-sorts. Not yet probed on main.

## Answered (Ev, `[ev]` PR 5395, 2026-10-20)

The edge form (option 2, refined). The final state is two coincident
cusp edges, wedge 0, one per cusp, built at the glue decision, with no
face–face row: the plate face carries them as a zero-area ring and the
bore wall is cut along its ruling (D1 tier 3, C7). Interim (option 3):
the op refuses typed, and so does a carry that lands a record whose
crescent reads material (the mould). Built by
`a-material-crescent-tangent-line-builds-two-coincident-cusp-edges`;
the interim is
`a-material-crescent-face-face-record-refuses-typed-at-the-op-and-the-carry`.
