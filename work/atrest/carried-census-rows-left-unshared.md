---
id: carried-census-rows-left-unshared
kind: issue
title: The carried census rows leave four review findings untaken: one carried-row type, finding_solids beside topo's, an unspoken carried overlap, and a seam that ignores cancel
status: open
opened: 2026-10-19
priority: P3
cost: M
refs: [5810, 5325]
---

The two reviews of PR #5325 raised these, and the fix passes left them
untaken (`docs/dual-review-log/rows/PR-5325.md`).

1. **One carried-row type** (review 1 S3, review 2 S-1). Four
   `{route, X, held?}` structs carry a part's rows:
   - `CarriedDeclaration`
   - `CarriedRefusal`
   - `CarriedInterference`
   - `CarriedAtRestFinding`

   In `crates/pncad-py/src/py/assembly.rs` the identical
   `through`/`of`/`via` getters are written once per class, and the two
   `same_row` bodies in `crates/editor-core/src/assembly.rs`
   (`impl CarriedRow`) are character-identical. A generic `Carried<X>`,
   with one Python route mixin, would remove the class.
2. **`finding_solids` beside topo's own reading** (review 2 S-2).
   `crates/editor-core/src/checks/at_rest.rs`'s `finding_solids`
   nearly duplicates `topo::census_uncleared_solids`
   (`crates/topo/src/census.rs`). It is one concept, "the two solids a
   census finding is about", with homes in two crates and no sentence
   relating them. Fold it into topo, or say why the two differ.
3. **A carried overlap has no sentence** (review 1 n3, review 2 S-5).
   - `CarriedInterference` has no `Say`/`Display` in Rust and no
     `__str__` in Python.
   - Its `finding.quiet` is an assertion id in the part's id space, with
     no `held` to speak it from.
   - `InterferenceFinding` has no sentence either, so this is the whole
     interference surface rather than the carried row alone.
4. **The seam ignores cancel** (review 1 S8). The part store localizes a
   part's overlaps at the seam (`assembly::seam_verdict`, one boolean per
   interfering pair) without observing the evaluation's `CancelToken`.
   The gather's census and `assemble` ignore it already.

