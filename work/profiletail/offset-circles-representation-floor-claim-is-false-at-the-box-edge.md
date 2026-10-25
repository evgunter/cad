---
id: offset-circles-representation-floor-claim-is-false-at-the-box-edge
kind: issue
title: offset_circles claims its C·u·scale representation floor is below ε for any scene the session box admits; at 1 km it is 1.4e-11 m, above CI's 1e-12 row and ε_min
status: open
opened: 2026-10-12
priority: P3
cost: M
---

(Filed by the ENCL lane on `encl/eps-floor` from its sweep for text that claims a size range with no number.)

## What

`crates/profile/src/sugar.rs`, `ArcCarrier::offset_circles`'s doc ("The representation floor is deliberately NOT in the gate") ends: "Its contribution to an accepted corner is at most `C·u·scale`, which is below ε for any scene the (deferred) session box would admit."

D4 ¶4 now fixes the box at 1 km and lets ε go down to `geom_core::tolerance::EPS_MIN` ≈ 2.2e-13 m. With C = `LEVER_ULPS` = 128 and u = 2⁻⁵³, `C·u·scale` at scale 1 km is about 1.4e-11 m. That is above CI's 1e-12 row and above ε_min. The claim holds at the default ε (1e-9 m) only out to about 70 km, and at 1e-12 only out to about 70 m.

## What is owed

Decide whether an accepted corner at large scale and small ε can carry a defect above ε that the gate does not see:
- If it can, the floor belongs in the gate or the refusal.
- If it cannot, the sentence needs the argument that makes it true, with its numbers.

Either way, the sentence names a scale, not "any scene".

## Re-homed at PATHS's close (2026-10-21)

Moved from `work/paths/` to PROFILETAIL (`work/profiletail/`) when PATHS closed. PATHS's live rows went to four programs opened at this close and cut along the priority seam (PROFILE, PROFILEREST, PROFILETAIL, PROFILEPROSE; `docs/doc-ledger/paths-leaves-the-tracker.md`). The item's id, content and history are unchanged.
