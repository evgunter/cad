---
id: the-declared-rest-zip-retires-at-stage-4-and-the-join-needs-three-arms
kind: issue
title: Stage 4 retires the declared-REST zip with declared pairs: the join must first build what the zip builds today (three arms), then boolean/rest.rs's surgery goes
status: closed
opened: 2026-10-06
priority: P0
cost: H
closed: 2026-10-08
refs: [d10-one-way-to-say-intent-is-unbuilt]
---


Filed by ZIP (2026-10-06) as input to stage 4 (the coincidence door),
on Ev's direction in chat: the zip's fate is stage 4's, ZIP's rows on
it are parked on the D10 hold, and no `[ev]` PR was opened.

## What

The declared-REST zip (`crates/topo/src/boolean/rest.rs`,
`try_rest_union`) is a second implementation of the boolean's seam
crossing and glue. `ops::through_the_join` opens it only when the join
refuses a union with declared coincident faces
(`decls.coincident_faces`), so it has no trigger once stage 4 retires
declared pairs. D10's boolean sentence ("glues what its verdicts decide
Zero where an arm exists for the carrier pair") makes those arms the
join's. Two designers (an Opus and a Fable lane, blinded, same problem
statement) independently recommended retiring the zip and moving what
it can do into the join. ZIP's measurement lanes back it.

## Measured (main `3f1e3b0d`–`cadf2ed1`, topo + sweep + editor-core, ~7300 tests)

- **What reaches the zip today.** The zip door opens 95 times and builds 91 unions, from 11–13 tests. Each one follows a join refusal of one of three kinds:

  | join refusal | zip builds | scenes |
  |---|---|---|
  | `RingHomingAmbiguous` | 41 | cylinder bore mates |
  | the tangent plane×cylinder `SectionInvariant`s | 33 | a plate on a rounded plate; ring-vertex probes |
  | `Euler(NotSameFace)` from `mekr` | 17 | ring-vertex union probes |

  The original zip suites (crosslap, `m5_s1_rest_zip`, `m9_3_zip`, the curved merge door, the torus rest) no longer reach the door: since JOIN-1's locus matching, the join builds them.
- **The corner.** The join chooses each segment's corner from the geometry at strut insertion and carries it as `HalfGerm.he`. The zip drops it, undoes the struts, and re-derives the corner from loop structure (`mint_chord`'s `halves_at`), which has no answer at a vertex the loop visits twice. `ChordEndpointRevisited`, `ChordBetweenIsolatedPierces` and `SegmentsBetweenIsolatedPierces` exist only because of that undo.
- **The glue.** The zip glues patch by patch through vertex-cycle congruence (`pair_patches`, `mirror_edges`), so both solids must cut the contact region identically. That glue is what needs a vertex of B wherever A has one inside the contact. The join discards each contact side whole and fuses only along the seam.
- **Ring placement.** The zip's chord `mef`s never re-home a face's rings. The join's do.
- **The straight chord.** `mint_chord`'s straight chord on a host with no twin (`twin == None`) is reached by 0 of about 1700 calls.
- **Admission.** The zip assumed the interiors disjoint and shipped `vol a + vol b` behind a join lever. ZIP's PR 4127 adds the check, so it now declines.

## What the join needs before the zip can go

1. **A partner-edge chord.** A segment lying along an edge of one solid takes that edge's curve as the other solid's chord, so no section of a tangent or coaxial germ pair is computed. This is the zip's `Twin`, moved, and it generalizes the join's `along_edge_spec`. It covers the 33.
2. **Ring re-homing on a curved chart, defined in aligned contact.** Two parts:
   - `chord_join::chart_ring_side` refuses a run whose azimuth window spans a full period. A full-turn face divided by a closed run has exactly that window, and it is decidable.
   - A pierce ring every one of whose vertices lies on a degenerate constant-azimuth ray reads `Undecided` and refuses `RingHomingAmbiguous`. In a resting mate the other solid's rulings put run vertices at exactly the ring's azimuth.

   This covers the 41, and both remaining ZIP scenes: the shaft wholly inside a full-turn bore, and the split collar's through span. Both refuse in the join first.
3. **The `mekr` `NotSameFace` refusal** in the ring-vertex unions (17). Its cause is unmeasured.

Order it so nothing regresses: ring homing first (the cylinder mates), the partner-edge chord second (fillets, torus), then delete `read_segments`, `undo_struts`, `realize_seam`, `mint_chord`, `mirror_edges`, `pair_patches`, the glue, the slit zip and band closure, the 18-arm `RestZipFrontier`, and the `through_the_join` door. Keep the carrier-pair doors (`carrier_pair_relation`, `face_carrier`, …) and move them elsewhere. The torus's `SectionLoopUndecided` at ε ≥ 3e-7 is a band escalation and stays a refusal.

**Ratified text.** `crates/topo/README.md` C7 says the zip removes conformal patches as interior and mints each seam once. It becomes a sentence about the join's finish when the zip is deleted. The sentence is agent-written (CONTACT-DESIGN C7, labelled "Design sketch only", condensed into the README by the 2026-09-03 sweep).

## Rows this subsumes (parked on the hold)

- `work/zip/blind-shaft-in-a-full-turn-bore-revisits-the-seam-vertex`
- `work/zip/a-vertex-of-one-solid-inside-the-rest-contact-has-no-twin`
- `work/zip/rest-zip-seam-chord-on-cylinder-wall`
- `work/zip/a-dip-inside-a-rest-contact-is-refused-by-the-result-gate` (once PR 4127 lands)
- every REST-lane row on ZIP's slate

Each closes when the zip is deleted, or moves to the join if its scene still refuses there.

## Closed (2026-10-08, INTENT stage 4 A, `intent/s4-a-join`)

The join builds what the zip built, and the zip is deleted. Measured first with a probe at the door: 185 openings, 125 builds (41 behind `RingHomingAmbiguous`, 57 behind tangent `SectionInvariant`, 27 behind `mekr`'s `NotSameFace`), all from sweep's suites; editor-core and topo open it 0 times.

1. **Ring re-homing on a wall chart.** A run vertex at the ray's azimuth reads by the half-open rule, so the rulings of the other solid no longer make every pierce-ring vertex degenerate; a row along the ray is met only by a vertex on it. A run whose window is exactly one period is read on the branch from its low edge, which builds the shaft wholly inside a full-turn bore.
2. **The tangent `SectionInvariant` and the `mekr` `NotSameFace` were one cause**, not a missing chord: a germ only tangent to a bound of its sector (the straight edge passing a fillet's tangent point) was minted in that sector, on the wall, while its locus is the face across the bound its segment lies in. `boolean::insert` now mints such a germ in the sector across the bound, keeping its crossing codes, and the run logic decides strut or fan from there. The partner-edge chord was built and then removed: no row reaches it once the germ is in the right face.
3. `try_rest_union` and its surgery, `RestZipFrontier`, `BooleanError::RestZipUnsupported` and the door in `through_the_join` are deleted; the carrier-pair doors moved to `boolean/carrier_pair.rs`.

