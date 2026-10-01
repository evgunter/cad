---
id: pcurve-envelope-escalates-at-interval-on-a-wide-arc
kind: issue
title: the pcurve envelope check escalates at the Interval scalar on a wide shallow arc, so extrude refuses a cell it built before it minted
status: open
opened: 2026-10-01
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
