---
id: tour-hollow-tube-scene
kind: issue
title: tour - hollow tube scene
status: closed
opened: 2026-08-25
github: 986
refs: [960]
closed: 2026-09-04
---

## From GitHub issue 986

Opened 2026-08-25; 0 comments.

VERBS-TUBEWALL (#960) shipped `tube_along_arc_hollow` — the tube door with a wall — but added no tour scene. The unit's acceptance is closed-form mass properties in `crates/sweep/tests/verbs_tubewall.rs`; a scene was deferred deliberately, because it moves a `docs/tess-budget-data/tess-budget-baseline.csv` row and that re-baseline is its own decision.

What the scene should show, from an outside consumer's seat through the public door:

- the windowed hollow elbow (annular section, both ends open) — the case where the bore is visible without any translucency trick;
- the full-period hollow torus, rendered the way the DEMO unit's hollow ring is (translucent, per the loop-tube precedent), so the toroidal cavity reads;
- the STEP frontier for the full-period form: it is a multi-shell CURVED solid, so it is expected to refuse `CurvedShellClassification` exactly as the hollow ring does. Pin it the way `demos/tour/src/klein.rs` pins wall 6 — a self-retiring declared gate (a DIFFERENT refusal, or a success, fails the tour), which is also what turns `docs/KERNEL-VERBS.md`'s currently-unpinned "joins the STEP row" claim into a receipt.

Costs a tess-budget baseline row (four faces at the window, four at the full period). Re-derive per the TESS-BUDGET runbook in the same PR.

## Home

The tube door is VERBS' own (`docs/KERNEL-VERBS.md` is in its `paths:`, VERBS-TUBEWALL shipped it), and the tour scenes are how that register's claims become receipts; the tess-budget re-baseline needs S-CERT's coordination.

## Closed (SHELL orchestrator, 2026-09-04)

Already delivered on main before this program opened, found at the
SHELL opening survey: `demos/tour/src/tubewall.rs` (first commit
38cb556f) renders both scenes this item asked for — the windowed
hollow elbow (`hollowelbow`, opaque, the bore visible into a cap)
and the full-period hollow torus (`hollowtorus`, translucent, the
hollow-ring precedent) — registered in `demos/tour/src/main.rs`;
the tess-budget baseline carries their rows
(`docs/tess-budget-data/tess-budget-baseline.csv`, `hollowelbow`
and `hollowtorus`); and `docs/KERNEL-VERBS.md`'s "joins the STEP
row" sentence is now a receipt — the `hollowtorus` scene declares
the writer's frontier at the body and pins
`CurvedShellClassification` as the third self-retiring probe of
that gate. Nothing remains to build.
