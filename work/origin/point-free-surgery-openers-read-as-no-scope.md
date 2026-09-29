---
id: point-free-surgery-openers-read-as-no-scope
kind: issue
title: the mutation-door guard reads a door that opens surgery point-free as NoScope, and a point-free re-mint by a non-Maintains door as none
status: open
opened: 2026-09-29
priority: P4
cost: E
refs: [live-guard-proves-ordering-not-identity]
---


Disclosed by PR 3424's fix pass (its blind-spot bullet in
`source_walk.rs`'s `mutation_doors`), filed so the disclosure is
scheduled. Every `code_contains` needle the mutation-door guards read
ends in `(` so a `use` line cannot satisfy it. A point-free call
already reds the closes, the backing asserts, the postcondition and an
undeclared door's re-mint, but is SILENT for the scope openers
`begin_surgery(` / `enter_surgery(` (the door reads `NoScope`) and for
a re-mint by a door declared non-`Maintains`. The fix is the one PR
3424 applied to the Live needles: match the path as a whole token and
exclude `use` lines another way.
