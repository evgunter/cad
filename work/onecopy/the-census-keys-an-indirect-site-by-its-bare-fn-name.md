---
id: the-census-keys-an-indirect-site-by-its-bare-fn-name
kind: issue
title: The predicate census keys an indirect site by its bare enclosing fn name, so a second site in another impl's fn of that name folds into the first
status: open
opened: 2026-11-17
priority: P3
cost: M
---


Found by the review of PR 7057 (SECTARMS, the main-red fix pass, 2026-11-17).

`test_utils::source::predicate_census` (`crates/test-utils/src/source.rs`, its `enclosing` closure) keys each indirect site as `<file>: <fn>: <payload>`. `<fn>` is the innermost `fn` item's bare name from `fn_items`, with no `impl` type or trait qualifier. The keys go into a set. Two sites in the same file, in different impls' fns of one name, and with the same payload, therefore read as one key.

The instance: geom-brep's roster (`crates/geom-brep/tests/recourse_roster.rs`, `INDIRECT`) lists `certify.rs: of: spec` and `certify.rs: turn: spec` for `decide_turn(spec, band)` in `ReadSpec::of` and `ReadSpec::turn`. `crates/geom-brep/src/certify.rs` has four `fn of` and three `fn turn`. A new `decide*(spec, …)` site in any of the others would fold into the listed key and leave the roster green. It would be an unlisted indirect site, which is the case the roster exists to red.

Keying by `impl <Type>::<fn>` changes every method-enclosed key in all six rosters that run the census (topo, geom-brep, geom-core, editor-core, sweep, profile). It is the same shape as `the-census-keys-no-closure-forwarder-by-its-binding`, and the two would naturally land together.
