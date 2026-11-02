---
id: the-gui-cannot-author-a-profile-of-several-loops
kind: issue
title: The GUI offers no way to author a profile of several loops; the bore preset is the only one
status: open
opened: 2026-11-02
priority: P0
cost: M
---

Reported by Ev (in chat, 2026-11-02): "it is not clear how to actually
create a profile with multiple loops in the gui". Filed by the VSEAM
orchestrator.

## What is there

A profile's program holds any number of loops, and containment decides
which are holes. `validate` orients each loop as outer or hole
(`crates/profile/README.md`, V3). The library authors these freely, and
a plate with three bolt holes is one profile. The GUI cannot:

- The add-profile form (`crates/viewer/src/pane/create.rs`, the
  profile section) offers one shape: circle, rectangle or a path chain.
  The only way it makes a second loop is the circle's "with bore"
  checkbox.
- The profile editor (`crates/viewer/src/pane/profile.rs`) labels loops
  ("loop 0", "loop 1") when there are several, but offers no way to add
  or remove one.

## Why P0

"Something the GUI cannot author at all" (`work/README.md`, Priority).
A profile with holes other than one concentric bore cannot be drawn,
and booleans of extrusions are the only workaround. It also gates
`the-rectangle-and-circle-templates-reopen-as-paths`: Ev ruled on PR
6143 to drop the bore preset ("if profiles are already multiple loops,
then there's no need for a union of profiles"). Dropping it before this
row lands would remove the GUI's only way to make a hole.

## What a fix needs

An "add loop" (and "remove loop") in both the create form and the edit
door. Each loop picks its own starting shape, using the same presets
the form offers for the first loop. The edit door's per-loop rows
already exist, and loops are one program entry each. Neighbours:
`a-profile-cannot-refer-to-its-own-earlier-parts` (a hole centred on
its boss), and `the-profile-editor-grew-past-a-small-window` (more loops
make a taller pane).
