---
id: an-unknown-edit-tag-in-a-log-refuses-without-naming-it
kind: issue
title: An unknown edit tag in a saved log refuses Unreadable without naming the tag: LoggedEdit's untagged wrapper swallows the variant error
status: open
opened: 2026-09-20
refs: [a-committed-profile-program-has-no-whole-program-edit]
---

## The finding

`PersistError::Unreadable`'s contract (`crates/editor-core/src/persist/mod.rs`)
is that `detail` is "the deserializer's own words, naming the
vocabulary it could not place", and the module docs promise a NEWER
document carrying a tag this build lacks refuses naming it. That holds
for a node in the snapshot (`unreadable_by_this_build::an_unknown_variant_refuses_naming_it`
mutates a node tag and reads the name in `detail`). It does NOT hold
for an edit in the LOG. Measured by
`edit_set_program::the_persisted_spelling_is_pinned_and_a_build_without_it_refuses_typed`
(`crates/editor-core/tests/edit_set_program.rs`): a save whose log
holds a `SetProgram` entry, with the tag re-spelled `SetProgramme`,
refuses `Unreadable` with

```
data did not match any variant of untagged enum Wire at line 474 column 3
```

— the line is the entry's, the arm is right, and the tag is nowhere in
the sentence.

## Why

`LoggedEdit`'s `Deserialize` (`crates/editor-core/src/edit.rs`) reads
an entry through a `#[serde(untagged)] enum Wire { WithRows(..),
Bare(DocEdit) }`, so an entry either shape refuses is reported by
serde as the untagged enum's miss, which discards both inner errors.
The `DocEdit` deserializer's own "unknown variant `SetProgramme`,
expected one of …" is produced and thrown away.

## What it costs

The recourse the arm exists to carry ("regenerate from a build that
knows the vocabulary") stands, but a person reading the refusal of a
file written by a NEWER build cannot tell from `detail` which edit the
file carries that theirs cannot — the one fact `Unreadable` was named
to give them. The first persisted `SetProgram` (this unit's
`reshaped_rod`) is the first file an older build will meet this on.

## A repair

Replace the untagged `Wire` with a hand-written visitor that tries the
bare `DocEdit` first and, on a map with an `edit` key, the rows shape —
or deserialize into a `serde_json::Value`-free intermediate that keeps
the inner error — so the miss reported is `DocEdit`'s own. The
persisted bytes do not move. The row is EDIT's (`persist/*`); PORT
announces on `persist/wire.rs` for the load door's structured refusals
and should be told when this lands.
