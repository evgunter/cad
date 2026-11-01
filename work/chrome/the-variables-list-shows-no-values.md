---
id: the-variables-list-shows-no-values
kind: issue
title: the variables list shows each variable's name and not its value
status: open
opened: 2026-10-31
priority: P1
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
