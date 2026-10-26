---
id: a-loft-set-members-cannot-add-a-section
kind: issue
title: SetMembers on a loft must keep its section count: a section it adds has no station to stand at
status: review
pr: 5748
opened: 2026-10-18
priority: P2
cost: M
parent: transform-retires-into-a-placement
branch: intent/loft-section-stations
---


In D a loft's sections after the first each carry a station: `LoftSection { profile, station }` (`crates/editor-core/src/node.rs`), the slides and spins that carry the first section's plane to theirs (fork-log row 161, D10 `[variables.kinds]`). `DocEdit::SetMembers` handed a loft re-points the section reads in place and keeps each station (`crates/editor-core/src/edit.rs`, the `Node::Loft` arm of the members edit). A different number of sections refuses `EditError::LoftSectionCount { node, sections, given }`, because an added section has no station and a dropped one's would be lost silently.

Before D, a section's plane was its profile's datum, so `SetMembers` could add or drop a section freely. D narrows that edit.

**Done when** a members edit that adds a section says where it stands. One shape is an explicit station argument at the door (a `SetLoftSections { node, sections: [(read, station)] }`, or a station list beside `members`); another is a dedicated insert/remove-section edit. Either way the station is written, never defaulted to the first section's plane, since a loft through coplanar sections is degenerate. Raised by the orchestrator on PR D.

## Handoff

PR 5748 adds `DocEdit::SetLoftSections { node, first, then: Vec<LoftSection<Operand, Formula>>, fresh }` (`crates/editor-core/src/edit.rs`). It writes the sections whole, each one after the first with its station, and keeps the degree. The rewritten loft goes through the insert door's `lower_node` and dimension check and is written by `write_reads`. A node that is not a loft refuses `EditError::SetLoftSectionsOnNonLoft`. `SetMembers` on a loft still keeps the count, and its `LoftSectionCount` recourse names `SetLoftSections`. Python has `DocEdit.set_loft_sections(node, first, then)` in `Node.loft`'s shape; the viewer authors no loft sections. Tests: `crates/editor-core/tests/intent_loft_set_sections.rs` and `crates/pncad-py/tests/test_loft_sections.py`.

The review's fix pass added `EditError::LoftShape { node, fault: LoftFault }`. It is asked by `check_loft_shape` from `check_written_node`, so both doors ask it, and from `set_slot`: fewer than two sections, an empty station, or a degree outside `1 ..= sections − 1` under the current values. The insert door and `SetLoftSections` lower through one helper (`lower_authored`). The load door does not ask the shape: an old snapshot loads and evaluation refuses it. A station that moves nothing by value is filed as `a-loft-station-that-moves-nothing-refuses-only-at-evaluation`.
