---
id: offset-derive-swallows-a-structure-refusal-from-interior-iso-u
kind: issue
title: topo::offset_derive falls back to the march on any interior_iso_u error, so an IsoRowError::Structure (a kernel defect) is swallowed rather than surfaced
status: open
opened: 2026-10-24
priority: P3
cost: E
---


(Filed by the ISO orchestrator from an ISO measurement lane, 2026-10-24, on main `bacd840c77`.)

`crates/topo/src/offset_derive.rs` reads `geom_brep::interior_iso_u` in two places:
- the level-row section (`match geom_brep::interior_iso_u(&net, u0) { Ok(c) => c, Err(_) => continue }`);
- `wall_row` (`interior_iso_u(..).ok()`).

Both fall back to the march on ANY error. Falling back is right for
`IsoRowError::WeightsNotSeparable`: the net's structure is legitimately
not a row, and the march is the general route. But `IsoRowError::Structure`
reports a malformed net, which is a kernel defect, and it is swallowed the same
way. A defect should refuse loudly (D9), not degrade to a slower route.

**Fix (E):** match the variants. Continue or march on `WeightsNotSeparable`, and
propagate or refuse on `Structure`. Add a test that feeds a malformed net and
expects the refusal.
