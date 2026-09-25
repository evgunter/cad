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
  `Recorder::insert`, both `apply`-door. After the rename no free
  function named `insert` in the `all` binary is the session door.
  One `insert` there is neither door: `PartStore::insert`
  (editor-core's `fixture::resolver`, used by
  `msolve3_placer_refused`) stores a part document and answers its
  `DocRef`.
- **`viewer`'s own `src`**, scope `crates/viewer/src/` (every `apply(`
  call, classified test or production against its file's first
  `#[cfg(test)]`): production has one insert door, `scene::insert`
  (fallible, `Result`-returning, correctly not a test helper). Test
  modules hold seven document-door insert spellings that cannot reach
  `tests/common` — a `let insert` closure in `drafts.rs` (a copy of
  `inserted` down to its expect text), inline `apply(.., InsertNode)`
  at two sites each in `drafts.rs`, `pane/profile.rs` and
  `session.rs`, and `widgets.rs`' own `edited`/`inserted` — recorded
  as evidence on `viewer-src-test-modules-restate-the-literal-doors`.
- **Every session-door site checked for the contract it wants**: all
  sit in suites that drive a `DocSession`. `docm1_face_frame` uses
  both doors on purpose (a document-door box, then the chrome's op,
  compared by `bit_eq`). No site was found wanting the other door.
- **Fix pass (review of `936a62f21`)**:
  - The fold minted a twin: `instance_authoring` and `story_assembly`
    each held a private `add_instance` (`session_insert(AddInstance)`
    then `pump`). One home now, `common::instance_in`.
  - `mate_tool_flow`'s six proposal commits spelled the session
    door inline and then read the mate back as the document's LAST
    `Node::Mate` — which would answer an earlier mate if the op had
    inserted something else. They now go through a local
    `commit_mate`: `session_insert`, then an assertion that the
    returned id IS a `Node::Mate`, which is what keeps the rows'
    `fault(mate).is_none()` from passing on an id it could never fail
    on. `committed_mate` is gone.
  - `assembly_display`'s `mate_nodes(..)[0]` reads were the same read
    by kind: both now take the id the session door answers
    (`add_seat_mate` returns it). `mate_nodes` stays for its one
    remaining row, which compares a fault's WHOLE `mates` payload to
    every mate in the recipe — a list, not an id.
  - `docm9_range_vs_probe::slab` names itself `editor-core`'s branch
    fixture, but the original is private to
    `editor-core/tests/docm9_range.rs`, not in `fixture/`, so the
    symlinked `crate::fixture` cannot hand it over. Homing it there is
    editor-core's ground; the viewer copy is four lines over the
    shared doors, with the same values.
  - `review_gui2_r2::inserted` keeps its name: it IS `common::inserted`
    with the suite's `tol` bound, the arity difference is the binding,
    and the suite never imports `common::inserted` by bare name, so
    nothing is shadowed.
  - Main added 26 more `insert(` calls in `combine_ops` and one in
    `frame_policy` while this unit was open; routed.
- **Left inline, deliberately**: `review_gui4_r1`'s landing row, whose
  SUBJECT is the committed edit, and rows that read the outcome's
  `withdrawn` half — they need the outcome, and the door answers an
  id.
- **Filed**: `the-post-to-shelf-mate-op-is-spelled-sixteen-times-in-seven-suites`.
