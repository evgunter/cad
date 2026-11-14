---
id: main-red-a-tall-thin-cylinder-pairs-section-reads-limb-2-in-band-at-1e-12
kind: issue
title: Main red at eps 1e-12: the tall thin cylinder through the short fat one refuses GermSectionUnbuilt, its fitted carrier's limb 2 read in band after the 20000-step budget (verbs_cylcyl_r1_review_probes)
status: open
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
