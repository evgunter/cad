---
id: written-length-rustdoc-cites-a-test-that-does-not-exist
kind: issue
title: Expr::written_length's rustdoc cites switch_display_units::every_authored_unit_reaches_a_literal_without_a_mismatch, which no test defines
status: open
opened: 2026-09-09
---


Found by LIB-HELPER (PR #2269) while adding `Expr::length_in` beside
`Expr::written_length` in `crates/editor-core/src/expr.rs`: the
`written_length` rustdoc names
`switch_display_units::every_authored_unit_reaches_a_literal_without_a_mismatch`
as the test that executes its no-mismatch claim, and no test by that
name exists anywhere in the tree (`grep -rn
every_authored_unit_reaches_a_literal_without_a_mismatch crates`
finds only the citation). Either the test was renamed and the doc not
moved with it, or it never landed. Left alone at LIB-HELPER so the
kernel diff stayed the two functions and their doc. Closing it means
finding the test that makes the claim (or writing it) and pointing
the doc at it; `expr.rs` is in no program's `paths:`, so it is filed
in LIB's fence as the program that last touched the file.
