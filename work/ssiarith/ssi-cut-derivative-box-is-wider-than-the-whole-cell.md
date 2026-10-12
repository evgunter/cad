---
id: ssi-cut-derivative-box-is-wider-than-the-whole-cell
kind: issue
title: NurbsBoxes::deriv_box cuts then differences, so its cut block is wider than the whole cell's box on a quarter of readings
status: open
opened: 2026-10-10
priority: P3
cost: E
---

Found by NURBS fork3 (designer B, probe `fork3_cut_box_versus_whole_box` on branch `nurbs/fork3-01DupxcD`), off its question.

`crates/geom-brep/src/ssi/enclose.rs:666` `NurbsBoxes::deriv_box` cuts a span cell's row to the tube window, then differences the cut row and divides by `degree/(b − a)`. Under either step form, on 9600 random readings its cut block is wider than the whole cell's box 25% of the time. So cutting makes the reading looser, where it should only make it tighter.

`crates/geom-brep/src/ssi/section.rs:270` `slope_over` does the other order: it differences, then cuts. The blossom primitive that NURBS fork3 builds (`CoeffWindow::blossom` / `restrict`, branch `nurbs/monotone-step-and-blossom`) makes that order the natural one. Re-measure once it lands.
