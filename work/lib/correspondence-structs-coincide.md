---
id: correspondence-structs-coincide
kind: issue
title: BlendVerb and ShellVerb coincide field for field: the generic correspondence waits for its third instance
status: open
opened: 2026-09-08
---

`crates/editor-core/src/verbs/blend.rs`'s `BlendVerb<T>` and
`crates/editor-core/src/verbs/shell.rs`'s `ShellVerb<T>` coincide
field for field — `build: fn(Vec<Key>, T) -> Verb<T>`, `emitter`,
`record: fn(VerbRecord<T>) -> Option<Rec>`, `slots: SlotJoin`,
`foreign_record` — differing only in the key type (`EdgeKey` /
`FaceKey`), the record type (`Option<BlendNaming>` / `ShellNaming`)
and the blends' `selection_label`/`no_records`. LIB-G17's fix pass
shared the small half (one `SlotJoin` type, one `feed_scalar_join`)
and deliberately did NOT write `Correspondence<T, Key, Rec>`: the spec
kept `blend.rs` closed for a reason (its module docs record the cost
of a shared match), two instances is where a generic is speculation,
and the third one-body-in verb is what would pay for it. When it
lands, the generic's fields are the five above, the per-verb
differences are the type parameters and two literals, and `wire_blend`
/ `wire_shell` become one lowering over it. Recorded from R1's S1/S2
and R2's Q1 on PR 2150.
