---
id: a-limb-two-refusal-of-an-exact-closed-form-reads-as-an-off-surface-carrier
kind: issue
title: a limb-2 refusal of an exact closed-form carrier reads at rest as 'the declared carrier is off the surface' when the miss is the kernel's degree-1 image
status: closed
opened: 2026-10-14
priority: P2
cost: M
closed: 2026-10-14
branch: pcert/limb-two-image-too-coarse
pr: 5160
---


Found by the review of PR 5097 (`pcert/spline-pair-any-carrier`).

## What happens

Since the spline-pair edge certificates take a closed-form carrier, an
EXACT circle on a spline wall (the arc rim of
`sweep`'s `an_intersection_circle_on_a_spline_wall_is_refused_at_the_door`)
refuses at limb 2 on its degree-1 chart image's chord error
(`edge_nurbs::PXN_IMAGE_DEGREE = 1`, about 4.6e-4 m there). The refusal
renders through `PlaneNurbsRefusal`'s `Limb` arm and `topo::validate`'s
`classify_certify` as a declared carrier that is off its surfaces
("the certificate cannot show the carrier is on them" / the BOUND
reason), and the at-rest recourse speaks to the carrier. The carrier is
exact; the miss is the kernel's own image, which is too coarse for a
curved foot path.

## What closes it

A wording (and an ending) that tells "the carrier is off the surface"
from "the certificate's image of an on-surface carrier is too coarse",
which needs the limb to say which of the two its bound is about, or a
finer image (degree or schedule) for curved closed forms.

## Closed (PCERT, 2026-10-14)

**The tell.** At a break where limb 2's composite reads past the band,
`ssi::certify::nurbs_limbs` now reads the carrier's own foot there, as
limb 1 does: `project_from_seed`, decided at `ssi_on_locus_foot`. A
Zero decision proves the carrier is on the surface at that break, so
the refusal names the new `SsiLimb::ImageMiss`, the pcurve's own miss.
- It keeps `PlaneNurbsHullValue`'s `Unsized::Bound` recourse.
- If the foot reads in band or past it, the refusal stays
  `HullValueChart`, worded as a bound on the miss, never as a defect.

**The words.** Each refusal now names what its number covers.

| where | what changed |
|---|---|
| `PlaneNurbsRefusal::Limb` Display (`edge_nurbs`) | per limb: `HullSup` reads a spline surface through the certificate's own image, and limb 1 found the carrier on that surface at every sample; `HullValueChart` is a bound on the miss, not the miss; `ImageMiss` says the image is too coarse and the carrier is not shown off its surfaces; `Tube` is a uniqueness claim |
| `SsiError::CertificateLimb` Display (`ssi.rs`) | the same split, for the fitted lane |
| `CertCheck::PlaneNurbsHullValue`'s description (`certify.rs`) | names the reading as a bound |
| `CertifyError::PlaneNurbs` (`certify.rs`) | labels "the spline Intersection lane", since the NURBS × NURBS lane refuses through it too |
| topo `validate::classify_certify` | the limb-2 bound reads a spline face "through the check's own image of it"; `ImageMiss` has its own reason |

**Row.** `an_intersection_circle_on_a_spline_wall_is_refused_at_the_door`
asserts `ImageMiss` and its words. The mutant with the tell removed
goes red: it reads `HullValueChart`.

**The finer image** (a knot-aware schedule plus a higher degree) is not
this item's. The measurements are added to
`work/ssiedge/plane-nurbs-certificate-bound-does-not-refine-with-eps`,
the parked design row that already holds that question.
