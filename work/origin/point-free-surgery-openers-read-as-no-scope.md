---
id: point-free-surgery-openers-read-as-no-scope
kind: issue
title: the mutation-door guard reads a door that opens surgery point-free as NoScope, and a point-free re-mint by a non-Maintains door as none
status: closed
opened: 2026-09-29
priority: P4
cost: E
refs: [live-guard-proves-ordering-not-identity]
closed: 2026-09-29
pr: 3425
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

## Closed (2026-09-29, PR 3425)

`MutationDoor::names` matches a needle as a whole token (one matcher,
shared with `live.rs`), with `use` declarations blanked when a door is
read and a same-named module path told apart by its `::name`. A
point-free `begin_surgery` now reds the tier-1 guard and a point-free
`mint_pcurves` in a `Neither` door the posture guard (both planted, then
removed); no door on the tree changed classification. The sweep's
other `(`-terminated needles are filed on their owners' slates.
