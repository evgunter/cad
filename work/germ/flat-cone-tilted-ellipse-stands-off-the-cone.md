---
id: flat-cone-tilted-ellipse-stands-off-the-cone
kind: issue
title: A flat cone's TiltedEllipse stands far off the cone where k = c² − sin²α cancels
status: open
opened: 2026-10-13
---

`plane_cone_section`'s `TiltedEllipse` lane
(`crates/geom-brep/src/intersect.rs`, the `pn_conic_type` Negative arm,
`let k = c.powi(2) - sin_a.powi(2);`, main line ~2420) serves ellipses far
off the cone for a flat cone (`π/2 − α ≈ 1e-4`) cut off the apex by a
plane near square to the axis. There `c ≈ 1` and `sin α ≈ 1`, so
`k = c² − sin²α` cancels: each of `c²` and `sin²α` carries rounding of
order `u`, against a `k` of order `(π/2 − α)²`, and the semi-axes
(`|δ|·sin α·cos α/k`, `|δ|·sin α/√k`) and the centre (`δ/k`) take that
relative error whole. Nothing in the arm charges it: the conic-type
trilean decides only the sign of `D`.

**Measured** (holdout review of PR 4966, reviewer A's probe, `ε = 1e-12`):
ellipses of 1e3 to 3e4 m stand up to `1.1e7·ε` off the cone, 84 of the
probe's 397 served ellipses. The counts are identical on main and on PR
4966's head `5c565c0a`, whose change reads only the seam; this lane reads
no seam.

**Related:** `work/germ/conic-section-just-past-its-band-fails-certification-downstream.md`
(a conic just past its band that fails certification downstream: the
same lane, a different pose family).

**What is owed:** either compute `k` without the cancellation (e.g.
`(c − sin α)(c + sin α)` with `c − sin α` read from a well-conditioned
form such as `cos(α) − cos(angle(a, n))`-type identities), or charge the
cancellation in the lane's admission so such poses escalate typed rather
than serve an ellipse off the cone; then pin the probe's worst pose as a
row.
