---
id: a-crossings-sense-residue-left-in-the-emitters-and-the-remap
kind: issue
title: EdgeSense.sense drops edge_reading's error, member_cells skips a Crossing's sense by .., and RemapError has one variant
status: open
opened: 2026-10-27
priority: P3
cost: E
---


Left over from `a-through-senses-names-are-invisible-to-the-name-walkers`. That lane did not take these items: the first two sit in files another lane was editing at the time, and the third changes the public API.

- **`EdgeSense.sense` (`names/emit_topo.rs`, `senses_at`).** It holds `Option<Sense>`, filled with `edge_reading(..).read(..).ok()`. `sense_of` then re-raises any `None` as `NamingError::Emission { what: UNSIDED }`. Today `EdgeReading::read` refuses only with `UNSIDED`, so nothing is lost yet. A second refusal would be relabelled as UNSIDED, though. Fix: hold the `Result` and return its error.
- **`member_cells` (`names/emit_union.rs`).** It matches `RoleSeg::Crossing { edge, face, .. }`. Skipping the sense is correct here, because a `Through`'s things discriminate a crossing and are not cells it is made of. But the `..` would also let a future field that carries names go unnoticed. Fix: spell it as `sense: _`, the way `name_args` and `embedded` now do.
- **`RemapError` (`refactor.rs`).** It has a single variant, `Unmapped`, and the code destructures it three times (`SplitError::reaches`, `SplitError::straddles`, `InlineError::stranded`). It is public: it is re-exported from `lib.rs` and listed in `crates/pncad/tests/all.rs`'s surface census. Folding it into `Unmapped` moves its `Display`, so the change also needs pncad-py's prose census.
