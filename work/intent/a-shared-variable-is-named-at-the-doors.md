---
id: a-shared-variable-is-named-at-the-doors
kind: issue
title: Build FORK-7: an unnamed variable has one reader; sharing refuses until it is named (slot doors, formula door, façade, load walk, GUI accept-offer)
status: closed
priority: P1
cost: M
blocked_on: [operations-define-output-variables]
refs: [a-shared-variable-has-a-name]
opened: 2026-10-08
closed: 2026-10-10
branch: intent/fork7-shared-is-named
pr: 4463
---

Build the FORK-7 ruling (PR 4296; VR2, VR6, VR7, VR9 as rewritten). An unnamed variable has exactly one reader (a slot or a definition; one formula reading it twice is one reader); outputs are exempt. Give an unnamed variable a second reader, or clear the name of a shared one, and the door refuses with a typed error naming the variable to name (`SharedVarNeedsName`). The doors: the slot door, the formula door, the Rust façade and Python (passing an unnamed id twice), and an edit's fresh table (an entry carries a name). The load walk checks unnamed-has-one-reader in place of anonymous-is-read, refusing with the regenerate recourse. VR7's lifecycle becomes "an unnamed variable goes with its reader". In the GUI (after #4247), accepting an unnamed offer opens the empty name field and commits the name and the share as one step. Also fixed by this: `Doc::unparse` writes an unnamed variable as its value, so a formula text edit re-mints it and silently splits a share through it (FORK-7 designer B); with no shared unnamed variable, the round trip is exact in meaning. Count the corpus and test fixtures that share an unnamed variable first (the designers found none in the corpus).
