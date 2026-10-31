---
id: a-touch-the-analysis-decides-crossing-is-read-as-undecidable
kind: issue
title: A touch the analysis decides is a crossing (the materials' interiors meet there) is reported as undecidable, not as interference
status: closed
opened: 2026-10-26
priority: P1
cost: M
refs: [two-copies-filling-one-volume-certify-at-rest]
closed: 2026-10-27
---


Found by `two-copies-filling-one-volume-certify-at-rest`'s sweep (refs
that row) for other paths that report a real overlap as
`CensusUndecidable`.

## What

The touch analysis (`census.rs` `rest::judge`, `TouchVerdict`) has a
DECIDED verdict `TouchVerdict::Crossing`: "the two materials' interiors
meet at the touch, on decided readings". Interiors are open, so that
is an overlap of positive volume. But `TouchVerdict::refusal` maps it
to `Undecided::MixedTouch`, and arm 2 of `sweep_cross_solid_backstop`
reports it through `blocks` as `CensusUndecidable` ("one passes into
the other where they touch"). `census_interference_solids` does not
read that, so between two copies at rest the at-rest door treats a
decided overlap as could-not-look. Since stage 5 C that means it never
refuses at `Error`, and no assertion can say the overlap is intended.

The planar coincident subset of this path (copies whose faces lie on
one plane facing the same way and share interior) is now decided
before `blocks` runs, as `SameSideInterference`. The rest is what
`MixedTouch` still carries: an overlap with no vertex strictly inside,
no pierce, no same-side crossing and no aligned coplanar face pair,
whose crossing shows only at a touch. One witness in the tree is the
pyramid pair `a_vertex_read_by_two_sector_passes.rs` admits ("a pyramid
notched by another", its `three_prime` allowlist).

## What closing it takes

Report a decided `Crossing` touch between two solids as interference
that `census_interference_solids` reads, naming the touch's two cells,
and keep `MixedTouch` for nothing else, or retire it. Check first that
`Crossing` is never minted from an in-band reading (its doc says
decided readings only).

## A second path: two solids' wedges on one line (2026-10-26)

Arm 2's `Named::of` maps a cross-solid `CoincidentWedgesOverlap` to
`Said::Wedges(None)`, and so to `MixedTouch` too. The census now reads
that finding as the pair's interference (`census_interference_solids`),
so arm 2 reports one pair decided and undecidable at once. Witness:
`coincident_line_wedges::a_pinwheel_resting_on_confirmed_face_pairs_refuses_its_doubled_core`,
four decided wedge overlaps plus `MixedTouch`. Between two copies at
rest the door folds both into one finding, which stays loud, because
`localize`'s `covered` holds a solid-pair `CensusUndecidable` uncovered.
Closing this row should retire this mapping with the `Crossing` one.

## Closed

2026-10-27, ATREST's crossing-touch lane (branch
`atrest/a-crossing-touch-is-interference`).

- **`Crossing` is decided only.** `link::rest::judge` returns it on
  `crosses` (both ends decidedly on opposite sides, the crossing point
  decidedly inside the face's sector, decidedly off every cut), or on a
  probe that `Link::reads_over` places `In` (`None` on any reading in
  band), or `On` a sector with the pair `Aligned`. Each is confirmed
  again inside the ball. Every in-band reading sets `notes.in_band` and
  ends as `Unread(InBand)`. No in-band case was found that needed to stay
  undecidable. The review found a degenerate one: a piece whose probe
  lies on another face of its own link (a fin, a doubled face with no
  material) could read `In` and mint a crossing. Such a piece now
  decides nothing (`census::tests::a_slit_reads_degenerate`, the fin
  into the floor).
- **A decided crossing touch is interference.** Arm 2 reports it as
  `ValidationError::TouchInterference { a, b }`, the two `Cell`s of the
  site that read it. `census_interference_solids` reads it, and so does
  `test_support::is_decided_overlap`. It is a new arm and not a reused
  one: `InstanceInterference` names a vertex strictly inside, and
  `SameSideInterference` names a face pair, and neither is true of
  this class.
- **A decided interference outranks every undecided reading.** A
  crossing anywhere in a pair's findings or records decides it, even
  when an undecided finding stands earlier.
- **A pair decided by a standing finding is not read again.** If the
  census already reads one of the pair's findings as interference (a
  cross-solid `CoincidentWedgesOverlap`, an `EdgeFacePierce`, or an
  `EdgeEdgeCross` with `SameSide`), `blocks` answers `Stands` before
  reading anything else, and arm 2 pushes neither a refusal nor the
  probe's undecided orderings. A pierce or wedge finding that reaches
  `blocks`' between-solids arm all the same means the two resolvers of
  a finding's solids disagree, and refuses (`Undecided::Unresolved`). The wedges path was this
  row's second path; pierces and same-side crosses are the same
  defect: they used to add `CensusUndecidable` "another finding reports
  their boundaries crossing" beside their own interference.
- **`MixedTouch` retires.** Every reading it carried was a decided
  crossing. The undecided touches already have their own reasons
  (`Touch*`). `Undecided::Crossing` remains only for one solid's own
  boundary crossing itself.
- **Witness:**
  `a_decided_overlap_decides_its_pair::a_tetrahedron_inscribed_in_a_cube_interferes_at_every_pose`.
  The tetrahedron's corners are on the cube's, its edges lie in the
  cube's faces, and no face of one is coplanar with a face of the
  other. The test asserts one `TouchInterference` and no
  `CensusUndecidable`.
- **Restated rows:** the pinwheel row, the sunk beam and sunk ridges in
  `contact5_gate_and_beam`, `contact7_touch_sweeps::refused`, and the
  allowlist in `a_vertex_read_by_two_sector_passes`.
