---
id: pcurve-envelope-escalates-at-interval-on-a-wide-arc
kind: issue
title: the pcurve envelope check escalates at the Interval scalar on a wide shallow arc, so extrude refuses a cell it built before it minted
status: closed
opened: 2026-10-01
closed: 2026-10-02
---

Found by PCERT's `pcert/at-rest-rows-mandatory` (pcurve rows mandatory
at rest, C4), by execution. `sweep::extrude` now closes with
`topo::mint_pcurves`, so the pcurve certificate runs on every extruded
wall at every scalar, where before it ran on none.

At `Interval`, the shallow-arc grid of
`crates/sweep/tests/shallow_arc_extrude_grid_interval.rs` (a square
whose top edge is an arc of bulge `b`, scale `l`, offset `off`) now
refuses at the extrude for the wide-radius cells. At ε = 1e-9, every
cell with `l = 50, b = 1e-4` (radius ≈ 6.25e4), at all three offsets:

    Pcurve(Certify { error: Escalated { check: Envelope, sample: 0,
      cause: Indeterminate { margin: Enclosure { lo: 0.0, hi: 1.0477e-9 },
      band: Band { zero: 1e-9, escalate: 1e-8 },
      predicate: Some("pcurve_envelope") } } })

The census moved `(54, 6, 0, 0)` → `(51, 6, 3, 0)` at 1e-9 and
`(29, 22, 9, 0)` → `(24, 22, 14, 0)` at 1e-12; 1e-6 is unchanged. The
same body certifies at `f64`. The envelope's closed-form bound for a
`Harmonic` image (`crates/geom-brep/src/pcurve_cache.rs`,
`EnvelopeStatement`) is evaluated in the scalar's own arithmetic, and its
enclosure grows with the arc's radius until it straddles the band, for
a quantity whose true value is zero (the image is the carrier's exact
closed form).

The question for this program: whether the envelope's bound can be
stated so its enclosure does not scale with the radius (a relative or
factored form), or whether a wide arc at the certified scalar is an
honest refusal. Re-baselined, not fixed, on that branch.

## Closed

Closed by `pcert/certificate-incidence-fidelity`. On a periodic chart,
check 4 is now the carrier's incidence with the chart plus the stored
image's fidelity to the image re-derived from it
(`EnvelopeStatement::MapResidualClosedForm`). Neither term pushes an
angle back through `sin`/`cos`, so the enclosure no longer grows with
the arc's radius. The grid's census is main's again at 1e-9,
`(54, 6, 0, 0)`. At 1e-12 it is `(26, 22, 12, 0)`: main's nine
Euler-gate refusals, plus three cells at `off = 1000` whose wall rows
escalate at check 3's `MapResidual` samples. No cell refuses at the
envelope. Those three belong to check 3's standing, which is on
`work/sym/pcurve-certificate-checks-widen-past-the-band-over-a-parameter-box.md`.
