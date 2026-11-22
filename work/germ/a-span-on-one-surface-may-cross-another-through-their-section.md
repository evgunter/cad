---
id: a-span-on-one-surface-may-cross-another-through-their-section
kind: issue
title: A span on one surface may cross another through their section: the general co-surface crossing rule is unstated
status: open
opened: 2026-11-20
priority: P3
cost: H
design: true
---


## What

A span on surface A crosses surface B exactly where it crosses the section A ∩ B. Read that way, the crossing is a co-surface question on A: the span against the section's curves, through the co-surface arm (`curve_pair::on_surface`). Its 3D gap to B is not read at all. The general rule would route every such span through that arm.

Today only one case does this: `factored_crossing` (C5's curve × face paragraph, `crates/geom-brep/README.md`). It reads an edge on one of two cylinders of equal radii with meeting axes against the other cylinder through their bisector planes. It is reached only where the surface-pair pose decides that pose. Where it declines (at a node, or for a conic in a bisector plane) it refuses typed and never hands back to the quadric door.

## Owed

A design for the general rule. It must say which pairs it serves, how the section is read (closed form or certified section), and how it relates to the `cross` arms the span's kinds route to. The crossing reading's design (C5's U0 revision, branch `germ/crossing-reading-c5`) deferred it.
