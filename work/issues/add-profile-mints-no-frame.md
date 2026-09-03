---
id: add-profile-mints-no-frame
kind: issue
title: The add-profile form cannot mint the frame it needs, and names the ones it finds by node number
status: open
opened: 2026-09-03
refs: [sketch-frame-from-face]
---

## What

A profile is drawn on a `Datum::Frame` node, and `AddProfile` names one
that already exists — deliberately, so one submit inserts one node and
the frame a person drew on is the frame they can see and edit
afterwards. Two things about the form follow from that and are not
right yet.

**1. An empty document cannot draw a sketch.** The form says

> on frame — none in this document — add a frame datum first

which is true and is a dead end: the commonest first act in a new
document is a sketch on world XY, and it now costs a trip to a
different form to author a frame whose numbers are `(0,0,0)`,
`(1,0,0)`, `(0,1,0)`. The chrome states an obligation instead of
offering to discharge it. The op is right; what is missing is a form
affordance — an "on a new XY frame" choice that commits the frame and
the profile as one gesture, or the add-datum form reachable from here.

Whichever it is, it has to keep the property the current shape was
chosen for: **one submit, one committed edit** is what makes undo walk
back the way a person authored. A two-node gesture is a real decision
about the commit door, not a widget.

**2. The picker names frames by node number.** The combo reads
`feature 3`, `feature 7`. A frame is an oriented plane and the useful
label is what plane it is — "XY at z = 0", "on feature 5's top" — which
is exactly the information the tree row for a `Datum frame` also does
not carry (`tree.rs` labels it `"Datum frame"` and stops). Two frames a
centimetre apart are indistinguishable in the picker today.

## Why it is filed and not fixed

Both are chrome, and both touch the same question the frame-from-face
fork touches (`sketch-frame-from-face`): a frame's LABEL is a different
problem once a frame can be derived from a face, because then the
honest label names the face. Fixing the label first would be work
thrown away if that fork goes the derived way.

(1) does not wait on the fork and could go first.

## Where it stands

`gauth` and `gui` are both closed; this is unowned residue until a
program claims it.
