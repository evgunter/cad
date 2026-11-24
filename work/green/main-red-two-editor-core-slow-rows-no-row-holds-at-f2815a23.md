---
id: main-red-two-editor-core-slow-rows-no-row-holds-at-f2815a23
kind: issue
title: Main red: two editor-core slow-set rows red on f2815a23 that no open row names (a family's carry in either order; a holed cylinder wall's cells)
status: open
opened: 2026-11-24
priority: P0
cost: M
---

Found by BOXFINISH's expression-question lane (PR 7488) on 2026-11-24.
Both rows are red on `f2815a23` itself (main, a clean checkout,
`cargo nextest run -p editor-core --ignore-default-filter`, default ε,
`CARGO_PROFILE_{DEV,TEST}_OPT_LEVEL=1`) and red on that PR's head with
the same names; the PR's diff reaches neither. On the PR's head after
it merged main at `3e19f921` both are still red, with the panic text
below (main itself was not re-run at `3e19f921`). The closed row
`work/green/main-red-rows-two-lanes-found-red-on-their-merge-base-are-held-by-closed-rows-or-none.md`
names neither, and no open row does.

- `g_family_review_rows::a_familys_carry_is_every_members_whichever_order_they_stand_in`:
  panics at `crates/editor-core/tests/fixture/mod.rs:1585:52`,
  ``called `Result::unwrap()` on an `Err` value: SlotVarKind { var:
  SpokenVar { .. name: None }, node: SpokenNode { .. kind:
  Some("Subtract"), label: None }, slot: Operand(Cut(0)), found: Body,
  expected: Is(Bodies) }``. A fixture hands a `Subtract`'s cut operand
  a `Body` variable where the slot now expects `Bodies`: a slot-kind
  change the fixture did not follow.
- `trim_3_windows_interval::a_holed_cylinder_wall_drops_the_cells_in_its_hole`:
  panics at `crates/editor-core/tests/fixture/mod.rs:4426:18`,
  `a clearance's verdict over the leaf: None`.
- `revalue_corpus::revalue_die_chamfer` and
  `revalue_corpus::revalue_measured_web` (added by the node-key lane,
  PR 7470; red on main at `f2815a232a` and on that PR's merged heads
  with the same text): both panic at
  `crates/editor-core/tests/revalue_corpus.rs:394:5`, ``a decision's
  DAG re-valued at p0 is not the p0 build's own bits — revalue no
  longer computes an op as Sym builds it``, `die_chamfer` with
  `drifted: 322` of 5,432 compared and `measured_web` with
  `drifted: 39` of 2,014, laundered names `carrier_on_surface_1`,
  `carrier_on_surface_2`, `witness_on_surface_1` among them: the shape
  of `main-red-revalue-corpus-carrier-and-witness-on-surface-drift-since-7411`,
  which does not name these two documents.
- `demos/tour`'s `eps_regression::tour_runs_green_at_eps_1e_6` (same
  lane; red on main at `f2815a232a` and on the PR's head merged with
  it, same text; not re-run on a later main): panics at
  `tests/eps_regression.rs:84:5`, ``demo-tour <outdir>` failed at eps
  Some("1e-6") (exit status: 101)``.

Not bisected.

**Owed:** bisect each on the first parent; fix the fixture or what
moved, or re-state the row with the moving PR's reason.
