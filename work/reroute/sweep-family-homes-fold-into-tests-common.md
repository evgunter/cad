---
id: sweep-family-homes-fold-into-tests-common
kind: issue
title: sweep tests - shell7_common, shell8_common, mate2_common and revolve_common fold into content-named tests/common modules, under a one-tree rule
status: open
opened: 2026-09-29
priority: P4
cost: M
refs: [sweep-family-homes-sit-outside-the-tests-common-routing-list, sweep-suites-wrap-tol-witness-in-a-private-tol]
---

**The ruling this routes onto.** S-DUP asked it as the design fork
`sweep-family-homes-sit-outside-the-tests-common-routing-list` (PR 3385,
row 7 of `docs/DESIGN-FORK-LOG.md`). Ev accepted the two designers'
shared recommendation on 2026-09-29 (*"yep"*): **no family homes.** The
full question and both reports are in PR 3385.

**The unit.**
- Fold `crates/sweep/tests/shell7_common.rs`, `shell8_common.rs`,
  `mate2_common/` and `revolve_common` into `crates/sweep/tests/common/`.
  Each item goes into a module named for what it holds (body authoring,
  readers, checks, oracles), never for the unit that first needed it.
  Several such modules exist already (`shell_operands`, `torus_walls`,
  `oracles`, `cavity`, …).
- An item with one reader goes back to that suite. A surviving copy
  folds or carries the ``NOT `common::` `` marker.
- `shell7_common.rs` and `shell8_common.rs` are registered as `#[path]`
  suites in `all.rs` with no tests. Those registrations go.
- `common/mod.rs`'s routing list: the `revolve_common` bullet becomes
  one rule. `tests/common` is the binary's only shared tree, and a
  module in it is named for what it holds. A row, or the existing
  suite/helper instrument, reds if a second shared tree appears.

**Sequencing.** Several of these items are owned by open rows: the
per-suite `revolved` helpers (`work/fixture/sweep-revolve-about-y-helper-and-its-fixtures-spelled-per-suite.md`),
the planar cap finders (`work/helper/sweep-planar-cap-finders-spelled-per-suite.md`),
and the solid walks. Take them in the same batch, or fold around them
and say which.

**Census.** Readers as measured at PR 3385, re-taken at the lane's
merge base per method item 1:
- `shell7_common`: 9 suites.
- `shell8_common`: 13.
- `mate2_common`: 5.
- `revolve_common`: 33.

Instrument: `git grep -lE '(crate|super)::<home>\b' -- crates/sweep/tests`.
