---
id: three-doors-named-insert-mean-two-different-constructions
kind: issue
title: Three doors named insert in one test binary mean two different constructions
status: open
opened: 2026-09-20
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
