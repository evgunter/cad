---
id: stackup-base-opts-spells-its-own-nominal-box
kind: issue
title: stackup's base_opts spells its own [0, 0] box beside ParamBox::at_nominal
status: closed
opened: 2026-10-17
priority: P3
cost: E
refs: [the-pinned-profile-lift-launders-at-sym]
pr: 5248
closed: 2026-10-17
---

PR 5248's second review counted four spellings of "the nominal, asked for as an analysis" and asked for one. `stackup::base_opts` (`crates/editor-core/src/stackup.rs:601`) builds `Varying { lo: 0, hi: 0 }` over `names` by hand, beside `ParamBox::at_nominal` (`crates/editor-core/src/analysis.rs`). The PR took the drive-witness copies (`drive::witness_options`) and left this one.

**A documented no.** `at_nominal` collapses an existing box. The sensitivity pass has none: its driver (`stackup::driver`, `stackup.rs:518`) holds only the document's toleranced variables, `toleranced_params(doc)`. Its seeded passes vary no box either, so there is nothing to collapse. Spelling the base through `at_nominal` would mean minting a box only to collapse it. The hand-built box over `names` is the direct statement.

**When it is worth doing.** A third caller that needs the nominal as an analysis over a bare set of names, with no box, would justify a `ParamBox::nominal_over(names)` constructor shared with `base_opts`. Until then, a constructor with one caller is not worth adding.
