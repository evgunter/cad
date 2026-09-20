---
id: tone-doc-argues-from-a-site-that-now-reads-the-value
kind: issue
title: frame::Tone's doc argues from the Features pane's comment, which is now a read of Tone
status: open
opened: 2026-09-19
---



Found by `tone-is-a-value-in-frame-and-a-comment-in-two-panes`'s lane,
which could not fix it: `crates/viewer/src/frame.rs` is serialized
across several VNEWS rows and that lane was fenced out of the file.

`Tone`'s own doc (`crates/viewer/src/frame.rs`, the `pub enum Tone`
header, ~`:1162-1170`) argues the rule from a site it did not reach:

> The Features pane argues it explicitly for rows — a poisoned row is
> deliberately QUIET so the eye goes to the failed row a reader can do
> something about — and until this type existed no value stated it, so
> four badges each picked a colour at the call site and the rule lived
> only in prose.

Both halves are now false of the tree. The Features pane does not argue
it: `tree::RowStatus::tone()` states it and `pane::features` reads the
value. And the rule no longer "lives only in prose" anywhere — the
sentence is an account of the state before the badge family became a
value, kept in the present tense.

**What it should say instead is the argument, not its history**: a
poisoned row is `Advisory` because it points at the row that owns the
failure, and the tone is what makes the eye go to that one row. The
citation for the rule is `RowStatus::tone()`, which is a second
producer of `Tone` outside this module and the first one the type's
own text does not know about.

Ride it on whichever VNEWS unit next holds `frame.rs`.
