---
id: a-planar-faces-volume-term-reads-its-unweighted-mean-vertex-so-in-band-vertices-lever-over-the-face
kind: issue
title: A planar face's volume term reads at its unweighted mean vertex, so a few vertices in band of its plane lever over the whole face (four sweep rows' volumes at ε 1e-6, since PR 6505)
status: open
opened: 2026-11-14
priority: P2
cost: M
design: true
refs: [the-planar-face-reading-is-two-formulas-at-two-scalars, a-face-moved-onto-a-record-reads-its-volume-at-the-records-anchor, a-needle-channels-f64-shell-volume-rounds-by-up-to-8-6-percent-at-eps-1e-9-since-pr-6505]
---


Found by GREEN's ε 1e-6 lane (`green/eps-1e-6`) from nightly
43371638371, filed P0 as four rows whose volumes "the one surface
record moves". Re-read 2026-11-25: no face in any of the four is moved
onto another record. The shift is the planar volume reading PR 6505
introduced (`af696485a7`), and it is within what the kernel promises.
Rebanded P2 (Ev, in chat): the volume measurement here is poor in a
fixable way, not a wrong answer.

## What fails

At `CAD_TOLERANCE_EPS=1e-6`, on main `b363f5fa1` and `6003877ad`:

| row (`sweep::all`) | first message | off by |
|---|---|---|
| `a_curved_keel_in_the_plane::a_curved_keel_in_the_plane_splits_sound_and_sections_with_no_spur` | the V turned −20°, its tip 0.6 ε off the plane: the halves hold 40.32583257462023, the body 40.3258330933859 | 5.2e-7, against `1e-9 · whole` |
| `a_plane_moved_to_a_cone_apex::a_half_space_flush_with_a_frustum_wedge_cap_builds_the_union` | `y0 = 0.25, tan α = 1/64`, `0.5·ε` over the cap, wedge ∪ half-space: 24.0000679165704 against 24.000068913518643 | 1.0e-6, against 3.9e-8 |
| `parallel_cylinder_join::one_face_walls_read_their_axes_as_an_angle` | four runs tipped `0.2·zero` | 1.2e-7, against `VOLUME_CLAIM = 1e-7` |
| `parallel_cylinder_join::seams_lying_on_the_walls_read_the_axes_across_the_region` | six runs tipped `1.5·zero / 3.2` | 2.0e-7 to 2.8e-7, against 1e-7 |

Every body passes tier 2, tier 3′, the certificate and the operand
check. Bisected first-parent at ε 1e-6: green at `e996c0049`, red at
`9e1484043` (the merge of PR 6505).

## The cause, measured

A probe printed every face's record, sense, vertices and
`topo::props::face_flux_terms` term at ε 1e-6, on main and on
`e996c0049`. In all four rows the two builds have the same faces on
the same records, and every curved face's term is bit-identical. The
whole difference is in a few planar faces' terms.

`geom_brep::props::planar_face` reads a planar face's flux as
`at · A⃗`, where `at` is `props::face_anchor`'s mean of the face's
vertices (`[geom-brep.face-anchored.plane]`). Before PR 6505 it read
`origin · A⃗` about the plane record's origin. A face whose vertices
stand off its own plane by `δ_i`, in band, now reads
`mean(δ_i) · |A⃗|` more flux. The mean is unweighted, so a few
vertices standing off a large face lever across its whole area:

- **The cone cap.** The brick's bottom face (area 11.99) keeps its own
  record `z = −0.5ε`. Its outer four vertices lie on it. Its hole ring
  is the wedge's cap footprint, whose four vertices lie at `z = 0`,
  the wedge's cap plane, decided one carrier with it. The mean vertex
  stands `0.25ε` off, so the term moves by `−0.25ε · 11.99 = −3.00e-6`
  (5.996e-6 before, 2.998e-6 now), and the volume by a third of that:
  −0.9994e-6 of the measured −0.9994e-6 (−9.969e-7 now against
  +2.44e-9 before). The honest ambiguity here is the `0.5ε` slab over
  the cap's 0.0073 m², about 4e-9.
