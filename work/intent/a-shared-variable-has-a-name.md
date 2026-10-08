---
id: a-shared-variable-has-a-name
kind: issue
title: "FORK-7: must every variable have a name? (VR2, VR6, VR7, VR9; Ev on PR 4247)"
status: open
opened: 2026-10-08
priority: P1
cost: E
needs_ev: true
refs: [d10-one-way-to-say-intent-is-unbuilt]
---

Raised by Ev on PR 4247 (the GUI's variables): "do we need to support anonymous variables?" and "why can't the kernel just require that you name it? would this just be so annoying it's not even worth considering?"

A designer pair agreed on the first reports (`docs/DESIGN-FORK-LOG.md` row 87; reports on `design/intent-s2-fork7-A` and `-B`). No name is required, but no variable is unspeakable:
- an unnamed variable has exactly one reader (a slot or a definition), which is how it is spoken;
- a variable two readers share has a name the person or caller gave it; giving an unnamed variable a second reader, or clearing the name of a shared one, refuses until it is named;
- an output is spoken by its operation and port, a name optional;
- the kernel mints no name and the GUI proposes none.

Requiring a name on every variable was weighed and recommended against: it fights D10's mint-on-type, makes every written quantity (`w + 5 mm`, `extrude(depth=12*mm)`) a declaration, and makes inline refuse on nearly every carried variable.

The `[ev]` PR edits D10's Variables paragraph and VR2, VR6, VR7, VR9. Ev's answer closes this row. If approved, a build unit follows: the one-reader invariant at the slot doors, the formula door and the load walk; the GUI's accept-offer on an unnamed variable opens the name field (#4247 choices 3, 8, 9).
