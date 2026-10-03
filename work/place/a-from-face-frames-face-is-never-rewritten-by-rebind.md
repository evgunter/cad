---
id: a-from-face-frames-face-is-never-rewritten-by-rebind
kind: issue
title: a FromFace mate frame's face is never rewritten by Rebind, so repairing or rebinding a head strands its frame
status: closed
opened: 2026-10-02
priority: P1
cost: M
branch: place/p2-face
parent: a-from-face-mate-side-cannot-cross-the-split-or-inline-seam
closed: 2026-10-03
pr: 3934
---


Filed by the PLACE orchestrator from the designer pair on `[ev]` #3888 (both designers confirmed it by reading; no row has run it).

## What

`Node::payload_names`' Mate arm (`crates/editor-core/src/node.rs`) lists the two heads and not a `FromFace` frame's face, and `DocEdit::writes_a_mates_datum` (`edit.rs`) says only a mate's insert writes its alignment. So:
- a part edit renames the contact face and the head is repaired by `Rebind`: the frame still names the old row and the side refuses `FaceUnresolved`, repairable only by deleting and re-inserting the mate;
- a head rebound onto an instance of another part: the frame's part-local row is read in the new part, and since node ids repeat across documents built alike, it can resolve silently to a different face.

## Repair

If Ev takes #3888's recommendation, `MateFrame::FromFace` stores no face (its frame is the head's own face) and this closes with that change. Otherwise the frame's face must be rewritten wherever a head is. Pin it with a row: a part-side rename, `UpdateReference`, then `Rebind` of the head.

## Closed

Closed by construction on `place/p2-face`: a `FromFace` frame names no
face, so the head `Rebind` rewrites is the whole of what the frame
reads. Pinned by `p2_face::a_rename_update_and_rebind_carry_the_face_side_with_the_head`
(a part-side rename, `UpdateReference`, then `Rebind` of the head: the
side faults `NoSuchName` between and sits on the rebuilt face after)
and `p2_face::a_head_rebound_onto_another_parts_instance_reads_the_new_heads_face`.