- **The keel.** The upper half's section face at `y = 1` (area 20.62)
  holds the V's tip vertices at `1 + 0.6ε`. Its term moves by
  −1.556e-6 (−2.250e-6 before, −3.806e-6 now): the upper half reads
  5.19e-7 less, the lower half and the whole body are bit-identical,
  and that is the halves' whole miss. A split no longer conserves the
  reading.
- **The parallel cylinders.** The tipped rod's two caps (area 2.247
  each) hold vertices the join placed with the axes decided parallel,
  a mean `0.05ε` and `0.11ε` off the tipped plane. The two terms move
  by −1.03e-7 and −2.57e-7, a volume of −1.20e-7: the whole miss of
  `S ba` (az 0 and π alike). The other runs move by the same
  mechanism, some under the 1e-7 claim (`U ab` at az 0, −8.0e-8).

The untipped closed form the cylinder rows use as `want` is exact
enough. Before PR 6505 every run read it to about 1e-12.

## What the kernel promises, and why this is still worth fixing

The kernel promises a certified enclosure of the reading
(`[tolerance.measure]`, DESIGN). A face whose vertices are in band of
its plane is ambiguous by up to band × area, and every shift above is
inside that. So the rows' bounds are tighter than any promise: the
cylinder rows' `VOLUME_CLAIM` does not scale with ε, and the cone and
keel rows assume the large face is read where its record is.

What is poor is where the reading spends that slack:

- the shift is `mean(δ_i) · area`, not the stand-off over the part of
  the face that stands off (cone: 1e-6 against about 4e-9);
- it depends on how many vertices the boundary has, not on its
  shape: a vertex added on an edge moves the reading;
- a split no longer conserves it (the keel);
- the closed-form volume is how many rows tell a right body from a
  wrong one, and noise at `ε · area` makes it a weaker check at
  coarse ε.

## What a fix has to keep

The mean-vertex anchor fixed a real case. A face moved onto a record
in band of its own reads its own boundary, not the record's offset at
a far anchor (`a-face-moved-onto-a-record-reads-its-volume-at-the-records-anchor`;
the far-origin union row asserts it to 1e-9). Reading the record's
plane instead was tried by ONEDOOR's fix-2 lane on PR 6505: R1's
slotted plate (`crates/topo/tests/a_verdict_moves_only_the_faces_it_read.rs`,
A∪P) then read 33ε off, because a moved face stands off its record
(`the-planar-face-reading-is-two-formulas-at-two-scalars`). So neither
plain reading serves both cases. Directions, none measured:

- read a face's record at the face (the mean vertex's foot on the
  record's plane, `extent::anchored_at`) where its boundary lies on
  its record, and the fan where the face was moved;
- weight the anchor by what each vertex stands for (its share of
  the boundary's length or of the fan's area), not by vertex count;
- the one-formula candidate in
  `the-planar-face-reading-is-two-formulas-at-two-scalars`, the fan's
  normal part read along the loops' own unit normal.

Whichever is taken changes `[geom-brep.face-anchored.plane]` and the
interval lane's `props::quad_lane::planar_face_about` with it.
`a-needle-channels-f64-shell-volume-rounds-by-up-to-8-6-percent-at-eps-1e-9-since-pr-6505`
went red at the same merge and may be the same term; not measured.

## Pinned

Each of the four is under `test_utils::pinned_red::row` at ε 1e-6, on
the messages above. Take the pins off with the fix. If the fix leaves
a shift that the tolerance model admits, restate that row's bound from
the stand-off and the face's area, with the argument, rather than
loosening it to fit.

`band_strip_mouth_cups::a_cups_mouth_a_hair_across_the_trimline_crosses_and_one_inside_goes`
went red at the same merge with a different stop; it is
`work/band/a-cups-fixture-boolean-is-unfinished-at-eps-1e-6.md`.
