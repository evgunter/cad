---
id: seat-line-takes-a-hand-built-slice-while-seats-holds-the-roles
kind: issue
title: seat_line takes a hand-built (Seat, Option<RecipeNodeId>) slice, so five panels re-list roles a Seats value already holds
status: closed
opened: 2026-09-20
priority: P3
cost: E
closed: 2026-09-25
branch: vnews/one-seat-line
pr: 3281
---



Filed from the style review of
`seat-line-spells-the-list-mark-as-a-literal` (#2917).

## The shape

`seats::seat_line` (`crates/viewer/src/seats.rs`) takes
`&[(Seat, Option<RecipeNodeId>)]` — a slice of pairs the caller
assembles. But the value that OWNS both halves already exists:
`Seats` holds its roles and its picks together, and the module doc
says why in as many words — *"The roles travel WITH the seats rather
than being re-supplied per call: a drop's event has to name the role,
and a value that knew its picks but not what they were for could not
compose that sentence."*

`seat_line`'s signature re-supplies them per call. Five panels in
`crates/viewer/src/pane/create.rs` re-list, by hand, roles their tool
already constructed a `Seats` from:

- the revolve panel — `(Seat::RevolveProfile, tool.profile())`,
  `(Seat::RevolveAxis, tool.axis())`, against
  `Seats::new([Seat::RevolveProfile, Seat::RevolveAxis])` in
  `crates/viewer/src/revolvetool.rs`
- the combine panel — `(Seat::OperandA, tool.a())`,
  `(Seat::OperandB, tool.b())`
- the split panel — `(Seat::SplitTarget, tool.target())`,
  `(Seat::SplitPlane, tool.plane())`
- the transform panel — `(Seat::TransformBody, tool.input())`
- the pattern panel — `(Seat::PatternBody, tool.input())`,
  `(Seat::PatternAxis, tool.axis())`

Each pairing is re-asserted at the panel, so a tool whose roles change
compiles fine with a panel that still names the old ones in the old
order — and the order is load-bearing here, since these roles are not
symmetric by the module's own argument.

## What a fix has to decide

Whether `seat_line` takes `&Seats` (the value that knows), or whether
the tools grow a door that hands the pairs over. The first deletes the
five hand-lists outright; the second keeps a per-tool accessor and is
the shape the mate tool would need, since its picks are faces rather
than nodes and it holds no `Seats`
(`mate-panel-hand-rolls-the-seat-line`). Decide those two together or
the second re-opens the first.

`Seats` exposes `held(i)` by index and stores `roles`, so the door
`seat_line` would need is whatever makes "(role, pick) for each seat"
answerable without an index — which does not exist yet and is part of
the fix rather than a precondition.

## Home

VNEWS's: `crates/viewer/src/seats.rs`,
`crates/viewer/src/pane/create.rs`. `revolvetool.rs`, `combine.rs`
and the other tool modules are named as evidence, not as fix sites,
unless the second shape is chosen.

## Closed (`vnews/one-seat-line`, PR 3281, 2026-09-25)

**`seat_line` takes `&Seats`; the per-tool door is the mate tool's
alone.** The fork was decided by which value knows the roles. For
every seated tool that value is its `Seats`, so `seat_line(&Seats)`
reads the roles and their order off it (a private `Seats::each`, one
entry per SEAT — a one-seat tool's role-twice is read once, by the
same private `arity` the pick rule now reads) and the seven tools
expose `seats()`. A per-tool door handing pairs over would have moved
each hand-list from the panel into the tool, one file closer to the
`Seats::new` it duplicates but still a second statement of it. The
mate tool holds no `Seats`, so its door is on the value that DOES know
its roles — `MateToolState::line` — and both doors compose through one
function, `seats::picks_line`. Neither re-opens the other: the
composition is shared below both, and each role list is stated once,
where the state is.

All seven seated panels (the row counted five; the part and duplicate
panels had landed since) now call `pane::create::seats_row(ui,
tool.seats(), theme)` and name no `Seat`. Pinned by
`pane::create::tests::the_boolean_panel_says_which_operand_each_pick_is`
(headless, fixed text) and `combine_ops::the_seat_line_names_the_roles`.
