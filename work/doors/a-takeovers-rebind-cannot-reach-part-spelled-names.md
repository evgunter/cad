---
id: a-takeovers-rebind-cannot-reach-part-spelled-names
kind: issue
title: A takeover of an instance's copy cannot Rebind its part-spelled references: Rebind holds from.node to the host's mint log
status: open
opened: 2026-10-26
priority: P3
cost: M
design: true
---

Filed by D′3 (PR 5636) on the INTENT orchestrator's 2026-10-26 ruling on AUTHTAIL's stale-mate question 3 (`work/intent/log.md`, 10-25 entry). AUTHTAIL's takeover design writes a `DocEdit::Rebind` for each carried name read in the copy (`work/doors/a-mate-on-a-taken-over-copys-face-goes-stale.md`, `## Decided`). `Rebind` holds `from.node` to the host's mint log and refuses `RebindUnknownName` otherwise (`crates/editor-core/src/edit.rs:2841`). A takeover of an instance's copy reads names spelled in the instance's part, whose nodes the host never minted, so it cannot rebind those references and leaves them stale.

**Decide when dispatched:** whether `Rebind` admits a name whose node is minted in the part an instance in the host reads, checked through the instance (`DocEdit::UpdateReference`: no edit door reads across the seam). The other option is for the takeover to leave such references stale, with their diagnosis.
