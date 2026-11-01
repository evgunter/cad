---
id: flat-cone-tilted-ellipse-stands-off-the-cone
kind: issue
title: A flat cone's TiltedEllipse stands far off the cone where k = c² − sin²α cancels
status: closed
branch: germ/flat-cone-tilted-ellipse
refs: [conic-section-just-past-its-band-fails-certification-downstream]
opened: 2026-10-13
closed: 2026-10-14
pr: 5043
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

## Fixed (`germ/flat-cone-tilted-ellipse`)

**Root cause: two cancellations, one in K and one in the centre.**
Measured with an mpmath oracle of the stored cone and plane (60
digits), then with `test_utils::cone_cut::ConeCut::ellipse_miss`
(double-double):

- **K.** `c = a·n` and `sin α`, both near 1, carry the plane's tilt θ
  off square and the aperture's complement β = π/2 − α only to `u/θ`
  and `u/β`. Their difference of squares hands K that error whole. The
  same K is `−D·M`, read off `s = ‖a×n‖` and `cos α`, which carry θ
  and β to `u` (`plane_cone_section`, the `pn_conic_type` Negative
  arm, `let k = (T::zero() - discr) * (sin_a * s + cos_a * c.abs())`,
  `crates/geom-brep/src/intersect.rs` ~2444).
- **The centre.** `apex − (c·a − sin²α·n)·δ/K` reads a vector whose
  normal component cancels to K, with error `u`, and then divides it
  by K: an off-plane miss of `u·A/β`. The centre is now the apex's foot
  on the plane, moved `c·δ·s/K` along the major axis (`let center = apex
  - n * apex_gap + u_major * (...)`, same arm). It lies in the plane by
  construction.

Each is enough on its own to put a flat cone's ellipse 3e2 to 4e5 u of
its size off the cut. Before the fix, the probe's flat cones
(β = 1e-5 to 1e-2) were all off at ε = 1e-12, worst 9.5e7·ε, and even
ordinary α = 1.4 cones stood up to 23 u·size off. After it, every
family (flat, needle, ordinary) stands within 8.4 u·size of both
surfaces, over about 340k served ellipses at the three ε rows.

**No arm charge.** What remains is the stored carrier's own
resolution (`u·A`), which Ev's decision on PR 4812 assigns to the
anchored storage. It is not a refusal at the arm, so the existing
trilean stays the only door.

**Pinned:** `crates/geom-brep/tests/flat_cone_tilted_ellipse.rs`, a
static row of 18 flat-cone poses (each ≤ 0.9 u·size fixed and
≥ 1.4e3 u·size pre-fix), plus a fuzz over flat, needle and ordinary
cones.
