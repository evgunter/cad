---
id: a-takeovers-rebind-cannot-reach-part-spelled-names
kind: issue
title: A takeover of an instance's copy cannot Rebind its part-spelled references: Rebind holds from.node to the host's mint log
status: closed
opened: 2026-10-26
priority: P3
cost: M
design: true
closed: 2026-11-01
---

Filed by D′3 (PR 5636) on the INTENT orchestrator's 2026-10-26 ruling on AUTHTAIL's stale-mate question 3 (`work/intent/log.md`, 10-25 entry). AUTHTAIL's takeover design writes a `DocEdit::Rebind` for each carried name read in the copy (`work/doors/a-mate-on-a-taken-over-copys-face-goes-stale.md`, `## Decided`). `Rebind` holds `from.node` to the host's mint log and refuses `RebindUnknownName` otherwise (`crates/editor-core/src/edit.rs:2841`). A takeover of an instance's copy reads names spelled in the instance's part, whose nodes the host never minted, so it cannot rebind those references and leaves them stale.

**Decide when dispatched:** whether `Rebind` admits a name whose node is minted in the part an instance in the host reads, checked through the instance (`DocEdit::UpdateReference`: no edit door reads across the seam). The other option is for the takeover to leave such references stale, with their diagnosis.

## Closed

Closed in the PR 6264 fix pass, which corrected the mechanism. `Rebind` already admitted a part-spelled `from` or `to` where its body reaches an instance (the `foreign` carve-out in the `DocEdit::Rebind` arm of `crates/editor-core/src/edit.rs`). The refusal a takeover of an instance's copy met was `NameStepNeverMinted`, not `RebindUnknownName`: the result's name for the face is the host's (a fillet of the instance, `From { read, of: <the part's name> }`), so the outer carve-out did not apply, and `check_name_steps` then walked into the embedded part name and held its `Lateral` step to the host's mint log.

The exemption now follows each embedded name. `check_name_steps(.., parted)` stops at every level spelled in ids the host never minted (`StableName::piece_steps_outside`, `crates/editor-core/src/names/role.rs`), at all four edit-door sites: the selection insert door, `check_appearance_name`, `check_payload_refs` (an instance record's `outer`), and `Rebind`'s target. The load door's payload exemption (`persist/check.rs`) now reads "where an instance is" as the edit door does, for an instance record's `outer` among them. A part-spelled level is the part's to check at evaluation (`work/intent/a-part-name-in-the-host-is-spelled-in-the-parts-ids.md`); no edit door reads across the seam. The row is `crates/editor-core/tests/takeover_door.rs::an_instances_copy_taken_over_with_a_lateral_face_held`.
