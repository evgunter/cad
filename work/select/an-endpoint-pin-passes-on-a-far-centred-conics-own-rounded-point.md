---
id: an-endpoint-pin-passes-on-a-far-centred-conics-own-rounded-point
kind: issue
title: certify's endpoint residuals are not fail-closed on a far-centred conic: an end minted as eval(t0) pins while the stored curve passes u*far/2 away, past K*eps at 1e-12 from far 1e6
status: open
opened: 2026-11-19
priority: P3
cost: M
---


Found by the phase-1 probe of ONEROUND's `oneround/conic-vertex`
(2026-11-19), on merge base `b80352cd9`; filed here because
`crates/geom-brep/src/certify.rs` is this program's ground. Probe and
run commands: branch `analysis/oneround/conic-vertex-probes` (never
merged), `analysis/oneround/conic-vertex/` (`probe-c.patch`,
`probe_endpoint_pin`). The class is ONEROUND's
`a-conics-point-is-evaluated-from-its-far-centre-not-its-vertex`,
which listed this site as "fail-closed only (a misread refuses)".

## The site

Check 3 of `run_checks` in `certify.rs` (`carrier_endpoint_start`,
`carrier_endpoint_end`): `|spec.carrier.eval(t) − start|`; Zero pins,
anything else refuses. `eval` sums the point from the conic's stored
centre and rounds at about `0.5·u·far`.

## What main reads

Pose: a tilted ellipse of semi-axes `(far, far/100)` seamed at an
in-box vertex, its centre `far` out, `t0 = 0`; `Band::new(eps, 10·eps)`;
4,000 poses per cell; truth is the stored curve's exact vertex
(`two_product`, `two_sum`).

- `start` offset 0.05ε…30ε from the true vertex: pinned with the true
  gap past 10ε: 0 in every cell. Refused with the true gap under ε:
  168 at ε 1e-12 for far 1e6 and 1e7. That is the fail-closed
  direction.
- `start` minted as `carrier.eval(t0)`, the usual mint: the read is 0
  and it pins, while the stored curve's vertex is 5.0e-13, 5.0e-12,
  5.0e-11, 5.0e-10 away at far 1e4…1e7. Pinned with the true gap past
  10ε: 4000 of 4000 at ε 1e-12 for far 1e6 and 1e7; 0 at ε 1e-9.

So the wrong kind is definite and false (Zero, "the end is on the
vertex", with the true gap past Kε), from far 1e6 at ε 1e-12; nothing
at ε 1e-9 up to far 1e7.

## What a vertex-relative point does

Nothing at the seam: evaluated from the vertex with the vertex summed
in plain f64, the constant is the same rounded `c + u·a` and every
count is identical to main. Only an exact vertex, or one that is
stored, removes it.

## Unprobed

`t0 ≠ 0` (no exact truth was built for a general parameter) and the
other residuals of `run_checks`.
