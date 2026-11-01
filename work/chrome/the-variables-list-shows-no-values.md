---
id: the-variables-list-shows-no-values
kind: issue
title: the variables list shows each variable's name and not its value
status: closed
opened: 2026-10-31
priority: P1
cost: E
branch: chrome/variable-values
closed: 2026-11-01
pr: 6242
---

## Finding

Found driving the viewer (2026-11-01; Ev: P1, "that sounds quite annoying"). The Properties pane's variables section lists each declared variable by name only (`t`), with no value, dimension or unit. To read `t`'s value you select it and read its row, or find a slot it drives (`distance: driven by an expression over t (currently 0.02 m)`). The list should say each variable's value in the working notation.

```text
click "New variable…"
click "name"
type "t"
click "length"
click "0.0000"
type "0.02"
key Enter
click "Create variable"
settle
dump
```

## Closed (2026-11-01, PR 6242)

The variables list now shows each variable's value. A free variable reads `t = 20 mm` in its declared unit, the unit it was written in and that its own field shows. A defined variable reads `w = 2 * t = 40 mm`, its computed value in the working notation (#3639). A definition that fails reads `?`, with its refusal under the line, said once: while that variable is selected, its own row carries the refusal instead. "What a reading shows" has one home, `props::reading_text`, shared with the driven-slot field. Filed: `work/issues/a-count-promotion-refusal-names-a-function-the-parser-does-not-have.md`.
