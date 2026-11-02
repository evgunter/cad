---
id: an-empty-profile-is-a-document-state
kind: unit
title: A profile with no loops is an ordinary document state; a sweep of it refuses at the sweep
status: closed
opened: 2026-11-02
priority: P1
cost: M
parent: the-profile-create-form-is-a-second-profile-editor
branch: forms/empty-profile
closed: 2026-11-02
pr: 6393
---

## The unit

Ev, on `[ev]` PR 6388 (2026-11-02): "if this just means 'empty
profiles are ok', then yes this is good". The profile unit's **New
profile** button inserts a profile with no loops, and the one editor
fills it in (`crates/viewer/README.md`, "A create gesture commits the
node"). Today the document refuses that:
`profile::validate_loops` raises `ProfileError::EmptyProfile` at the
edit door (`ProfileProgramRefused`), and `SessionOp::AddProfile`'s doc
says an empty list refuses.

End state:

- A profile node with no loops is admitted at every door: insert,
  `SetProgram`, save and load. It evaluates. It draws nothing, names
  nothing, and its tree row is an ordinary profile row, not a fault.
- Every consumer that sweeps or otherwise needs a region (extrude,
  revolve, loft, and whatever else reads a profile's loops; sweep for
  them) refuses an empty profile **at its own door**, typed, in the
  words `EmptyProfile` already has ("profile has no loops — nothing to
  sweep"), on the consumer's badge.
- The profile editor can remove a profile's last loop.
- Prose that says a profile has at least one loop is corrected where
  it stands.

Ground: `crates/profile` (PROFILE and its tails), `crates/editor-core`
eval (TALLY, WIRE), and the viewer's profile editor. The crossings are
announced in the PR.

## Closed — 2026-11-02, PR 6393

A profile with no loops is admitted at every door and evaluates Ok,
drawing and naming nothing. Extrude, revolve, path sweep and every
loft path refuse it at their own door, in one set of words from
`sweep::EMPTY_PROFILE`, under one tag (`empty_profile`), with the
recourse "add a loop to the profile". The profile editor removes the
last loop, and its preview says the profile has no loops yet.
`ProfileError::EmptyProfile` is gone; validation can no longer raise
it.
