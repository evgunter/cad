---
id: error-and-check-text-overflows-its-region
kind: issue
title: viewer: error and check text runs off the page or wraps past its region, and the messages are wordy (Ev's request)
status: dispatched
opened: 2026-09-17
priority: P0
cost: E
branch: chrome/wrap-in-region
---

**Ev reported this** (in chat, 2026-09-17). There are two halves.

1. **Layout.** Error messages and check results either run off the
   page or wrap strangely. They should wrap inside the region they are
   drawn in, and not return all the way to the window's left edge.
   Sibling of `the-toolbar-row-does-not-wrap.md`. It may be the same
   egui cause: text laid out against the full available width rather
   than the width of its own container.
2. **Concision.** Error messages should be edited to be shorter. Much
   of the text the viewer shows comes from the kernel's typed refusals
   (`Display` impls across `profile`, `editor-core` and others), not
   from `viewer`. So this half reaches past VIEW's territory, and
   whoever takes it should split off per-crate rows as needed rather
   than editing kernel prose from a viewer lane.

Not investigated when filed.

## A worked example (Ev, 2026-09-17)

The Boolean refusal Ev hit unioning two dumbbell halves is about 280
words. It covers the MAY-vs-DOES box semantics, which pairs are live,
the dispatch table's wiring, and why the refusal is structural.
Nearly all of that is for kernel developers, not the person holding
the mouse. It also opens by naming undeclared coincidence as "the
common case" before saying that this case is a torus×plane pair,
which points the reader at the wrong recourse. The full text is in
this session's chat; a fixture that fails the same way (a torus face
against a plane face in a union) reproduces it.
