# S-REROUTE — plan

**Charter.** Each row names a door (or, for the rustdoc row, the rule in
S-DUP's method item 13) that is already on main; the unit routes the
remaining members onto it, one PR per batch of rows. S-DUP's method items
(`work/dup/plan.md`) bind every lane here; while S-DUP lives, briefs cite
that file by path.

**What differs from S-DUP.**

- **A member that cannot reach the door is not a routing failure.** A
  `src` test module cannot see `tests/common`; a demo is written from the
  public API's seat and does not import `test_support`. The row says which
  members are out of reach and why, and closes without them rather than
  minting a second home — a second home is S-DUP's question, sent back by
  `git mv`.
- **Rows are cheap and many; batch them.** One lane takes several rows in
  one crate, and a batch lands as one PR (Ev, 2026-09-26: combine units to
  spare CI).
- **Gate on local CI while hosted queues** (Ev, 2026-09-26), commits
  marked `[skip ci]`.

No exit criteria are set: the program closes when its slate is empty.
