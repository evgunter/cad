---
id: transform-rigid-recertifies-images-enclosed-apart
kind: issue
title: transform_rigid maps a body's surfaces and its edge carriers separately, so the images of one widened rigid map are enclosed apart and re-certification loses their correlation: on the chain this is what poisons the wedge
status: open
opened: 2026-10-02
priority: P2
refs: [SYM-15]
---


## What

`topo::transform_rigid` (`crates/topo/src/transform.rs`) maps a body's
pieces separately through one rigid map, then re-certifies every edge
against the mapped surfaces (`EdgeCurve::certify_via`, `:729`):
- the points, then the surfaces (`map_surface`, `:645`);
- each edge's carrier (`map_carrier`, `:679`).

Over a widened map (an interval angle, a symbolic parameter box) each
image is enclosed on its own. Re-certification then subtracts images
whose correlation the map guaranteed and the enclosures no longer
carry: a point of the carrier minus its surface's anchor,
`axial_radial`'s `q = p − anchor` (`crates/geom-brep/src/implicit.rs:104`).

**On the four-link chain this is what poisons the wedge** (SYM-15; the
chain item's "What Phase 1 found"):
- The tip pin's cap-circle point and its cylinder's axis each carry the
  tip's lateral half-width `δ`, so the radial vector is enclosed
  `r ± 2δ`.
- At `2δ = r` the cylinder's gradient (`implicit.rs:220`, `w / radius`)
  reaches zero, and no tangent plane is defined over the box.
- That box does not move with ε. Its onset was bisected by SYM-15's
  review (`sym/15-review` @ `e282874edc`, env `SYM15_POISON_BISECT`) at
  `3.702434e-1`–`3.702470e-1` of the study at two links and
  `1.110986e-1`–`1.110997e-1` at four, at `1e-9`, `1e-6` and `1e-5`
  alike.
- At the default ε it is the chain's certifiable box. The true `‖w‖` is
  `r` at every point of the box, because a rigid map preserves
  `p₀ − o₀`.
- The refusal now names the cause: "the surfaces' tangent planes at
  sample 4 are undefined" (`CertCheck::TangentPlanes`, SYM-15).

## Candidates

- **Transport the source certificate across the isometry.** A rigid
  map preserves every dihedral angle and every residual, so the source
  edge's certificate holds of its image. Re-certifying the image from
  its enclosures reads the same facts through wider boxes.
- **Carry `p − origin` through the map.** Map the radial vector as one
  vector, `R·(p₀ − o₀)`, rather than subtracting two mapped points.
- **The symbolic tier already holds the correlation.** At the poisoned
  sample the cylinder residual (`implicit.rs:166`,
  `(‖w‖² − r²)/2r`) is numerically `[-4.0e-4, 1.23e-3]`, far outside the
  band. It passes only because `Sym::sign_within`'s discharge
  (`crates/geom-core/src/sym.rs:5295`) proves `‖w‖² − r² ≡ 0` (SYM-15's
  review, the same probe). A reading that asked the tier for `‖w‖`
  rather than its enclosure would have it.

## Home

SHELL, whose `program.md` paths hold `crates/topo/src/transform.rs`.
Filed by SYM-15, from the chain item it re-homes.
