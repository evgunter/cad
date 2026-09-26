---
id: dangling-two-lanes-argument-is-restated-outside-danglingref
kind: issue
title: The Dangling two-lanes argument is restated in pncad and pncad-py instead of pointing at DanglingRef
status: open
opened: 2026-09-26
priority: P4
cost: E
---


## Finding

Found by S-DUP's stale-vs-foreign fold (2026-09-26, `dup/b6-c`) on its
repo-wide pass. It is off that unit's fence and on this slate because
both code sites sit in `lib`'s territory.

`topo::readback::DanglingRef`'s rustdoc is the home of one argument:
a read-back's two `Dangling` lanes are different facts about the model.
An entity key that does not resolve is a stale (or foreign) handle, and
a geometry key reached from a live entity that does not resolve is a
corrupt body. Two `lib` doors restate that argument in their own words
instead of pointing at `DanglingRef`:

- `crates/pncad/src/select.rs`, the rustdoc on the
  `pub use topo::readback::{DanglingRef, Pose, ReadbackError}`
  re-export (~:126-135): *"a topological key that does not resolve is a
  stale or foreign handle, while a geometry key reached from a live
  entity that does not resolve is a dangling reference inside the
  body"*.
- `crates/pncad-py/src/tags.rs`, the doc above the read-back tag
  function (~:2329-2335): the same sentence, with the two tag names
  interleaved.

The fix is a pointer at each (`[`DanglingRef`]`, which both crates can
reach), keeping each door's own reason: the re-export's reason is that
the refusal is matchable through the curated list, and the tags' reason
is that a caller branches on which invariant broke.

**Not members:** `crates/pncad-py/pncad.pyi` (~:735) is the Python stub,
whose readers cannot follow a Rust link, so it restates the argument
legitimately. `crates/topo/tests/display_contract.rs`'s
`readback_error_display_names_its_content_not_its_struct` doc is a test
naming what it pins. `topo/src/readback.rs`'s own `Display` comment
restated the argument too, and was folded onto `DanglingRef` in the
unit that filed this row.

**Instruments**, over every tracked file with no path argument, at
`032999ff2`. First `git grep -n -i 'stale or foreign handle'`, which
finds the three single-line spellings. Then, for the spellings that
wrap, `git grep -n -i -E 'stale or$'` (`display_contract.rs`, plus two
unrelated hits in `topo/src/shell.rs` and `viewer/src/pickindex.rs`) and
`'foreign handle'` (`readback.rs`). **Blind spot**: a restatement that
uses neither phrase.
