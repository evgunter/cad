---
id: a-revolves-wall-classification-is-no-door-row
kind: issue
title: A revolve reads a meridian leg as a cylinder (or a plane, or a sphere) from its ends' values, and no door row records it
status: open
opened: 2026-10-29
priority: P1
cost: M
---

Found by INTENT stage 6 PR B's review (probe `r2_probe_split_bore`, adopted as `coincidence_door::a_wall_whose_two_ends_are_two_variables_is_not_one_radius`).

`sweep/src/revolve/axis.rs` `classify_segment` (~458–466) reads a straight meridian leg's radial delta `dr = vb.r − va.r` with `ClearanceDecision::LineRadial`. On `Sign::Zero` it builds `WallKind::Cylinder { radius: va.r }`: the wall's radius is its first end's alone. The rim circles keep each end's own radius (`revolve/surfaces.rs`, `revolve/full.rs`: `radius: verts[i].r`). `LineAxial` (a plane annulus) and `ArcCenter` (a sphere rather than a torus) are the same shape. Each is a coincidence decided from values that no `NamedCoincidence` row records, so the door never asks whether it holds across the family.

The consequence: a bore wall reading `bore` at one end and `bore2` (a second variable of equal value) at the other is a cylinder of radius `bore` while the values agree, and a cone once `bore2` moves. The kernel's carrier-pair verdict (`topo::boolean::carrier_pair_decided`) reads that cylinder at `Sym` and proves the bore against a shaft of radius `bore`. It proves it only when the first end reads `bore` (`shaft_and_bearing::split_bore(true)`), and not with the ends swapped. The door's own two-cylinder reading (`coincide/replay.rs` `one_cylinder`) asks every rim of each wall to be its radius, so it does not prove either order. The kernel verdict's proof is still accepted beside it.

**Done when** the revolve's value-decided wall kinds (`LineRadial`, `LineAxial`, `ArcCenter`) are door rows of the revolve node, or the `Sym` carrier carries both ends of the leg so a verdict reading it sees the second variable. Either way, `split_bore(true)`'s bore rows stop proving, and the pin above flips to unproven in both orders.
