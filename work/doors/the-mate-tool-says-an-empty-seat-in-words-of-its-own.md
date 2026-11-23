---
id: the-mate-tool-says-an-empty-seat-in-words-of-its-own
kind: issue
title: The mate tool says an empty seat, and a missing door, in words of its own rather than the seats' vocabulary
status: open
opened: 2026-11-22
priority: P3
cost: E
---


## Finding

Found by FORMWORDS' seat-vocabulary unit (`formwords/seat-vocab`),
which gave every creation form's "what this commit waits for" one home:
`crate::seats::SeatError`, said `no <role> picked yet`, and a panel's
open pick `crate::seats::NONE_PICKED`. The mate tool's panel line
already composes through `seats::picks_line`; its refusals do not.
`crates/viewer/src/matetool.rs`, `MateToolError`'s `Display`:

- `SeatEmpty` says `mate {n}'s {side}{seat} seat is empty`;
- `NoDoor` says `pick the body or the copy to place first`.

Both answer "what is this waiting for" for an empty pick, in a third
and a fourth spelling beside `SeatError`'s. The `seats` module doc
states why the mate tool keeps its own STATE (an interchangeable
pair, faces rather than nodes) and that it shares the panel line; it
does not argue that the refusal words should differ, and no test
holds the difference.

Not taken in the finding unit: `matetool.rs` is not FORMWORDS' ground,
and the mate editor's empty-row behaviour is open on
`work/doors/a-pick-opening-the-edit-door-starts-a-mate-row.md`, whose
answer may change when `SeatEmpty` is raised at all.

## Owed

Say `SeatEmpty` in the seats' shape (`no mate 1 to face picked yet`,
composed from the role words the panel line already uses), or state in
`matetool`'s module doc why the editor's refusal is worded apart, with
the test that holds it.
