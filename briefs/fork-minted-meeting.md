# Design question: what the carry owes a record that lands on a minted point meeting the door cannot decide

You are a designer lane. Read these first:

- `docs/prompts/designer.md` IN FULL, by path. It binds you.
- `CLAUDE.md`.
- `docs/DESIGN.md`: `[topology.records]` and `[topology.tiers]`.
- `crates/topo/README.md`, the tier 3′ carry paragraph. Above all, read "the record lands on every such pair whose interiors meet" and "the boolean's tool decides every such meeting at its door". These were ratified by Ev through PR 7727 (`a24107b0d9`) and earlier. Use `git log --all -S'<short phrase>' -- crates/topo/README.md` to see who wrote what.
- `crates/topo/src/boolean/records.rs` (`landings`, `CarryRefusal`), `crates/topo/src/boolean/lineage.rs` (`MintedBacking`), and the carried door in `crates/topo/src/boolean/ops.rs` (~4620, where the boolean decides minted vertex pairs itself).
- On `origin/main`, the rows `work/heldrecords/main-red-removed-members-refuses-three-slow-carry-rows.md`, `work/heldrecords/the-boolean-door-decides-no-minted-vertex-on-edge-meeting.md` and `a-ring-vertex-minted-in-the-pierced-face-has-no-mints-row` (find it with `work.py`).
- The branch `origin/heldrecords/removed-members-red` (PR 7799) and its review probes on `origin/heldrecords/review-removed-members-red-798492`.

## The problem

A boolean carries its operands' contact records through its lineage. A record lands on each pair of result cells (images of its two cells) whose interiors meet. Where both cells of such a pair were minted by the op, the door must decide the meeting. Today it can decide only a vertex–vertex meeting.

PR 7745 began naming the cut's section edges on the carried path, so that a record whose touch the cut ran along is read as structure. That is needed by `topo three_solids_on_one_line::a_contact_line_beside_a_face_holding_it_builds_in_every_op`.

Since then `sweep a_pierce_held_in_a_curved_face::a_plane_face_holding_the_pierce_over_a_cylinder_builds_in_every_op_and_order` refuses `JoinDesync`. An edge-in-face record lands as follows:

- on (the edge, the kept face piece), a non-minted pair, as it did before;
- and also on (a vertex the cut minted on the edge, a section edge minted in the face). These meet at a point mid-edge, where the vertex lies on the section edge.

The door cannot decide a minted vertex-on-edge meeting. It would need a vertex-to-curve margin, and the section edge has no input cells or Mints row.

The reviewer traced the pierce row: 12 such meetings, each beside a non-minted landing. In every result the same vertex–edge pair is recorded by another route. The row's t3′ findings are only its pinned ones.

## Options seen so far

These are not exhaustive; reject or reframe them.

- **(A) The door decides minted vertex-on-edge meetings.** This needs the section edge's Mints row and a vertex-to-curve margin. It is large and depends on the Mints gap.
- **(B) Stop naming section edges on the carried path.** The pierce builds, but three_solids refuses Stale. A valid body is refused.
- **(C) Set aside such an undecidable minted point meeting where the same record also lands on a non-minted pair.** It is neither landed nor consumed, and nothing is recorded. As the only landing it still refuses. t3′ on the result is the backstop. The reviewer notes that the pierce row's t3′ assertion admits any `UnrecordedContact`, so that backstop is not held mechanically there. (C) also leaves "decides every such meeting at its door" false as written.

## What needs deciding

The final state of the code, and of the README clause, for a record that lands on a minted meeting its door cannot decide. Is a meeting between a cell the op minted on the record's own cell and a section cell a landing of the record at all, or something else? For example, the op's own new contact, which the op's output records under `[topology.records]` rather than the carry. Is (C) a sound interim or an unsound one, and what would make it a final state? If (A), what is its smallest form? The question may sit at another layer; that is yours to judge.

## Constraints

- Read only. Change nothing, push nothing, and write nothing in the repo or the session scratchpad.
- Private scratch: `~/.local/share/cad-work/heldrecords/minted-designer-<your letter>/`. Do not read anything else under `~/.local/share/cad-work/heldrecords/`.
- Report per `docs/prompts/designer.md` §5, ≤150 lines, by `mcp__claude-code-remote__send_message` to `session_018u3gVKp4QiofNJ9Z1mzgD8`.
