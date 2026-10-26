---
id: d-prime3-fallback-diagnosis-blames-the-sources-minting-node-for-a-re-pointed-copy
kind: issue
title: On D′3's branch a name stale in a re-pointed copy is diagnosed against the source's minting node ("Extrude B's payload differs"), not the copy that changed
status: parked
opened: 2026-10-25
priority: P3
cost: E
rides_with: assemble-is-an-operator-and-the-product-is-a-slot
blocked_on: [assemble-is-an-operator-and-the-product-is-a-slot]
---


Filed by AUTHTAIL from a designer's read of PR 5636's branch (likely; read from `ResolveError::vanished_fallback` in `crates/editor-core/src/resolve/mod.rs`, not run). On main, a mate on a face of world copy C goes stale when a fillet or mirror takes C over, and the diagnosis names `Place C`, which is true: C's payload changed. D′3 makes copies carry their source's names verbatim, so the name's minting node becomes B's extrude, and the no-prior fallback would say "Extrude B's payload differs", which is false. The takeover re-seat that removes the everyday staleness is AUTHTAIL's `a-mate-on-a-taken-over-copys-face-goes-stale` (parked on D′3). A stale reference that remains, such as a consumed face, still needs a true diagnosis.
