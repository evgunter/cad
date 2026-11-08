---
id: test-utils-slow-censuses-read-a-stale-ledger-after-the-finished-body
kind: issue
title: test-utils' slow-set source censuses are red on main: the reader ledger still lists product_gate_attribution.rs (renamed by PR 6371), and several walks miss newly arrived files
status: open
opened: 2026-11-08
priority: P1
cost: E
---


Reported by PR 6584's lane on 2026-11-08, from its run of test-utils' slow set. None of its files is involved. All of these are red on main:
- `reader_census::every_site_that_reads_rust_source_is_in_the_ledger` and `every_shared_entry_actually_reaches_the_shared_lexer`. The ledger still lists `product_gate_attribution.rs`, which PR 6371 renamed to `product_finishes_once.rs`; that part is ONEHOME's own debris.
- `hand_written_impl_census::every_hand_written_walk_is_held_to_its_declaration`, on `editor-core/src/eval/parts.rs` `Carried` and `geom-brep/src/ssi/pair.rs` `Section`.
- `the_walk_still_sees_every_file_holding_one`. Four files arrived (`spline_incidence.rs`, `implicit_chart.rs`, `boolean/holdings.rs`, `viewer/matetool.rs`), and `ssi/plane.rs` is gone.
- `deny_unknown_fields_census::the_walk_still_sees_every_attribute_site`.

These run in the slow set only (nightly), so no PR gate saw them. The fix is to re-point each ledger entry and walk at the tree as it stands, after checking that each arrival is held to its declaration.
