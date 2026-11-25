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

**Not bisected.** One run of the row at 1e-12 costs 13 minutes on the nightly's runner. The nights it first failed at 1e-12 were not read either: this lane read the 1e-6 rows.

**Owed.** Name the merge. Then decide: either the section builds at 1e-12, or the refusal is a frontier the row should admit by name, with the reason the residual's floor sits at 1.1e-12 for this pair. `ssi-refinement-can-spend-hours-on-one-branch-before-the-step-wall` is the cost side of the same march.

**Pinned** (`green/eps-1e-6`): the row is under `test_utils::pinned_red::row` at ε 1e-12 on this refusal (and at 1e-6 on another stop, `work/spiricarm/main-red-three-cylinder-pair-volume-pads-pass-the-fixed-budget-at-1e-6.md`). Take the pin off with the fix.

## Seen by `oneround/unsourced-unions` (PR 7294), 2026-11-21

At 1e-12 (release build, the branch's base: PR 7227 with main at `b80352cd`), `verbs_cylcyl_r1_review_probes::every_reachable_crossing_pose_refuses_typed_or_answers_correctly` was red in both modes of a differential (each site under test on its old read, and on its new one), with identical payloads. It stopped somewhere the pinned row does not name: `skew axes, crossing walls/intersect: ERR Euler(RebasedCarrier { edge: EdgeKey(17v1), error: PeriodicView(Span { asked: (-0.9887583186488748, 0.01124168…` (`crates/sweep/tests/verbs_cylcyl_r1_review_probes.rs:236`), not at the tall thin pair's limb 2. The row's pin names only that stop.

## Closed (2026-11-25, `ssiarith/tall-thin-limb-2`)

**The refusal no longer happens on main.** Measured on main `6003877ad8`
(release, the pair alone, each op through the row's own
`differential::outcome`):

| ε | union | subtract | intersect |
|---|---|---|---|
| 1e-12 | OK SOUND, pad 3.8e-12 | OK SOUND, pad 3.7e-12 | OK SOUND, pad 4.1e-12 |
| 1e-9 | OK SOUND, pad 1.7e-10 | OK SOUND, pad 1.2e-10 | OK SOUND, pad 1.7e-10 |
| 1e-6 | OK BAD, pad 5.8e-7 | OK BAD, pad 4.4e-7 | OK BAD, pad 5.8e-7 |

Every volume is the closed form's to the digits printed. At 1e-6 each
enclosure is wider than the row's fixed pad budget, the class of
`work/spiricarm/main-red-three-cylinder-pair-volume-pads-pass-the-fixed-budget-at-1e-6.md`
(evidence added there); the row stops on its first pose there before
reaching this one.

**Not bisected.** The merge that took the section past limb 2's band
was not named: one run of the full row costs about 13 minutes hosted,
and the row had already moved to an earlier stop. The nightly of main
`6a680cb845` (run 45332993102) is red on that earlier stop:
`skew axes, crossing walls/intersect: ERR Euler(DescriptionUncertified { … PeriodicView(Span { asked: … }) })`,
the class of `work/sectarms/main-red-the-tilted-rods-other-spins-row-asks-a-periodic-view-past-its-window.md`
(evidence added there).

**The row.** The tall thin pose joins the poses whose every op answers,
so the row now holds it SOUND at the default ε; the 1e-12 pin names the
skew pair's stop instead of this one. `ssi-refinement-can-spend-hours-on-one-branch-before-the-step-wall`
stays open: the cost side was not measured here. — (SSIARITH orchestrator)
