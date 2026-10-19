---
id: conic-section-just-past-its-band-fails-certification-downstream
kind: issue
title: plane x cylinder and plane x cone mint a section whose construction error outgrows the band just past their degenerate trilean, and it refuses at certification instead of at the arm
status: open
opened: 2026-10-01
priority: P2
cost: M
design: true
branch: germ/conic-past-band
pr: 4812
refs: [focal-section-vertex-azimuth-is-read-off-a-cancelling-sum, focal-section-envelope-bounds-the-whole-ellipse-not-the-span, focal-section-image-on-a-flat-cone-misses-by-a-cos-squared-factor]
---

Found by the plane × cone split lane (`plane-cone-elliptic-section-split-refusal`),
measuring tilts toward the parabola.

## What happens

Both conic arms in `crates/geom-brep/src/intersect.rs` decide their
degenerate boundary with one trilean levered at the caller's `extent`
and mint the exact `Ellipse` on the definite side:

- `plane_cylinder_section`'s `pc_axis_plane_parallel`
  (`c = axis·n`): the semi-major axis is `r/|c|`;
- `plane_cone_section`'s `pn_conic_type`
  (`D = sin α·‖axis×n‖ − cos α·|axis·n|`): the semi-major axis is
  `|δ|·sin α·cos α / (c² − sin²α)`, with `c² − sin²α ∝ |D|`.

Just past the band the minted carrier is enormous, and its centre and
axes carry an absolute rounding error that scales with it. The arm
calls it zero-residual; certification downstream does not. Measured
through `topo::splitting::split` at `Tol::witness()` (band `1e-9` /
`1e-8`):

| fixture | tilt | outcome |
|---|---|---|
| cylinder r=1, h=1 (revolve), plane through `(0, 0.5, 0)` | `π/2 − 100ε` off the axis | `Join(Euler(Certification { Escalated { EndpointEnd, carrier_endpoint_end, 2.4e-9 } }))` |
| same | `π/2 − 1000ε` and beyond | splits |
| frustum `1 → 1/2`, `α = atan ½` (revolve), plane through `(0, 0.5, 0)` | parabola `− 3ε` / `+ 3ε` | `Escalated(pn_conic_type)` — the arm's own band |
| same | parabola `− 10ε` | `Join(Euler(Certification { Escalated { Surface1Residual, carrier_on_surface, 2.2e-9 } }))` |
| same | parabola `− 100ε` | `Pcurves(Certify { ResidualExceeded { Envelope } })` |
| same | parabola `− 1000ε` | `Pcurves(Certify { Escalated { Envelope, pcurve_envelope, 5.4e-9 } })` |
| same | parabola `− 10⁴ε` and beyond | splits, valid at every tier |

(`ε = 1e-9` here.) Every outcome is typed and no invalid body is
emitted, but between the arm's band and the point the carrier is
accurate enough, the refusal comes from a later door with the wrong
cause: a certification residual rather than "this section is too close
to degenerate to construct at this tolerance".

## What closes it

