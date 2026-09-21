---
id: author
kind: program
title: AUTHOR — what the GUI cannot author: the doors a person needs to build geometry without leaving the viewer
status: open
opened: 2026-09-20
area: gui
prefix: author/
tag: (AUTHOR orchestrator)
ab_band: 6800-6899
paths: [crates/viewer/src/forms.rs, crates/viewer/src/props.rs, crates/viewer/src/sketch.rs, crates/viewer/src/drafts.rs, crates/viewer/src/pane/create.rs, crates/viewer/src/pane/properties.rs, crates/viewer/src/session/author.rs, crates/viewer/src/session/refuse.rs]
keep_out: [opened by CHROME's 2026-09-20 priority-seam cut (Ev, in chat) per work/README.md Track size - CHROME keeps the viewer's reported defects and its entrenching architecture and its band 1600-1699, the six prose rows of the same cut went to VDOC whose charter is claims the tree makes about itself, the kernel doors these rows wait on are EDIT's (the DocEdit vocabulary) and WIRE's (the placement lift) - a row here that needs one says so and rides the kernel unit rather than reaching across, crates/viewer/src/app.rs and session.rs are shared with CHROME VSEAM and VGEOM by the 2026-09-17 re-scope and stay shared, announce the seam in the PR]
priority: P0
---

**The standing goal's GUI half.** Ev's very-high priority is authoring
arbitrary geometry through the UI; most of what remains for it is
kernel-side, and this program is the part that is not.

What the viewer cannot do today, each row a door rather than a defect:
nothing in it can mint a `Datum::FaceFrame`, **so a profile cannot be
placed on a picked face at all** — the single most ordinary thing a
person opens a CAD program to do; the add-profile form cannot mint the
frame it needs and names the ones it finds by node number; there is no
`AddPart` op, so a `Part { Instance(i) }` node is reachable only from a
file or from Python; `SessionOp::AddBoolean`'s doc promises a
declaration vocabulary no `DocEdit` provides; there is no duplicate
node (Ev's request); and a parameter row's value field is a bare
`DragValue` with no parser, no unit authoring and no no-op guard, so
the one door that does author cannot be typed into.

Two rows are not missing doors but wrong answers on the authoring
path, and travel with them: the path preview draws nothing once any
authored step refuses, so the author cannot see what to fix, and the
probe reports every negative extrude distance as valid.

Charter and order: `work/author/plan.md`; narrative in
`work/author/log.md`.

## Territory widened 2026-09-21, after the first two dispatches

The opening `paths` named three files and **not one of them is where
either of the first two units did its work**. Every row on this slate
is a creation or property FORM, and those live in
`crates/viewer/src/pane/create.rs` and `pane/properties.rs`, with the
vocabularies they fill in at `drafts.rs`, `session/author.rs` and
`session/refuse.rs`. A program whose declared territory cannot see its
own units tells `work.py territory` nothing, which is the one thing
that list is for: a lane learns at the moment it matters which other
program claims the ground it is on.

Added rather than swapped — the three opening globs stay, `forms.rs`
in particular being where the offering enums live. **All five
additions are shared ground with CHROME**, and with VIEW, VNEWS,
VSEAM or VGEOM depending on the file; that is legitimate and expected
(`work/README.md`, 2026-09-20: shared ground is fine, what is owed is
awareness while a lane is LIVE). The seam is announced on CHROME's
log. No `keep_out` clause is written for the overlap, because none of
it needs explaining beyond this paragraph.
