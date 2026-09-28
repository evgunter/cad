---
id: drafts-moved-true-arm-covers-a-held-state-that-refuses
kind: issue
title: drafts' ProfileEdit::moved answers true for a held state that does not lower, under a name that says only moved
status: open
opened: 2026-09-28
priority: P4
cost: E
---


Found by the sibling sweep of
`folded-moved-true-arm-covers-a-fold-that-did-not-move`: the same
shape, a bool whose name covers fewer states than its `true` arm.

## The mismatch

`crates/viewer/src/drafts.rs`, `ProfileEdit::moved` (~`:385`). Its
doc line asks *"whether applying would write anything"*, and its body
answers `!untouched`, where `untouched` holds only when the held state
lowers AND has the committed program's structure AND its edit list is
empty. So `true` covers two states: a held state that edits the
program, and one that **does not lower or does not match the
committed structure** — the doc comment says so (*"counts as moved:
applying it is how the refusal is said"*). Applying the second writes
nothing; it refuses. The name (*moved*) and the doc's first line
(*would write anything*) both say the narrower thing.

The value is right for its callers: `pane::profile`'s edit-mode pane
(`crates/viewer/src/pane/profile.rs`, `let moved = edit.moved()`, ~`:93`)
feeds it to `apply_and_revert`, which enables Revert on it and Apply on
`moved && !refused`, and a held state that refuses is exactly one the
user should be able to revert. So, as with `folded_moved`, this is a
prose-and-name row, not a behaviour one.

## The fix

Name the question the callers ask — whether the held state differs
from the committed program at all (it edits it, or it cannot be read
as it) — and make the first doc line say that; re-word `apply_and_revert`'s
parameter and the `moved` locals in `pane/profile.rs` with it.

## Home

`drafts.rs` is claimed by `author` and `chrome` (`work.py territory`),
and the only production caller is `pane/profile.rs`, which is this
program's; filed here because the words are what the row is about.
Announce on `author`'s log when it is taken.
