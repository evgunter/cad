---
id: a-combine-re-targets-a-copy-another-copy-is-mated-to
kind: issue
title: The combine re-targets a copy that another copy's mate reads, stranding that copy silently
status: closed
pr: 6850
branch: placing/target-row
opened: 2026-11-12
closed: 2026-11-12
---


## Finding

Probed on `main` at `97af755105` (the PLACING fork-target designers'
owed probe, PR 6600). A plate copy placed into a `new_space()` by hand,
and a bolt Placed twice in its own space (copies 1 and 2), copy 2
resting on copy 1's top cap (`SessionOp::AddMate`). Then
`SessionOp::AddBoolean { Union[plate copy, copy 1] }`:

- the union is **admitted with no refusal**: `in_one_space`
  (`crates/viewer/src/session.rs`, the `self_placed` branch) re-targets
  copy 1 into the plate's space by `DocEdit::SetParam` on its
  `OperandSlot::Space`, without asking who else reads copy 1 where it
  stood;
- the outcome's maintenance is one `Maintenance::StrandedSpace` naming
  copy 2's placement, reading the bolt's space and the plate's;
- after the landing copy 2's tree row is `Failed`: "this reads the space
  of Extrude … and the space of Space …, and only a placement relates
  two spaces".

So a combine moves an existing placement as a side effect and strands
another copy's mate, which the person never asked for.

## Disposition

Fixed by the PR carrying
`the-mate-editor-asks-before-moving-a-copy-into-the-targets-space`
(Ev's ruling on `[ev]` PR 6600): the combine no longer re-targets an
existing copy; it refuses a copy in another space, naming that copy's
"into" row, and the probe is pinned as a test there
(`crates/viewer/tests/placement_target.rs`).
