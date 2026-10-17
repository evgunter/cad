---
id: pcurve-envelope-terms-read-orthogonal-components-in-l1
kind: issue
title: pcurve envelope: terms that sum orthogonal components (the sphere pole meridian, the projected cone's drift, the parallels' radial and axial halves, fidelity's channels), a lever up to √2
status: closed
opened: 2026-10-12
priority: P3
cost: E
branch: pcert/envelope-radial-pair-whole
pr: 4952
closed: 2026-10-13
---


(PCERT implementer, the sweep of
`pcurve-envelope-terms-sum-the-cos-and-sin-coefficients-of-a-deviation`,
which fixed the `cos t`/`sin t` pairs; these two are the neighbouring
shape it could not match.)

## What

Two check-4 terms bound one vector by the sum of its components along
orthogonal directions, `|x| + |y|` for `‖(x, y)‖`:

- `incidence`, `SphereMeridian { pole: true }` in
  `crates/geom-brep/src/pcurve_cache.rs`: **orientation**
  `|b·e| + |σ·(a·e) + ‖b_r‖|`, the axial and the radial component of
  the `sin t` coefficient's defect.
- the projected envelope's cone arm in
  `crates/geom-brep/src/pcurve_cache/projected.rs`:
  `drift = |sin − s| + |cos − c|`, the stored half-angle's sine and
  cosine against the twin's, levered by the slant reach — the radial
  and the axial displacement of one point.

Each overshoots by up to `√2`. Both are rounding-scale on a minted
image, so neither is measured to cost a row; an attach-path image that
stands near the band off in both components would read up to `√2` of
it. The fix is `harmonic_sup_scalar` (the exact norm of the pair) at
each site, and a row whose two components put it in `(zero/√2, zero)`.

## More instances (review 4842)

The PR's reviewers found more, each executed or read at its site:

- **The radial and axial halves of one pair** (the second reviewer,
  executed): `ConeRim`, `SphereParallel` and `TorusParallel` in
  `incidence` read a tilted circle's radial deviation in `Radius` +
  `Orientation` and its axial deviation in `Tilt`, and the envelope sums
  the slots. The two halves are orthogonal (radial ⊥ axis), so the sum
  overshoots by up to `√2`; a circle `0.55·zero` off radially and
  `0.55·zero` axially escalates where its true deviation, `0.78·zero`,
  is in band. (The radial slots also carry the lever-3 split filed as
  `pcurve-radius-and-orientation-split-one-radial-pair`.)
- **Fidelity's two channels** (the first reviewer): `fidelity` adds
  `FidelityU` (the azimuth channel in metres) and `FidelityV`, and on
  every periodic chart the two parameter directions are orthogonal, so
  a stored image moved in both reads up to `√2` of its displacement.
  Each channel's own `cos`/`sin` pair is already read exactly.
- **`chart_v_du` in `crates/topo/src/chord_join.rs`** (the first
  reviewer): the gate `split_chart_azimuth_linear` reads the harmonic
  azimuth amplitude as `|pa.x| + |pb.x|`. That one is the `cos t`/`sin t`
  pair shape itself, not this file's ℓ1 shape, but it is in `topo`,
  where `geom-brep`'s `crate::harmonic` is not reachable; the fix there
  is `√(pa.x² + pb.x²)` (or a `pub` home for the helper), and a row whose
  azimuth pair stands in `(zero/√2, zero)` (levered at the radius).
- **Not an instance**: `frame_defect`'s cone term,
  `(d_axis·|cos α| + d_radial·|sin α|)·v_reach` (the second reviewer
  asked). Its two defects are not orthogonal: `d_radial` carries the
  `u_ref` tilt toward the axis, an axial displacement in the same
  direction as `d_axis`'s, so the sum is not loose by `√2` in general.

## Closed (PR 4952, 2026-10-13)

Each instance reads its vector at its norm (rows in
`crates/geom-brep/src/pcurve_cache/radial_pair_rows.rs` and
`crates/topo/src/chord_join/v_du_rows.rs`, each killed-mutant named in
the PR body):

- the pole start's orientation: `σ(b·e, σ·(a·e) + ‖b_r‖)`. No circle
  reaches its band position (a circle's radial part there is second
  order in its tilt), so its row reads `incidence` directly;
- the projected cone's drift: `σ(sin − s, cos − c)`;
- the parallels' radial and axial halves: one `σ` with the radial
  pair (closed with `pcurve-radius-and-orientation-split-one-radial-pair`),
  and the still (zero-winding) classes' `Drift + Tilt` as `σ(a, b)` —
  a sweep hit;
- fidelity: `EnvelopeTerms::total` reads `√(FidelityU² + FidelityV²)`
  where `chart_stretch_sup` answers (cylinder, sphere, torus:
  `S_u ⊥ S_v`); the cone keeps the sum, argued at the site (its chord
  and ruling legs meet with a non-negative cross term), and the
  projected envelope never sets the flag, each with a row;
- topo's `split_chart_azimuth_linear` gate: `√(pa.x² + pb.x²)`, spelled
  locally (`geom_brep::harmonic` stays crate-private), and, a sweep
  hit in the same function, `chart_v_du`'s `∫ |dv|` bound
  `√(pa.y² + pb.y²) + |pl.y|`.

Not instances: `frame_defect`'s cone term (as this item said), topo's
ring length `r·|Δu| + v_var` (an upper bound on `∫ √(r²u′² + v′²)` needs
the ℓ1), the spiric `f_drift` and cap (a scalar triangle; `f` is not
harmonic), and the focal-section envelope's torus `meridional + phase`
(the meridional chord lies in `span(ρ̂, n̂)` and the phase remainder
along `ρ̂`, so they are not orthogonal; each already reads its own pair
exactly).

## PCERT B2a (PR 5244, 2026-10-17)

topo's `split_chart_azimuth_linear` gate is gone: a cylinder image's
azimuth is a `geom_brep::Channel::Angle`, which has no harmonic pair, so
`chart_v_du` takes it by type and reads only the axial channel's pair
(`topo-v-var`).
