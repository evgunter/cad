---
id: three-doors-named-insert-mean-two-different-constructions
kind: issue
title: Three doors named insert in one test binary mean two different constructions
status: closed
branch: dup/viewer-insert-doors
opened: 2026-09-20
closed: 2026-09-24
pr: 3150
priority: P1
cost: E
---


## Finding

- **Where**: `crates/viewer/tests/`, one aggregated binary, three names
  in scope at once:
  - `common::insert(session: &mut DocSession, op: SessionOp)` — drives
    a **session op** and asserts the outcome committed exactly one
    insert.
  - `common::inserted(doc, node, tol)` and its in-place twin
    `common::insert_into(&mut doc, node, tol)` — drive **`apply`**
    directly, no session, no outcome assertions.
  - `review_gui2_r2::insert(doc, node)` — a private one-line adapter
    over `common::inserted` binding that suite's `tol()`.
- **Why it is a finding and the adapter is not.** The adapter is fine:
  one line, twelve call sites, partial application of a parameter that
  never varies in that file. The defect is that `insert` and `inserted`
  differ by two letters and by **which door they go through** — one
  exercises the session contract, the other bypasses it — and a reader
  choosing between them at a call site has nothing but the argument
  types to go on. The S-DUP unit that routed nine files onto
  `inserted`/`insert_into` made the collision denser without touching
  the names.
- **Importance**: medium. A fixture that meant to exercise the session
  and reaches for `inserted` gets a document with no outcome assertions
  and no refusal check, and nothing says so.
- **Confidence**: sure about the three doors; the remedy is a naming
  judgement this row does not settle. The obvious candidate is to
  rename the session door for what it drives (`perform_insert`,
  `session_insert`) and leave the `apply` pair alone, since the pair is
  now the larger population.
- **Instrument, and its blind spot**: read `common/mod.rs`'s public
  surface and `git grep -n 'fn insert'` over `crates/viewer/tests/`.
  It is name-shaped and therefore blind, structurally, to a fourth door
  that means one of these two under a third name — which is the whole
  shape of this finding.
- **Raised by**: the S-DUP lane's fix pass on
  `viewer-review-suite-fixtures-have-no-oracle-role`, 2026-09-20,
  measured at `ab086f8c1`.

## Why this sits on S-DUP's slate

`crates/viewer/tests/` is claimed by `chrome`, `tcost`, `tint`, `vdoc`
and `view` (`work.py territory`), with no single ground-owner. It is
not a duplication — the two constructions are genuinely different — so
it is filed here only because the unit that concentrated the population
found it; `tint` or `view` is the likelier owner, by `git mv`.

## Disposition (2026-09-24, `dup/viewer-insert-doors`)

- **Renamed**: `common::insert` is now `common::session_insert`, and
  its rustdoc says which door it is and names the other one;
  `inserted`'s rustdoc names the session door back. 142 call sites in
  14 suites plus `common::xy_frame_in` routed; `docs/AUTH-1-SPEC.md`'s
  one prose mention followed.
- **The adapter** `review_gui2_r2::insert` is now
  `review_gui2_r2::inserted`: it is `common::inserted` with `tol`
  bound, so it takes that door's name. Six call sites, not twelve.
- **Fourth doors, under other names** (the row's blind spot):
  `docm9_range_vs_probe::insert` was a private copy of
  `common::insert_into` (plus a longhand `edit_into`, `edited`,
  `xy_frame` and `square`); folded, its `lit`/`scalar` helpers gone.
  `frame_labels` (3) and `msolve3_placer_refused` (2) spelled
  `common::edited(.., DocEdit::InsertNode { .. })` for `inserted`;
  folded. On the session side, `assembly_display::add_seat_mate`,
  `instance_authoring::add_instance` and an inline pattern insert in
  `gesture_table` were the session door without its `InsertNode`
  check; folded onto `session_insert`.
- **Also in this binary, not ours to rename**: `crate::fixture` (the
  `editor-core` tree symlinked in) exports `fixture::insert` and
  `Recorder::insert`, both `apply`-door. After the rename every
  `insert` in the `all` binary means the document's door, which is
  the reading the rename makes consistent. `viewer`'s own `src`
  holds `scene::insert` (production, fallible) and
  `widgets::value_field_tests::inserted` (an in-crate copy), both
  document-door.
- **Every session-door site checked for the contract it wants**: all
  sit in suites that drive a `DocSession`. `docm1_face_frame` uses
  both doors on purpose (a document-door box, then the chrome's op,
  compared by `bit_eq`). No site was found wanting the other door.
- **Left inline, deliberately**: rows whose SUBJECT is the committed
  edit (`mate_tool_flow`'s four proposal commits, `review_gui4_r1`'s
  landing) or that read the outcome's `withdrawn` half. `mate_tool_flow`
  reads the mate back by kind (`committed_mate`) because its rows
  assert `fault(mate).is_none()`, which a non-mate id satisfies
  vacuously; the session door checks `InsertNode`, not the node's
  kind, so that read stays.
- **Filed**: `the-post-to-shelf-mate-op-is-spelled-sixteen-times-in-seven-suites`.
