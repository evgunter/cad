---
id: the-projection-tool-writes-a-member-read-a-pick-already-seats
kind: issue
title: The projection tool still writes a member read, where the README says projecting a pattern member writes nothing
status: open
opened: 2026-11-01
priority: P2
cost: M
---


Found by `a-new-body-lands-unplaced` (branch `spaces/lands-unplaced`).

`crates/viewer/README.md` (*The mate tool commits into a placement's
bundle*, the landing paragraph) says "Projecting a pattern member writes
nothing: a pick on a drawn member seats `family[k]`." The landing unit
took the world placement off the projection (`DocSession::add_part`,
`crates/viewer/src/session.rs`, now `creating(move |run| run.insert(node))`
over `combine::part_node`), so it no longer places a second copy. It still
writes the member read, a `Place` of `xs[k]` into the family's space by
the identity bundle: the picture then draws that member through the new
node instead of through the family, in the same place.

Writing nothing needs the tool to go, not one more branch in its door:
`SessionOp::AddPart`, `PartSelectSpec`, `ToolKind::Part`,
`combine::PartTool`, the part form in `pane/create.rs`
(`part_selector_rows`, `PROJECTION_IS_PLACED`) and the rows that drive
them (`combine_ops`, `gesture_table`, `node_labels`, `review_gui2_r1`,
`review_gui2_r2`, `select_pick`). A pick on a drawn member already seats
that member (`world::copy_source`, PR 6265's change (a)).

Row: `combine_ops::a_pattern_takes_its_body_over_and_a_member_read_places_nothing`
holds that the member read is one insert and no world placement; it
becomes a row that the op is gone.

## Evidence added by PLACING's INTENT sweep (2026-11-02)

`AddPart` / `PartTool` / `combine::part_node` rebuild the retired `Node::Part` / `PartSelect::Instance` (Stage 3 G) as an identity `Place` of `xs[k]`, where `[repetition]` says `xs[k]` is a read. That is one of the INTENT retirements surviving under another name (Ev, PR 6281, asked for these). The sweep suggests P1 for this row, which is unbanded today. The band is SPACES' call. (PLACING orchestrator)
