---
id: a-loft-set-members-cannot-add-a-section
kind: issue
title: SetMembers on a loft must keep its section count: a section it adds has no station to stand at
status: open
opened: 2026-10-18
priority: P2
cost: M
parent: transform-retires-into-a-placement
---


In D a loft's sections after the first each carry a station: `LoftSection { profile, station }` (`crates/editor-core/src/node.rs`), the slides and spins that carry the first section's plane to theirs (fork-log row 161, D10 `[variables.kinds]`). `DocEdit::SetMembers` handed a loft re-points the section reads in place and keeps each station (`crates/editor-core/src/edit.rs`, the `Node::Loft` arm of the members edit). A different number of sections refuses `EditError::LoftSectionCount { node, sections, given }`, because an added section has no station and a dropped one's would be lost silently.

Before D, a section's plane was its profile's datum, so `SetMembers` could add or drop a section freely. D narrows that edit.

**Done when** a members edit that adds a section says where it stands. One shape is an explicit station argument at the door (a `SetLoftSections { node, sections: [(read, station)] }`, or a station list beside `members`); another is a dedicated insert/remove-section edit. Either way the station is written, never defaulted to the first section's plane, since a loft through coplanar sections is degenerate. Raised by the orchestrator on PR D.
