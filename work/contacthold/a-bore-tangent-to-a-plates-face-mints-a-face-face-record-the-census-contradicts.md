---
id: a-bore-tangent-to-a-plates-face-mints-a-face-face-record-the-census-contradicts
kind: issue
title: A plate minus a rod tangent to its face from inside mints a face–face record the census contradicts on the material crescent
status: open
opened: 2026-10-19
priority: P2
design: true
refs: [a-two-carrier-face-pair-off-its-tangent-line-reads-contradicted-not-stale, an-at-rest-shared-crescent-reads-as-a-contradicted-contact-not-an-interference-finding, the-census-discovers-no-same-solid-tangent-touch-through-face-interiors]
needs_ev: true
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


