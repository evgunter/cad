---
id: d-prime3-fallback-diagnosis-blames-the-sources-minting-node-for-a-re-pointed-copy
kind: issue
title: On D′3's branch a name stale in a re-pointed copy is diagnosed against the source's minting node ("Extrude B's payload differs"), not the copy that changed
status: open
opened: 2026-10-25
priority: P3
cost: E
---


Filed by AUTHTAIL from a designer's read of PR 5636's branch (likely; read from `ResolveError::vanished_fallback` in `crates/editor-core/src/resolve/mod.rs`, not run). On main, a mate on a face of world copy C goes stale when a fillet or mirror takes C over, and the diagnosis names `Place C`, which is true: C's payload changed. D′3 makes copies carry their source's names verbatim, so the name's minting node becomes B's extrude, and the no-prior fallback would say "Extrude B's payload differs", which is false. The takeover re-seat that removes the everyday staleness is AUTHTAIL's `a-mate-on-a-taken-over-copys-face-goes-stale` (parked on D′3). A stale reference that remains, such as a consumed face, still needs a true diagnosis.

## Released (INTENT, 2026-10-26)

D′3 merged (PR 5636). `ResolveError::vanished_fallback` is still on main, so the row stands on main as filed. Run it there before fixing it: the row was read on the branch, not run.

## Slate triage (2026-10-28, INTENT)

Moved from `work/intent/` by `git mv`, id and body unchanged: residue of INTENT's D10 build that belongs to REFS's charter (names scoped by their read). INTENT keeps D10's core (G and its riders, the stage-3 umbrellas, the carry, the refactor rows, the roots rename); Ev asked for INTENT's slate to be cut to its charter and its residue handed on (in chat, 2026-10-28).
