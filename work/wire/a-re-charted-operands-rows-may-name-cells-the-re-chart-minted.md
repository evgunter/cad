---
id: a-re-charted-operands-rows-may-name-cells-the-re-chart-minted
kind: issue
title: A boolean over re-charted sphere operands may emit rows naming cells the re-chart minted, which no input holds
status: closed
opened: 2026-10-19
priority: P1
cost: E
closed: 2026-10-27
---


Filed by WIRE's `wire-citerows` lane (branch
`wire/boolean-rows-every-decision`).

## What

`boolean/ops.rs` `through_the_join` re-runs a sphere boolean over
re-cut operands: `apply_recuts` re-charts some shells (carved and
grafted) and `apply_cut_ins` cuts faces along chart meridians. Every
row a boolean emits names its INPUTS' cells (`crate::coincidence`), and
that unit maps a cut-in's cells back (`ChartCuts::input_cell`: a
division to the face it was cut from, a meridian to the face it runs
in, an arc split to its arc). Nothing maps a re-chart's: the comment
beside the call says `carve` keeps every kept entity's key, but a
grafted shell's cells are new keys. Once every margined decision is a
row, a decision on a re-charted shell's cell reaches the document as a
cell no input names, and `editor-core` `coincide::name_rows` refuses the
node ("a coincidence row names a cell no input of its node names"), as
the cut-in did on `emit_sphere_cut_in_cap` before the mapping.

Unmeasured: no test in `topo` or `editor-core` failed on it. Find a
re-chart fixture with a Zero decision on the grafted shell, and map its
cells or refuse typed.

Signed (WIRE implementer, wire-citerows).

## Slate triage (2026-10-26, CLEAVE)

- Moved to WIRE (`work/wire/`) by `git mv`, id and body unchanged: whose lane filed it: the rows a node publishes and the lineage they are carried by. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

## Measured (CLEAVE, PR 5753, 2026-10-26)

PR 5753 measured with a debug-build check at `boolean/ops.rs`
`boolean_op_with`'s exit (`rows_name_input_cells`): every input cell a
row names must be held by its input (`records::holds`). Four `sweep`
tests trip it, at the default ε:
- `m5_s13_pips::two_pips_cut_under_the_group_arm`: 4 cells
- `review_pr12_probes::probe_h_door_a_closed_tool`: 2 cells
- `spheres_crossing_off_every_edge::a_ball_crossed_by_two_partners_builds`:
  42 cells (in the slow set)
- `spheres_crossing_off_every_edge::a_ball_escaping_through_a_plane_and_a_sphere`:
  24 cells (in the slow set)

The same counts hold at ε 1e-6 and 1e-12, and on main at all three ε.
Every hit is a `VertexFusion` `Meet` naming an edge or face with a
generation-3 key. All four tests take the re-chart path: `apply_recuts`
runs with operands to re-chart in each. The first two subtract a
two-ball union from a slab, and their row is
`Meet {A Face(1v1), B Edge(3v3)}`.

`through_the_join`'s containment re-cut re-charts both of B's balls
(`apply_recuts`). Each ball is two lunes, two meridians and two poles
(`sweep::test_support::ball_about_origin`). Each ball is carved, turned
rigidly about its centre (`transform_rigid`), and rebuilt:
- **No shell is kept, so the first turned ball becomes the rebuilt
  body with its input keys unchanged.** Its cells keep the input's keys,
  but they now stand elsewhere on the sphere. A row naming one of them
  passes the check and names the wrong input cell. Nothing detects this
  case.
- **The second turned ball is grafted** (`graft_solids_disjoint`), and
  its cells get new keys: faces 3v1/4v1 become 4v3/3v3, and so on. The
  turned meridian 3v3 crosses the slab's top, and the vertex minted
  there is decided on face 1. `ChartCuts::input_cell` maps cut-ins only,
  so the row names 3v3, which no input holds.

This is not the ring tie's class, a vertex minted inside one input cell
that reads back to it. A turned cell lies across the group's input
cells: a turned meridian generally crosses both input lunes and a
meridian of the input chart. So it has no one input cell to read back
to.

What it needs is a ruling on how a re-charted cell is spelled. The
options I see:
- by the input cells its place lies in, which is a geometric decision
  per cell;
- by the group's carrier, every face of the input group in one `Meet`,
  which needs `Coincidence` to hold a list once a group has more than
  two faces;
- or a typed refusal, which would turn both tests from builds into
  refusals.

Whichever is chosen also has to cover the first ball, whose keys are
unchanged.

## Acceptance

**This row closes only when the row-cell door check lands green.** The
check is on branch `cleave/rows-name-input-cells-guard`, as commit
e14e5f73551869bac076044cd550837112569714. That commit adds
`rows_name_input_cells` at `boolean_op_with`'s exit; cherry-pick it onto
the fix. It left PR 5753 because main still builds these rows (CLEAVE
orchestrator's ruling, 2026-10-26).

What the check measured:
- **Re-chart cells in `sweep`:** 72, the same at ε default, 1e-6 and
  1e-12, on main and on 5753's head.
- **Per-PR tests it turns red:**
  - `m5_s13_pips::two_pips_cut_under_the_group_arm` (4)
  - `review_pr12_probes::probe_h_door_a_closed_tool` (2)
- **Nightly tests it turns red:**
  - `spheres_crossing_off_every_edge::a_ball_crossed_by_two_partners_builds` (42)
  - `spheres_crossing_off_every_edge::a_ball_escaping_through_a_plane_and_a_sphere` (24)
- **The case the check cannot see:** the first turned ball keeps its
  input keys at moved places, so a row naming one of its cells passes
  the check while naming the wrong input cell. The fix has to cover
  that case as well as the grafted ball.


Closed by PR 5795 (2026-10-27). `Recharts` maps each re-charted shell's cells, moved keys included, and a re-charted cell reads as the set of its input shell's faces. A row that states a point becomes their sorted `Meet`. Any other relation refuses `RechartedRowUnstated`. The door check `rows_name_input_cells` runs in release. Census: 806 two-face shells and 1676 set meets, with zero refusals.
