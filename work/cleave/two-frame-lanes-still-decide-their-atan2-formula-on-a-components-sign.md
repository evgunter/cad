---
id: two-frame-lanes-still-decide-their-atan2-formula-on-a-components-sign
kind: issue
title: Two frame lanes still pick their atan2 formula on a component's sign, a second cut on a landmark axis: split_conic_phase_frame (a's sign) and props_torus_lift_branch (cos v's sign)
status: open
opened: 2026-10-18
priority: P3
cost: E
---

Found by the round-4 delta review of PCERT B1 (PR 5225). It is outside that PR, which leaves both sites unchanged.

## What is there

PCERT B1 moved geom-brep's chart azimuth frame lane (`stable_azimuth`) off `x`'s sign and onto the side of the principal window's cut line (`pcurve_chart_cut_side`), and removed topo's copy (`split_chart_azimuth_frame`). The old lane put a second cut at `x = 0`, the quarter-turn landmark where directions cluster. A direction exactly there took either formula as a document moved, and the revalue instrument read that as a flip: `revalue_declared_tangency`'s two exact-zero flips, gone with B1.

Two more copies of the old lane remain. Each chooses between `atan2(y, x)` and `atan2(−y, −x) + π` by one component's sign:

- **`split_conic_phase_frame`**, `crates/topo/src/splitting/classify.rs:625`, decides on `a`'s sign (cleave territory). The phase `φ` feeds the roots `φ ± δ`, which later decisions read. So a direction exactly on `a = 0` gives two expressions under one name across a move: the revalue hazard class B1 closed in geom-brep.
- **`props_torus_lift_branch`**, `crates/geom-brep/src/props/curved.rs:4099`, decides on `cos v`'s sign (flux territory). The audit reads it as verdict-neutral, since any branch serves the Green form (`torus_angle`'s doc). The same second cut on a landmark axis is still there.

- **`curve_pair_pose_branch`**, `crates/geom-brep/src/curve_pair/pose.rs` `angle` (from #5268), decides on `x`'s sign through `compare`. It is a third copy, and it reads the first harmonic's phase for `Range::first_harmonic`, which later root decisions read. #5268 landed after this item was written; it is listed here because it is the same shape.

## The fix

Decide each lane on the side of a cut line at a generic angle, as B1's `stable_azimuth` does, so the read's one jump is not on a landmark axis. Better still, read through one helper: `geom_brep::Azimuth::principal`, or a sibling for a window of the caller's own. Each site then has one expression on each side, and the revalue instrument stops seeing a landmark direction flip. Before choosing, check whether `φ ± δ` is decided downstream at the scale of a move. If it is, the topo site is the one to fix first.

