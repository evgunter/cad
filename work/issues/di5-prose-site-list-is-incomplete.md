---
id: di5-prose-site-list-is-incomplete
kind: issue
title: "DI5's list of prose sites to revise is incomplete: the viewer states the free-move-is-display-only rule in four places, not two"
status: open
opened: 2026-09-04
---

Announce from VIEW, found while VIEW-7 (#1873) documented the
value-gesture / free-move overlap. Filed in `work/issues/` because the
build is **CHROME's** (`work/chrome/no-persistent-setplacement-session-op.md`,
parked) and the ruling is **DOCM's**; a VIEW branch may not edit either
slate (`docs/prompts/implementer-discipline.md` §6).

## The ruling and its site list

`docs/DOCM-IDENTITY-DESIGN.md` is **RATIFIED (Ev, 2026-09-04)**, and
DI5 rules that a free-move release emits one `DocEdit::SetPlacement`:
`CommitFreeMove` becomes the committed edit and `moves` in
`DisplayState` empties. Its consequences paragraph names the prose it
will have to revise:

> G3's sentence "hiding and free-move are display state, never
> persisted" narrows to hiding; `crates/viewer/README.md` and the
> `display.rs` module doc say so, and the round-trip row that pinned
> the old boundary flips to pin the new one.

**Two sites. There are at least four**, and the two DI5 does not name
are the ones that carry an ARGUMENT rather than a statement:

1. `crates/viewer/README.md` — named. ✔
2. `crates/viewer/src/display.rs`'s module doc — named. ✔
3. **`SessionOp::permitted_during_value_gesture`'s doc comment**
   (`crates/viewer/src/session/op.rs`). VIEW-7 added a soundness
   argument here for why a value gesture and a free-move probe may
   overlap, and one of its load-bearing premises is that the free-move
   gesture's previews and committed frames enter no `Doc`. **Under DI5
   that premise goes false**, and the consequence is not cosmetic:
   `CommitFreeMove => true` becomes the only permitted row that
   commits to history while a value gesture is open, while `commit`
   applies against `history.doc()` and the value gesture previews
   against its own snapshot (`Gesture::base`). **That row has to be
   re-decided when DI5 lands**, and the argument does not carry to it.
4. `crates/viewer/README.md`'s gesture-safety clause — a *different*
   clause from the one DI5 names, which gives the naming rationale for
   the same table.

## Why this is worth a row rather than a comment

Sites 3 and 4 both name DI5 explicitly, so they cannot rot silently —
that was VIEW's fix at the merge and it is the reason this is an
announce and not a defect. What it costs is that **DI5's own
consequences paragraph is now an incomplete checklist**, and whoever
takes `no-persistent-setplacement-session-op` will work from it.

The ask is one of two things, and it is DOCM's and CHROME's to pick:
extend DI5's site list, or record that the list is indicative and the
sweep is the implementer's. Either is fine; believing the list is
complete is not.

## The sweep this rests on

`grep -rn "never persisted\|display state\|display-only"` over
`crates/viewer/src` and `crates/viewer/README.md`, then reading each
hit for whether it states the rule or merely mentions it. **Blind
spot:** a site that states the rule in its own words without any of
those three phrases would not match, and sites 3 and 4 were found by
having written one of them rather than by the grep.

Signed: (VIEW orchestrator)
