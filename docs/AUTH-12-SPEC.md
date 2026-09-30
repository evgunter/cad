# AUTH-12 — every tool the chrome has is reachable, and a row says so

**Row**: `work/author/no-row-holds-that-the-create-pane-offers-the-tools-it-has`
(P1). Read it in full.

**Branch** `author/tool-census`. **Never merge; I merge.**

## What changed since the row was filed

The row (2026-09-22) says the nine tool panels hang off a
`ViewerBehavior` "no test can build", so the choice was between a new
fixture and lowering seven panels to free functions. **That premise is
stale.** Since 2026-09-25 (`9c9fb7ba7`), `app.rs`'s
`properties_pane_tests` has run real frames of the whole app
(`app_frame`, `Driven`, `painted_with_tool`). It paints `ViewerApp` in
full, opens collapsed sections by clicking them, and reads what was
painted. AUTH-10 used it. So this unit needs no new harness and no
refactor of the panels.

## The unit

1. **A census row over `ToolKind::ALL`** in the whole-app harness. For
   every kind, starting from the landed startup document:
   - open whatever section hosts its activation button;
   - find the button;
   - click it;
   - assert that the tool is open (`Tools`' open state, or the panel's
     own seat line painted).

   Deleting any `self.<kind>_tool_ui(ui)` call from `create_ui` or
   `properties_ui` must turn it red. The sweep is over `ToolKind::ALL`,
   so a tenth kind is covered the day it exists.
2. **One spelling of each activation label.** `ToolKind::label()` says
   it is "the tool's name, for sentences and buttons", but all nine
   buttons spell their label as a literal (`"Pattern tool…"` at
   `pane/create.rs` ~:1423, and so on, `"Mate tool…"` at ~:603). Route
   the buttons through `label()`, with one function for the button's
   form (capitalised, with the ellipsis). The census row then finds each
   button by that same function, not by a copied literal.
3. **The three inline forms** (add-datum, add-profile, extrude) and
   `add_part_ui` are named in the row as being in the same position,
   with no activation button. Say what reaches them, and hold it the
   same way if it is cheap (each section's heading painted, say).
   Otherwise leave them to the row's closure note.

## Check these first

- Whether any tool is offered conditionally, e.g. only once a body
  exists. If it is, the census must build the document that offers it
  rather than skip it. A kind that cannot be reached from the startup
  document needs a fixture, not a `continue`.
- Whether `ToolKind::ALL`'s doc ("no production code reads it: the
  chrome names each kind it offers literally") becomes false once the
  buttons route through `label()`. It probably stays true, since
  buttons still name kinds one at a time. Keep it true either way.

## Traps

- Eleven of eleven AUTHOR units have minted a fresh duplication while
  closing one. Here, the likely culprits are the test copying the
  button literals, or a second label function beside `label()`.
- A row that can pass vacuously: assert that the sweep visited
  `ToolKind::ALL.len()` kinds.

## Scope, verification, deliverable

In: `pane/create.rs`, `pane/properties.rs`, `tools.rs`, `app.rs`
(tests). These are shared with CHROME, VSEAM and VNEWS; post seam
notes.

- Rows that go red: the census row, under a mutation that deletes one
  panel call from `create_ui` and another from `properties_ui` (the mate
  tool). Also, a button whose label is spelled differently from
  `label()`. Name the mutations, restore each from a byte copy, and
  **touch** afterwards.
- Local:
  - fmt;
  - clippy, both feature sets, `-D warnings`;
  - `doc-gate.sh`;
  - every `scripts/gates/*.sh`;
  - `--lib` and `--test all` as separate runs;
  - `work.py lint`;
  - the `work.py territory` output in the PR.
- `CARGO_TARGET_DIR=/root/auth-12-target` on every invocation,
  including excluded roots. Scratch in `/root/auth-12-scratch/`. Wrap
  `cargo` in `local-scripts/with-build-slot.sh -- <cmd>`.
- CI: confirm the head SHA, read `gate ok`, and say what the change
  filter selected.

PR titled `AUTH-12: every tool the chrome has is reachable, and a row
says so`. Report to me; don't merge.
