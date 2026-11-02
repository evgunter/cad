---
id: members-read-searches-the-family-once-per-reader-member
kind: issue
title: The viewer's members_read collects every member of a family and searches it linearly once per reader member
status: open
priority: P3
cost: M
opened: 2026-11-02
---


Found by a designer on the WIRE fork `a-family-has-no-member-ceiling` (2026-11-02); not witnessed.

- `crates/viewer/src/world.rs:132` `members_read` collects every member's environment from `Doc::member_envs`, then runs a linear `position` search for each member of the reader. That is O(N_reader × N_family).
- `world.rs:170` `body_count` calls `.count()` on `member_envs`, binding every member's environment just to count them.

Both work out membership from the recipe, although the evaluation already holds it (`Members`). If they read the evaluation instead, they would be bounded by whatever the ceiling fork decides. The sibling is `a-per-member-slot-reading-lists-every-member-eagerly.md`.
