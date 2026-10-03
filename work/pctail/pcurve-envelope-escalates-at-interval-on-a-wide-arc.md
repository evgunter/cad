---
id: pcurve-envelope-escalates-at-interval-on-a-wide-arc
kind: issue
title: the pcurve envelope check escalates at the Interval scalar on a wide shallow arc, so extrude refuses a cell it built before it minted
status: open
opened: 2026-10-01
priority: P2
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

## Restated, then reopened

**What the restatement did** (`pcert/certificate-incidence-fidelity`,
PR 3812, part 1). On a periodic chart check 4 became the carrier's
incidence with the chart plus the stored image's fidelity to the image
re-derived from it (`EnvelopeStatement::MapResidualClosedForm`).
Neither term pushes an angle back through `sin`/`cos`, so the
envelope's enclosure stopped growing with the arc's radius. The grid's
census was main's again at 1e-9, `(54, 6, 0, 0)`.

**This issue was closed on that measurement, and it should not have
been** (PR 3812's review R2): at 1e-12 one cell, `off = 1000, l = 1e-3,
b = 0.5`, still refused at `Envelope`.

**Then the frame was metered** (the review's other MAJOR). The lemma
assumes an orthonormal frame, and over a box nothing enforced it, so
check 4 now adds `EnvelopeTerm::Frame`: the stored chart's distance from
its Gram–Schmidt twin, bounded in the frame's invariants
(`‖axis‖² − 1`, `‖u_ref‖² − 1`, `axis·u_ref`) and levered by the radius
and the image's axial reach. The wide-arc cells' wall cylinder carries
the rim normalised at `Interval` as its `u_ref`, so `u_ref·u_ref − 1`
encloses a few ulps. Times the `6.25e4` m radius that is about `6e-10`
m, and the envelope's sum (`[0, 1.37e-9]` at 1e-9) escalates. The true
defect of a rounded unit vector is `R·2⁻⁵³`, about `7e-12` m here; the
enclosure is what the interval scalar can certify of it.

The census at the head of the fix pass is:

| ε | census | what moved |
|---|---|---|
| 1e-6 | `(51, 9, 0, 0)` | unchanged |
| 1e-9 | `(51, 6, 3, 0)` | the three `l = 50, b = 1e-4` cells, one at each offset, at `Envelope` |
| 1e-12 | `(24, 22, 14, 0)` | five extrude refusals past main's nine |

So the issue's subject (a wide shallow arc's wall refused at
`Envelope` over the `Interval` scalar) occurs again, for a sounder
reason. Two ways out:

- tighten what the interval scalar can say of a normalised vector's
  norm, so `u·u − 1` of a rounded unit vector encloses its true
  `2⁻⁵²`-scale defect rather than the dependency-widened one; or
- have the extrude store a frame whose unit-ness is exact at every
  scalar (for example, the rim and its norm kept apart, and the
  normalisation done where the chart is read).