An arm-side admission criterion on the carrier's own conditioning, in
the shape `cone_cylinder_section`'s `coc_station_reach` already takes
for its circles (`BeyondOperandExtent`): refuse at the arm when the
constructed carrier's position error — growing like the semi-major
axis — cannot be held under the band. What the right lever is (the
semi-major axis against `extent`, or against the band's own reach) is
the design question; a naive `A ≤ extent` refuses legitimate long
sections (the frustum cut at tilt `1.0` has `A ≈ 3.5` against a face
extent near `2.2`, and certifies exactly).

## The region is wider than the table (dual review of #3688, 2026-10-01)

The table above starts at the arm's band. Measured by the review's
certificate sweep, which drives `chart_pcurve` + `PcurveCache::certify`
on the plane × cone ellipse directly (`ε = 1e-9`), the downstream
refusal reaches much farther from the parabola, and it grows with the
cone's half-angle and the scale:

| cone | tilt (fraction of the parabolic limit) | outcome |
|---|---|---|
| `α = 1`, unit scale | 0.999 | certifies (envelope 1.2e-10 on the `+` traversal) |
| `α = 1`, unit scale | 0.99999 | `ResidualExceeded { MapResidual }` (map residual **1.8e-6 m**) on one traversal, `Escalated(pcurve_map_residual)` on the other |
| `α = 1`, scale 1000 | 0.999 | `ResidualExceeded { MapResidual }` (1.5e-7 m) |
| `α = 1.55`, unit scale | 0.95 | `Escalated(pcurve_map_residual)`, margin 1.0e-9–1.4e-9 |
| `α = 1.55`, unit scale | 0.999 | `ResidualExceeded { MapResidual }` (1.8e-7 m) |

The carrier itself stays accurate across that range (its own residual
1.3e-10 at `α = 1`, 0.99999); it is the chart image's map residual that
grows, so an admission criterion has to bound the image's conditioning
as well as the carrier's.

**The refusal's text misnames the cause.** In the split, a frustum cut
`1e-8` short of the parabola (`φ = 1.1071`) refuses at the Euler gate:

> an Euler operation refused: geometry attachment gate: the
> start-endpoint residual at sample 0 escalated: margin 2.03e-9 lies
> inside the ambiguity band (1e-9, 1e-8). There is no way through: this
> is a kernel defect; report it

The input is legal and only ill-conditioned at this ε, so "no way
through" and "kernel defect" are both false: a looser tolerance or a
tilt farther from the parabola goes through. The ending is
`CertifyError::render(Reading::Build)`'s for every attachment-gate
residual (`geom-brep/src/certify.rs`), which is right for a carrier the
kernel built from well-conditioned data. The arm-side criterion above
is what makes it right here too: the section refuses at the arm, with a
tolerance recourse, before a carrier reaches the gate. Until then this
is the class's mislabelled door.

## A cylinder's elongated ellipse at ε = 1e-12 (CLEAVE, `cleave/minted-edge-inband-cert`)

Measured by `sweep/examples/split_near_tangent_census_probe.rs`
(`curved` family) on main `f813abf9`. The unit cylinder (`z ∈ [0, 1]`,
extruded) split by the plane through `(cos φ (1 + δ), sin φ (1 + δ), 0.5)`
with normal `(cos φ, sin φ, 1e-6)`, so the section is an ellipse of
semi-major axis about 1e6 m: at ε = 1e-12, for `φ ∈ {0, 0.5}` and every
`δ ∈ [−1e-7, 1e-7]` the census tried, the join refuses
`Join(Euler(Certification { ResidualExceeded { EndpointStart } }))`,
margins 1.0e-11 to 1.16e-10 (80 runs; 6 more refuse `EndpointEnd`, 5
`NotTransverse`), under the same "kernel defect; report it" ending. At
ε = 1e-9 and 1e-6 the same poses refuse earlier or split. The in-band
neighbours of this door, where the miss is the input's rather than the
carrier's rounding, are filed as CLEAVE's
`a-section-curves-end-read-in-band-of-its-vertex-refuses-as-a-kernel-defect-at-the-join`.

## The lever is a fork (GERM measurement, 2026-10-12)

Measured by `sweep/examples/conic_past_band_probe.rs`: the split of a
frustum (five half-angles, three scales) and of a cylinder (three
scales) by planes tilted toward the degenerate limit, at all three ε,
axis-aligned and in a generic pose, with the arm's carrier residual
(double-double), the chart image's map residual over the consumed
arcs and the whole turn, and its certificate. The tables and the
options are the draft PR's body. In short:

- **The cylinder** refuses downstream only at the Euler gate's
  endpoint residual, and that door is read by `u·A` (`u` the rounding
  unit, `A` the semi-major) alone: at ε = 1e-9 the unit and 100-unit
  cylinders certify to `u·A/ε ≈ 0.22` and refuse from 0.40 (the
  0.01-unit one refuses first at the sector door,
  `Reduce(SliverSector)`); at 1e-12 the crossover is 0.71–1.26. The extent plays no part, and the carrier and its image
  stay at rounding throughout.
- **The cone** refuses mostly at the image (`Pcurves(Certify)`,
  envelope or map residual), at `u·A/ε` anywhere from 1.4e-5 to 0.34:
  the image's whole-turn miss grows like `u·A/k²` at a generic pose
  (`k² = 1 − tan²α·tan²φ`), with a further `~1/cos²α` on flat cones
  (α = 1.55 refuses at half the parabolic tilt at 1e-12). No single
  quantity among `u·A`, `A/extent`, `u·A/k`, `u·A/(k·cos²α)`
  separates certifying from refusing poses: each overlaps by at least
  3× in the sweep.
- **Most of the cone's image error is avoidable one layer down**:
  `chart_pcurve` reads the vertex azimuth off a sum that cancels near
  the parabola, and reading it off the major axis cuts the whole-turn
  miss 10–1000× (filed on PCERT, with the envelope's whole-ellipse
  reading and the flat-cone factor: `refs`).

So several levers are viable and they trade off (what they refuse,
which layer owns the conditioning): an arm criterion on the carrier
alone, one that also bounds the image, the image fixed first and
then the carrier criterion, or the late refusal re-worded. The
frustum counterexample above (`A ≈ 3.4` against an extent of 2.0,
tilt `0.9·(π/2 − α)`) certifies at every ε, and no criterion that
reads `A` against the extent survives the sweep.

## Note from CLEAVE (2026-10-12)

CLEAVE's section-end design fork, with two designers converging, splits
this row's "what closes it" in two:

- **The carrier half** is CLEAVE's. Re-posing the ellipse at its
  nearest principal vertex
  (`work/cleave/a-section-ellipse-is-re-posed-at-its-nearest-principal-vertex.md`)
  removes the parameter term ulp(π/2)·A. The section table then admits
  only on ρ ≤ ε/K and refuses the far-centre term (about
  5·u·|centre − face|) as a kernel limit with no lever
  (`work/cleave/the-section-table-admits-a-curve-on-its-resolution.md`).
  Your 1.82e-12 at about 1e4 and 1.16e-10 at 1e6 are that far-centre
  term, to the digit.
- **The chart-image half** stays yours: the α = 1.55 `MapResidual`
  table, where the carrier is accurate and the pcurve image is not.

## Decided (Ev, PR 4812, 2026-10-12)

- **End state: the conic carrier is stored in the form it was born in,
  and the other form is derived.** A kernel-minted section is stored
  anchored on the arc its edge uses (parameter zero there, the centre
  derived, never stored), so its resolution is `u` times the arc
  distance from the anchor, not `u·A`. An imported ellipse keeps the
  file's centre form exactly as read, with the anchored form as an
  internal cache, so export writes the original and round-trips
  bit-exactly; export of a minted ellipse writes the derived centre.
  Evaluators read the anchored form either way. No public API changes:
  nothing public reads the ellipse's fields.
- **No refusal at the arm.** The existing degeneracy trilean stays the
  only door there.
- **PCERT's three chart fixes land now** (`refs`).
- **Interim:** a miss within a carrier's own resolution ends as a
  kernel limitation (D4 ¶1(i)'s last resort), not "kernel defect", and
  nothing that certifies today starts refusing.
- **Deferred:** a certification-time resolution check that refuses with
  a geometry lever waits for D4 ¶4's size range to be built generally;
  it would be that clause's first instance.
- **Owners.** The carrier half is CLEAVE's (note below):
  `a-section-ellipse-is-re-posed-at-its-nearest-principal-vertex` is a
  first step consistent with this end state (it removes the parameter
  term). This decision supersedes the arm refusal in
  `the-section-table-admits-a-curve-on-its-resolution`: the far-centre
  term it would refuse is removed by deriving the centre. The
  chart-image half stays this row's (PCERT's fixes).

## Note from PCERT (2026-10-13)

`focal-section-vertex-azimuth-is-read-off-a-cancelling-sum` is fixed
(`pcert/focal-section-vertex-azimuth`): `chart_pcurve` reads the cone
image's vertex azimuth off the major axis, and the skew-pose whole-turn
miss drops 10–1000× to `u·A/k`. That narrows the chart-image half and
does not close it: at unit scale, ε = 1e-9, the skew-pose cone still
refuses at the image from about `k < 5e-3` (`β`'s own conditioning,
`u·A/k`), and `α = 1.55` still misses by 8.8e-7 at `f = 0.9999`
(`focal-section-image-on-a-flat-cone-misses-by-a-cos-squared-factor`).
The span envelope (`focal-section-envelope-bounds-the-whole-ellipse-not-the-span`)
is untouched.


## Second note from PCERT (2026-10-13)

`cone-section-interval-mint-escalates-on-a-beta-enclosure-of-u-over-k-levered-near-a-parabola`
is fixed (`pcert/cone-section-interval-beta`): the cone arm reads
`√(1 − ecc²)` as the projection's axis ratio `|rb|/|ra|`, which also
removes `β`'s `u/k` at `f64`. So "`β`'s own conditioning" above no longer
limits the image. `conic_past_band_probe` (`FAMILY=cone ROT=1
NO_SPLIT=1 SCALES=1`, ε = 1e-9), whole-turn miss before → after:

| α | f | k | before | after |
|---|---|---|---|---|
| 0.2 | 0.999999 | 3.8e-3 | 1.7e-9, escalates | 1.7e-10, certifies |
| 0.2 | 0.9999999 | 1.2e-3 | 5.6e-8, refuses | 1.4e-9, certifies |
| atan ½ | 0.999999 | 2.4e-3 | 1.4e-8, refuses | 3.3e-10, certifies |
| atan ½ | 0.9999999 | 7.4e-4 | 9.9e-8, escalates | 2.4e-9, escalates |
| 1.0 | 0.99999 | 5.0e-3 | 4.2e-9, escalates | 8.7e-11, certifies |
| 1.0 | 0.999999 | 1.6e-3 | 4.3e-8, refuses | 7.6e-10, certifies |
| 1.3 | 0.9999 | 1.5e-2 | 1.0e-9, escalates | 2.5e-11, certifies |
| 1.3 | 0.99999 | 4.6e-3 | 8.2e-9, escalates | 1.7e-10, certifies |
| 1.55 | 0.999 | 4.5e-2 | 3.6e-8, escalates | 2.4e-9, escalates |
| 1.55 | 0.9999 | 1.4e-2 | 8.8e-7, refuses | 3.1e-8, refuses |

What remains at unit scale is `k` below about 1e-3 (`u·A/k` with `β`
now at rounding) and the flat cone
(`focal-section-image-on-a-flat-cone-misses-by-a-cos-squared-factor`).
The carrier's own rounding, `u·A`, sits at the band there: the same
`α`, `f` and scale in `pcurve_cone_section.rs`'s `skew_section` pose
(another rotation) is refused before any image, at chart incidence
(enclosure `[0, 1.9e-9]`). That is this row's carrier half.

## Third note from PCERT (2026-10-14)

The span envelope (`focal-section-envelope-bounds-the-whole-ellipse-not-the-span`)
is closed as overtaken by PRs 4960 and 5004 (closed in PR 5050). No pose of
this row's cone sweep refuses at the envelope now: at the item's
`f = 0.999999` the consumed arcs miss by 1.0e-10 and certify (envelope
1.2e-10), where the whole-turn miss was 1.1e-8. The refusals left are the
Euler gate's `EndpointStart` on the carrier (this row's carrier half) and a
`MapResidual` sample of the consumed arc itself, which no span bound
reaches.


## Fourth note from PCERT (2026-10-14)

The line above under CLEAVE's note, "the chart-image half … the
α = 1.55 `MapResidual` table, where the carrier is accurate and the
pcurve image is not", no longer holds after #5043. PCERT's
`focal-section-image-on-a-flat-cone-misses-by-a-cos-squared-factor` is
closed as fixed by it (PR 5128).

The `1/cos²α` was the carrier's, not the image's. With the arm's
`K = −D·M` and apex-foot centre, the α = 1.55 table's whole-turn
residuals are about `5·u·A`:
- at ε = 1e-12: S = 1, f = 0.5 reads 3.6e-14, and S = 0.01, f = 0.9
  reads 1.6e-15, both splitting;
- at ε = 1e-9: S = 100, f = 0.5 reads 3.6e-12.

What this row still holds at α = 1.55 is the near-parabola mechanism,
uniform in α: S = 1, f = 0.999 escalates at the edge's `EndpointStart`,
margin 1.28e-12, at ε = 1e-12. The 2026-10-13 β closure noted its own
part the same way.
