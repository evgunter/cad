---
id: main-red-a-tall-thin-cylinder-pairs-section-reads-limb-2-in-band-at-1e-12
kind: issue
title: Main red at eps 1e-12: the tall thin cylinder through the short fat one refuses GermSectionUnbuilt, its fitted carrier's limb 2 read in band after the 20000-step budget (verbs_cylcyl_r1_review_probes)
status: closed
closed: 2026-11-25
branch: ssiarith/tall-thin-limb-2
opened: 2026-11-14
priority: P1
cost: M
---


Found by GREEN's ε 1e-6 lane (`green/eps-1e-6`), from nightly 43371638371 (main `e36675c1`), where the row took 782 s.

**What fails.** `CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p sweep --test all -E 'test(=verbs_cylcyl_r1_review_probes::every_reachable_crossing_pose_refuses_typed_or_answers_correctly)'`, on main `b363f5fa1` (45 minutes in debug on a loaded four-core box):

> tall thin through short fat/union: refused GermSectionUnbuilt, not a frontier refusal: the Boolean cannot build the section where the first operand's cylinder face meets the second operand's cylinder face: ssi: refined over 6 rounds to 18677 samples, then the 20000-step budget stopped it: whether the fitted carrier passes limb 2 (control-hull sup-norm bound) is undecided: margin 1.1170404878857462e-12 lies inside the ambiguity band (1e-12, 1e-11). Recourse: loosen the tolerance; the refused residual stopped falling, so at this ε and scale it is the arithmetic's floor, not the geometry's size, as a last resort; this refusal may indicate a kernel bug worth reporting

The row admits `NodeUnbuilt`, `GermFrameUnsupported` and `FallbackExtentUnsupported` as frontier refusals; `GermSectionUnbuilt` is not one. The raise is `crates/geom-brep/src/ssi.rs`' refinement, stopped by its step budget with limb 2 in band.

**Not bisected.** The merge that took the section past limb 2's band
was not named: one run of the full row costs about 13 minutes hosted,
and by the nightly of main `6a680cb845` (run 45332993102) the row had
moved to an earlier stop, `skew axes, crossing walls/intersect: ERR
Euler(DescriptionUncertified { … PeriodicView(Span { asked: … }) })`,
the class of
`work/sectarms/main-red-the-tilted-rods-other-spins-row-asks-a-periodic-view-past-its-window.md`
(evidence added there). On main `6003877ad8` that stop is gone as well:
the whole row passes at 1e-12 (release, 361 s) and at 1e-9 (62 s).

**The row.** Its 1e-12 pin is off, and the tall thin pose joins the
poses whose every op answers, so the row holds it SOUND at the default
ε and at 1e-12. Its 1e-6 pin, the perpendicular pair's union past the
pad budget, had itself moved on main (pad 6.7e-6 → 2.1e-6, the volume
one digit off the pinned text); it now names the class, `OK BAD` on that
union with its closed form, not the digits.
`ssi-refinement-can-spend-hours-on-one-branch-before-the-step-wall`
stays open: the cost side was not measured here. — (SSIARITH orchestrator)
