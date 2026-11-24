---
id: ledger-entries-still-argued-as-the-weakest-bound-that-works
kind: issue
title: Bounds ledger entries not yet re-argued as necessity of the read
status: open
opened: 2026-11-20
priority: P2
cost: M
parent: from-computed-claims-a-zero-tangent-at-a-dual
---


From the review of PR 7190, 2026-11-20. The `Bounds` scope rule (`crates/geom-core/src/real.rs`) now asks an entry to show that a bracket READ is necessary and that it decides nothing. PR 7190's fix pass restated the entries whose own text names the read (the census's boxes, `ring_pairs`, the certified quadrature's lift, the clearance engine's lift, `general_chart_image`'s rungs, the at-rest validator's door). The ones below it could not restate from their text, because the code under them reads no bracket of its own: each carries a bracket door to name a callee, or to form a lane.

## Entries owed an argument in the rule's terms

- **SEAT-4, `verbs::run`** (in the `Bounds` trait's own doc): reads no bracket; carries `Bounds` for its callees. Restated to say so and to rest on their entries. Owed: whether a header that only names callees needs an entry at all, or the gate should not count it.
- **M7-8, 2026-09-02, `NurbsLane::certified`** and the lane constructors like it (`FittedLane::certified`, `QuadLane::certified`, `RegionLane::certified`, `ClearanceLane::certified`, `ShellDoor::certified`, and since PR 7190 `SplineFoot::certified`): each holds function pointers and reads no bracket; the compound is the bound of the routines it points at. The entry argues "the compound is forced rather than preferred: no sole bound spells decides AND may certify". `Certifies` now spells "certifies" with no bracket right, so the sentence is out of date, and what forces the bracket door is the pointee's signature.
- **The three pins PR 7190 raised** (`topo/src/props.rs` 11 → 14, `topo/src/shell.rs` 2 → 4, `geom-brep/src/certify.rs` 4 → 6): finished-body doors and a wiring row that name `CertifiedPolicy` or `Certifies` beside a `CertifiedBounds` they already had. None reads a bracket it did not read before; each is counted because the gate fires on the spelling.
- **`carrier_touch`'s three reachability rides** (`curved_face_arm`, `wall_crossing`, `settle_deferred`) and **the census's three `validate` doors**: stated in the ledger as rides that read no bracket.

## What it asks

One ruling for the class: a function that names a bracket door only to call or point at one that reads is either (a) not an entry (and the gate learns to tell it, which needs bodies read), or (b) an entry that cites its callee's and owes nothing else. Then the list above is closed one way or the other.
