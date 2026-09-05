---
id: refusal-edit-arm-doubles-a-prefix-and-splits-one-mistake
kind: issue
title: Refusal::Edit's wrapper doubles EditError's prefix, and one mistyped parameter now gets two sentences
status: dispatched
opened: 2026-09-04
refs: [set-param-prechecks-what-the-door-refuses, 1846]
branch: view/edit-door-wording
---


Exposed by `set-param-prechecks-what-the-door-refuses` (#1846), which
routed `SetParam`'s undeclared-name case to the edit door. The routing
is right; what it made visible is that the sentence the door produces
is not fit for the status line, and that layer 3 now answers one user
mistake two different ways. One defect seen from three sides.

## 1. The wrapper doubles a prefix, and the name is debug-quoted

The rendered refusal is:

> the edit was refused: edit: parameter "tapper" is not declared, so a
> value edit has no declaration to carry forward — declare it first

`Refusal::Edit` composes `"the edit was refused: {error}"`
(`crates/viewer/src/session/refuse.rs:384`) over an `EditError` whose
own `Display` already opens with `edit: `
(`crates/editor-core/src/edit.rs:900-905`). The name is interpolated
`{:?}`, so it arrives in double quotes. Every flat arm in `refuse.rs`
renders a name bare — compare `NoSuchSlot`'s "node 3 has no radius
slot" and `SelfBoolean`'s "a boolean needs two different bodies" — so
this is the one shape in the vocabulary that reads as a dump.

It is not one arm's problem: 54 of `EditError`'s `Display` arms open
with the `edit: ` literal and 9 interpolate a payload with `{:?}`
(`crates/editor-core/src/edit.rs:771` onward). Every one of them can
reach a user through `Refusal::Edit`.

**Whose half is whose.** The doubled prefix, and whether to wrap at
all, are VIEW's — `refuse.rs:384` is layer 3's composition and layer 3
is where it is read. The `edit: ` prefix and the `{:?}` quoting are
DOCM's, in `crates/editor-core/src/edit.rs`, and DOCM has no reason to
know the viewer's status line renders `EditError` verbatim to a person:
from editor-core's seat a debug-quoted name in a library error is
ordinary. **Nothing here edits `edit.rs`** — that is the handoff this
item exists to make.

The delegation rule VIEW ratified (`crates/viewer/README.md`, *"the
layer that raised the failure names it"*) is what makes the wording
DOCM's to fix rather than VIEW's to paper over.

## 2. The test that should have caught it exercises one arm

`crates/viewer/tests/panel_edits.rs:411-432` is titled *"Every refusal
renders through `Display`, not through a debug dump"* and asserts
`!rendered.contains('{') && !rendered.contains('"')`. It exercises
exactly one arm — `Io`, via a missing file — so its title claims the
universal and its evidence is one point. The `Refusal::Edit` path fails
its stated assertion today and the test passes.

That row is CHROME's glob, so **nothing here edits it** either. What it
needs is to walk the vocabulary rather than one arm; `Refusal` has no
`ALL` value to walk, which is its own small design question.

## 3. One mistake, two sentences, depending on the widget

A parameter name that does not exist now refuses differently by route:

- **typed** into the parameter field → `SetParam` → the edit door →
  "the edit was refused: edit: parameter "tapper" is not declared…"
  (`crates/viewer/src/session.rs:950`)
- **dragged** on the parameter row → `BeginParamGesture` → the layer-3
  lookup → "no document parameter named tapper"
  (`crates/viewer/src/session.rs:977`)

Before #1846 the two agreed, because both went through
`Refusal::NoSuchParam`. Both routes are individually correct — the
gesture really is a lookup with no edit behind it, which is why the
lookup arm stays — so this is not an argument for putting the pre-check
back. It is the observation that the sentence a user sees for one
mistake is now widget-dependent, and nothing in the tree records a
decision that it should be. This half is entirely VIEW's.

## What resolving it looks like

The halves are not independent, so they sequence: DOCM decides
`EditError`'s user-facing wording first (whether a name is debug-quoted,
whether `edit: ` belongs in a library error's text); VIEW then decides
whether `Refusal::Edit` still needs a wrapper and whether the two routes
above should converge; CHROME's test walks the vocabulary instead of one
arm.
