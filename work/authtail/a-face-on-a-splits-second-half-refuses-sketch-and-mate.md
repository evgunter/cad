---
id: a-face-on-a-splits-second-half-refuses-sketch-and-mate
kind: issue
title: Sketching on or mating a face picked on a split's second half refuses: the gesture drops on_body's port and reads the split's first output
status: closed
opened: 2026-10-25
priority: P1
cost: E
closed: 2026-10-25
pr: 5693
---


Found by the re-verify of PR 5683 (the mirror tool), which fixed the same bug in its own door by keeping the port through `world::seat_read`. Two sibling sites still drop `world::on_body`'s body index:

- `DocSession::sketch_on_face` (`crates/viewer/src/session.rs`): a face on a split's lower-half copy refuses `WrongNodeKind { Split, wanted: Body }`.
- The mate tool's `on` side (`crates/viewer/src/matetool.rs`, where it lowers a face pick): it reads `Node(split)`, and the commit refuses `AmbiguousOutput { ports: [above, below] }`.

Both refuse typed, so neither draws a wrong picture. But neither gesture can author against the second half of a split. The repair is the mirror door's: keep the port. The fixture is the mirror suite's `a_face_on_a_splits_lower_half_reads_that_half` (`crates/viewer/tests/mirror_authoring.rs`, once PR 5683 lands): a block split, its lower half placed off the identity, a face clicked on that half. Sweep every other `on_body` caller for a dropped index.

## Closed

PR 5693. `DocSession::sketch_on_face` and the mate tool's `on` side keep `world::on_body`'s port through `world::seat_read`, as `add_mirror` does. Rows: `sketch_on_face::a_face_on_a_splits_lower_half_is_sketched_on_that_half`, `mate_tool_flow::a_face_on_a_splits_lower_half_is_mated_on_that_half`. The sweep of every `on_body` caller, and of the `copy_of()` / `seat_of` shapes, is in the PR body; the remaining hits read no body.
