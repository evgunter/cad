---
id: main-red-two-editor-core-slow-rows-no-row-holds-at-f2815a23
kind: issue
title: Main red: two editor-core slow-set rows red on f2815a23 that no open row names (a family's carry in either order; a holed cylinder wall's cells)
status: closed
opened: 2026-11-24
priority: P0
cost: M
closed: 2026-11-24
pr: 7737
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

## Closed (2026-11-24, GREEN U21)

Each item was bisected on main's first parent on hosted CI. The probes
are throwaway runs on `green/u21-probe`: 45369489326 and 45377944124 for
editor-core at the default ε, and 45382969404 for the tour at 1e-6.

- **`g_family_review_rows::a_familys_carry_is_every_members_whichever_order_they_stand_in`.**
  - **Where it turned.** It passes at `6246f581b7` and fails at
    `1cf86c9e96`, the merge of PR 7506 (`formwords/binders`).
  - **Class: an expected consequence of a ratified change.** The fixture
    was wrong, not the kernel. D10 `[repetition]`'s *Containment* and
    *Reading a family* apply under binders: a definition that reads `k`
    is the member at the binder's key, and only a binder's yields are a
    family.
  - **Why the read was refused.** `tools` is the tool placed at
    `scalar(k)·step`, which is not yielded. So the fixture's
    `tools[k]`, an indexed read, asked a family of a `Body`, and the
    door refused it by kind (`found: Body, expected: Is(Bodies)`). That
    is the refusal the clause describes.
  - **Which side is right.** The list seat itself is right: since PR
    7426, a subtract's tool is a list seat, and DESIGN.md says so.
  - **Fix.** The fixture reads `tools` plainly, so the cut reads `k`
    through it and is the binder's yield, as the next line already
    writes.
- **`trim_3_windows_interval::a_holed_cylinder_wall_drops_the_cells_in_its_hole`.**
  - **Where it turned.** It passes at `1bd32ee664` and fails at
    `323c16d098`, the merge of PR 7408 (`sectcone/certified-foot`).
  - **The refusal.** A temporary print of the leaf's `Interval` replay
    shows the holed cylinder's `Subtract` refusing
    `Escalated { Coincidence(Section, Moot), … predicate: Some("section_site_foot_bend") }`,
    so the assertion has no verdict.
  - **Class: a regression in SECTCONE's ground, already fixed.** This is
    the straight-carrier-at-`Interval` cause recorded on
    `work/sectcone/a-carried-sites-foot-escalates-where-its-polish-stalls-or-clamps-at-a-run-end.md`,
    whose fix is PR 7734 (`75e99a78b6`). The probe on main with PR 7734
    in it is in this PR's body.
- **`revalue_corpus::revalue_die_chamfer` and `revalue_measured_web`.**
  - **Class: inherited.** They are two of the ten rows already on
    `work/sym/a-sym-posed-reading-carries-a-value-its-node-does-not-compute.md`
    (P1, `design: true`), with the same drift counts.
  - **Where it turned.** It passes at `1883da3554` and fails at
    `3547fe5764` (PR 7411).
  - **Action.** They are left red and unheld, as that row decided, and
    noted there.
- **`demos/tour` `eps_regression::tour_runs_green_at_eps_1e_6`.**
  - **Where it turned.** It passes at `b4ffffec5b` and fails at
    `f5744c8c20` (PR 7395, `the-cone-torus-pair-door`).
  - **The refusal.** PR 7395 retired lily wall 2, but at 1e-6 the
    lantern's weld still refuses
    `CurvedPairUnsupported { site: InteriorLoopGuard, kind: Cone, other_kind: Torus }`.
  - **Action.** Filed on SECTCONE's slate as
    `work/sectcone/the-lily-lanterns-weld-still-refuses-at-the-interior-loop-guard-at-1e-6.md`
    (P1). It is not held, and the judgement is SECTCONE's.
