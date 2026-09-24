---
id: no-row-holds-that-the-create-pane-offers-the-tools-it-has
kind: issue
title: No row holds that the chrome CALLS its tool panels — nine activation buttons across create_ui and properties_ui are reachable only through a ViewerBehavior no test can build
status: open
opened: 2026-09-22
priority: P1
cost: D
---



## Finding

Found by AUTH-4's implementer lane, inside AUTHOR's own fence, while
discharging the brief's standing warning that *a unit-tested helper is
not a wired one*.

Nine modal tool panels put the tools on screen, and they are reached
from TWO call sites, not one:

- `ViewerBehavior::create_ui` (`crates/viewer/src/pane/create.rs`)
  calls eight — `revolve_tool_ui`, `boolean_tool_ui`, `split_tool_ui`,
  `transform_tool_ui`, `pattern_tool_ui`, `blend_tool_ui`, and as of
  AUTH-4 `projection_tool_ui` and `duplicate_tool_ui`;
- `ViewerBehavior::properties_ui` (`crates/viewer/src/pane/properties.rs`)
  calls the ninth, `mate_tool_ui` (defined in `pane/create.rs`), right
  beside `create_ui` itself.

Seven of the nine shipped before AUTH-4. Each paints its own
activation button (`"Pattern tool…"`, `"Projection tool…"`, …) and its
own seat line. `create_ui` also hosts three inline creation FORMS —
add-datum, add-profile, extrude — that are not tools and have no
activation button, and `properties_ui` calls one more adjacent panel,
`add_part_ui` (the `Add part…` chooser for an instance of another
document), which is not a tool either. Both are in the same position
as the nine and are named here so the census is the whole of it.

**Nothing in the tree asserts any of it.** Grepping the whole crate
and its suites for the activation labels finds them at exactly one
site apiece — the method that paints them. Deleting a
`self.projection_tool_ui(ui)` line from `create_ui`, a
`self.mate_tool_ui(ui)` line from `properties_ui`, or any of the other
calls, reddens no row: the tool becomes unreachable from the UI and
the suite stays green.

**Where the row lives.** On AUTHOR's slate, deliberately, after the
2026-09-22 cut gave `pane/create.rs` to FORMS as well: FORMS' charter
is the VOCABULARY the forms answer in, and says a new door on that
ground is AUTHOR's. What this row is about is neither — it is whether a
tool is reachable at all, which is the authoring goal's own question.

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
  itself. That is a change at seven shipped panels in a file
  claimed by five programs (`chrome`, `view`, `vnews`, `vseam`,
  `author`), so it wants its own sitting.

AUTH-4 did what it could inside its fence instead: its two panels'
composed rows are free functions
(`create::part_selector_rows`, `create::duplicate_note`) driven by
`pane::create::tests`, so the SENTENCES are held. What is not held,
for its two panels and for the seven that shipped before them, is that
`create_ui` calls them at all.
