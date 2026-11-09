---
id: four-hand-rolled-scope-readers-predate-source-scope
kind: issue
title: Four hand-rolled scope readers predate source-scope
status: open
opened: 2026-11-09
---

Four guards answer *what does this name mean here* by reading text, from
before `crates/source-scope` was the home for that question. The
`reader_census` scope ledger
(`crates/test-utils/tests/reader_census.rs`, `SCOPE_LEDGER`) lists each one
as `HandRolled` against this row, and its count
(`HAND_ROLLED_SCOPE_READERS_TODAY`) only goes down.

- `crates/pncad-py/src/prose_census.rs:352` `use_bindings` and `:564`
  `normalize`. These resolve a name through the file's `use` items and
  `crate`/`self`/`super` prefixes. The doc says a `use` inside an inline
  `mod` or a fn body is credited to the whole file, which widens the
  result.
- `crates/pncad-py/src/tests.rs:7341` `parse_tag_body`. It reads the
  `use … as R;` shorthands a fn opens with.
- `crates/pncad/tests/all.rs:1733` `use_statement_roots`. It finds the
  roots of a file's `use` statements by word-scanning for `use`.
- `crates/test-utils/tests/hand_written_impl_census.rs:398`
  `self_aliases`. It follows `let me = self;` exactly one `let` deep. Its
  own doc says a match-arm, closure or argument binding is not followed.

**Fix.** Move each one onto `source_scope::read`, either `Source::uses`
or the `Bound` an event resolves to. Then change its ledger line to
`Parsed` and lower the count. `test-utils` has to stay zero-dependency,
so `hand_written_impl_census.rs` needs a dev-dependency on source-scope
from test-utils' test graph. Check first that this adds no cycle, since
source-scope dev-depends on test-utils.
