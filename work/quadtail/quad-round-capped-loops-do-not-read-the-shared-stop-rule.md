---
id: quad-round-capped-loops-do-not-read-the-shared-stop-rule
kind: issue
title: the quadrature loops stop on a round count, not on the crate's stop rule (crate::descent)
status: closed
opened: 2026-10-12
priority: P4
cost: M
parent: the-measurement-door-refines-until-a-predicate-resolves
closed: 2026-10-26
---


Filed from ENCL's one-stop-rule unit (PR 4800), at the reviewer's finding.

PR 4800 gives `geom-brep` one stop rule for certify-and-refine loops, `crate::descent` (`crates/geom-brep/src/descent.rs`). It has three parts:
- an idle round is one that does not lower the LOWEST certified bound so far;
- the loop stalls on the second idle round in a row (`PATIENCE`);
- work has a per-object cap and a cumulative budget, and a stall speaks over a wall.

The offset fit (`offset_fit::refine_rounds_at`) and limb 2's hull subdivision (`ssi::certify::subdivide`) both read it. The quadrature loops in `crates/geom-brep/src/props/quad.rs` do not. Each runs to a fixed round count:
- `cylinder_cut_face_rounds` (`for round in first..=QUAD_MAX_ROUNDS`, 12);
- `nurbs_patch_face_rounds` (`QUAD2_MAX_ROUNDS`, 6, and `QUAD2_RATIONAL_MAX_ROUNDS`, 7);
- `trimmed_patch_face_rounds` (`TRIM_MAX_ROUNDS`, = `QUAD2_MAX_ROUNDS`).

Each ends on `PropsError::QuadratureBudget`.

Owed: a decision on whether these loops should read `descent`. The questions:
- Is a quadrature round's enclosure width a bound whose "lowest so far" means what it means for a fit?
- Would a stall (two rounds that do not narrow the width) refuse earlier than the round count, and is that earlier refusal right?
- Is the round count a work bound (a wall) in the rule's sense, or a structure choice the resumable `window.first` API depends on?

If they should not, the answer belongs in `descent`'s module doc, which already names these loops as not reading the rule. Measure first: the per-round width traces over the props suites, in the way PR 4800 measured the offset fit's.

## Closed

Closed into the measurement door (`work/quad/the-measurement-door-refines-until-a-predicate-resolves`). PR 5589 replaced the round caps with one stop rule per lane (`Refinable`/`Floor`/`Wall`, `crate::descent`'s stall as the backstop) under one weighted work wall, and unit C removed the last round-capped reading, the interim number doors.
