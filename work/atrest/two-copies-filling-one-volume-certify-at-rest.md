---
id: two-copies-filling-one-volume-certify-at-rest
kind: issue
title: Copies whose material coincides over an area are read as undecidable, not as interference
status: closed
opened: 2026-10-24
closed: 2026-10-26
priority: P1
cost: M
refs: [curved-copies-filling-one-volume-read-as-undecidable]
pr: 5745
---


Found by review 1 of PR 5637 (stage 4 I, `mates-declare-no-contact`).

## What

**The review's probes do not fill one volume.** Its two probes
(`r1_probe_a_wrong_sense_mate_quiets_nothing`,
`r1_probe_two_unmated_copies_filling_one_volume`, on
`review/atrest-mates-declare-no-contact-R1`) seat a unit cube on a unit
cube with a plane mate whose `Flip` is stripped. A plain plane mate
equates outward normals, so the copy turns over about x. The product's
vertices put it at y in [-1, 0] beside the base `[0, 1]³`, sharing the
y = 0 wall face to face, not inside it. Its five rows say so: the two
y = 0 walls rest (opposed), and the caps and x walls lie flush (aligned,
boundaries meeting, interiors apart). So Ok with no interference is the
right answer there, mated or pinned where it solved.

**Copies that do fill one volume were refused, but as undecidable.**
Two coincident unit cubes, a half-height cube flush in the base, and a
quarter block in its corner have no vertex of either strictly inside the
other and nothing piercing: every touch is a coincidence. The census
kept every touch a finding and refused the solid pair through arm 2's
`blocks` ("one passes into the other where they touch",
`CensusUndecidable`). That is a verdict the at-rest door reads as
could-not-look, not as interference: since stage 5 C it never refuses
at `Error`, and no assertion can say the overlap is intended.

## Why

The census read an overlap only off a vertex strictly inside, a pierce
or a same-side crossing. Where the boundaries coincide, none of those
exists. What does exist is a face of each on one carrier, facing the
same way, sharing interior: each face has its solid's material on its
inner side, so both solids hold the half-ball under every shared
interior point. The flush arm already knew such a pair was no flush
pair ("the two materials overlap there") and wrote nothing for it.

## Closed

2026-10-26. The planar face-pair arm (`census.rs` `planar_face_pairs`)
reads aligned pairs as well as opposed ones. A pair Door 1 decides on
one carrier with the senses aligned, whose interiors the region door or
the faces' own reading decide meet (`same_side_overlap`, shared with
`flush_at_rest`), is an overlap. Arm 2 of `sweep_cross_solid_backstop`
reports it as `ValidationError::SameSideInterference` (new: the two
faces), which `census_interference_solids` reads as interference. Once
an interference decides a pair, arm 2 adds no undecidable finding from
an ordering its probe could not decide. All three shapes above now
report one `SameSideInterference` and no `CensusUndecidable`. At the
editor's gate, the coincident copies (mated or pinned) and the
unflipped seat at (0.5, 0.5, 1) each give one bounded, loud
`InterferenceFinding`.

Rows: `crates/topo/tests/two_copies_filling_one_volume.rs`, and
`crates/editor-core/tests/asm_r2b_assembly.rs`
(`a_seat_without_its_flip_turns_the_copy_over_beside_its_base`,
`copies_filling_one_volume_are_one_interference`).

Not closed here: coincident faces the planar reading cannot see (curved
walls, discs bounded by an arc) are still refused undecidable. Filed as
`curved-copies-filling-one-volume-read-as-undecidable`.
