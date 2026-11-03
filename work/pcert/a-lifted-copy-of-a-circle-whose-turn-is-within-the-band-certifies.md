---
id: a-lifted-copy-of-a-circle-whose-turn-is-within-the-band-certifies
kind: issue
title: A lifted copy of a circle whose turn is within the band reads Zero in check 4's fidelity term
status: closed
opened: 2026-10-28
closed: 2026-10-28
---

Found by E1c's L1 (PR 5889): with the on-face generator reaching a
cone's zero-winding rim, the pinned envelope sweep's trial 2244 drew one
of radius about 1.1e-10 whose image was renamed a whole period over.

## What happens

A row stored a whole period off its placed image is a lifted copy, no
row (C4). Check 4 sees it through `placement_terms`/`fidelity` in
`crates/geom-brep/src/pcurve_cache.rs`: the offset past half a period,
levered at the azimuth's arm in metres. No whole-period integer is read
(the ratified rule stands, orchestrator ruling (A) on E1c's L1). On a
circle whose half turn is within the zero band, the arm is the circle's
radius, so the term reads `π·ρ` and decides Zero: the copy is the same
point set to within the tolerance, and no metric reading can tell it
from the row.

## Measurement (E1c, every ε row)

`pcurve_cache::envelope_lemma_fuzz::a_lifted_copy_of_a_circle_whose_turn_is_within_the_band_never_reaches_check_4`:
a cone rim of radius `0.1·zero`, set `1e3·escalate` above the apex,
derives `ConeRim { beta: 0 }`. Its copy at `k = −1` reads `FidelityU`
Zero in `periodic_envelope`. At the door, `PcurveCache::certify` refuses
the rim, the exact image and the copy alike, with `IntervalNotForward` at
`f64` and at `Interval`: check 2 meters the span through the carrier's
rate, and a whole turn of a circle this small is a length within the
band. A rim of radius `1e3·zero` passes check 2 and refuses its copy on
`FidelityU`.

## Disposition

Closed as unreachable at the door, by measurement: the pinned row
refuses the rim, its exact image and its lifted copy at check 2
(`IntervalNotForward`) at `f64` and at `Interval`, at every ε row CI
runs, before check 4 reads any term, so the lift the term cannot see is
never certified there, and nothing downstream reads its uv lift. The readers
that would care (chart_region's uv loops, trimming, the
fundamental-domain readers) read certified caches only, so none was
walked further.

The envelope sweep's C4 assertion skips a row only where the term's own
lever reads a whole period Zero (`π` times the azimuth's arm over the
row's chart box, decided as the term is), counts those rows
(`Tally::vacuous`), and keeps a floor of 0.9 on the renamings it still
checks.
