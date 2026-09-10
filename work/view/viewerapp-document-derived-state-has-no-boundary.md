---
id: viewerapp-document-derived-state-has-no-boundary
kind: issue
title: ViewerApp's document-derived fields are reset by hand at one door with nothing at the declaration naming the set
status: open
opened: 2026-09-07
refs: [2103]
---


Filed by #2103, which was asked to dispose of the `## Where else to
look` lead in `field-censuses-inside-view-survived-the-debug-sweep`
rather than close over it. That lead named two hand-written field
blocks in `ViewerApp`. **Neither is a field census** — the argument is
below — and the second one is a different defect, which is this row.

## Why neither block is a census

A census is a walk whose correctness argument is that its list IS the
value's fields, so the population comes from the declaration and a
destructure can hold it there. `ViewerApp` (`crates/viewer/src/app.rs:288-477`)
has **32** fields, and neither block claims to cover them:

- `crates/viewer/src/app.rs:809-816`, inside `sync_scene`'s `Ok(mesh)`
  arm, writes six — `scene_generation`, `scene_display`, `scene_focus`,
  `scene`, `revision`, `scene_fault`. Its own comment states the
  population: *"Marked current ONLY on success"*. These are the outputs
  of one rebuild and the key it is current for; a 33rd field
  (`theme`, say) has no claim on the block, and no pattern over
  `ViewerApp` could say which fields do. Ordinary bookkeeping.
- `crates/viewer/src/app.rs:963-966`, the `None if opened` arm of
  `perform_batch`, writes three — `fit_on_scene = true`,
  `fit_delta_on_scene = true`, `budget_delta = None`. Also not a
  census, and for the same reason: most of `ViewerApp` (`theme`,
  `tree`, `input`, `drafts`, `split_dragged`, the key preferences) is
  session chrome that MUST survive an `Open`, so the population is not
  the declaration's.

## The defect the second block is

The population that block draws from is *the fields derived from the
outgoing document*, and **nothing at the declaration marks that set**.
A 33rd field derived from the open document lands outside the reset
with no error and no reader, which is the stale-across-`Open` shape —
material about the last document surviving into the next one.

The layer below has already answered this exact question, differently.
`DocSession::clear_for_new_document`
(`crates/viewer/src/session.rs:1591-1594`) is two statements,
`Derived::none()` and `display.clear()`, and its doc says why
(`:1563-1568`): *"one value rebuilt from nothing ([`Derived`]) rather
than a field-by-field walk each door has to remember"*. The
document-derived half was collected into ONE value, so the census
became a type and there is no list to fall behind.

So this is not the census class and destructuring does not reach it: a
pattern over `ViewerApp` would name all 32 fields and force a decision
about every one of them at a door that is about three. What `ViewerApp`
lacks is the boundary — the `Derived` move, one layer up.

## What it would cost to check

The cheap half is a sentence: the two `fit_*` intentions and
`budget_delta` are what a replaced document invalidates, said at the
declaration rather than only at the door (the door's comment,
`app.rs:957-962`, carries the argument and the declarations do not).
The real fix is the `Derived` one, and it is entangled with
`new-document-owes-the-reframe-open-gets` and `the-picture-key-never-became-a-type`,
which are open on neighbouring halves of the same state.
