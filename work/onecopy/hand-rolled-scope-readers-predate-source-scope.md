---
id: hand-rolled-scope-readers-predate-source-scope
kind: issue
title: Five hand-rolled scope readers predate source-scope
status: open
opened: 2026-11-09
---

Five guards answer *what does this name mean here* by reading text, from
before `crates/source-scope` was the home for that question. The
`reader_census` scope ledger
(`crates/test-utils/tests/reader_census.rs`, `SCOPE_LEDGER`) lists each one
as `HandRolled` against this row, keyed by item (`path::fn`) in
`HAND_ROLLED_AT_THE_HOMES_ARRIVAL`. That record holds the readers as
they stood when the home landed. The census refuses an entry no line
holds, which makes a converted reader leave, and
`scripts/gates/scope-readers-only-shrink.sh` refuses an entry the merge
base does not hold.

- `crates/pncad-py/src/prose_census.rs:352` `use_bindings` and `:564`
  `normalize`. These resolve a name through the file's `use` items and
  `crate`/`self`/`super` prefixes. The doc says a `use` inside an inline
  `mod` or a fn body is credited to the whole file, which widens the
  result.
- `crates/pncad-py/src/tests.rs:7341` `parse_tag_body`. It reads the
  `use … as R;` shorthands a fn opens with.
- `crates/pncad/tests/all.rs:1733` `use_statement_roots`. It finds the
  roots of a file's `use` statements by word-scanning for `use`.
- `crates/geom-core/tests/bounds_census.rs:620` `is_sole_bracket`. It
  decides whether a bound names the bracket door by comparing its
  spelling with `Bounds`, `geom_core::Bounds` and `crate::Bounds`, so it
  misses an aliased import. The keyword needle cannot see this shape, so
  its ledger line is held by the fn's name (`Seen::Anchor`).
- `crates/test-utils/tests/hand_written_impl_census.rs:398`
  `self_aliases`. It follows `let me = self;` exactly one `let` deep. Its
  own doc says a match-arm, closure or argument binding is not followed.

**Fix.** Move each one onto `source_scope::read`, either `Source::uses`
or the `Bound` an event resolves to. Then change its ledger line to
`Parsed` and drop its `path::fn` entries from the record. `test-utils` has to stay zero-dependency,
so `hand_written_impl_census.rs` needs a dev-dependency on source-scope
from test-utils' test graph. Check first that this adds no cycle, since
source-scope dev-depends on test-utils.
