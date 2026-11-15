---
id: loft-struts-and-the-hole-bridge-divide-a-placed-difference-by-its-length
kind: issue
title: Loft struts and the hole bridge divide the difference of two placed vertices by its length, against the sweep README's unqualified sentence
status: open
opened: 2026-11-15
priority: P2
cost: M
design: true
refs: [6972, 6936]
---

Found by both reviews of PR 6972 (2026-11-15). Line numbers are at that
PR's reviewed head `032ac185`.

**What is there.** `crates/sweep/README.md`, "Conventions every sweep
keeps" (ratified on PR 6936), says: "No sweep fits a plane to vertices
it placed, or divides the difference of two of them by its own length."
PR 6972 makes that true of caps, walls, rims and the extrude's strut on
the depth arm. It is not true of:
- **loft struts**: `EdgeCurveSpec::line_between(far, near)` and
  `(upper, lower)` (`crates/sweep/src/loft.rs:1326`, `:1344`), and
  `mev_line` (`loft.rs:1513`, `crates/sweep/src/swept.rs:911`);
  `line_between` is `(p1 − p0)/‖p1 − p0‖`
  (`crates/geom-brep/src/certify.rs:1963-1977`);
- **the hole bridge**, the same `mev_line`;
- **the vector extrusion**: its wall normal is `place(d) × (v/‖v‖)`,
  normalised, and its strut `v/‖v‖` (`crates/sweep/src/extrude.rs:694`,
  `:831`). That arm has its own row from PR 6972's fix pass, and STRUT's
  `work/strut/extrude-arc-walls-are-ruled-in-n-not-w.md` beside it.

**Why it was left.** The struts and the bridge are scaffolds: edges
restated or killed before the body is at rest. And no frame holds a
loft strut's direction: it joins two sections' vertices.

**The question, which touches ratified text.** Either the sentence
means "at rest" and should say so (a change to the README, Ev's), or a
scaffold is held to it too and the struts owe a direction decided from
the sections' frames. What decides it: whether a scaffold's box-wide
direction can reach a certificate or a stored datum before it is
killed. Not measured.
