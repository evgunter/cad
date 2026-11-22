---
id: the-plane-arms-harmonic-envelope-reads-zero-between-two-far-centres
kind: issue
title: carrier_harmonic's plane arm differences two far centres uncharged: the envelope reads 0 against a true gap up to 1e-9, and only the sampled schedule stands behind it
status: open
opened: 2026-11-19
priority: P3
cost: M
---


Found by the phase-1 probe of ONEROUND's `oneround/conic-vertex`
(2026-11-19), on merge base `b80352cd9`; filed here because
`crates/geom-brep/src/pcurve_cache.rs` is this program's ground. Probe
and run commands: branch `analysis/oneround/conic-vertex-probes`
(never merged), `analysis/oneround/conic-vertex/` (`probe-c.patch`,
`probe_plane_arm`). The class is ONEROUND's
`a-conics-point-is-evaluated-from-its-far-centre-not-its-vertex`,
which called this site "lean, its difference mostly widens the
envelope".

## The site

Check 4 of `run_harmonic_checks` (`carrier_harmonic`'s plane arm):
`|image.c − carrier.c| + sup(d_a, d_b) + |d_l|·reach`, with
`image.c = origin + u_ref·p0.x + v_ref·p0.y` (`plane_image`) and
`carrier.c = frame.origin`. Both centres are far, so their difference
rounds at about `u·far`, uncharged.

## What main reads

Pose: plane frame `u_ref = (0.6, 0.8, 0)`; a conic of semi-axes
`(far, far/100)` seamed at its in-box vertex; the carrier's centre set
to the f64 image centre, as a mint would. The envelope then reads
exactly 0 while the true centre gap is the image's own rounding:
1.0e-12, 1.5e-11, 1.2e-10, 1.0e-9 at far 1e4…1e7. 400 poses per cell,
`Band::new(eps, 10·eps)`.

- Certified with the true gap past 10ε: 0 everywhere.
- ε 1e-9: certified 400 / 400 / 400 / 103 at far 1e4…1e7 (worst true
  gap 1.0e-9, inside the band).
- ε 1e-12: certified 76 / 0 / 0 / 0; the rest refuse at check 3, the
  sampled `MapResidual` schedule, which reads both points and sees the
  same rounding.

So no wrong certification was found. What is filed is that the
envelope term does not widen here, it shrinks to zero, and the guard
that stands is the sampled schedule and not the envelope.

## What a vertex-relative point does

The site reads `frame.origin` directly and is unchanged. Evaluating
the carrier's point from the vertex moves only check 3: at ε 1e-12,
far 1e4, 187 certify (3 with the true gap in (ε, 1.04e-12], inside the
band); at ε 1e-9, far 1e7, 0 certify, because the image side is still
in centre form.

## The re-posing to try

Compare vertices (`c + a` on both sides, exactly) and the `a`, `b`
vectors. Not built.
