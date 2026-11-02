---
id: limb-three-reads-no-margin-on-a-long-near-parallel-cylinder-arc
kind: issue
title: Limb 3's one-arc walk reads no margin on a 2 m, nearly parallel cylinder arc, so two 1 mm rods 3e-7 rad apart over a 1 m region refuse TubeNotOneArc after the ruling sweep counts them
status: open
opened: 2026-11-02
priority: P2
cost: H
---


Filed by the SECTARMS cylinder × cylinder ruling-sweep lane (branch
`sectarms/cylinder-ruling-sweep`, 2026-11-02).

## What

`crates/geom-brep/tests/cylinder_pair_section.rs`
`thin_rods_read_crossing_at_the_region_and_refuse_typed`: two 1 mm rods,
axes `5e-4` apart, tilted `0.3·ε/r` (3e-7 rad at ε 1e-9), over a region
of radius 1 m about the origin. Since the ruling sweep
(`geom_brep::pair_section::ruling`) the section is counted without any
3-D sweep: two open arcs, each about 2 m long, `1.9 mm` apart, each with
its two cut ends on the region's sphere.

The march then seeds each arc at its witness and traces it to the
sphere, and the certificate refuses
`SsiError::TubeNotOneArc { rungs: 10, cause: Undecided(..) }` with
`ssi_tube_one_arc` read on an `Invalid` margin: every rung of the tube
ladder (`ssi::certify::limb_three`) was a graph, and the one-arc walk
(`ssi/one_arc.rs`) read no margin it could classify
(`Shortfall::Undecided`).

## Why it is not the count's

The arcs' tubes need a radius under half their 1.9 mm separation, and
each arc is 2 m long, so the chain is long against its tube; the walk's
undecided reading is the certificate's limit on that shape, not the
count's or the seeding's. The 3-D accounting sweep this row's pose used
to refuse at (`CellBudget`, then `TubeNotOneArc` before any accounting)
no longer runs for a ruled pair.

## What a fix has to supply

A one-arc reading that decides on a long, nearly straight chain whose
tube is a small fraction of its length, or a typed refusal that names
that shape (the chain's length over its tube radius) rather than an
unreadable margin. The row re-pins at a carry when it lands.
