---
id: sym-solve-atan2-has-no-branch-rule-about-pi
kind: issue
title: Sym's solve_atan2 has no branch rule about π, so an angle near the cut is the whole circle
status: closed
opened: 2026-10-18
priority: P2
cost: E
closed: 2026-10-23
refs: [5613]
---

Interval's `SolveScalar::solve_atan2` (`crates/editor-core/src/eval/mod.rs:3312-3322`) reads an angle across the negative x axis on the branch about π, because the principal branch's cut widens the enclosure to the whole circle there. Sym's (`crates/editor-core/src/eval/mod.rs:3457-3459`) is the plain `geom_core::Real::atan2`. A copy whose chart x lies near −x, with a little width in y, therefore reads a turn that spans [−π, π] on the symbolic tier. That is the `[-3.8, 1.8]` map the INTENT stage 3 D m10 lane measured.

D sets a value's spin from the two charts' frames on the box lanes (`Chart::turned`, `crates/editor-core/src/mate/value.rs`), so a spin value no longer reads it. It is still read at:
- the chart's spin readback (`crates/editor-core/src/mate/value.rs:120`, `spin_at`);
- the coset fold's axis-mate turns (`crates/editor-core/src/mate/coset.rs:1362`, `:1431`, `:1480`, `:1526`).

Those decide at axis-mate translations on Sym.

**Done when** Sym reads the angle through its lane scalar's branch rule (the value channel's branch chosen as Interval chooses it), with a row: an axis mate whose chart x is −x under a box translates the same on Sym as on Interval. If D′ rewrites the chart, it absorbs this.

## Closed

Closed by D′2 (`a-mate-reads-only-geometry-and-a-value-charts-on-the-target-space`, PR 5613, 2026-10-23). `SolveScalar::atan2_branch` (`Atan2Branch { Zero, AboutPi, Principal }`) is the branch rule: Interval decides it from its enclosure, and Sym and Dual read the angle on their lane scalar's branch. Row: `mate::solve::tests::the_symbolic_tier_reads_an_angle_on_its_lanes_branch`.
