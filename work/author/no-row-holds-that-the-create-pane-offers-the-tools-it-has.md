---
id: no-row-holds-that-the-create-pane-offers-the-tools-it-has
kind: issue
title: No row holds that the create pane CALLS its tool panels: seven activation buttons and two new ones are reachable only through a ViewerBehavior no test can build
status: open
opened: 2026-09-22
priority: P1
cost: D
---



## Finding

Found by AUTH-4's implementer lane, inside AUTHOR's own fence, while
discharging the brief's standing warning that *a unit-tested helper is
not a wired one*.

`ViewerBehavior::create_ui` (`crates/viewer/src/pane/create.rs`) is
what puts every creation and combining tool on screen: it calls
`boolean_tool_ui`, `split_tool_ui`, `transform_tool_ui`,
`pattern_tool_ui`, `blend_tool_ui`, `revolve_tool_ui` and — as of
AUTH-4 — `part_tool_ui` and `duplicate_tool_ui`. Each of those methods
paints its own activation button (`"Pattern tool…"`, `"Part tool…"`,
…) and its own seat line.

**Nothing in the tree asserts any of it.** Grepping the whole crate
and its suites for the activation labels finds them at exactly one
site apiece — the method that paints them. Deleting a
`self.part_tool_ui(ui)` line from `create_ui`, or any of the other
seven, reddens no row: the tool becomes unreachable from the UI and
the suite stays green.

## Why it is filed and not fixed

The panels hang off `ViewerBehavior`, which borrows the whole
application — a session, a scene, a pick index, a camera, a display
view, the drafts, the tools and the part chooser — so
`crate::pane::headless` cannot reach them. `pane.rs`'s own docs say
this outright: *"What it still cannot reach is a pane METHOD … A row a
test must drive is therefore a free function over the `Ui`, and the
method's job is to call it."*

Two shapes would close it, and choosing between them is the work:

- **A `ViewerBehavior` fixture.** Nothing in the tree builds one
  today, so this is a new harness rather than a row.
- **Free functions the methods call.** Every tool panel needs the same
  four pieces — `tools`, `drafts`, `ops`, `notices` — and so does
  `tool_commit_row`, which the panels all end with. Lowering
  `tool_commit_row` to a free function over those four would let each
  panel become one, at which point a headless row drives the panel
  itself. That is a change at seven shipped call sites in a file
  claimed by five programs (`chrome`, `view`, `vnews`, `vseam`,
  `author`), so it wants its own sitting.

AUTH-4 did what it could inside its fence instead: its two panels'
composed rows are free functions
(`create::part_selector_rows`, `create::duplicate_note`) driven by
`pane::create::tests`, so the SENTENCES are held. What is not held,
for its two panels and for the seven that shipped before them, is that
`create_ui` calls them at all.
