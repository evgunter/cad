---
id: a-profile-reshaping-replays-a-member-indexed-step-outside-a-member
kind: issue
title: a profile whose step reads a member index is refused by the edit door, which replays it outside any member
status: open
opened: 2026-11-01
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
