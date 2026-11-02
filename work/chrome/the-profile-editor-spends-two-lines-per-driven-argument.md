---
id: the-profile-editor-spends-two-lines-per-driven-argument
kind: issue
title: viewer: the profile editor spends two sentences and a line of edit links on every driven step argument
status: open
opened: 2026-11-01
priority: P1
cost: M
refs: [the-profile-create-form-is-a-second-profile-editor]
---
Reported by Ev (chat, 2026-11-01). Filed P1 at Ev's request. Under
each argument an expression drives, the profile editor draws:

```
loop 0 step 2 · target x = l
loop 0 step 2 · target x: driven by an expression over l (currently 0.03 m) — edit the expression?
edit l
loop 0 step 2 · target y = w
loop 0 step 2 · target y: driven by an expression over w (currently 0.02 m) — edit the expression?
edit w
...
```

The argument's own field already shows the expression and its value,
so both sentences repeat it.

## Where

`slot_notes` (`crates/viewer/src/pane/properties.rs:1769`), reached
from `held_slot_doors_ui` (`properties.rs:874`) via `slot_notes_ui` /
`slot_readings_ui`:

- the `<slot> = <source>` line, `properties.rs:1781-1787`;
- the `Refusal::affordance` sentence, `properties.rs:1789-1802`
  (worded at `crates/viewer/src/session/refuse.rs:702`);
- the `edit <var>` links, `properties.rs:1803-1810`. These are in a
  `horizontal_wrapped` per row, so a step with three driven arguments
  puts three link lines under three sentence pairs.

## What Ev wants

- Drop the two sentences in the profile editor. The value is already
  on the argument's own line.
- Put all the `edit l` / `edit w` / `edit b` links on one line,
  separated by ` · ` like the row labels above them. Once
  `a-variable-is-edited-in-a-separate-widget-not-on-its-own-row` lands,
  these links point at the variable's row.

Check whether the Properties pane's slot rows (same `slot_notes`) want
the same trim. The affordance sentence is the ratified refusal wording
for a *refused direct edit* (`refuse.rs:702`), so drawing it standing,
ahead of any edit, is the part to question.
