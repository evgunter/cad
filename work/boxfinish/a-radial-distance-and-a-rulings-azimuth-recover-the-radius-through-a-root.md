---
id: a-radial-distance-and-a-rulings-azimuth-recover-the-radius-through-a-root
kind: unit
title: A point's distance to a cylinder or sphere and a ruling's chart azimuth recover the radius through a root or an angle, where each holds the stored radius
status: dispatched
opened: 2026-11-15
priority: P0
cost: M
branch: boxfinish/radial-readers
---


Cut from `a-verb-stores-a-radius-as-a-magnitude-and-the-symbolic-tier-cannot-use-the-sign-its-gate-decided`
(2026-11-15) as the readers' half of the designers' agreed answer; that
row is parked on this one.

**Measured** (the doors' lane, `boxfinish/doors-read-arcs` at
`95b1e397`; the tour's plate over its study box, one leaf):

- With the circle verb storing `r` and the doors reading the recorded
  radial, an arc's carrier start, carrier end and mapped source all
  certify with no registration, and the hole's extrude then refuses at
  `pcurve_line_chart_incident [0, 1.6e-4]`: `geom-brep`'s
  `pcurve_cache.rs` `line_incidence` → `incidence_at` →
  `chart_distance` reads the strut's distance to the cylinder as
  `‖w‖ − R`, here `√(r²) − r`.
- With `|r| ≡ r` registered instead, it refuses at
  `pcurve_map_residual [0, 1.6e-4]`: the strut's chart azimuth, minted
  in the same file's cylinder-ruling arm as
  `Azimuth::of(w_r·cv, w_r·u_ref)`, the angle of `(r, 0)`, which is 0
  only where `r > 0` and which no registration reaches.

**The unit.** Each reader uses a sign-free form of what it already
holds; neither needs a new stored datum.

- **The distance:** `‖w‖ − R` is `(‖w‖² − R²)/(‖w‖ + R)`, the same
  real wherever the denominator is non-zero, with a numerator that is a
  polynomial and is zero whenever the point was built on the surface.
  The kernel already has the spelling (`norm_gap`, in the same file,
  written "so `√(p²) = p` is never asked of the arithmetic"). One
  function, used by `chart_distance`'s cylinder and sphere arms.
- **The azimuth:** the angle of a radial vector `w` is the angle of
  `w/R` for the chart's stored radius `R`; for the strut that is the
  recorded direction itself.

**The tests that say it is closed.**
- On `boxfinish/doors-read-arcs` with the circle verb's `abs` removed
  (the lane's probe `PROBE_R`), the hole's extrude and the uncut plate
  pass the two checks above over the tour's study box; what refuses
  next, if anything, is reported by payload.
- At `f64` and `Interval` the re-posed distance and azimuth are the
  same reals: a differential, head against merge base, with every
  changed answer listed.
- An audit of `geom-brep`, `topo` and `sweep` for the same two shapes
  (`norm() − radius` on a point built on the surface; `Azimuth::of` of
  an `r`-scaled vector), each hit classed as an identity reader on a
  certified path or a real margin, and filed where it is not this
  unit's.

## Measured by the unit's lane (2026-11-15), and its scope widened

On `boxfinish/doors-read-arcs` (`95b1e397`) with the circle verb
storing `r`, the tour's study box, `Sym<Interval>`, one leaf:

- **Both re-posings fold**, with and without the doors' registrations:
  `chart_distance`'s cylinder and sphere arms through the existing
  `norm_gap` (numerator from `q·q`), and the ruling's phase as
  `Azimuth::of(w_r·cv / R, w_r·u_ref / R)`. The hole's extrude then
  passes every check. No fold is missing in the tier.
- **The uncut plate** has no geometry refusal.
- **The cut plate** refuses at its first `Subtract`: the hole is
  unfinished, `VolumeUncomputable { Escalated { "props_meridian_on_surface",
  [0, 3.2e-4] } }`, raised in `geom-brep` `props/curved.rs`
  `cylinder_boundary`'s line arm: `q.norm() − radius`, the same shape in
  the volume reading's incidence premise.

**Widened (the orchestrator's call):** the closing test is the cut
plate, so an identity reader of this shape that the probe meets on the
cut plate's path is this unit's wherever in `geom-brep` or `topo` it
lives, re-posed through the one gap function, until the plate stops
refusing on a `|r| − r` shape or refuses on something else (reported
and filed). Real margins are left; the rest of the audit is filed.
