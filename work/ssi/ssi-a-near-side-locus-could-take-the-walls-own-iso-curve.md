---
id: ssi-a-near-side-locus-could-take-the-walls-own-iso-curve
kind: issue
title: A locus the boundary pass certified within Kε of a side could take the wall's own boundary iso-curve as its carrier, instead of a fitted curve the certificate refuses on rational walls
status: open
opened: 2026-10-03
priority: P2
---



(SSI implementer on PR 3862, from Ev's ruling on that PR, 2026-10-03.)

## What

On the plane × NURBS lane a region is reported only where the boundary
pass certifies the locus within ε of a corner or side (C3,
`crates/geom-brep/README.md`). A plane between ε and `Kε` off a side
therefore traces the branch along the edge, and on a rational wall that
branch, a metre long, is refused by the certificate: limb 2, or the fit's
sample budget at fine ε. Ev ruled this acceptable (option (i)): the same
walls refuse identically mid-wall, so it is the certificate's existing
limit, not a boundary defect, and there is no fallback to a region.

Pinned by `a_rational_walls_plane_three_eps_off_its_edge_meets_the_certificate_limit`
(`crates/geom-brep/tests/m5_pr7_ssi.rs`), measured 2026-10-03 on the
plane `x = 3ε`; the plane `x = 1 mm`, mid-wall, answers the same kind
on every wall at every ε:

| wall | ε 1e-6 | ε 1e-9 | ε 1e-12 |
|---|---|---|---|
| twisted bilinear, weights `1, 3, ½, 2.5` | branch, 1 m | `CertificateEscalated` (limb 2) | `FitSampleBudget` |
| biquadratic, centre weight ½ | branch, 1 m | `CertificateEscalated` | `CertificateEscalated` |
| biquadratic, centre weight 9 | `CertificateEscalated` | `CertificateLimb` | `FitSampleBudget` |
| flat square, weights `1, 3, ½, 2.5` | branch, 1 m | `CertificateEscalated` | `FitSampleBudget` |
| flat square, weights `2, ½, 1, 4` | branch, 1 m | `CertificateLimb` | `FitSampleBudget` |

The square of weights `2, ½, 1, 4` refuses the same way at `x = 0.9ε`:
its side's certified distance reads loose with those weights, so even a
plane within ε of the side is not certified coincident and traces.

## The idea ("use what we already know")

When the boundary pass has certified the locus within `Kε` of a side (a
side `SideClass::Apart` in `ssi/boundary.rs`: its strip holds the cover,
but not within ε), the wall's own boundary iso-curve along that side is
a candidate carrier for the branch. It is exact on the wall, so it needs
no fit and the fit budget and limb 2 never see it; limb 1 checks that it
lies on the locus within the band. A coincident flush edge (within ε)
still gives a region, per the ratified clause; this candidate is for the
band between ε and `Kε` only.

Open questions for whoever builds it: the branch's ends are the crossings
on the two adjacent sides, which lie up to `Kε` off the iso-curve's ends,
so the carrier needs its ends joined to them (or the crossings
re-settled onto it); and where the side is only partly within the band,
the candidate covers only that stretch.

## Related

- `plane-nurbs-certificate-bound-does-not-refine-with-eps`: limb 2's
  between-samples bound does not refine with ε, the refusal at 1e-9.
- `plane-nurbs-ssi-does-not-certify-a-curved-dome`: the same certificate
  limit on curved walls mid-wall.
