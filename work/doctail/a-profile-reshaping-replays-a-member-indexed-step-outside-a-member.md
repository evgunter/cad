---
id: a-profile-reshaping-replays-a-member-indexed-step-outside-a-member
kind: issue
title: a profile whose step reads a member index is refused by the edit door, which replays it outside any member
status: closed
opened: 2026-11-01
priority: P1
cost: M
design: true
closed: 2026-11-18
---

## Finding

Found by the sweep of `spaces/pattern-value-rows` (readers that evaluate
a family's slot outside a member). With an index `k` declared, typing
`scalar(k) * 1 mm` into a profile step's argument through
`SessionOp::SetSlotExpression` (on `common::parametric_plate`, whose
extrude names the profile's pieces) is refused:

```text
Edit(ProfileProgramRefused { refusal: Resolve { slot: Profile { loop_: 0, step: 0, arg: PointX },
  source: DefinitionRefused { source: IndexOutsideMember { .. } } } })
```

The same text over the extrude's `Distance` slot is admitted and the
extrude becomes a family of three. The refusal comes from
`undrawn_kept_pieces` (`crates/editor-core/src/edit.rs`), which replays
the new program with `written.var_env::<f64>()` — the environment
outside any member, where an index has no value (`Doc::bind_definitions`
refuses it `IndexOutsideMember`). A profile reaching an index is an
operation evaluated once per member (D10 `[repetition]`), so the check
either has to replay it per member (`Doc::member_envs`, which the
viewer's slot rows now use) or refuse it in its own words as a
profile family the door does not hold. Which one is D10's call; the
present refusal names neither.

The viewer's profile editor has the same shape at `held_program`
(`crates/viewer/src/sketch.rs`, `resolve_loops` over `doc.var_env`),
reachable only once this door admits such a profile.

## Banded (2026-11-17, DOCTAIL triage)

P1, M, design. A profile that varies per member of a family cannot be
authored at all (the door refuses it in words that name neither
answer), while the same index on an extrude's distance is admitted.
That is verb breadth beyond the everyday shapes. Which answer applies
(replay per member, or refuse it as a profile family in its own words)
is the open design question.

## Closed

2026-11-18, by replaying per member, as Ev ruled on #7075: the door no
longer replays a profile at all (it asks the lattice, values erased),
and `undrawn_kept_pieces` reads each program's structure. A profile
whose step reads `k` is admitted and evaluated once per member, and
each member's walls are named `Member { (k), of: Lateral(piece) }`
through the generic member machinery (`eval/members.rs`):
`g_index_families::a_profile_reading_the_index_is_a_family_named_by_its_pieces`.
The viewer's `held_program` reads no value on main, and the path
editor holds an index-driven argument pending in its own words
(`drafts.rs`, `unread_drivers`). The coincidence door's on-face bound
still reads `doc.var_env`; filed on COINCIDE as
`the-coincidence-on-face-bound-reads-a-profile-at-the-current-values`.
