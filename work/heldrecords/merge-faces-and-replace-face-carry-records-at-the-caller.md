---
id: merge-faces-and-replace-face-carry-records-at-the-caller
kind: issue
title: merge_faces and replace_face return construction state and leave their record carry to the caller, so no gate reads what they carried
status: open
opened: 2026-11-23
priority: P2
cost: M
---


Left open by PR 7124 (`embed/records`, 2026-11-23): not that unit's
(those doors return construction state, so there is no gate to feed),
and no row held it.

`MergeCoplanarOutcome::carry_contacts`
(`crates/topo/src/merge_faces.rs:173`) and the face-offset verbs'
`carry_contacts` (`crates/topo/src/replace_face.rs:1861`) hand the
caller a function that carries records through the door's lineage. The
caller applies it, or does not, and finishes the body with whatever it
then holds: the split, the shell and the blend carry inside the door
and refuse there (`Carry`), these two do not.

**Owed.** Each of the two finishes what it built, or returns a `Built`
that already records what it carried, so the records reach a gate by
the door's own hand.
